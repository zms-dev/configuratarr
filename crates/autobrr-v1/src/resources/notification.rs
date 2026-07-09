//! `/api/notification` — outbound notification targets (Discord, Telegram, …).
//!
//! GET lists them, POST creates one; the API exposes no update or delete, so
//! this is a `sync = custom` create-or-leave resource
//! ([`core_lib::reconcile::create_only`], keyed by `name`). The POST body is the
//! full typed notification, encoded through the descriptor (maps
//! `notification_type` → the wire `type`, and carries the credential fields).
//! No prune.

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

use crate::resources::notification_event::NotificationEvent;

/// `/api/notification` — a notification target.
#[resource(sync = custom, case = snake, list = get("/api/notification"))]
pub struct Notification {
    /// Display name — its identity.
    #[key]
    pub name: String,
    /// Provider kind. One of: `DISCORD`, `NOTIFIARR`, `IFTTT`, `JOIN`,
    /// `MATTERMOST`, `MATRIX`, `PUSH_BULLET`, `PUSHOVER`, `ROCKETCHAT`, `SLACK`,
    /// `TELEGRAM`, `GOTIFY`, `NTFY`, `LUNASEA`, `SHOUTRRR`, `WEBHOOK`.
    #[wire(name = "type")]
    pub notification_type: String,
    /// Whether the target is active.
    pub enabled: bool,
    /// Events that trigger this notification.
    pub events: Vec<NotificationEvent>,
    /// Webhook URL (Discord and similar).
    pub webhook: Option<String>,
    /// Provider API key / auth token where required.
    pub api_key: Option<SecretValue>,
    /// Bot token (Telegram and similar).
    pub token: Option<SecretValue>,
    /// Target channel / chat id.
    pub channel: Option<String>,
    /// Topic (ntfy and similar).
    pub topic: Option<String>,
    /// Provider host (self-hosted Gotify/ntfy).
    pub host: Option<String>,
    /// Message title (ntfy and similar).
    pub title: Option<String>,
    /// Icon / avatar override.
    pub icon: Option<String>,
    /// Username for auth-protected providers (ntfy, Matrix).
    pub username: Option<String>,
    /// Password for auth-protected providers.
    pub password: Option<SecretValue>,
    /// Rooms to post to (Matrix).
    pub rooms: Option<String>,
    /// Explicit targets (Shoutrrr and similar).
    pub targets: Option<String>,
    /// Target device names (Pushover).
    pub devices: Option<String>,
    /// Message priority (Pushover / Gotify / ntfy).
    pub priority: Option<i32>,
    /// Notification sound (Pushover / ntfy).
    pub sound: Option<String>,
    /// HTTP method (generic webhook providers).
    pub method: Option<String>,
    /// Extra headers (generic webhook providers).
    pub headers: Option<String>,
}

impl CustomSync for Notification {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/notification").await?;
            let present = reconcile::present_keys(&live, "name");
            let client = client.clone();
            reconcile::create_only(desired, "name", &present, execute, move |_name, cfg| {
                let client = client.clone();
                async move {
                    // Encode through the descriptor so `type`/secrets land right.
                    let wire = engine::encode(&engine::decode_config::<Self>(&cfg)?)?;
                    let _: Value = client.post("/api/notification", &wire).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
