//! Opaque key/value field as the *arr API returns it. Used only by the raw
//! fallback provider — typed providers express their settings as real fields
//! and let the fields-blob codec render the `{name, value}` array.
//!
//! `value` is arbitrary JSON, so it uses the `Json` field kind (`FieldKind::Json`).

use core_lib::Json;
use core_macros::nested;

/// An opaque key/value field as the *arr API returns it inside a provider config.
#[nested]
pub struct Field {
    /// Field identifier within the config contract, e.g. `"host"`.
    pub name: Option<String>,
    /// Field value; the concrete type depends on the field kind.
    pub value: Option<Json>,
    /// Human-readable label displayed in the Radarr UI.
    pub label: Option<String>,
    /// Field kind hint, e.g. `"textbox"`, `"select"`, `"checkbox"`.
    #[wire(name = "type")]
    pub field_type: Option<String>,
    /// When `true`, the field is only shown in Radarr's advanced settings mode.
    #[wire(read_only)]
    pub advanced: bool,
}
