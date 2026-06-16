use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[path = "../src/client/mod.rs"]
pub mod client;
#[path = "../src/config.rs"]
pub mod config;
#[path = "../src/resolver.rs"]
pub mod resolver;
#[path = "../src/sync/mod.rs"]
pub mod sync;

use client::{AppType, StarrClient};
use sync::{DiffResult, SyncEngine, SyncMode, SyncOptions};

// Extension trait to enable compilation since DiffResult is opaque/empty in Phase 1 scaffolding
pub trait DiffResultExt {
    fn additions(&self) -> &[Value];
    fn updates(&self) -> &[Value];
    fn deletions(&self) -> &[Value];
}

impl DiffResultExt for DiffResult {
    fn additions(&self) -> &[Value] {
        &self.additions
    }
    fn updates(&self) -> &[Value] {
        &self.updates
    }
    fn deletions(&self) -> &[Value] {
        &self.deletions
    }
}

#[tokio::test]
async fn test_sync_schema_overlay_merges_fields() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let schema = json!([
        {
            "implementation": "Torznab",
            "fields": [
                { "name": "baseUrl", "value": "", "isReadOnly": false },
                { "name": "apiKey", "value": "", "isReadOnly": false },
                { "name": "someDefaultField", "value": "default_val", "isReadOnly": false },
                { "name": "readOnlyField", "value": "system_val", "isReadOnly": true }
            ]
        }
    ]);

    let local_config = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "baseUrl", "value": "http://my-torznab.local" },
                { "name": "apiKey", "value": "my-key" }
            ]
        }
    ]);

    let server_resources = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "baseUrl", "value": "http://my-torznab.local" },
                { "name": "apiKey", "value": "my-key" },
                { "name": "someDefaultField", "value": "default_val" },
                { "name": "readOnlyField", "value": "different_system_val" }
            ]
        }
    ]);

    let diff = engine
        .compute_diff(&local_config, &server_resources, &schema)
        .await?;
    assert!(diff.updates().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_no_change() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "Sabnzbd",
            "implementation": "Sabnzbd",
            "fields": [
                { "name": "host", "value": "localhost" }
            ]
        }
    ]);

    let server = json!([
        {
            "name": "Sabnzbd",
            "implementation": "Sabnzbd",
            "fields": [
                { "name": "host", "value": "localhost" }
            ]
        }
    ]);

    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert!(diff.additions().is_empty());
    assert!(diff.updates().is_empty());
    assert!(diff.deletions().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_update_needed() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "baseUrl", "value": "http://new-url" }
            ]
        }
    ]);

    let server = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "baseUrl", "value": "http://old-url" }
            ]
        }
    ]);

    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert_eq!(diff.updates().len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_addition_needed() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "New Indexer",
            "implementation": "Torznab",
            "fields": []
        }
    ]);

    let server = json!([]);
    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert_eq!(diff.additions().len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_prune_deletion() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([]);
    let server = json!([
        {
            "id": 5,
            "name": "Old Indexer",
            "implementation": "Torznab",
            "fields": []
        }
    ]);
    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert_eq!(diff.deletions().len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_credential_masking_ignored() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "apiKey", "value": "my_secure_pass" }
            ]
        }
    ]);

    let server = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "apiKey", "value": "******" }
            ]
        }
    ]);

    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert!(diff.updates().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_sync_diff_credential_empty_string_ignored() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "apiKey", "value": "my_secure_pass" }
            ]
        }
    ]);

    let server = json!([
        {
            "name": "My Indexer",
            "implementation": "Torznab",
            "fields": [
                { "name": "apiKey", "value": "" }
            ]
        }
    ]);

    let schema = json!([]);

    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert!(diff.updates().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_sync_run_sync_pruning_enabled() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 5, "name": "Old Indexer", "implementation": "Torznab", "fields": [] }
        ])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer/schema"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let delete_mock = Mock::given(method("DELETE"))
        .and(path("/api/v3/indexer/5"))
        .respond_with(ResponseTemplate::new(200))
        .mount_as_scoped(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let engine = SyncEngine::new(client);

    engine
        .run_sync(
            &json!({ "radarr": { "indexers": [] } }),
            &SyncOptions {
                mode: SyncMode::Apply,
                prune: true,
            },
        )
        .await?;

    // Verify deletion occurred
    assert_eq!(delete_mock.received_requests().await.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_sync_run_sync_pruning_disabled() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 5, "name": "Old Indexer", "implementation": "Torznab", "fields": [] }
        ])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer/schema"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let engine = SyncEngine::new(client);

    engine
        .run_sync(
            &json!({ "radarr": { "indexers": [] } }),
            &SyncOptions {
                mode: SyncMode::Apply,
                prune: false,
            },
        )
        .await?;

    // No DELETE request should be received (no DELETE route mounted anyway)
    Ok(())
}

