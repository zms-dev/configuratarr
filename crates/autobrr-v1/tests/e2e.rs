//! End-to-end tests against a live autobrr.
//!
//! Drive the real apply engine (connect → GET live → plan/reconcile → write),
//! exactly as the CLI does. Guarded by `AUTOBRR_URL` + `AUTOBRR_API_KEY`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts autobrr, onboards a user, mints a key,
//! exports the env vars):
//!   nix develop .#e2e-autobrr --command \
//!     cargo nextest run -p autobrr-v1 --test e2e --run-ignored all -j1
//!
//! Runtime assumptions validated here:
//!   1. auth — the API key is accepted in the `X-API-Token` header;
//!   2. casing — autobrr serialises/accepts snake_case JSON;
//!   3. the custom seams are idempotent — `create_only` (api keys,
//!      notifications), the `filter` two-step create (POST subset → PATCH full),
//!      the `indexer` create/update (base id on create, stored id on update), and
//!      the `irc_network` structural-subset diff;
//!   4. `proxy` crud round-trips and prunes.

use std::time::Duration;

use autobrr_v1::AutobrrV1;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("AUTOBRR_URL", "AUTOBRR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<AutobrrV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("autobrr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: `/api/config` answers OK (and confirms
/// the token).
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("autobrr should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Create-only api key: created on the first apply, unchanged on the second.
#[tokio::test]
#[ignore]
async fn api_key_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "api_keys": [{ "name": "cfg-e2e", "scopes": [] }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second api-key apply should create nothing"
    );
    assert_eq!(second.unchanged, 1);
}

/// Create-only notification: created once, then unchanged.
#[tokio::test]
#[ignore]
async fn notification_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "notifications": [{
        "name": "cfg-e2e-discord",
        "notification_type": "DISCORD",
        "enabled": true,
        "events": ["PUSH_APPROVED"],
        "webhook": "https://discord.com/api/webhooks/1/abc",
    }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(second.unchanged, 1);
}

/// Download client: created on the first apply, unchanged on the second (crud
/// diff makes a repeated apply a no-op).
#[tokio::test]
#[ignore]
async fn download_client_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "download_clients": [{
        "name": "cfg-e2e-qbit",
        "client_type": "QBITTORRENT",
        "enabled": true,
        "host": "http://localhost",
        "port": 8080,
        "username": "admin",
        "password": "adminadmin",
    }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second dl-client apply should create nothing"
    );
}

/// Filter two-step create (POST subset → PATCH full), then a repeated apply is a
/// no-op — validates both the two-step create and the structural subset diff.
#[tokio::test]
#[ignore]
async fn filter_two_step_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "filters": [{
        "name": "cfg-e2e-1080p",
        "enabled": true,
        "priority": 5,
        "resolutions": ["1080p"],
        "codecs": ["x265"],
        "match_release_types": ["MOVIE"],
    }] });
    let first = run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    assert_eq!(first.created, 1, "filter should be created");
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(second.updated, 0, "second filter apply should be a no-op");
    assert_eq!(second.unchanged, 1);
}

/// Full-coverage guard: a filter carrying the newly-modelled scalar/array fields
/// and a fully-specified action, plus a dedup profile attached by
/// `${ref.release_profile_duplicate.<name>}`. Conformance can't catch a field
/// autobrr rejects at runtime (the `base_url` lesson), so this asserts the live
/// API accepts every new field, that the dedup id lands, and that it's idempotent.
#[tokio::test]
#[ignore]
async fn filter_full_fields_and_dedup_ref() {
    use core_lib::Service;

    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "download_clients": [{
            "name": "cfg-e2e-qbit2", "client_type": "QBITTORRENT", "enabled": true,
            "host": "http://localhost", "port": 8080,
        }],
        "release_profile_duplicates": [{
            "name": "cfg-e2e-dedup",
            "title": true, "year": true, "resolution": true, "group": true, "proper": true,
        }],
        "filters": [{
            "name": "cfg-e2e-full",
            "enabled": true,
            "priority": 5,
            "resolutions": ["1080p"],
            "announce_types": ["NEW"],
            "scene": false,
            "freeleech": true,
            "freeleech_percent": "100",
            "max_downloads": 5,
            "max_downloads_unit": "DAY",
            "min_seeders": 2,
            "max_seeders": 500,
            "match_description": "internal",
            "log": false,
            "release_profile_duplicate_id": "${ref.release_profile_duplicate.cfg-e2e-dedup}",
            "actions": [{
                "name": "qb", "action_type": "QBITTORRENT", "enabled": true,
                "client_id": "${ref.download_client.cfg-e2e-qbit2}",
                "category": "movies", "content_layout": "ORIGINAL", "priority": "max",
                "limit_ratio": 2.0, "limit_seed_time": 1440, "limit_upload_speed": 1000,
                "skip_hash_check": true, "paused": false,
            }],
        }],
    });

    // Live must accept every new field (an unknown/mistyped one 500s in apply).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The dedup ref resolved and landed as the profile's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let dups: Vec<Value> = client
        .get("/api/release/profiles/duplicate")
        .await
        .expect("list dedup");
    let dup_id = dups
        .iter()
        .find(|d| d.get("name").and_then(Value::as_str) == Some("cfg-e2e-dedup"))
        .and_then(|d| d.get("id").and_then(Value::as_i64))
        .expect("dedup present");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-full"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    assert_eq!(
        full.get("release_profile_duplicate_id")
            .and_then(Value::as_i64),
        Some(dup_id),
        "dedup id attached: {full:?}"
    );

    // Idempotent — the subset diff over the new fields must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// Cross-reference a custom-sync producer: a filter attaches an `indexer` by
