//! `/api/irc` — IRC networks autobrr connects to for announce parsing.
//!
//! `sync = custom`. A generic crud diff can't keep IRC idempotent: autobrr
//! enriches each `channels[]` entry on read with server fields (`id`, `detached`,
//! `monitoring`, …), and the whole-array merge would replace the live rich array
//! with the declared minimal one on every apply — a perpetual "update". So, like
//! [`Filter`](crate::resources::filter::Filter), this hook diffs the declared
//! config as a **structural subset** of live (extra server fields ignored) and
//! only writes on real drift.
//!
//! Write paths are irregular: create is `POST /api/irc`, update is
//! `PUT /api/irc/network/{id}` (a bare `/api/irc/{id}` 404s). No prune (the custom
//! seam carries no `prune` flag; deleting a network you didn't author would be
//! surprising — remove it in autobrr). Credentials (`pass`, `auth.password`,
//! channel keys) are write-only — returned `<redacted>`, so a declared value
//! reads as drift and re-applies as an update.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::irc_auth::IrcAuth;
use crate::resources::irc_channel::IrcChannel;

/// `/api/irc` — an IRC network with its channels and auth.
#[resource(sync = custom, case = snake, list = get("/api/irc"))]
pub struct IrcNetwork {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Network name — its identity (`${ref.irc_network.<name>}`).
    #[key]
    pub name: String,
    /// Whether autobrr connects to this network.
    pub enabled: bool,
    /// IRC server hostname.
    pub server: String,
    /// IRC server port.
    pub port: i32,
    /// Connect over TLS.
    pub tls: Option<bool>,
    /// Skip TLS certificate verification.
    pub tls_skip_verify: Option<bool>,
    /// Bot nick to use on the network.
    pub nick: String,
    /// Server password (PASS), where required (write-only).
    pub pass: Option<SecretValue>,
    /// NickServ / SASL authentication.
    pub auth: Option<IrcAuth>,
    /// Command sent to request an invite (e.g. `/msg gatekeeper !invite`).
    pub invite_command: Option<String>,
    /// Connect through a bouncer instead of directly.
    pub use_bouncer: Option<bool>,
    /// Bouncer address, where `use_bouncer` is set.
    pub bouncer_addr: Option<String>,
    /// Enable IRCv3 bot mode.
    pub bot_mode: Option<bool>,
    /// Route this network through a proxy.
    pub use_proxy: Option<bool>,
    /// Proxy to route through (`${ref.proxy.<name>}`).
    #[reference(proxy)]
    pub proxy_id: Option<i32>,
    /// Channels to join.
    pub channels: Vec<IrcChannel>,
    /// Runtime: current connection health (read-only).
    #[wire(read_only)]
    pub healthy: Option<bool>,
    /// Runtime: whether currently connected (read-only).
    #[wire(read_only)]
    pub connected: Option<bool>,
    /// Runtime: the nick currently in use (read-only).
    #[wire(read_only)]
    pub current_nick: Option<String>,
}

impl CustomSync for IrcNetwork {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/irc").await?;
            // Full desired wire (read-only runtime fields omitted by encode).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert(
                &wire,
                &live,
                "name",
                diff::subset,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/irc", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // autobrr's update reads the id from the body (WHERE id=?);
                    // `#[id]` omits it on encode, so echo the live id back. The
                    // update path is irregular (`/api/irc/network/{id}`, not
                    // `/api/irc/{id}`, which 404s).
                    reconcile::echo(&mut w, "id", l);
                    async move {
                        let _: Value = client.put(&format!("/api/irc/network/{id}"), &w).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
