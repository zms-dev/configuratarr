//! Autobrr v1 resources.
//!
//! autobrr is **not** an *arr: snake_case JSON, `X-API-Token` header auth, no
//! provider/fields-blob concept. The API is asymmetric — only some resources are
//! createable — so the managed surface is: `download_client` (crud, no delete),
//! `api_key` + `notification` (create-only, via
//! [`core_lib::reconcile::create_only`]), and `filter` (a bespoke two-step
//! `sync = custom`). indexers / feeds / irc are read-only + toggle-only upstream
//! and aren't modelled. autobrr's app config (`/api/config`) is **not** modelled:
//! it writes `config.toml`, which is read-only under the NixOS `services.autobrr`
//! module (own it there via `services.autobrr.settings`, not here).

// create-only collections (reconcile::create_only)
pub mod api_key;
pub mod notification;
pub mod notification_event;

// crud collection + its nested settings
pub mod download_client;
pub mod download_client_basic;
pub mod download_client_rules;
pub mod download_client_settings;

// filter (custom two-step) + its nested types
pub mod action;
pub mod external_filter;
pub mod filter;
pub mod filter_indexer;
