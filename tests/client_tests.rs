use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[path = "../src/client/mod.rs"]
pub mod client;
#[path = "../src/config.rs"]
pub mod config;
#[path = "../src/resolver.rs"]
pub mod resolver;

mod common;
use common::StarrMockServer;

use client::{AppType, StarrClient};

#[tokio::test]
async fn test_client_sends_x_api_key_header() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .and(header("X-Api-Key", "my-secret-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("my-secret-api-key"));
    client.check_status().await?;
    Ok(())
}

#[tokio::test]
async fn test_client_login_cookie_capture() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/login"))
        .respond_with(ResponseTemplate::new(200).insert_header("Set-Cookie", "sid=mock_cookie_val"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .and(header("Cookie", "sid=mock_cookie_val"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), None);
    client.login().await?;
    client.check_status().await?;
    Ok(())
}

#[tokio::test]
async fn test_client_login_failure() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/login"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), None);
    let result = client.login().await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_client_wait_for_status_immediate_success() -> Result<(), anyhow::Error> {
    let mock_server = StarrMockServer::start().await;
    mock_server.mock_status("/api/v3", "Radarr", 200).await;

    // Use all StarrMockServer helper methods to prevent dead_code warning before Phase 3 implementation
    if false {
        mock_server.mock_login(200).await;
        mock_server.mock_get("", json!(null)).await;
        mock_server.mock_post("", json!(null), json!(null)).await;
        mock_server.mock_put("", json!(null), json!(null)).await;
        mock_server.mock_delete("").await;
    }

    let client = StarrClient::new(AppType::Radarr, &mock_server.url(), Some("key"));
    client.wait_for_status(5).await?;
    Ok(())
}

#[tokio::test]
async fn test_client_wait_for_status_retries_success() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    let call_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(move |_req: &wiremock::Request| {
            let count = call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count == 0 {
                ResponseTemplate::new(503)
            } else {
                ResponseTemplate::new(200).set_body_json(json!({
                    "appName": "Radarr",
                    "version": "1.0.0"
                }))
            }
        })
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    client.wait_for_status(5).await?;
    Ok(())
}

#[tokio::test]
async fn test_client_wait_for_status_timeout() -> Result<(), anyhow::Error> {
    let mock_server = StarrMockServer::start().await;
    mock_server.mock_status("/api/v3", "Radarr", 503).await;

    let client = StarrClient::new(AppType::Radarr, &mock_server.url(), Some("key"));
    let result = client.wait_for_status(1).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_client_resolves_correct_api_prefix() -> Result<(), anyhow::Error> {
    let server = MockServer::start().await;

    // Radarr uses /api/v3
    Mock::given(method("GET"))
        .and(path("/api/v3/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Radarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    // Lidarr uses /api/v1
    Mock::given(method("GET"))
        .and(path("/api/v1/system/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "appName": "Lidarr",
            "version": "1.0.0"
        })))
        .mount(&server)
        .await;

    let radarr_client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let lidarr_client = StarrClient::new(AppType::Lidarr, &server.uri(), Some("key"));

    radarr_client.check_status().await?;
    lidarr_client.check_status().await?;
    Ok(())
}
