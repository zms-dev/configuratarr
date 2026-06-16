pub mod display;

use crate::client::{AppType, StarrClient};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    Plan,
    Apply,
}

#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub mode: SyncMode,
    pub prune: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DiffResult {
    pub additions: Vec<Value>,
    pub updates: Vec<Value>,
    pub deletions: Vec<Value>,
}

pub struct SyncEngine {
    pub client: StarrClient,
}

impl SyncEngine {
    pub fn new(client: StarrClient) -> Self {
        Self { client }
    }

    pub async fn compute_diff(
        &self,
        local_config: &Value,
        server_resources: &Value,
        schema: &Value,
    ) -> Result<DiffResult, anyhow::Error> {
        if !local_config.is_array() {
            return Err(anyhow::anyhow!("local_config is not an array"));
        }
        if !server_resources.is_array() {
            return Err(anyhow::anyhow!("server_resources is not an array"));
        }
        if !schema.is_array() {
            return Err(anyhow::anyhow!("schema is not an array"));
        }

        let local_arr = local_config
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("local_config is not an array"))?;
        let server_arr = server_resources
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("server_resources is not an array"))?;
        let schema_arr = schema
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("schema is not an array"))?;

        let mut diff = DiffResult::default();

        for l_val in local_arr {
            let Some(l) = l_val.as_object() else {
                continue;
            };

            let l_name = l.get("name").and_then(|v| v.as_str());
            let l_path = l.get("path").and_then(|v| v.as_str());

            if l_name.is_none() && l_path.is_none() {
                return Err(anyhow::anyhow!("Either 'name' or 'path' is required"));
            }

            let l_impl = l.get("implementation").and_then(|v| v.as_str());

            let schema_def = if let Some(impl_str) = l_impl {
                schema_arr
                    .iter()
                    .find(|s| s.get("implementation").and_then(|v| v.as_str()) == Some(impl_str))
            } else {
                None
            };

            let mut l_fields = std::collections::HashMap::new();

            let schema_fields = schema_def
                .and_then(|s| s.get("fields"))
                .and_then(|v| v.as_array());

            if let Some(fields) = schema_fields {
                for f_val in fields {
                    if let Some(f) = f_val.as_object() {
                        let is_ro = f
                            .get("isReadOnly")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        if !is_ro {
                            let f_name_opt = f.get("name").and_then(|v| v.as_str());
                            let f_val_opt = f.get("value");
                            if let (Some(name), Some(val)) = (f_name_opt, f_val_opt) {
                                l_fields.insert(name.to_string(), val.clone());
                            }
                        }
                    }
                }
            }

            if let Some(local_fields) = l.get("fields").and_then(|v| v.as_array()) {
                for f_val in local_fields {
                    if let Some(f) = f_val.as_object() {
                        let f_name_opt = f.get("name").and_then(|v| v.as_str());
                        let f_val_opt = f.get("value");
                        if let (Some(name), Some(val)) = (f_name_opt, f_val_opt) {
                            let is_readonly = schema_def
                                .and_then(|s| s.get("fields"))
                                .and_then(|v| v.as_array())
                                .and_then(|s_fields| {
                                    s_fields.iter().find(|sf| {
                                        sf.get("name").and_then(|n| n.as_str()) == Some(name)
                                    })
                                })
                                .and_then(|sf| sf.get("isReadOnly"))
                                .and_then(|ro| ro.as_bool())
                                .unwrap_or(false);

                            if !is_readonly {
                                l_fields.insert(name.to_string(), val.clone());
                            }
                        }
                    }
                }
            }

            let server_match = server_arr.iter().find(|s| {
                if let Some(so) = s.as_object() {
                    if let Some(name) = l_name {
                        so.get("name").and_then(|n| n.as_str()) == Some(name)
                    } else if let Some(path) = l_path {
                        so.get("path").and_then(|p| p.as_str()) == Some(path)
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if let Some(s_val) = server_match {
                let Some(s) = s_val.as_object() else {
                    continue;
                };

                let mut mismatch = false;

                for (k, v) in l.iter() {
                    if k != "fields" && k != "id" && s.get(k) != Some(v) {
                        mismatch = true;
                        break;
                    }
                }

                if !mismatch {
                    for (name, local_val) in &l_fields {
                        let s_field =
                            s.get("fields")
                                .and_then(|v| v.as_array())
                                .and_then(|fields| {
                                    fields.iter().find(|sf| {
                                        sf.get("name").and_then(|n| n.as_str())
                                            == Some(name.as_str())
                                    })
                                });

                        match s_field {
                            None => {
                                mismatch = true;
                                break;
                            }
                            Some(s_field_val) => {
                                let server_val = s_field_val.get("value").unwrap_or(&Value::Null);
                                if local_val != server_val {
                                    let is_cred = is_credential_field(name);
                                    let s_str_opt = server_val.as_str();
                                    let local_non_empty =
                                        local_val.as_str().map(|s| !s.is_empty()).unwrap_or(false);
                                    let is_bypass = is_cred
                                        && s_str_opt
                                            .map(|s| {
                                                (s == "******" || s.is_empty()) && local_non_empty
                                            })
                                            .unwrap_or(false);
                                    if !is_bypass {
                                        mismatch = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if mismatch {
                    let mut update_payload = s.clone();

                    for (k, v) in l.iter() {
                        if k != "fields" && k != "id" {
                            update_payload.insert(k.clone(), v.clone());
                        }
                    }

                    let mut updated_fields = Vec::new();
                    let mut s_field_names = std::collections::HashSet::new();

                    if let Some(s_fields_arr) = s.get("fields").and_then(|v| v.as_array()) {
                        for sf_val in s_fields_arr {
                            if let Some(sf) = sf_val.as_object() {
                                if let Some(f_name) = sf.get("name").and_then(|v| v.as_str()) {
                                    s_field_names.insert(f_name.to_string());
                                    let mut new_sf = sf.clone();

                                    if let Some(local_val) = l_fields.get(f_name) {
                                        let is_readonly = schema_def
                                            .and_then(|s| s.get("fields"))
                                            .and_then(|v| v.as_array())
                                            .and_then(|s_fields| {
                                                s_fields.iter().find(|schema_f| {
                                                    schema_f.get("name").and_then(|n| n.as_str())
                                                        == Some(f_name)
                                                })
                                            })
                                            .and_then(|schema_f| schema_f.get("isReadOnly"))
                                            .and_then(|ro| ro.as_bool())
                                            .unwrap_or(false);

                                        if !is_readonly {
                                            new_sf.insert("value".to_string(), local_val.clone());
                                        }
                                    }
                                    updated_fields.push(Value::Object(new_sf));
                                } else {
                                    updated_fields.push(sf_val.clone());
                                }
                            } else {
                                updated_fields.push(sf_val.clone());
                            }
                        }
                    }

                    let mut extra_keys: Vec<&String> = l_fields
                        .keys()
                        .filter(|k| !s_field_names.contains(*k))
                        .collect();
                    extra_keys.sort();
                    for k in extra_keys {
                        let mut obj = serde_json::Map::new();
                        obj.insert("name".to_string(), Value::String(k.clone()));
                        if let Some(val) = l_fields.get(k) {
                            obj.insert("value".to_string(), val.clone());
                        }
                        updated_fields.push(Value::Object(obj));
                    }

                    update_payload.insert("fields".to_string(), Value::Array(updated_fields));
                    diff.updates.push(Value::Object(update_payload));
                }
            } else {
                let mut payload = if let Some(s) = schema_def {
                    s.as_object().cloned().unwrap_or_default()
                } else {
                    serde_json::Map::new()
                };

                for (k, v) in l.iter() {
                    if k != "fields" && k != "id" {
                        payload.insert(k.clone(), v.clone());
                    }
                }

                let mut resolved_fields = Vec::new();
                let mut seen_fields = std::collections::HashSet::new();

                if let Some(fields) = schema_fields {
                    for f_val in fields {
                        if let Some(f) = f_val.as_object() {
                            if let Some(f_name) = f.get("name").and_then(|v| v.as_str()) {
                                seen_fields.insert(f_name.to_string());
                                let is_ro = f
                                    .get("isReadOnly")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false);
                                let mut new_f = f.clone();
                                let val_opt = l_fields.get(f_name);
                                if let (false, Some(val)) = (is_ro, val_opt) {
                                    new_f.insert("value".to_string(), val.clone());
                                }
                                resolved_fields.push(Value::Object(new_f));
                            } else {
                                resolved_fields.push(f_val.clone());
                            }
                        } else {
                            resolved_fields.push(f_val.clone());
                        }
                    }

                    let mut extra_keys: Vec<&String> = l_fields
                        .keys()
                        .filter(|k| !seen_fields.contains(*k))
                        .collect();
                    extra_keys.sort();
                    for k in extra_keys {
                        let mut obj = serde_json::Map::new();
                        obj.insert("name".to_string(), Value::String(k.clone()));
                        if let Some(val) = l_fields.get(k) {
                            obj.insert("value".to_string(), val.clone());
                        }
                        resolved_fields.push(Value::Object(obj));
                    }
                } else {
                    let mut keys: Vec<&String> = l_fields.keys().collect();
                    keys.sort();
                    for k in keys {
                        let mut obj = serde_json::Map::new();
                        obj.insert("name".to_string(), Value::String(k.clone()));
                        if let Some(val) = l_fields.get(k) {
                            obj.insert("value".to_string(), val.clone());
                        }
                        resolved_fields.push(Value::Object(obj));
                    }
                }

                payload.insert("fields".to_string(), Value::Array(resolved_fields));
                diff.additions.push(Value::Object(payload));
            }
        }

        for s_val in server_arr {
            let Some(s) = s_val.as_object() else {
                continue;
            };
            let s_name = s.get("name").and_then(|v| v.as_str());
            let s_path = s.get("path").and_then(|v| v.as_str());
            if s_name.is_none() && s_path.is_none() {
                continue;
            }

            let local_match = local_arr.iter().find(|l| {
                if let Some(lo) = l.as_object() {
                    if let Some(name) = s_name {
                        lo.get("name").and_then(|n| n.as_str()) == Some(name)
                    } else if let Some(path) = s_path {
                        lo.get("path").and_then(|p| p.as_str()) == Some(path)
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            if local_match.is_none() {
                diff.deletions.push(s_val.clone());
            }
        }

        Ok(diff)
    }

    pub async fn run_sync(
        &self,
        local_config: &Value,
        options: &SyncOptions,
    ) -> Result<(), anyhow::Error> {
        let app_key = match self.client.app_type {
            AppType::Radarr => "radarr",
            AppType::Sonarr => "sonarr",
            AppType::Prowlarr => "prowlarr",
            AppType::Lidarr => "lidarr",
            AppType::Readarr => "readarr",
        };

        let Some(app_section) = local_config.get(app_key) else {
            return Ok(());
        };

        if app_section.is_null() {
            return Ok(());
        }

        let mappings = [
            ("downloadClients", "downloadclient", true),
            ("indexers", "indexer", true),
            ("notifications", "notification", true),
            ("qualityProfiles", "qualityprofile", false),
            ("customFormats", "customformat", false),
            ("rootFolders", "rootfolder", false),
            ("importLists", "importlist", true),
            ("metadataProfiles", "metadataprofile", false),
            ("releaseProfiles", "releaseprofile", false),
        ];

        for (resource_key, endpoint, has_schema) in mappings {
            if let Some(local_list) = app_section.get(resource_key) {
                if !local_list.is_array() {
                    continue;
                }

                let server_resources = self.client.get_resources(endpoint).await?;
                let schema = if has_schema {
                    self.client.get_schema(endpoint).await?
                } else {
                    Value::Array(vec![])
                };
                let diff = self
                    .compute_diff(local_list, &server_resources, &schema)
                    .await?;

                crate::sync::display::print_diff(&diff);

                if options.mode == SyncMode::Apply {
                    for add in &diff.additions {
                        self.client.create_resource(endpoint, add.clone()).await?;
                    }

                    for upd in &diff.updates {
                        let id = upd
                            .get("id")
                            .and_then(|v| v.as_i64())
                            .ok_or_else(|| anyhow::anyhow!("Missing ID in update payload"))?
                            as i32;
                        self.client
                            .update_resource(endpoint, id, upd.clone())
                            .await?;
                    }

                    if options.prune {
                        for del in &diff.deletions {
                            let id =
                                del.get("id").and_then(|v| v.as_i64()).ok_or_else(|| {
                                    anyhow::anyhow!("Missing ID in deletion payload")
                                })? as i32;
                            self.client.delete_resource(endpoint, id).await?;
                        }
                    }
                }
            }
        }

        let singletons = [
            ("ui", "config/ui"),
            ("naming", "config/naming"),
            ("mediaManagement", "config/mediamanagement"),
        ];

        for (resource_key, endpoint) in singletons {
            if let Some(local_val) = app_section.get(resource_key) {
                if local_val.is_null() {
                    continue;
                }
                let local_obj = local_val.as_object().ok_or_else(|| {
                    anyhow::anyhow!("Expected object for singleton '{}'", resource_key)
                })?;

                let server_val = match self.client.get_resources(endpoint).await {
                    Ok(val) => val,
                    Err(e) => {
                        tracing::warn!(
                            "Failed to fetch singleton '{}' from endpoint '{}': {}. Skipping.",
                            resource_key,
                            endpoint,
                            e
                        );
                        continue;
                    }
                };

                let server_obj = server_val.as_object().ok_or_else(|| {
                    anyhow::anyhow!("Expected object from endpoint '{}'", endpoint)
                })?;

                let mut updated_obj = server_obj.clone();
                let mut has_diff = false;
                let mut plan_lines = Vec::new();

                for (k, v) in local_obj {
                    let server_field_val = server_obj.get(k);
                    if server_field_val != Some(v) {
                        has_diff = true;
                        let server_disp = server_field_val
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "None".to_string());
                        plan_lines.push(format!("  ~ {} : {} -> {}", k, server_disp, v));
                        updated_obj.insert(k.clone(), v.clone());
                    }
                }

                if has_diff {
                    println!(
                        "{}",
                        colored::Colorize::yellow(&*format!(
                            "~ [Update] Global Singleton Config '{}':",
                            resource_key
                        ))
                    );
                    for line in plan_lines {
                        println!("{}", colored::Colorize::cyan(&*line));
                    }

                    if options.mode == SyncMode::Apply {
                        self.client
                            .update_resource(endpoint, 1, Value::Object(updated_obj))
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn is_credential_field(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("key")
        || lower.contains("password")
        || lower.contains("token")
        || lower.contains("secret")
        || lower.contains("passphrase")
}
