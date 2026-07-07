//! `/api/feeds` — RSS/Torznab/Newznab feeds. A feed is the polling half of a
//! feed-type indexer: autobrr's UI creates the indexer and the feed as two calls,
//! and the feed carries an `indexer_id` back to its parent indexer.
//!
//! `sync = custom`, keyed by `name`, create + update by id (`POST /api/feeds`,
//! `PUT /api/feeds/{id}`); no prune — matching the other autobrr resources.
//! `api_key`/`cookie` are redacted on read, so idempotency is a structural-subset
//! test on the readable fields ([`crate::resources::filter::is_subset`]).
//!
//! FK: `indexer_id` → the managed feed-type indexer this feed polls
//! (`${ref.indexer.<name>}`). A feed is meaningless without its indexer, so
//! declare the indexer too and point the feed at it.

use core_lib::engine;
use core_lib::{
    Change, ChangeKind, CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue,
};
use core_macros::resource;
use serde_json::Value;

use crate::resources::filter::is_subset;

/// `/api/feeds` — a configured feed.
#[resource(sync = custom, case = snake, list = get("/api/feeds"))]
pub struct Feed {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.feed.<name>}`).
    #[key]
    pub name: String,
    /// The feed-type indexer this feed polls (`${ref.indexer.<name>}`).
    #[reference(indexer)]
    pub indexer_id: Option<i32>,
    /// Feed protocol: `TORZNAB`, `NEWZNAB`, or `RSS`.
    #[wire(name = "type")]
    pub feed_type: String,
    /// Whether the feed is active.
    pub enabled: Option<bool>,
    /// Feed URL.
    pub url: Option<String>,
    /// Poll interval, minutes.
    pub interval: Option<i32>,
    /// Per-request timeout, seconds.
    pub timeout: Option<i32>,
    /// Ignore items older than this many minutes (`0` = no limit).
    pub max_age: Option<i32>,
    /// Torznab/Newznab category ids to fetch.
    pub categories: Vec<i32>,
    /// API key for the feed source, where required.
    pub api_key: Option<SecretValue>,
    /// Cookie to send with feed requests (private trackers).
    pub cookie: Option<SecretValue>,
    /// Skip TLS certificate verification.
    pub tls_skip_verify: Option<bool>,
}

impl CustomSync for Feed {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/feeds").await?;
            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let name = cfg
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("feed entry is missing `name`"))?;
                // Full desired wire (refs already resolved to ids; id is read-only).
                let wire = engine::encode(&engine::decode_config::<Self>(cfg)?)?;

                let existing = live
                    .iter()
                    .find(|f| f.get("name").and_then(Value::as_str) == Some(name));

                let kind = match existing {
                    Some(live_feed) => {
                        // autobrr echoes the FK only nested (`indexer.id`), never as
                        // a top-level `indexer_id`, so lift it up for the subset diff
                        // — otherwise the resolved `indexer_id` would re-update forever.
                        let mut live_norm = live_feed.clone();
                        if let (Some(obj), Some(iid)) = (
                            live_norm.as_object_mut(),
                            live_feed.get("indexer").and_then(|i| i.get("id")).cloned(),
                        ) {
                            obj.insert("indexer_id".to_string(), iid);
                        }
                        if is_subset(&wire, &live_norm) {
                            ChangeKind::Unchanged
                        } else {
                            let id = live_feed.get("id").cloned().unwrap_or(Value::Null);
                            if execute {
                                let _: Value =
                                    client.put(&format!("/api/feeds/{id}"), &wire).await?;
                            }
                            ChangeKind::Updated
                        }
                    }
                    None => {
                        if execute {
                            let _: Value = client.post("/api/feeds", &wire).await?;
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
