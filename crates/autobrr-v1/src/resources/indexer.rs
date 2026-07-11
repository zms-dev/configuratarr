//! `/api/indexer` — indexers (torznab/newznab/rss/IRC-tracker definitions).
//!
//! `sync = custom`. autobrr's indexer API can't be driven by the generic crud
//! merge for two reasons:
//!
//! 1. **The stored `identifier` is server-rewritten.** You create with a base
//!    definition id (`torznab`), but autobrr stores it namespaced per instance
//!    (`torznab-my-torznab`). An update that sends the base id 500s with
//!    "could not find definition" — the update must echo the *stored* identifier.
//! 2. **`settings` is a write-only, open string map.** The `{ url, api_key, … }`
//!    values are never returned on read (GET exposes the definition schema, and
//!    secret settings are redacted), so there is nothing to diff or merge.
//!
//! So this hook creates by name (base identifier), and on re-apply PUTs the full
//! config back using the *live* identifier + re-sent settings. Idempotency is
//! judged on the readable fields (`enabled`, `implementation`, `base_url`); a
//! settings-only edit can't be detected (the values aren't readable) and is
//! always re-sent on an update triggered by another change. No prune (the custom
//! seam carries no `prune` flag; matches the "don't delete server-owned defs"
//! caution).

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, Json, RefStore, engine, reconcile};
use core_macros::resource;
use serde_json::Value;

/// `/api/indexer` — a configured indexer instance.
#[resource(sync = custom, case = snake, list = get("/api/indexer"))]
pub struct Indexer {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.indexer.<name>}`).
    #[key]
    pub name: String,
    /// Definition id to instantiate (e.g. `torznab`, `beyond-hd`). Sent verbatim
    /// on create; autobrr namespaces the stored value per instance.
    pub identifier: String,
    /// Definition implementation: `torznab`, `newznab`, `rss`, or `irc`.
    pub implementation: String,
    /// Tracker base URL. **Required for `irc` indexers** — autobrr rejects an
    /// empty `base_url` there (`indexer baseURL must not be empty`); it maps the
    /// indexer into the IRC announce handler by it. A top-level field, not a
    /// `settings` entry.
    pub base_url: Option<String>,
    /// Whether the indexer is active.
    #[default(true)]
    pub enabled: bool,
    /// Route this indexer's HTTP through a proxy.
    pub use_proxy: Option<bool>,
    /// Proxy to route through (`${ref.proxy.<name>}`).
    #[reference(proxy)]
    pub proxy_id: Option<i32>,
    /// Definition settings as a flat `name: value` map. For a torznab/newznab
    /// indexer: `{ url: "...", api_key: "..." }`. For an `irc` indexer, the IRC
    /// login: `{ nick: "...", "auth.account": "...", "auth.password": "..." }` —
    /// autobrr derives the IRC network from the indexer, so the login lives here,
    /// not on a separate `irc_networks` entry. Write-only — never returned on read.
    pub settings: Json,
}

/// Live readable fields match the desired wire (name already matched by the
/// caller). `settings`/`identifier` are excluded — settings values aren't
/// readable and the live identifier is the server-rewritten form.
fn readable_matches(wire: &Value, live: &Value) -> bool {
    // Coalesce absent/null: an unset `base_url` is omitted on encode but read back
    // as `null`, and the two must compare equal (else torznab indexers churn).
    let eq = |k: &str| wire.get(k).unwrap_or(&Value::Null) == live.get(k).unwrap_or(&Value::Null);
    eq("enabled") && eq("implementation") && eq("base_url")
}

impl CustomSync for Indexer {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/indexer").await?;
            // Full desired wire: { name, identifier(base), implementation,
            // enabled, settings{...} }.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert(
                &wire,
                &live,
                "name",
                readable_matches,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/indexer", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // autobrr reads the id from the body, and the base `identifier`
                    // 500s on update ("could not find definition") — it must echo
                    // the server-rewritten stored identifier instead.
                    reconcile::echo(&mut w, "id", l);
                    reconcile::echo(&mut w, "identifier", l);
                    async move {
                        let _: Value = client.put(&format!("/api/indexer/{id}"), &w).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