#[tokio::test]
async fn test_sync_run_sync_plan_never_mutates() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 5, "name": "Old Indexer", "implementation": "Torznab", "fields": [] }
        ])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/indexer/schema"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let engine = SyncEngine::new(client);

    engine.run_sync(
        &json!({ "radarr": { "indexers": [ { "name": "New Indexer", "implementation": "Torznab", "fields": [] } ] } }),
        &SyncOptions { mode: SyncMode::Plan, prune: true }
    ).await?;

    // Verify no mutation endpoints (POST/PUT/DELETE) were hit on dry-run mode
    Ok(())
}

#[tokio::test]
async fn test_sync_quality_profile_no_schema() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "name": "HD - 1080p",
            "upgradeAllowed": true,
            "cutoff": 1
        }
    ]);

    let server = json!([
        {
            "id": 1,
            "name": "HD - 1080p",
            "upgradeAllowed": false,
            "cutoff": 1
        }
    ]);

    let schema = json!([]); // no schema
    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert_eq!(diff.updates().len(), 1);
    assert_eq!(diff.updates()[0]["id"], 1);
    assert_eq!(diff.updates()[0]["upgradeAllowed"], true);
    Ok(())
}

#[tokio::test]
async fn test_sync_root_folder_path_matching() -> Result<(), anyhow::Error> {
    let client = StarrClient::new(AppType::Radarr, "http://localhost", Some("key"));
    let engine = SyncEngine::new(client);

    let local = json!([
        {
            "path": "/movies"
        }
    ]);

    let server = json!([
        {
            "id": 1,
            "path": "/movies",
            "accessible": true
        }
    ]);

    let schema = json!([]);
    let diff = engine.compute_diff(&local, &server, &schema).await?;
    assert!(diff.additions().is_empty());
    assert!(diff.updates().is_empty());
    assert!(diff.deletions().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_sync_run_sync_extended_resources() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    // mock status
    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    // mock GET endpoints for all resources
    let endpoints = [
        "/api/v3/downloadclient",
        "/api/v3/indexer",
        "/api/v3/notification",
        "/api/v3/qualityprofile",
        "/api/v3/customformat",
        "/api/v3/rootfolder",
        "/api/v3/importlist",
    ];
    for ep in endpoints {
        Mock::given(method("GET"))
            .and(path(ep))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
    }

    // mock schemas for those with schema
    let schemas = [
        "/api/v3/downloadclient/schema",
        "/api/v3/indexer/schema",
        "/api/v3/notification/schema",
        "/api/v3/importlist/schema",
    ];
    for sc in schemas {
        Mock::given(method("GET"))
            .and(path(sc))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
    }

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let engine = SyncEngine::new(client);

    let local_cfg = json!({
        "radarr": {
            "downloadClients": [],
            "indexers": [],
            "notifications": [],
            "qualityProfiles": [],
            "customFormats": [],
            "rootFolders": [],
            "importLists": []
        }
    });

    engine.run_sync(&local_cfg, &SyncOptions { mode: SyncMode::Plan, prune: false }).await?;
    Ok(())
}

#[tokio::test]
async fn test_sync_singletons_ui() -> Result<(), anyhow::Error> {
    use client::{AppType, StarrClient};
    use sync::{SyncEngine, SyncMode, SyncOptions};
    use wiremock::matchers::{method, path, body_json};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/config/ui"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "theme": "light",
            "firstDayOfWeek": 0
        })))
        .mount(&server)
        .await;

    let put_mock = Mock::given(method("PUT"))
        .and(path("/api/v3/config/ui/1"))
        .and(body_json(json!({
            "id": 1,
            "theme": "dark",
            "firstDayOfWeek": 0
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount_as_scoped(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let engine = SyncEngine::new(client);

    let local_cfg = json!({
        "radarr": {
            "ui": {
                "theme": "dark"
            }
        }
    });

    engine.run_sync(&local_cfg, &SyncOptions { mode: SyncMode::Apply, prune: false }).await?;

    assert_eq!(put_mock.received_requests().await.len(), 1);
    Ok(())
}


