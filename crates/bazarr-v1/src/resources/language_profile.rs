//! Language profiles — bazarr's per-title subtitle-language policy.
//!
//! These are written through the **same** `/api/system/settings` POST as the
//! rest of the config, but as a top-level `languages-profiles` form field
//! (a JSON string, **not** a `settings-*` key) with **full-replace-by-`profileId`**
//! semantics: the submitted list is the complete desired set, and bazarr deletes
//! any profile whose `profileId` is absent. Read back from
//! `/api/system/languages/profiles`.
//!
//! The reconcile hook ([`crate::resources::settings`]) translates these
//! (snake_case) config structs into bazarr's exact stored JSON shape before
//! diffing/POSTing — e.g. `profile_id` → `profileId`, `original_format` → the
//! `originalFormat` **int** (`0`/`1`/null) bazarr stores. Item booleans are
//! Python-style `"True"`/`"False"`/`"Both"` strings, matching what bazarr
//! returns, so the round-trip is idempotent.

use core_macros::nested;

/// One language entry within a [`LanguageProfile`].
///
/// The `forced`, `hi`, `audio_exclude` and `audio_only_include` flags are
/// bazarr's own tri-/bi-state strings — `"True"`, `"False"` (and `"Both"` for
/// `forced`/`hi`) — mirrored verbatim so a profile round-trips unchanged.
#[nested]
pub struct LanguageProfileItem {
    /// Item id (unique within the profile; referenced by the profile `cutoff`).
    pub id: i32,
    /// Language code (alpha-2, e.g. `en`).
    pub language: Option<String>,
    /// Match forced subtitles: `"True"` / `"False"` / `"Both"`.
    pub forced: Option<String>,
    /// Match hearing-impaired subtitles: `"True"` / `"False"` / `"Both"`.
    pub hi: Option<String>,
    /// Skip when this language is present in the audio: `"True"` / `"False"`.
    pub audio_exclude: Option<String>,
    /// Only include when this language is the audio: `"True"` / `"False"`.
    pub audio_only_include: Option<String>,
}

/// A language profile — the ordered set of subtitle languages bazarr wants for a
/// title, plus the cutoff and must-/must-not-contain rules.
#[nested]
pub struct LanguageProfile {
    /// Stable profile id (the natural key; profiles are replaced by this id).
    pub profile_id: i32,
    /// Display name.
    pub name: Option<String>,
    /// Cutoff item `id` — stop searching once this language is found (`65535` =
    /// any of the profile's languages; unset = never cut off early).
    pub cutoff: Option<i32>,
    /// The languages this profile wants, in priority order.
    pub items: Vec<LanguageProfileItem>,
    /// Release must contain all of these strings.
    pub must_contain: Vec<String>,
    /// Release must contain none of these strings.
    pub must_not_contain: Vec<String>,
    /// Prefer the original-format subtitle.
    pub original_format: Option<bool>,
    /// Sonarr/Radarr tag this profile is scoped to.
    pub tag: Option<String>,
}
