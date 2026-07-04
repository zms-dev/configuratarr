//! `/api/filters` — the flagship resource: release-matching filters.
//!
//! autobrr creates a filter in **two steps**: `POST /api/filters` accepts only
//! the small `FilterCreate` subset (name + a few list fields), returns the new
//! id, and the full ~40-field config is then applied via
//! `PATCH /api/filters/{id}`. A single `crud` POST would ship a half-configured
//! filter, so this is a `sync = custom` hook that owns the two-step create, the
//! update (PATCH), and the change detection. No prune (the custom seam carries no
//! `prune` flag).
//!
//! **Idempotency.** Live filters carry server-added fields — an `id`, and ids on
//! nested `actions` — that the desired config never sets. So "changed?" is a
//! **structural subset** test ([`is_subset`]): the filter is in sync when every
//! declared key/value already matches live (extra server keys are ignored),
//! rather than a merge-equality that server ids would always trip.

use core_lib::engine;
use core_lib::{Change, ChangeKind, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Map, Value, json};

use crate::resources::action::Action;
use crate::resources::external_filter::ExternalFilter;
use crate::resources::filter_indexer::FilterIndexer;

/// Array keys the `POST /api/filters` (`FilterCreate`) body **must** carry —
/// autobrr stores them `NOT NULL`, so they're forced to `[]` when the config
/// omits them. (`name`/`enabled` are copied straight from the wire.)
const CREATE_ARRAY_KEYS: &[&str] = &["resolutions", "codecs", "sources", "containers", "origins"];

/// `/api/filters` — a release-matching filter.
#[resource(sync = custom, case = snake, list = get("/api/filters"))]
pub struct Filter {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Filter name — its identity.
    #[key]
    pub name: String,
    /// Whether the filter is active.
    pub enabled: Option<bool>,
    /// Evaluation priority (higher wins).
    pub priority: Option<i32>,
    /// Treat match/except patterns as regular expressions.
    pub use_regex: Option<bool>,
    /// Match release years (e.g. `2020-2024`).
    pub years: Option<String>,
    /// Match seasons (e.g. `1,3-5`).
    pub seasons: Option<String>,
    /// Match episodes.
    pub episodes: Option<String>,
    /// Releases must match these terms.
    pub match_releases: Option<String>,
    /// Releases must not match these terms.
    pub except_releases: Option<String>,
    /// Match these release groups.
    pub match_release_groups: Option<String>,
    /// Exclude these release groups.
    pub except_release_groups: Option<String>,
    /// Match these categories.
    pub match_categories: Option<String>,
    /// Exclude these categories.
    pub except_categories: Option<String>,
    /// Match these tags.
    pub tags: Option<String>,
    /// Exclude these tags.
    pub except_tags: Option<String>,
    /// Logic for `tags` (`ANY` / `ALL`).
    pub tags_match_logic: Option<String>,
    /// Logic for `except_tags` (`ANY` / `ALL`).
    pub except_tags_match_logic: Option<String>,
    /// Smarter episode/season handling.
    pub smart_episode: Option<bool>,
    /// Match these resolutions.
    pub resolutions: Vec<String>,
    /// Match these video codecs.
    pub codecs: Vec<String>,
    /// Match these sources.
    pub sources: Vec<String>,
    /// Match these containers.
    pub containers: Vec<String>,
    /// Match these HDR formats.
    pub match_hdr: Vec<String>,
    /// Exclude these HDR formats.
    pub except_hdr: Vec<String>,
    /// Match these "other" tags (REPACK, PROPER, …).
    pub match_other: Vec<String>,
    /// Exclude these "other" tags.
    pub except_other: Vec<String>,
    /// Match these languages.
    pub match_language: Vec<String>,
    /// Exclude these languages.
    pub except_language: Vec<String>,
    /// Match these audio formats.
    pub formats: Vec<String>,
    /// Match these audio qualities.
    pub quality: Vec<String>,
    /// Match these media types.
    pub media: Vec<String>,
    /// Match these release types.
    pub match_release_types: Vec<String>,
    /// Match these origins (INTERNAL, SCENE, …).
    pub origins: Vec<String>,
    /// Exclude these origins.
    pub except_origins: Vec<String>,
    /// Minimum release size (e.g. `1GB`).
    pub min_size: Option<String>,
    /// Maximum release size.
    pub max_size: Option<String>,
    /// Delay before pushing, seconds.
    pub delay: Option<i32>,
    /// Indexers this filter is attached to.
    pub indexers: Vec<FilterIndexer>,
    /// Actions run on a matched release.
    pub actions: Vec<Action>,
    /// External (webhook/exec) checks.
    pub external: Vec<ExternalFilter>,
}

