use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiField {
    pub name: String,
    pub value: Value,
}

/// Base fields common to all dynamic Starr resource collections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResourceEnvelope {
    pub name: String,
    pub implementation: String,
    pub config_contract: String,
    pub fields: Vec<ApiField>,
}

/// Download Clients have a top-level `enable` flag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientEnvelope {
    #[serde(flatten)]
    pub base: BaseResourceEnvelope,
    pub enable: bool,
    pub priority: i32,
    pub protocol: String,
    pub remove_completed_downloads: bool,
    pub remove_failed_downloads: bool,
}

/// Indexers do NOT have a top-level enable toggle, but rather capability toggles
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerEnvelope {
    #[serde(flatten)]
    pub base: BaseResourceEnvelope,
    pub enable_rss: bool,
    pub enable_automatic_search: bool,
    pub enable_interactive_search: bool,
    pub priority: i32,
    pub protocol: String,
}

/// Notifications/Connections do NOT have a top-level enable toggle
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationEnvelope {
    #[serde(flatten)]
    pub base: BaseResourceEnvelope,
}

/// Host properties matching /config/host parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommonHostConfig {
    pub url: String,
    pub api_key: Option<String>,
}

/// Configuration namespace grouped by application
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub host: Option<CommonHostConfig>,
    pub download_clients: Option<Vec<DownloadClientEnvelope>>,
    pub indexers: Option<Vec<IndexerEnvelope>>,
    pub notifications: Option<Vec<NotificationEnvelope>>,
    pub quality_profiles: Option<Vec<Value>>,
    pub custom_formats: Option<Vec<Value>>,
    pub root_folders: Option<Vec<Value>>,
    pub import_lists: Option<Vec<Value>>,
    pub metadata_profiles: Option<Vec<Value>>,
    pub release_profiles: Option<Vec<Value>>,
    pub ui: Option<Value>,
    pub naming: Option<Value>,
    pub media_management: Option<Value>,
}

/// Master configuration structure loading from YAML
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub radarr: Option<AppConfig>,
    pub sonarr: Option<AppConfig>,
    pub prowlarr: Option<AppConfig>,
    pub lidarr: Option<AppConfig>,
    pub readarr: Option<AppConfig>,
}

/// Custom value parser for Clap key=value pairs
pub fn parse_key_val(s: &str) -> Result<(String, Value), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;

    let key = s[..pos].trim().to_string();
    let val_str = s[pos + 1..].trim();

    if key.is_empty() {
        return Err("KEY cannot be empty".to_string());
    }

    // Attempt to parse as typed JSON (boolean, numbers, arrays, objects)
    let value =
        serde_json::from_str(val_str).unwrap_or_else(|_| Value::String(val_str.to_string()));

    Ok((key, value))
}

/// Parses a YAML string, resolves placeholders at the AST level, and deserializes to Config
pub fn load_config_from_str(s: &str) -> Result<Config, anyhow::Error> {
    let mut raw_val: Value = serde_saphyr::from_str(s)
        .map_err(|e| anyhow::anyhow!("Failed to parse YAML configuration: {}", e))?;

    if raw_val.is_null() {
        return Ok(Config::default());
    }

    // Resolve secrets recursively inside the JSON Value AST
    crate::resolver::resolve_secrets(&mut raw_val)?;

    // Attempt deserialization into the final typed configuration struct
    let config: Config = serde_json::from_value(raw_val)
        .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;

    Ok(config)
}

/// Loads and validates configuration from a file path
pub fn load_config_from_file(path: &str) -> Result<Config, anyhow::Error> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file '{}': {}", path, e))?;
    load_config_from_str(&content)
}
