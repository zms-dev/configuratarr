//! Enabled languages + language profiles.
//!
//! Both write through `/api/system/settings` (the same POST as [`Settings`]) but
//! as **side-channel** form fields, not `settings-*` keys:
//! * `languages-enabled` — one repeated field per enabled alpha-2 code;
//! * `languages-profiles` — a **JSON string** of the full profile list, with
//!   **full-replace-by-`profileId`** semantics (bazarr deletes any profile not
//!   listed).
//!
//! They're read back from separate endpoints (`/api/system/languages`,
//! `/api/system/languages/profiles`) and the profile wire shape is irregular
//! (mixed casing: `profileId`/`mustContain` camel but item keys snake;
//! `originalFormat` an int; item booleans Python `"True"`/`"False"` strings). No
//! stock codec models that, so this is a legitimate `sync = custom` reconcile —
//! it owns its own translation, unlike [`Settings`], whose regular sections go
//! through the engine codec.
//!
//! [`Settings`]: crate::resources::settings::Settings

use std::collections::BTreeSet;

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Value, json};

use crate::resources::language_profile::LanguageProfile;

const SETTINGS_PATH: &str = "/api/system/settings";
const PROFILES_PATH: &str = "/api/system/languages/profiles";
const LANGUAGES_PATH: &str = "/api/system/languages";

/// Enabled subtitle languages + the language-profile set, reconciled together
/// (profiles reference enabled languages). Written via the settings POST.
#[resource(sync = custom, list = get("/api/system/languages/profiles"))]
pub struct Languages {
    /// Enabled subtitle-language codes (alpha-2, e.g. `en`). Replaces the enabled
    /// set; declare the languages your profiles reference.
    pub enabled_languages: Vec<String>,
    /// Language profiles (full-replace by `profile_id`): the declared list is the
    /// complete desired set; bazarr deletes any profile not listed.
    pub language_profiles: Vec<LanguageProfile>,
}

/// A profile item field as a string, defaulting to bazarr's own default when the
/// user omitted it — so the built item carries the full key set bazarr stores.
fn item_str(item: &Value, key: &str, default: &str) -> String {
    item.get(key)
        .and_then(Value::as_str)
        .unwrap_or(default)
        .to_string()
}

/// Translate one config profile item into bazarr's exact stored JSON shape.
fn wire_item(item: &Value) -> Value {
    json!({
        "id": item.get("id").cloned().unwrap_or(Value::Null),
        "language": item.get("language").cloned().unwrap_or(Value::Null),
        "audio_exclude": item_str(item, "audio_exclude", "False"),
        "audio_only_include": item_str(item, "audio_only_include", "False"),
        "forced": item_str(item, "forced", "False"),
        "hi": item_str(item, "hi", "False"),
    })
}

/// Translate one config profile (snake_case) into bazarr's exact stored JSON
/// shape (`profileId`, `mustContain`, …; `originalFormat` as the `0`/`1`/null int
/// bazarr stores). Building the desired in the read shape makes the diff
/// structural and the write idempotent.
fn wire_profile(p: &Value) -> Value {
    let original_format = match p.get("original_format") {
        Some(Value::Bool(b)) => json!(i32::from(*b)),
        _ => Value::Null,
    };
    let items: Vec<Value> = p
        .get("items")
        .and_then(Value::as_array)
        .map(|a| a.iter().map(wire_item).collect())
        .unwrap_or_default();
    json!({
        "profileId": p.get("profile_id").cloned().unwrap_or(Value::Null),
        "name": p.get("name").cloned().unwrap_or(Value::Null),
        "cutoff": p.get("cutoff").cloned().unwrap_or(Value::Null),
        "items": items,
        "mustContain": p.get("must_contain").cloned().unwrap_or_else(|| json!([])),
        "mustNotContain": p.get("must_not_contain").cloned().unwrap_or_else(|| json!([])),
        "originalFormat": original_format,
        "tag": p.get("tag").cloned().unwrap_or(Value::Null),
    })
}