/// `${ref.indexer.<name>}`. Proves the engine now exports custom-sync ids into
/// the RefStore (an unresolved ref would fail `apply`), that autobrr stores the
/// resolved numeric id (`indexers[0].id`), and that it settles idempotently (a
/// dropped attachment would perpetually re-update).
#[tokio::test]
#[ignore]
async fn filter_attaches_indexer_by_ref() {
    use core_lib::Service;

    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "indexers": [{
            "name": "cfg-e2e-refidx",
            "identifier": "torznab",
            "implementation": "torznab",
            "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
        }],
        "filters": [{
            "name": "cfg-e2e-reffilter",
            "enabled": true,
            "indexers": [{ "id": "${ref.indexer.cfg-e2e-refidx}" }],
        }],
    });

    // A successful apply already proves the ref resolved (an unresolved
    // `${ref.indexer.*}` errors out in `run`).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The attachment must land server-side as the indexer's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx_id = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-refidx"))
        .and_then(|i| i.get("id").and_then(Value::as_i64))
        .expect("indexer present");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-reffilter"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    let attached: Vec<i64> = full["indexers"]
        .as_array()
        .map(|a| a.iter().filter_map(|i| i["id"].as_i64()).collect())
        .unwrap_or_default();
    assert!(
        attached.contains(&idx_id),
        "filter should have indexer {idx_id} attached, got {attached:?}"
    );

    // Idempotent: a dropped attachment would re-update forever.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// Feed create referencing its indexer by `${ref.indexer.<name>}`: proves the
/// feed→indexer FK resolves out of the RefStore, autobrr stores the resolved
/// numeric `indexer_id`, and a repeated apply is a no-op (the redacted `api_key`
/// must not perpetually re-update via the structural-subset diff).
#[tokio::test]
#[ignore]
async fn feed_references_indexer_idempotent() {
    use core_lib::Service;

    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "indexers": [{
            "name": "cfg-e2e-feedidx",
            "identifier": "torznab",
            "implementation": "torznab",
            "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
        }],
        "feeds": [{
            "name": "cfg-e2e-feed",
            "indexer_id": "${ref.indexer.cfg-e2e-feedidx}",
            "feed_type": "TORZNAB",
            "enabled": true,
            "url": "https://tracker.example.org/rss",
            "interval": 15,
        }],
    });

    // Successful apply proves the ref resolved (an unresolved one errors in `run`).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The feed's indexer_id landed as the indexer's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx_id = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-feedidx"))
        .and_then(|i| i.get("id").and_then(Value::as_i64))
        .expect("indexer present");
    let feeds: Vec<Value> = client.get("/api/feeds").await.expect("list feeds");
    let feed = feeds
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-feed"))
        .expect("feed present");
    assert_eq!(
        feed.get("indexer")
            .and_then(|i| i.get("id"))
            .and_then(Value::as_i64),
        Some(idx_id),
        "feed indexer attached: {feed:?}"
    );

    // Idempotent — the subset diff over the redacted api_key must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.updated, 0,
        "second feed apply is a no-op: {second:?}"
    );
}

/// List create referencing a filter by `${ref.filter.<name>}`: an external
/// (`PLAINTEXT`) list needs a `url` + at least one filter. Proves the
/// list→filter FK resolves, the attachment lands, and a repeated apply is a
/// no-op (the redacted `api_key` must not perpetually re-update).
#[tokio::test]
#[ignore]
async fn list_references_filter_idempotent() {
    use core_lib::Service;

    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "filters": [{
            "name": "cfg-e2e-listfilter",
            "enabled": true,
            "resolutions": ["1080p"],
        }],
        // `enabled: false` so autobrr doesn't fetch titles from `url` on
        // create/update — the e2e VM is hermetic (no DNS), and the fetch is the
        // only network dependency; the filter attachment + diff don't need it.
        "lists": [{
            "name": "cfg-e2e-list",
            "list_type": "PLAINTEXT",
            "enabled": false,
            "url": "https://example.org/list.txt",
            "filters": [{ "id": "${ref.filter.cfg-e2e-listfilter}" }],
        }],
    });

    // Successful apply proves the filter ref resolved.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The filter attachment landed server-side as the filter's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-listfilter"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let lists: Vec<Value> = client.get("/api/lists").await.expect("list lists");
    let list = lists
        .iter()
        .find(|l| l.get("name").and_then(Value::as_str) == Some("cfg-e2e-list"))
        .expect("list present");
    let attached: Vec<i64> = list["filters"]
        .as_array()
        .map(|a| a.iter().filter_map(|f| f["id"].as_i64()).collect())
        .unwrap_or_default();
    assert!(
        attached.contains(&fid),
        "list should have filter {fid} attached, got {attached:?}"
    );

    // Idempotent — the subset diff over the redacted api_key must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.updated, 0,
        "second list apply is a no-op: {second:?}"
    );
}

