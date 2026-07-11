//! End-to-end tests against a live Jellyfin.
//!
//! Drive the real apply engine (connect → GET live → plan → merge → POST),
//! exactly as the CLI does. Guarded by `JELLYFIN_URL` + `JELLYFIN_API_KEY`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Jellyfin, exports the env vars):
//!   nix develop .#e2e-jellyfin --command \
//!     cargo nextest run -p jellyfin-v11 --test e2e --run-ignored all -j1
//!
//! NOTE: this is where the two Jellyfin runtime assumptions are validated:
//!   1. auth — the API key is accepted in the `Authorization` header;
//!   2. casing — the server serialises/accepts camelCase JSON by default.

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use jellyfin_v11::JellyfinV11;
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("JELLYFIN_URL", "JELLYFIN_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<JellyfinV11>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("jellyfin healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: `/System/Info/Public` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<JellyfinV11>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("jellyfin should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// An empty singleton config is a no-op — presence-masking must not clobber the
/// server's live settings with type defaults.
#[tokio::test]
#[ignore]
async fn singleton_empty_config_noop() {
    let Some((url, key)) = env() else { return };
    let report = run(
        &url,
        &key,
        json!({ "branding_options": {} }),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "empty singleton must no-op: {report:?}"
    );
}

/// Bulk-replace round-trip: set the plugin repository list, then re-apply
/// idempotently (the whole list is replaced in one POST; second apply no-ops).
#[tokio::test]
#[ignore]
async fn repositories_bulk_replace_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({
        "repositories": [
            {
                "name": "configuratarr-e2e",
                "url": "https://example.invalid/manifest.json",
                "enabled": true
            }
        ]
    });

    // First apply sets the list (updated, or already matching from a prior run).
    let _ = run(&url, &key, desired.clone(), ApplyOptions::default()).await;

    // Re-apply identical desired state: no write.
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second bulk-replace apply must be a no-op: {r2:?}"
    );
}

/// Custom-sync round-trip: declare an API key by app name; created on first
/// apply, a no-op on re-apply (create-or-leave, no prune).
#[tokio::test]
#[ignore]
async fn auth_key_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({ "auth_keys": [ { "app": "configuratarr-e2e" } ] });

    // First apply creates the key (or it's already present from a prior run).
    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "expected the api key present after apply: {r1:?}"
    );

    // Re-apply identical desired state: the key exists, so no write.
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second custom-sync apply must be a no-op: {r2:?}"
    );
}

/// Multi-endpoint custom sync: create a user (GUID id) and set policy bits, then
/// re-apply idempotently. Exercises `/Users/New` + `/Users/{guid}/Policy` and the
/// string-id ref path.
#[tokio::test]
#[ignore]
async fn user_create_policy_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({
        "users": [
            {
                "name": "configuratarr-e2e",
                "password": "Configuratarr-e2e-pw1",
                "is_administrator": false,
                "is_disabled": false,
                "enable_content_deletion": true
            }
        ]
    });

    // First apply creates the user + applies policy (or already present).
    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.updated + r1.unchanged >= 1,
        "expected the user present after apply: {r1:?}"
    );

    // Re-apply identical desired state: user exists, policy matches → no-op.
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second user apply must be a no-op: {r2:?}"
    );
}

/// Custom sync with query-param create: add a media library, then re-apply
/// idempotently (create-or-leave, keyed by name).
#[tokio::test]
#[ignore]
async fn library_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({
        "libraries": [
            {
                "name": "configuratarr-e2e",
                "collection_type": "movies",
                "paths": ["/tmp/configuratarr-e2e-media"]
            }
        ]
    });

    // First apply creates the library (or it's already present).
    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "expected the library present after apply: {r1:?}"
    );

    // Re-apply identical desired state: no write.
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second library apply must be a no-op: {r2:?}"
    );
}

/// Singleton update round-trip: set a branding field, then re-apply idempotently.
#[tokio::test]
#[ignore]
async fn branding_update_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({
        "branding_options": { "login_disclaimer": "configuratarr e2e" }
    });

    // First apply may update (or already match from a prior run).
    let _ = run(&url, &key, desired.clone(), ApplyOptions::default()).await;

    // Re-apply identical desired state: no-op.
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second apply must be a no-op: {r2:?}"
    );
}