/// The declared profiles as bazarr wire JSON, sorted by `profileId` for an
/// order-insensitive comparison with the live list.
fn wire_profiles(declared: &[Value]) -> Vec<Value> {
    let mut v: Vec<Value> = declared.iter().map(wire_profile).collect();
    v.sort_by_key(|p| p.get("profileId").and_then(Value::as_i64).unwrap_or(0));
    v
}

/// Sorted live list for the same order-insensitive comparison.
fn sorted_live_profiles(live: &[Value]) -> Vec<Value> {
    let mut v = live.to_vec();
    v.sort_by_key(|p| p.get("profileId").and_then(Value::as_i64).unwrap_or(0));
    v
}

/// The set of enabled language codes from `/api/system/languages`.
fn enabled_langs(live: &[Value]) -> BTreeSet<String> {
    live.iter()
        .filter(|l| l.get("enabled").and_then(Value::as_bool).unwrap_or(false))
        .filter_map(|l| l.get("code2").and_then(Value::as_str).map(String::from))
        .collect()
}

impl CustomSync for Languages {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };

            let declared_langs = cfg.get("enabled_languages").and_then(Value::as_array);
            let langs_synced = match declared_langs {
                None => true,
                Some(langs) => {
                    let want: BTreeSet<String> = langs
                        .iter()
                        .filter_map(|l| l.as_str().map(String::from))
                        .collect();
                    let live: Vec<Value> = client.get(LANGUAGES_PATH).await?;
                    want == enabled_langs(&live)
                }
            };

            let declared_profiles = cfg.get("language_profiles").and_then(Value::as_array);
            let profiles_synced = match declared_profiles {
                None => true,
                Some(profiles) => {
                    let live: Vec<Value> = client.get(PROFILES_PATH).await?;
                    wire_profiles(profiles) == sorted_live_profiles(&live)
                }
            };

            if langs_synced && profiles_synced {
                return Ok(vec![Change::unchanged("languages")]);
            }

            let mut pairs: Vec<(String, String)> = Vec::new();
            if let Some(langs) = declared_langs {
                for code in langs.iter().filter_map(Value::as_str) {
                    pairs.push(("languages-enabled".to_string(), code.to_string()));
                }
            }
            if let Some(profiles) = declared_profiles {
                let json = Value::Array(wire_profiles(profiles)).to_string();
                pairs.push(("languages-profiles".to_string(), json));
            }
            if execute {
                let _: Value = client.post_form(SETTINGS_PATH, &pairs).await?;
            }

            let mut change = Change::updated("languages");
            if let Some(langs) = declared_langs {
                change = change.with("enabled_languages", langs.len().to_string());
            }
            if let Some(profiles) = declared_profiles {
                change = change.with("language_profiles", profiles.len().to_string());
            }
            Ok(vec![change])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn wire_profile_matches_bazarr_stored_shape() {
        let cfg = json!({
            "profile_id": 1,
            "name": "English",
            "cutoff": null,
            "items": [{ "id": 1, "language": "en" }],
            "must_contain": [],
            "must_not_contain": [],
            "original_format": false,
            "tag": null,
        });
        let expected = json!({
            "profileId": 1,
            "name": "English",
            "cutoff": null,
            "items": [{
                "id": 1,
                "language": "en",
                "audio_exclude": "False",
                "audio_only_include": "False",
                "forced": "False",
                "hi": "False",
            }],
            "mustContain": [],
            "mustNotContain": [],
            "originalFormat": 0,
            "tag": null,
        });
        assert_eq!(wire_profile(&cfg), expected);
    }

    #[test]
    fn original_format_maps_bool_to_int_else_null() {
        assert_eq!(
            wire_profile(&json!({ "original_format": true }))["originalFormat"],
            json!(1)
        );
        assert_eq!(
            wire_profile(&json!({ "original_format": false }))["originalFormat"],
            json!(0)
        );
        assert_eq!(wire_profile(&json!({}))["originalFormat"], Value::Null);
    }
}
