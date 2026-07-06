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
    /// Cap the number of grabs.
    pub max_downloads: Option<i32>,
    /// Window the `max_downloads` cap applies over: `HOUR`, `DAY`, `WEEK`,
    /// `MONTH`, or `EVER`.
    pub max_downloads_unit: Option<String>,
    /// Match these announce types (`NEW`, `PROMO`, `PROMO_GP`, `RESURRECTED`).
    pub announce_types: Vec<String>,
    /// Match only scene releases.
    pub scene: Option<bool>,
    /// Match these bonus/reward tags (tracker-specific).
    pub bonus: Vec<String>,
    /// Match only freeleech releases.
    pub freeleech: Option<bool>,
    /// Match these freeleech percentages (e.g. `50,100`).
    pub freeleech_percent: Option<String>,
    /// Match these show/title terms.
    pub shows: Option<String>,
    /// Match months (e.g. `1,6-8`).
    pub months: Option<String>,
    /// Match days.
    pub days: Option<String>,
    /// Match these artists (music).
    pub artists: Option<String>,
    /// Match these albums (music).
    pub albums: Option<String>,
    /// Exclude these release types.
    pub except_release_types: Option<String>,
    /// Match only perfect FLAC (music).
    pub perfect_flac: Option<bool>,
    /// Require a CUE file (music).
    pub cue: Option<bool>,
    /// Require a log (music).
    pub log: Option<bool>,
    /// Minimum log score (music).
    pub log_score: Option<i32>,
    /// Match these uploaders.
    pub match_uploaders: Option<String>,
    /// Exclude these uploaders.
    pub except_uploaders: Option<String>,
    /// Match these record labels (music).
    pub match_record_labels: Option<String>,
    /// Exclude these record labels (music).
    pub except_record_labels: Option<String>,
    /// Match if the release carries any of these tags.
    pub tags_any: Option<String>,
    /// Exclude if the release carries any of these tags.
    pub except_tags_any: Option<String>,
    /// Match these release tags.
    pub match_release_tags: Option<String>,
    /// Exclude these release tags.
    pub except_release_tags: Option<String>,
    /// Treat release-tag patterns as regular expressions.
    pub use_regex_release_tags: Option<bool>,
    /// Match these terms in the release description.
    pub match_description: Option<String>,
    /// Exclude these terms in the release description.
    pub except_description: Option<String>,
    /// Treat description patterns as regular expressions.
    pub use_regex_description: Option<bool>,
    /// Minimum seeders.
    pub min_seeders: Option<i32>,
    /// Maximum seeders.
    pub max_seeders: Option<i32>,
    /// Minimum leechers.
    pub min_leechers: Option<i32>,
    /// Maximum leechers.
    pub max_leechers: Option<i32>,
    /// Dedup profile to apply (`${ref.release_profile_duplicate.<name>}`).
    #[reference(release_profile_duplicate)]
    pub release_profile_duplicate_id: Option<i32>,
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
pub(crate) fn is_empty(v: &Value) -> bool {
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
pub(crate) fn is_subset(want: &Value, have: &Value) -> bool {
    if is_empty(want) {
        return true;
    }
    // autobrr stores an unset nilable `*bool` as null, so a declared `false`
    // reads back as null (or false). Treat those as equal. Non-nullable bools
    // (e.g. irc `enabled`) always read back as a real boolean, never null, so a
    // genuine `true → false` toggle still diffs and triggers an update.
    if want == &Value::Bool(false) && (have.is_null() || have == &Value::Bool(false)) {
        return true;
    }
    match (want, have) {
        // A key absent from `have` is compared as null, so an empty/`false`
        // declared value (which autobrr drops on write) still counts as in sync.
        (Value::Object(w), Value::Object(h)) => w
            .iter()
            .all(|(k, wv)| is_subset(wv, h.get(k).unwrap_or(&Value::Null))),
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
