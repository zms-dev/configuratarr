//! `/Library/VirtualFolders` — media libraries.
//!
//! A library is keyed by name and created through **query parameters** (plus an
//! `AddVirtualFolderDto` body), with no per-library id — so it's a `sync =
//! custom` resource. The reconcile hook creates any declared library that isn't
//! present yet (create-or-leave; no prune, no path reconcile in this MVP).

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Value, json};

/// `/Library/VirtualFolders` — a media library (name + collection type + paths).
#[resource(sync = custom, list = get("/Library/VirtualFolders"))]
pub struct Library {
    /// Library display name — its identity.
    #[key]
    pub name: String,
    /// Collection type: `movies`, `tvshows`, `music`, `books`, … Omit for mixed.
    pub collection_type: Option<String>,
    /// Filesystem paths that make up the library.
    pub paths: Vec<String>,
}

impl CustomSync for Library {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/Library/VirtualFolders").await?;
            let present = reconcile::present_keys(&live, "Name");
            let client = client.clone();
            reconcile::create_only(desired, "name", &present, execute, move |name, cfg| {
                let client = client.clone();
                async move {
                    // Identity + paths go in the query string;
                    // `refreshLibrary=false` avoids a scan during apply.
                    let mut q = format!(
                        "/Library/VirtualFolders?refreshLibrary=false&name={}",
                        urlencode(&name)
                    );
                    if let Some(ct) = cfg.get("collection_type").and_then(Value::as_str) {
                        q.push_str(&format!("&collectionType={}", urlencode(ct)));
                    }
                    for p in cfg
                        .get("paths")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                    {
                        if let Some(p) = p.as_str() {
                            q.push_str(&format!("&paths={}", urlencode(p)));
                        }
                    }
                    let _: Value = client.post(&q, &json!({ "LibraryOptions": {} })).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}

/// Minimal percent-encoding for query values (space + the reserved chars that
/// break a query string). Enough for names and filesystem paths.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
