//! `/api/keys` — API keys, one per name.
//!
//! autobrr mints the key value server-side (you declare only the name + scopes),
//! and the API offers no update or delete — so this is a `sync = custom`
//! create-or-leave resource, reconciled via [`core_lib::reconcile::create_only`]:
//! GET the live keys, POST any declared name that isn't present yet. No prune
//! (the custom seam gets no `prune` flag; and "don't delete what you didn't
//! make" applies to server-minted credentials anyway).

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Value, json};

/// `/api/keys` — an API key issued under a name, with a set of scopes.
#[resource(sync = custom, case = snake, list = get("/api/keys"))]
pub struct ApiKey {
    /// Key name — its identity.
    #[key]
    pub name: String,
    /// Access scopes granted to the key (e.g. `["read", "write"]`).
    pub scopes: Vec<String>,
}

impl CustomSync for ApiKey {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/keys").await?;
            let present = reconcile::present_keys(&live, "name");
            let client = client.clone();
            reconcile::create_only(desired, "name", &present, execute, move |name, cfg| {
                let client = client.clone();
                // Body is the ApiKeyCreate shape: {name, scopes} — exactly the
                // resolved config for this entry.
                let body = json!({
                    "name": name,
                    "scopes": cfg.get("scopes").cloned().unwrap_or_else(|| json!([])),
                });
                async move {
                    let _: Value = client.post("/api/keys", &body).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
