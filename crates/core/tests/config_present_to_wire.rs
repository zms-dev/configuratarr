//! `engine::config_present_to_wire` — presence masking, including **nested**
//! single objects.
//!
//! A singleton's typed struct fills omitted fields with type defaults; the mask
//! drops everything the user didn't write so "omitted = unmanaged" holds on
//! merge. The nested-recursion case is the interesting one: a declared section
//! must carry only its declared inner keys, not the inner struct's defaults
//! (which would clobber the live values for keys the user never touched).

use core_lib::engine;
use core_macros::{nested, resource};

// Non-`Option` inner fields, so an omitted one still *encodes* to its type
// default — the masking is what must remove it, proving the recursion.
#[nested]
pub struct Section {
    pub a: bool,
    pub b: i32,
    pub c: String,
}

#[resource(sync = singleton, read = get("/s"), update = put("/s"))]
pub struct Cfg {
    pub section: Option<Section>,
    pub top: bool,
}

#[test]
fn masks_nested_object_to_declared_inner_keys() {
    // User declares only `section.a` (and `top`) — not `section.b` / `section.c`.
    let cfg = serde_json::json!({ "section": { "a": true }, "top": false });
    let wire = engine::config_present_to_wire::<Cfg>(&cfg).unwrap();
    // Recursion masks the inner object: only `a` survives. Without it, `b`/`c`
    // would appear as `0` / `""` and clobber the live values on merge.
    assert_eq!(
        wire,
        serde_json::json!({ "section": { "a": true }, "top": false })
    );
}

#[test]
fn absent_nested_section_is_omitted() {
    let cfg = serde_json::json!({ "top": true });
    let wire = engine::config_present_to_wire::<Cfg>(&cfg).unwrap();
    assert_eq!(wire, serde_json::json!({ "top": true }));
}