/// Proxy crud: created, then a repeated apply is a no-op (no secret sent, so the
/// merge is stable), then pruned via DELETE.
#[tokio::test]
#[ignore]
async fn proxy_crud_idempotent_prune() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "proxies": [{
        "name": "cfg-e2e-proxy",
        "enabled": true,
        "proxy_type": "SOCKS5",
        "addr": "socks5://127.0.0.1:1080",
    }] });
    // First apply may create or reconcile pre-existing state; the invariant is
    // that a *repeated* apply is a no-op (no secret sent → stable merge).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0, "second proxy apply creates nothing");
    assert_eq!(
        second.updated, 0,
        "second proxy apply is a no-op: {second:?}"
    );

    let pruned = run(
        &url,
        &key,
        json!({ "proxies": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(pruned.deleted >= 1, "proxy pruned: {pruned:?}");
}

/// Indexer custom sync: created, idempotent, then a toggled `enabled` drives an
/// update — which must echo the *server-stored* identifier (`torznab-<name>`);
/// sending the base id 500s. Regression guard for that update path.
#[tokio::test]
#[ignore]
async fn indexer_create_update_idempotent() {
    let Some((url, key)) = env() else { return };
    let mk = |enabled: bool| {
        json!({ "indexers": [{
        "name": "cfg-e2e-tz",
        "identifier": "torznab",
        "implementation": "torznab",
        "enabled": enabled,
        "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
    }] })
    };

    // Settle to enabled=true (create or reconcile pre-existing), then a repeated
    // apply must be a no-op.
    run(&url, &key, mk(true), ApplyOptions::default()).await;
    let second = run(&url, &key, mk(true), ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.unchanged, 1,
        "second indexer apply is a no-op: {second:?}"
    );

    // Toggle enabled → update via the stored identifier (the 500-prone path).
    let updated = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(updated.updated, 1, "indexer updated: {updated:?}");
    let settled = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(
        settled.unchanged, 1,
        "update settles to unchanged: {settled:?}"
    );
}

/// An `irc`-implementation indexer round-trips with a top-level `base_url` and
/// its IRC login in `settings`. autobrr rejects an IRC indexer whose `base_url`
/// is empty (`indexer baseURL must not be empty`), so a clean create+idempotent
/// apply proves configuratarr can produce a valid IRC indexer.
#[tokio::test]
#[ignore]
async fn irc_indexer_with_base_url() {
    use core_lib::Service;

    let Some((url, key)) = env() else { return };
    let cfg = json!({ "indexers": [{
        "name": "cfg-e2e-tl",
        "identifier": "torrentleech",
        "implementation": "irc",
        "base_url": "https://www.torrentleech.org",
        "settings": { "nick": "cfg_e2e_bot", "auth.account": "cfg", "auth.password": "pw" },
    }] });

    // A missing base_url would 500 inside apply; a clean apply proves it's sent.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // base_url persisted server-side.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-tl"))
        .expect("indexer present");
    assert_eq!(
        idx.get("base_url").and_then(Value::as_str),
        Some("https://www.torrentleech.org"),
        "base_url persisted: {idx:?}"
    );

    // Idempotent (base_url is now in the readable diff).
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// IRC network custom sync: the structural-subset diff makes a repeated apply a
/// no-op despite server-enriched `channels[]` + runtime fields; a toggled
/// `enabled` drives a `PUT /api/irc/network/{id}` update.
#[tokio::test]
#[ignore]
async fn irc_network_create_update_idempotent() {
    let Some((url, key)) = env() else { return };
    let mk = |enabled: bool| {
        json!({ "irc_networks": [{
        "name": "cfg-e2e-net",
        "enabled": enabled,
        "server": "irc.example.org",
        "port": 6697,
        "tls": true,
        "nick": "cfg-e2e-bot",
        "channels": [{ "name": "#announce" }],
    }] })
    };
    // Settle (create or reconcile pre-existing), then a repeated apply is a no-op.
    run(&url, &key, mk(true), ApplyOptions::default()).await;
    let second = run(&url, &key, mk(true), ApplyOptions::default()).await;
    assert_eq!(second.created, 0, "second irc apply creates nothing");
    assert_eq!(
        second.unchanged, 1,
        "second irc apply is a no-op: {second:?}"
    );

    // Toggle enabled → update via /api/irc/network/{id}.
    let updated = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(updated.updated, 1, "irc network updated: {updated:?}");
    let settled = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(
        settled.unchanged, 1,
        "update settles to unchanged: {settled:?}"
    );
}
