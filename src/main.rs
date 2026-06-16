mod cli;
mod client;
mod config;
mod resolver;
mod sync;

use crate::cli::{AppSubcommands, Cli, Commands, ResourceSubcommands, SingletonSubcommands};
use crate::client::{AppType, StarrClient, resolve_connection};
use crate::sync::{SyncEngine, SyncMode, SyncOptions};
use clap::Parser;
use serde_json::Value;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let cli_url = cli.url.as_deref();
    let cli_api_key = cli.api_key.as_deref();

    match &cli.command {
        Commands::Sync(sync_cmd) => {
            let config = crate::config::load_config_from_file(&sync_cmd.config)?;
            let mut apps = Vec::new();
            if let Some(ref r) = config.radarr {
                apps.push((AppType::Radarr, r));
            }
            if let Some(ref s) = config.sonarr {
                apps.push((AppType::Sonarr, s));
            }
            if let Some(ref p) = config.prowlarr {
                apps.push((AppType::Prowlarr, p));
            }
            if let Some(ref l) = config.lidarr {
                apps.push((AppType::Lidarr, l));
            }
            if let Some(ref r) = config.readarr {
                apps.push((AppType::Readarr, r));
            }

            if apps.is_empty() {
                println!("No applications configured for synchronization.");
                return Ok(());
            }

            for (app_type, app_config) in apps {
                let (url, api_key) =
                    resolve_connection(app_type, Some(app_config), cli_url, cli_api_key)?;
                let client = StarrClient::new(app_type, &url, api_key.as_deref());

                if api_key.is_none() {
                    client.login().await?;
                }

                if cli.wait {
                    println!("Waiting for {} to become online...", app_type.env_prefix());
                    client.wait_for_status(cli.wait_timeout).await?;
                }

                let mode = if sync_cmd.plan_apply.apply {
                    SyncMode::Apply
                } else {
                    SyncMode::Plan
                };
                let options = SyncOptions {
                    mode,
                    prune: sync_cmd.prune,
                };
                let engine = SyncEngine::new(client);
                let config_val = serde_json::to_value(&config)?;

                println!("Syncing {} config...", app_type.env_prefix());
                engine.run_sync(&config_val, &options).await?;
            }
        }
        Commands::Radarr(app_cmd) => run_app(AppType::Radarr, app_cmd, &cli).await?,
        Commands::Sonarr(app_cmd) => run_app(AppType::Sonarr, app_cmd, &cli).await?,
        Commands::Prowlarr(app_cmd) => run_app(AppType::Prowlarr, app_cmd, &cli).await?,
        Commands::Lidarr(app_cmd) => run_app(AppType::Lidarr, app_cmd, &cli).await?,
        Commands::Readarr(app_cmd) => run_app(AppType::Readarr, app_cmd, &cli).await?,
    }

    Ok(())
}

