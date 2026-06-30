//! Provider-resource: application. Composes the shared `Provider` envelope with
//! the typed `ApplicationProvider` variant (fields-blob), plus the
//! application-specific `syncLevel` field. Both composed pieces use `#[flatten]`,
//! so their keys sit top-level on the wire.

use core_macros::resource;

use crate::resources::application_sync_level::ApplicationSyncLevel;
use crate::resources::applications::ApplicationProvider;
use crate::resources::provider::Provider;

/// Prowlarr application — connects Prowlarr to an *arr app or other media manager
/// so that Prowlarr can push indexer definitions to it.
#[resource(
    sync = crud,
    list   = get("/api/v1/applications"),
    create = post("/api/v1/applications"),
    update = put("/api/v1/applications/${self.id}"),
    delete = delete("/api/v1/applications/${self.id}"),
)]
pub struct Application {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: ApplicationProvider,
    /// How Prowlarr syncs indexers to this application.
    pub sync_level: ApplicationSyncLevel,
    /// Internal command string returned by the API for test/action endpoints; not user-settable.
    #[wire(name = "testCommand", read_only)]
    pub test_command: Option<String>,
}
