//! Raw fallback for a provider implementation we don't model with a typed
//! variant. Carries the discriminators plus the untouched `fields` array so an
//! unknown provider still round-trips losslessly. Target of the `#[fallback]`
//! arm on each provider enum.

use core_macros::nested;

use crate::resources::field::Field;

/// Fallback representation for a provider implementation with no typed struct.
///
/// Carries the discriminator fields and the raw `fields` array so an unknown
/// provider can still round-trip losslessly.
#[nested]
pub struct RawProvider {
    /// Provider implementation class name, e.g. `"Transmission"`.
    pub implementation: String,
    /// Configuration contract class name that governs the fields schema.
    #[wire(name = "configContract")]
    pub config_contract: Option<String>,
    /// Raw key/value fields array as returned by the API.
    pub fields: Vec<Field>,
}