async fn run_app(
    app_type: AppType,
    app_cmd: &crate::cli::AppCmd,
    cli: &Cli,
) -> Result<(), anyhow::Error> {
    let cli_url = cli.url.as_deref();
    let cli_api_key = cli.api_key.as_deref();

    let config_path = match &app_cmd.command {
        AppSubcommands::Sync(sync_cmd) => &sync_cmd.config,
        _ => "configuratarr.yaml",
    };
    let config = if std::path::Path::new(config_path).exists() {
        match crate::config::load_config_from_file(config_path) {
            Ok(c) => Some(c),
            Err(e) => {
                if matches!(app_cmd.command, AppSubcommands::Sync(_)) {
                    return Err(e);
                }
                None
            }
        }
    } else {
        None
    };

    let app_config = config.as_ref().and_then(|c| match app_type {
        AppType::Radarr => c.radarr.as_ref(),
        AppType::Sonarr => c.sonarr.as_ref(),
        AppType::Prowlarr => c.prowlarr.as_ref(),
        AppType::Lidarr => c.lidarr.as_ref(),
        AppType::Readarr => c.readarr.as_ref(),
    });

    let (url, api_key) = resolve_connection(app_type, app_config, cli_url, cli_api_key)?;
    let client = StarrClient::new(app_type, &url, api_key.as_deref());

    if api_key.is_none() {
        client.login().await?;
    }

    if cli.wait {
        println!("Waiting for {} to become online...", app_type.env_prefix());
        client.wait_for_status(cli.wait_timeout).await?;
    }

    match &app_cmd.command {
        AppSubcommands::Sync(sync_cmd) => {
            let local_config = crate::config::load_config_from_file(&sync_cmd.config)?;
            let mode = if sync_cmd.plan_apply.apply {
                SyncMode::Apply
            } else {
                SyncMode::Plan
            };
            let options = SyncOptions {
                mode,
                prune: sync_cmd.prune,
            };
            let engine = SyncEngine::new(client);
            let config_val = serde_json::to_value(&local_config)?;
            engine.run_sync(&config_val, &options).await?;
        }
        AppSubcommands::Status => {
            let status = client.check_status().await?;
            println!("{}", serde_json::to_string_pretty(&status)?);
        }
        AppSubcommands::DownloadClient(res_cmd) => {
            run_resource(client, "downloadclient", &res_cmd.command).await?
        }
        AppSubcommands::Indexer(res_cmd) => {
            run_resource(client, "indexer", &res_cmd.command).await?
        }
        AppSubcommands::RootFolder(res_cmd) => {
            run_resource(client, "rootfolder", &res_cmd.command).await?
        }
        AppSubcommands::QualityProfile(res_cmd) => {
            run_resource(client, "qualityprofile", &res_cmd.command).await?
        }
        AppSubcommands::CustomFormat(res_cmd) => {
            run_resource(client, "customformat", &res_cmd.command).await?
        }
        AppSubcommands::MetadataProfile(res_cmd) => {
            run_resource(client, "metadataprofile", &res_cmd.command).await?
        }
        AppSubcommands::ReleaseProfile(res_cmd) => {
            run_resource(client, "releaseprofile", &res_cmd.command).await?
        }
        AppSubcommands::Ui(sing_cmd) => {
            run_singleton(client, "config/ui", &sing_cmd.command).await?
        }
        AppSubcommands::Naming(sing_cmd) => {
            run_singleton(client, "config/naming", &sing_cmd.command).await?
        }
        AppSubcommands::MediaManagement(sing_cmd) => {
            run_singleton(client, "config/mediamanagement", &sing_cmd.command).await?
        }
    }

    Ok(())
}

async fn run_resource(
    client: StarrClient,
    endpoint: &str,
    cmd: &ResourceSubcommands,
) -> Result<(), anyhow::Error> {
    match cmd {
        ResourceSubcommands::List => {
            let list = client.get_resources(endpoint).await?;
            println!("{}", serde_json::to_string_pretty(&list)?);
        }
        ResourceSubcommands::Delete { name } => {
            let list = client.get_resources(endpoint).await?;
            let items = list
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Expected array of resources from server"))?;

            let matched_item = items
                .iter()
                .find(|i| i.get("name").and_then(|v| v.as_str()) == Some(name.as_str()));
            let item = matched_item
                .ok_or_else(|| anyhow::anyhow!("Resource '{}' not found on server", name))?;
            let id = item
                .get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow::anyhow!("Resource missing id field"))?
                as i32;

            client.delete_resource(endpoint, id).await?;
            println!(
                "{}",
                serde_json::to_string(&serde_json::json!({
                    "deleted": true,
                    "endpoint": endpoint,
                    "name": name,
                    "id": id
                }))?
            );
        }
        ResourceSubcommands::Add { field } => {
            let name = field
                .iter()
                .find(|(k, _)| k == "name")
                .and_then(|(_, v)| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("'name' field is required"))?;

            // Verify if resource already exists
            let list = client.get_resources(endpoint).await?;
            if list.as_array().is_some_and(|items| {
                items
                    .iter()
                    .any(|i| i.get("name").and_then(|v| v.as_str()) == Some(name))
            }) {
                return Err(anyhow::anyhow!("Resource '{}' already exists", name));
            }

            let payload = build_payload(&client, endpoint, field).await?;
            let res = client.create_resource(endpoint, payload).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        ResourceSubcommands::Update { field } => {
            let name = field
                .iter()
                .find(|(k, _)| k == "name")
                .and_then(|(_, v)| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("'name' field is required"))?;

            let list = client.get_resources(endpoint).await?;
            let items = list
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Expected array of resources from server"))?;

            let matched_item = items
                .iter()
                .find(|i| i.get("name").and_then(|v| v.as_str()) == Some(name));

            let mut payload = build_payload(&client, endpoint, field).await?;

            if let Some(item) = matched_item {
                let id = item
                    .get("id")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| anyhow::anyhow!("Resource missing id field"))?
                    as i32;

                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("id".to_string(), serde_json::json!(id));
                }

                let res = client.update_resource(endpoint, id, payload).await?;
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                let res = client.create_resource(endpoint, payload).await?;
                println!("{}", serde_json::to_string_pretty(&res)?);
            }
        }
    }
    Ok(())
}

