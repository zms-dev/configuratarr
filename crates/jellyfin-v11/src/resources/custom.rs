//! Shared reconcile helpers for this crate's `sync = custom` resources.
//!
//! Deliberately in the **service crate**, not core: the engine only provides the
//! [`core_lib::CustomSync`] seam; recurring reconcile shapes are a per-service
//! convenience and shouldn't bloat the engine.

use std::future::Future;

use core_lib::Change;
use serde_json::Value;

/// The identity values already present server-side: the `field` (e.g. `"Name"`,
/// `"AppName"`) of each live item.
pub fn live_keys(live: &[Value], field: &str) -> Vec<String> {
    live.iter()
        .filter_map(|i| i.get(field).and_then(Value::as_str).map(String::from))
        .collect()
}

/// Reconcile a "keyed, create-or-leave" custom resource: for each desired item,
/// unchanged if one with the same `key_field` already exists, else run `create`.
/// No update, no prune (mirrors the *arr "don't delete what you didn't make"
/// caution). Returns one [`Change`] per item.
pub async fn reconcile_create_only<F, Fut>(
    desired: &[Value],
    key_field: &str,
    present: &[String],
    execute: bool,
    create: F,
) -> anyhow::Result<Vec<Change>>
where
    F: Fn(String, Value) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut changes = Vec::with_capacity(desired.len());
    for cfg in desired {
        let key = cfg
            .get(key_field)
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("entry is missing `{key_field}`"))?;

        if present.iter().any(|k| k == key) {
            changes.push(Change::unchanged(key));
            continue;
        }
        if execute {
            create(key.to_string(), cfg.clone()).await?;
        }
        changes.push(Change::created(key));
    }
    Ok(changes)
}