/// The `FilterCreate` body: `name` + `enabled` from the wire, plus the five
/// required array fields forced to `[]` when absent (autobrr rejects a NULL
/// there). The rest of the config is applied by the follow-up PATCH.
fn create_body(wire: &Value) -> Value {
    let obj = wire.as_object();
    let mut out = Map::new();
    for k in ["name", "enabled"] {
        if let Some(v) = obj.and_then(|o| o.get(k)) {
            out.insert(k.to_string(), v.clone());
        }
    }
    for k in CREATE_ARRAY_KEYS {
        let v = obj
            .and_then(|o| o.get(*k))
            .cloned()
            .unwrap_or_else(|| json!([]));
        out.insert((*k).to_string(), v);
    }
    Value::Object(out)
}

/// An empty declared value — one that can't meaningfully differ from a server
/// default (autobrr returns unset lists as `null`/absent and unset scalars as
/// `""`). Treated as always in-sync, since the typed `Vec`/`Option` fields can't
/// distinguish "declared empty" from "omitted".
fn is_empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

/// True when every value in `want` is already present (structurally) in `have`:
/// an empty declared value is always satisfied; objects match key-by-key on
/// `want`'s keys (extra `have` keys — e.g. server ids — ignored); arrays match
/// element-wise; scalars compare numeric-insensitively.
fn is_subset(want: &Value, have: &Value) -> bool {
    if is_empty(want) {
        return true;
    }
    match (want, have) {
        (Value::Object(w), Value::Object(h)) => w
            .iter()
            .all(|(k, wv)| is_empty(wv) || h.get(k).is_some_and(|hv| is_subset(wv, hv))),
        (Value::Array(w), Value::Array(h)) => {
            w.len() == h.len() && w.iter().zip(h).all(|(wv, hv)| is_subset(wv, hv))
        }
        _ => match (want.as_f64(), have.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => want == have,
        },
    }
}

impl CustomSync for Filter {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/filters").await?;
            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let name = cfg
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("filter entry is missing `name`"))?;
                // Full desired wire (id is read-only, so absent here).
                let wire = engine::encode(&engine::decode_config::<Self>(cfg)?)?;

                // The list endpoint returns a *trimmed* filter (no match/except
                // fields), so match by name here but fetch the full object by id
                // for the diff.
                let existing_id = live
                    .iter()
                    .find(|f| f.get("name").and_then(Value::as_str) == Some(name))
                    .and_then(|f| f.get("id").cloned());

                let kind = match existing_id {
                    Some(id) => {
                        let full: Value = client.get(&format!("/api/filters/{id}")).await?;
                        if is_subset(&wire, &full) {
                            // Already in sync — nothing to do.
                            ChangeKind::Unchanged
                        } else {
                            // Drifted — PATCH the full config by id.
                            if execute {
                                let _: Value =
                                    client.patch(&format!("/api/filters/{id}"), &wire).await?;
                            }
                            ChangeKind::Updated
                        }
                    }
                    // Absent — two-step create: POST the subset, then PATCH the rest.
                    None => {
                        if execute {
                            let created: Value =
                                client.post("/api/filters", &create_body(&wire)).await?;
                            let id = created.get("id").cloned().unwrap_or(Value::Null);
                            let _: Value =
                                client.patch(&format!("/api/filters/{id}"), &wire).await?;
                        }
                        ChangeKind::Created
                    }
                };

                changes.push(match kind {
                    ChangeKind::Created => Change::created(name),
                    ChangeKind::Updated => Change::updated(name),
                    ChangeKind::Unchanged => Change::unchanged(name),
                });
            }

            Ok(changes)
        })
    }
}
