//! Provider-resource: metadata. Composes the shared `Provider` envelope with the
//! typed `MetadataProvider` variant, plus the metadata-specific `enable` flag.
//! Both composed pieces use `#[flatten]`, so their keys sit top-level on the wire.

use core_macros::resource;

use crate::resources::metadata_providers::MetadataProvider;
use crate::resources::provider::Provider;

/// Metadata consumer — instructs Sonarr to write sidecar metadata files and
/// artwork alongside downloaded media using a specific plugin.
#[resource(
    sync = crud,
    list = get("/api/v3/metadata"),
    create = post("/api/v3/metadata"),
    update = put("/api/v3/metadata/${self.id}"),
    delete = delete("/api/v3/metadata/${self.id}"),
)]
pub struct Metadata {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: MetadataProvider,
    /// Whether this metadata consumer is active.
    pub enable: bool,
}
