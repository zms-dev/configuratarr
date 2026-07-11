//! `/api/lists` — autobrr lists: pull wanted titles from an *arr instance or an
//! external source (Trakt/MDBList/…) and auto-attach them to filters.
//!
//! `sync = custom`, keyed by `name`, create + update by id (`POST /api/lists`,
//! `PUT /api/lists/{id}`); no prune — matching the conservative pattern of the
//! other autobrr resources. `api_key` is redacted on read, so idempotency is the
//! structural-subset test on the readable fields ([`crate::diff::subset`]), not a
//! full equality that the redacted secret would always trip. The create/update
//! skeleton is [`core_lib::reconcile::upsert`].
//!
//! FKs: `client_id` → a managed download client (`${ref.download_client.<name>}`,
//! required for the *arr list types); `filters[].id` → managed filters
//! (`${ref.filter.<name>}`). autobrr requires at least one filter for *arr-type
//! lists and a `url` for the external list types.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::list_filter::ListFilter;

/// `/api/lists` — a configured list.
#[resource(sync = custom, case = snake, list = get("/api/lists"))]
pub struct List {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.list.<name>}`).
    #[key]
    pub name: String,
    /// List type: `RADARR`, `SONARR`, `LIDARR`, `READARR`, `WHISPARR` (*arr
    /// types, need `client_id`), or `MDBLIST`, `METACRITIC`, `PLAINTEXT`, `TRAKT`,
    /// `STEAM`, `ANILIST` (external types, need `url`).
    #[wire(name = "type")]
    pub list_type: String,
    /// Whether the list is active.
    pub enabled: Option<bool>,
    /// Download client to pull wanted titles from, for the *arr list types
    /// (`${ref.download_client.<name>}`).
    #[reference(download_client)]
    pub client_id: Option<i32>,
    /// Source URL, for the external list types.
    pub url: Option<String>,
    /// Extra HTTP headers to send when fetching an external list.
    pub headers: Vec<String>,
    /// API key for the external source, where required.
    pub api_key: Option<SecretValue>,
    /// Filters this list feeds into (`${ref.filter.<name>}`). autobrr requires at
    /// least one for the *arr list types.
    pub filters: Vec<ListFilter>,
    /// Match on the release name rather than the parsed title.
    pub match_release: Option<bool>,
    /// Only include titles carrying these tags.
    pub tags_included: Vec<String>,
    /// Exclude titles carrying these tags.
    pub tags_excluded: Vec<String>,
    /// Include unmonitored titles (*arr types).
    pub include_unmonitored: Option<bool>,
    /// Include alternate titles (*arr types).
    pub include_alternate_titles: Option<bool>,
    /// Append the year to the matched title.
    pub include_year: Option<bool>,
    /// Skip autobrr's title clean/sanitize pass.
    pub skip_clean_sanitize: Option<bool>,
}

impl CustomSync for List {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/lists").await?;
            // Full desired wire (refs already resolved to ids; id is read-only).
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
                        let _: Value = client.post("/api/lists", &w).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        let _: Value = client.put(&format!("/api/lists/{id}"), &w).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
