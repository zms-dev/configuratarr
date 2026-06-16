#[path = "../src/cli.rs"]
pub mod cli;
#[path = "../src/client/mod.rs"]
pub mod client;
#[path = "../src/config.rs"]
pub mod config;
#[path = "../src/resolver.rs"]
pub mod resolver;
#[path = "../src/sync/mod.rs"]
pub mod sync;

use clap::Parser;

#[test]
fn test_cli_mutating_sync_without_modes_fails() {
    let res = cli::Cli::try_parse_from(["configuratarr", "sync"]);
    assert!(res.is_err());
}

#[test]
fn test_cli_add_parse_ok() {
    let res = cli::Cli::try_parse_from([
        "configuratarr",
        "radarr",
        "download-client",
        "add",
        "--field",
        "name=test",
    ]);
    assert!(res.is_ok());
}

#[test]
fn test_cli_delete_parse_ok() {
    let res = cli::Cli::try_parse_from([
        "configuratarr",
        "radarr",
        "download-client",
        "delete",
        "test",
    ]);
    assert!(res.is_ok());
}

#[test]
fn test_cli_update_parse_ok() {
    let res = cli::Cli::try_parse_from([
        "configuratarr",
        "radarr",
        "download-client",
        "update",
        "--field",
        "name=test",
    ]);
    assert!(res.is_ok());
}

#[test]
fn test_cli_conflicting_plan_apply_fails() {
    let res = cli::Cli::try_parse_from(["configuratarr", "sync", "--plan", "--apply"]);
    assert!(res.is_err());
}

#[test]
fn test_cli_plan_mode_ok() {
    let res = cli::Cli::try_parse_from(["configuratarr", "sync", "--plan"]);
    assert!(res.is_ok());
}

#[test]
fn test_cli_apply_mode_ok() {
    let res = cli::Cli::try_parse_from(["configuratarr", "sync", "--apply"]);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_cli_global_wait_timeout() -> Result<(), anyhow::Error> {
    use client::{AppType, StarrClient};
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/api/v3/system/status"))
        .respond_with(wiremock::ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let client = StarrClient::new(AppType::Radarr, &server.uri(), Some("key"));
    let res = client.wait_for_status(2).await;
    assert!(res.is_err());
    Ok(())
}
