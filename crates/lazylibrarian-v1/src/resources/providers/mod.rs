//! LazyLibrarian provider families.
//!
//! Each family is a config.ini array section (`Newznab_<n>`, `Torznab_<n>`,
//! `RSS_<n>`, `IRC_<n>`, `GEN_<n>`), keyed by `DISPNAME`, listed via
//! `listProviders` and created via `addProvider&type=…`. There are two write
//! contracts (see [`reconcile_family`]): **newznab/torznab** support
//! `changeProvider` name-matched updates → [`core_lib::reconcile::upsert`];
//! **rss/irc/gen** cannot be updated by display name (`changeProvider` matches
//! them only by internal `NAME`) → [`core_lib::reconcile::create_only`]. No prune
//! (no `Change`-expressible delete). All `sync = custom`; every command is
//! `GET /api?cmd=…`.
//!
//! Apprise providers (`APPRISE_<n>`) are **not** modelled: `addProvider` accepts
//! only `newznab/torznab/rss/gen/irc` and `listProviders` returns no apprise array,
//! so the API cannot manage them.

use core_lib::apply::Change;
use core_lib::reconcile;
use core_lib::{HttpClient, engine};
use serde_json::Value;

use core_lib::Described;

pub mod direct;
pub mod irc;
pub mod newznab;
pub mod rss;
pub mod torznab;

/// Render a JSON scalar as a query-string value. Objects/arrays never occur in a
/// provider's flat field set.
fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => {
            if *b {
                "1".into()
            } else {
                "0".into()
            }
        }
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Whether `v` is a truthy JSON value (bool `true` or the int/string `1`).
fn truthy(v: &Value) -> bool {
    matches!(v, Value::Bool(true)) || v.as_i64() == Some(1) || v == &Value::String("1".into())
}

/// Compare a desired value against a live one as strings, tolerating
/// LazyLibrarian's interchangeable boolean spellings: `1`/`true` = true;
/// `0`/`false`/`""` (empty) = false. `listProviders` returns **every** field as a
/// string (`"0"`, `""`, `"1"`) and a *disabled* provider reads its `ENABLED` back
/// as `""`, so a raw typed `==` never matches — normalise both sides first.
fn same(want: &str, have: &str) -> bool {
    if want == have {
        return true;
    }
    let norm = |s: &str| match s.trim().to_ascii_lowercase().as_str() {
        "1" | "true" => Some(true),
        "0" | "false" | "" => Some(false),
        _ => None,
    };
    matches!((norm(want), norm(have)), (Some(a), Some(b)) if a == b)
}

/// Idempotency predicate: every declared (wire) field already matches the live
/// provider (compared as normalised strings). `listProviders` returns the full
/// record (including fields we don't manage), so a subset match means "in sync".
fn in_sync(wire: &Value, live: &Value) -> bool {
    wire.as_object()
        .map(|o| {
            o.iter().all(|(k, v)| {
                let have = live.get(k).map(scalar).unwrap_or_default();
                same(&scalar(v), &have)
            })
        })
        .unwrap_or(false)
}

/// `addProvider&type=…&name=<DISPNAME>&<FIELD>=…` for one wire record.
async fn add_provider(
    client: &HttpClient,
    add_type: &'static str,
    wire: &Value,
) -> anyhow::Result<()> {
    let mut pairs: Vec<(String, String)> = vec![
        ("cmd".into(), "addProvider".into()),
        ("type".into(), add_type.into()),
    ];
    if let Some(obj) = wire.as_object() {
        for (k, v) in obj {
            match k.as_str() {
                // DISPNAME is set via `name`; ENABLED via the lowercase `enabled`
                // (== 'true') that addProvider special-cases.
                "DISPNAME" => pairs.push(("name".into(), scalar(v))),
                "ENABLED" => pairs.push((
                    "enabled".into(),
                    if truthy(v) {
                        "true".into()
                    } else {
                        "false".into()
                    },
                )),
                _ => pairs.push((k.clone(), scalar(v))),
            }
        }
    }
    let refs: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    crate::http::get(client, &refs).await?;
    Ok(())
}

