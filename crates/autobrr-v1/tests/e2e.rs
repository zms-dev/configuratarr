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
//!   3. the two custom seams — `create_only` (api keys, notifications) and the
//!      `filter` two-step create (POST subset → PATCH full) — are idempotent.

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