async fn build_payload(
    client: &StarrClient,
    endpoint: &str,
    field: &[(String, Value)],
) -> Result<Value, anyhow::Error> {
    let implementation = field
        .iter()
        .find(|(k, _)| k == "implementation")
        .and_then(|(_, v)| v.as_str());

    let has_schema = endpoint == "downloadclient" || endpoint == "indexer";

    let payload = if has_schema {
        let impl_str = implementation.ok_or_else(|| {
            anyhow::anyhow!(
                "'implementation' field is required for dynamic resource type '{}'",
                endpoint
            )
        })?;
        let schema = client.get_schema(endpoint).await?;
        let schema_arr = schema
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Schema from server is not an array"))?;

        let schema_def = schema_arr
            .iter()
            .find(|s| s.get("implementation").and_then(|v| v.as_str()) == Some(impl_str))
            .ok_or_else(|| {
                anyhow::anyhow!("Implementation '{}' not found in server schemas", impl_str)
            })?;

        let mut resolved_fields = std::collections::HashMap::new();

        if let Some(fields) = schema_def.get("fields").and_then(|v| v.as_array()) {
            for f in fields {
                let is_ro = f
                    .get("isReadOnly")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if !is_ro {
                    let name_opt = f.get("name").and_then(|v| v.as_str());
                    let val_opt = f.get("value");
                    if let (Some(f_name), Some(f_val)) = (name_opt, val_opt) {
                        resolved_fields.insert(f_name.to_string(), f_val.clone());
                    }
                }
            }
        }

        for (k, v) in field.iter() {
            if k != "name" && k != "implementation" && k != "configContract" {
                let is_schema_field = schema_def
                    .get("fields")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .any(|f| f.get("name").and_then(|n| n.as_str()) == Some(k.as_str()))
                    })
                    .unwrap_or(false);
                if is_schema_field {
                    resolved_fields.insert(k.clone(), v.clone());
                }
            }
        }

        let mut payload_map = schema_def.as_object().cloned().unwrap_or_default();
        for (k, v) in field.iter() {
            if k == "name"
                || k == "implementation"
                || k == "configContract"
                || k == "enable"
                || k == "priority"
                || k == "protocol"
            {
                payload_map.insert(k.clone(), v.clone());
            }
        }

        let mut fields_list = Vec::new();
        if let Some(fields) = schema_def.get("fields").and_then(|v| v.as_array()) {
            for f in fields {
                if let Some(f_name) = f.get("name").and_then(|v| v.as_str()) {
                    let mut new_f = f.clone();
                    if let (Some(val), Some(obj)) =
                        (resolved_fields.get(f_name), new_f.as_object_mut())
                    {
                        obj.insert("value".to_string(), val.clone());
                    }
                    fields_list.push(new_f);
                }
            }
        }
        payload_map.insert("fields".to_string(), Value::Array(fields_list));
        Value::Object(payload_map)
    } else {
        let mut payload_map = serde_json::Map::new();
        for (k, v) in field.iter() {
            payload_map.insert(k.clone(), v.clone());
        }
        Value::Object(payload_map)
    };

    Ok(payload)
}

async fn run_singleton(
    client: StarrClient,
    endpoint: &str,
    cmd: &SingletonSubcommands,
) -> Result<(), anyhow::Error> {
    match cmd {
        SingletonSubcommands::Show => {
            let val = client.get_resources(endpoint).await?;
            println!("{}", serde_json::to_string_pretty(&val)?);
        }
        SingletonSubcommands::Update { field } => {
            let server_val = client.get_resources(endpoint).await?;
            let mut server_obj = server_val
                .as_object()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Expected object from endpoint '{}'", endpoint))?;

            for (k, v) in field {
                server_obj.insert(k.clone(), v.clone());
            }

            let res = client
                .update_resource(endpoint, 1, Value::Object(server_obj))
                .await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
    }
    Ok(())
}
