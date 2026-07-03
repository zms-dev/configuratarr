//! `/Repositories` — the plugin repository list.
//!
//! Jellyfin has no per-repository id: the whole list is replaced in one POST.
//! That's a `sync = custom` reconcile — GET the live list, and if the desired set
//! differs (order-insensitively) POST the whole thing. The comparison is
//! **structural** (by the repo's own fields), not serialization-based, so it
//! doesn't depend on JSON key ordering.

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

/// `/Repositories` — a plugin repository (replaced as part of the whole list).
#[resource(sync = custom, case = pascal, list = get("/Repositories"))]
pub struct Repository {
    /// Repository display name.
    pub name: Option<String>,
    /// Manifest URL for the repository.
    pub url: Option<String>,
    /// Whether this repository is enabled.
    pub enabled: bool,
}

/// Order-insensitive identity of the repo set: sorted `(Name, Url, Enabled)`
/// tuples. Structural, so it doesn't rely on JSON key ordering.
fn repo_set(items: &[Value]) -> Vec<(String, String, bool)> {
    let mut v: Vec<_> = items
        .iter()
        .map(|i| {
            (
                i.get("Name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                i.get("Url")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                i.get("Enabled").and_then(Value::as_bool).unwrap_or(false),
            )
        })
        .collect();
    v.sort();
    v
}

impl CustomSync for Repository {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/Repositories").await?;

            // Encode each desired config to its wire (PascalCase) form.
            let wire: Vec<Value> = desired
                .iter()
                .map(|cfg| engine::encode(&engine::decode_config::<Self>(cfg)?))
                .collect::<anyhow::Result<_>>()?;

            if repo_set(&live) == repo_set(&wire) {
                return Ok(vec![Change::unchanged("repositories")]);
            }
            if execute {
                let _: Value = client.post("/Repositories", &Value::Array(wire)).await?;
            }
            Ok(vec![Change::updated("repositories")])
        })
    }
}