/// Shared reconcile for one provider family.
///
/// `family` is the key under `listProviders`' result (`"newznab"`, `"direct"`, …);
/// `add_type` is the `addProvider&type=` value (`"newznab"`, `"gen"`, …). `desired`
/// are wire-encoded records (UPPERCASE keys incl. `DISPNAME`).
///
/// `updatable` splits the two write contracts LazyLibrarian actually supports:
/// * **newznab / torznab** (`updatable = true`) — `changeProvider` matches by
///   `DISPNAME` + `providertype`, so this is a full create-or-**update** upsert.
/// * **rss / irc / gen** (`updatable = false`) — `changeProvider` matches those
///   only by their internal `NAME` (`RSS_0`), so a name-keyed update returns
///   `Invalid parameter: name`. There is no way to update them by display name,
///   so they are **create-only** (create if absent, else leave) — no churn.
async fn reconcile_family(
    client: &HttpClient,
    desired: &[Value],
    execute: bool,
    family: &'static str,
    add_type: &'static str,
    updatable: bool,
) -> anyhow::Result<Vec<Change>> {
    let all = crate::http::get(client, &[("cmd", "listProviders")]).await?;
    let live: Vec<Value> = all
        .get(family)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    if !updatable {
        // create-only by DISPNAME
        let present = reconcile::present_keys(&live, "DISPNAME");
        let client = client.clone();
        return reconcile::create_only(desired, "DISPNAME", &present, execute, move |_k, wire| {
            let client = client.clone();
            async move { add_provider(&client, add_type, &wire).await }
        })
        .await;
    }

    let client = client.clone();
    reconcile::upsert(
        desired,
        &live,
        "DISPNAME",
        in_sync,
        execute,
        {
            let client = client.clone();
            move |wire: Value| {
                let client = client.clone();
                async move { add_provider(&client, add_type, &wire).await }
            }
        },
        // update — changeProvider&providertype=…&name=<DISPNAME>&<FIELD>=…
        {
            let client = client.clone();
            move |_live: &Value, wire: Value| {
                let client = client.clone();
                async move {
                    let dispname = wire.get("DISPNAME").map(scalar).unwrap_or_default();
                    let mut pairs: Vec<(String, String)> = vec![
                        ("cmd".into(), "changeProvider".into()),
                        ("providertype".into(), add_type.into()),
                        ("name".into(), dispname),
                    ];
                    if let Some(obj) = wire.as_object() {
                        for (k, v) in obj {
                            if k == "DISPNAME" {
                                continue; // identity, not a field to rewrite
                            }
                            pairs.push((k.clone(), scalar(v)));
                        }
                    }
                    let refs: Vec<(&str, &str)> = pairs
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    crate::http::get(&client, &refs).await?;
                    Ok(())
                }
            }
        },
    )
    .await
}

/// Encode a family's desired config items to wire form, then run the shared
/// reconcile. Called from each provider's `CustomSync::reconcile`.
async fn reconcile_encoded<T: Described>(
    client: &HttpClient,
    desired: &[Value],
    execute: bool,
    family: &'static str,
    add_type: &'static str,
    updatable: bool,
) -> anyhow::Result<Vec<Change>> {
    let wire: Vec<Value> = desired
        .iter()
        .map(|c| engine::encode_config::<T>(c))
        .collect::<anyhow::Result<_>>()?;
    reconcile_family(client, &wire, execute, family, add_type, updatable).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scalar_renders_query_values() {
        assert_eq!(scalar(&json!(true)), "1");
        assert_eq!(scalar(&json!(false)), "0");
        assert_eq!(scalar(&json!("host")), "host");
        assert_eq!(scalar(&json!(8080)), "8080");
        assert_eq!(scalar(&json!(null)), "");
    }

    #[test]
    fn truthy_matches_bool_and_one() {
        assert!(truthy(&json!(true)));
        assert!(truthy(&json!(1)));
        assert!(truthy(&json!("1")));
        assert!(!truthy(&json!(false)));
        assert!(!truthy(&json!(0)));
        assert!(!truthy(&json!("0")));
        assert!(!truthy(&json!("")));
    }

    #[test]
    fn same_normalises_bool_spellings_and_empty() {
        assert!(same("1", "1"));
        assert!(same("1", "true"));
        assert!(same("0", "false"));
        assert!(same("0", "")); // disabled reads back as ""
        assert!(same("", "false"));
        assert!(!same("1", "")); // enabled != disabled
        assert!(!same("1", "0"));
        assert!(same("https://x", "https://x"));
        assert!(!same("https://x", "https://y"));
    }

    #[test]
    fn in_sync_is_a_string_normalised_subset() {
        // live returns EVERY field as a string, plus fields we don't manage.
        let wire = json!({ "DISPNAME": "x", "ENABLED": true, "DLPRIORITY": 0, "HOST": "h" });
        let live = json!({ "DISPNAME": "x", "ENABLED": "1", "DLPRIORITY": "0",
                           "HOST": "h", "APICOUNT": "3" });
        assert!(in_sync(&wire, &live), "typed wire should match string live");

        // enabled true vs disabled ("") → drift
        let disabled = json!({ "DISPNAME": "x", "ENABLED": "", "DLPRIORITY": "0", "HOST": "h" });
        assert!(!in_sync(&wire, &disabled));

        // a declared field with a different value → drift
        let moved = json!({ "DISPNAME": "x", "ENABLED": "1", "DLPRIORITY": "0", "HOST": "other" });
        assert!(!in_sync(&wire, &moved));

        // a *disabled* provider we want disabled → in sync (0 == "")
        let want_off = json!({ "DISPNAME": "x", "ENABLED": false });
        assert!(in_sync(&want_off, &disabled));
    }
}
