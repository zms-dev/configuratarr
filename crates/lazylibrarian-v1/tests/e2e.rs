//! End-to-end tests against a live LazyLibrarian.
//!
//! Drive the real apply engine (connect → reconcile → write) exactly as the CLI
//! does. Guarded by `LAZYLIBRARIAN_URL` + `LAZYLIBRARIAN_API_KEY`; `#[ignore]` by
//! default.
//!
//! Run inside the e2e dev shell (starts LazyLibrarian, exports the env vars):
//!   nix develop .#e2e-lazylibrarian --command \
//!     cargo nextest run -p lazylibrarian-v1 --test e2e --run-ignored all -j1
//!
//! Runtime assumptions validated here:
//!   1. auth — the api key is accepted as the `?apikey=` query parameter
//!      (`Auth::ApiKeyQuery`), including on the `/api?cmd=getVersion` health probe;
//!   2. the query-dispatch write primitive (`get_query`) reaches every command;
//!   3. the custom seams converge — newznab/torznab upsert by `DISPNAME`,
//!      rss/irc/gen + magazines + authors create-only, config diffs per-variable
//!      via readCFG/writeCFG.
//!
//! The e2e instance persists state across runs, so assertions check
//! **idempotency** (a second apply makes no writes), not `created == 1`.

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use lazylibrarian_v1::LazyLibrarianV1;
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("LAZYLIBRARIAN_URL", "LAZYLIBRARIAN_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<LazyLibrarianV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("lazylibrarian healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// Apply `cfg` until it **converges**, then assert stability. Some LazyLibrarian
/// resources take two applies to settle — e.g. `addProvider` does not persist a
/// newznab's `enabled`, so the create leaves drift that the `changeProvider`
/// update fixes on the next apply. A correct config-sync resource must reach a
/// fixed point and stay there, so we apply three times and assert the last is a
/// pure no-op.
async fn assert_idempotent(url: &str, key: &str, cfg: Value) {
    run(url, key, cfg.clone(), ApplyOptions::default()).await; // create
    run(url, key, cfg.clone(), ApplyOptions::default()).await; // converge
    let last = run(url, key, cfg, ApplyOptions::default()).await; // stable
    assert_eq!(last.created, 0, "not converged — still creating: {last:?}");
    assert_eq!(last.updated, 0, "not converged — still updating: {last:?}");
    assert!(
        last.unchanged >= 1,
        "converged apply reported nothing unchanged: {last:?}"
    );
}

/// `wait_healthy` — `/api?cmd=getVersion` answers OK with the api key in the query
/// string, confirming the `Auth::ApiKeyQuery` seam end to end.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<LazyLibrarianV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("lazylibrarian should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Magazine create-only: added once, then unchanged.
#[tokio::test]
#[ignore]
async fn magazine_create_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "magazines": [{ "title": "CfgE2eMagazine" }] }),
    )
    .await;
}

/// Author create-only: added once, then unchanged. Uses a real author name —
/// `addAuthor` resolves the name against the book API and stores the canonical
/// form, so identity only round-trips for a name the source actually knows
/// (a fictional name never lands in `getIndex`). Network-dependent.
#[tokio::test]
#[ignore]
async fn author_create_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "authors": [{ "name": "Terry Pratchett" }] }),
    )
    .await;
}

/// Newznab provider upsert: created/updated to match, then unchanged.
#[tokio::test]
#[ignore]
async fn newznab_provider_upsert_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "newznab": [{
            "dispname": "cfg-e2e-newznab",
            "enabled": true,
            "host": "https://newznab.example.com",
            "api": "cfg-e2e-key",
            "book_cat": "7000,7020",
        }] }),
    )
    .await;
}

/// Torznab provider upsert: created/updated to match, then unchanged.
#[tokio::test]
#[ignore]
async fn torznab_provider_upsert_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "torznab": [{
            "dispname": "cfg-e2e-torznab",
            "enabled": true,
            "host": "https://torznab.example.com",
            "api": "cfg-e2e-key",
            "seeders": 1,
        }] }),
    )
    .await;
}

/// A **disabled** newznab provider (`enabled: false`) must converge: `false`
/// reads back as the empty string, so the string-normalised `in_sync` has to
/// treat `""` as false — otherwise it would churn forever.
#[tokio::test]
#[ignore]
async fn disabled_provider_converges() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "newznab": [{
            "dispname": "cfg-e2e-newznab-off",
            "enabled": false,
            "host": "https://off.example.com",
            "api": "k",
        }] }),
    )
    .await;
}

/// Direct/generic provider create-only (rss/irc/gen can't be updated by name).
#[tokio::test]
#[ignore]
async fn direct_provider_create_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "direct": [{
            "dispname": "cfg-e2e-direct",
            "host": "https://direct.example.com",
            "dl_types": "E",
        }] }),
    )
    .await;
}

/// RSS provider upsert: created/updated to match, then unchanged.
#[tokio::test]
#[ignore]
async fn rss_provider_upsert_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "rss": [{
            "dispname": "cfg-e2e-rss",
            "enabled": true,
            "host": "https://example.com/rss",
            "dl_types": "E",
        }] }),
    )
    .await;
}

/// Config singleton: General + a download-client section (qBittorrent) + the
/// torrent enable toggle are written via writeCFG, then re-applies read them back
/// equal and write nothing — proving download-client config converges per-variable.
#[tokio::test]
#[ignore]
async fn config_set_idempotent() {
    let Some((url, key)) = env() else { return };
    assert_idempotent(
        &url,
        &key,
        json!({ "config": {
            "general": { "match_ratio": 77 },
            // metadata-source config (the API section)
            "api": { "book_api": "GoogleBooks", "gr_sync": false },
            // post-processing dest folder pattern
            "postprocess": { "ebook_dest_folder": "$Author/$Title" },
            // a notifier section
            "pushover": { "use_pushover": false, "keys": "cfg-e2e" },
            // download client + its enable toggle
            "torrent": { "tor_downloader_qbittorrent": true, "keep_seeding": true },
            "qbittorrent": {
                "host": "http://localhost",
                "port": 8080,
                "user": "admin",
                "pass": "adminpass",
                "label": "lazylibrarian",
            },
        } }),
    )
    .await;
}
