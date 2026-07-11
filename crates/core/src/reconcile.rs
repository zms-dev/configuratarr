//! Reconcile primitives for `sync = custom` hooks.
//!
//! The [`Custom`](crate::SyncKind::Custom) seam hands a hook the raw
//! `(client, desired, refs, execute)` and trusts it to own its HTTP, ordering,
//! and idempotency. Two mechanics recur in *every* hook and are easy to get
//! wrong: **honouring `execute`** (a preview must perform no writes) and
//! **turning the outcome into report [`Change`]s**. Forget the first in one write
//! path and a `plan` silently mutates the server.
//!
//! These helpers own exactly those two mechanics for the recurring hook *shapes*,
//! so a hook expresses only its service-specific diff/write and can't leak a
//! write into a preview. Service-specific *translation* (wire shaping, bespoke
//! multi-endpoint flows) still lives in the service crate — this is the shared
//! floor beneath it, not a home for per-service logic.
//!
//! Shapes covered:
//! * [`create_only`] — keyed create-or-leave (no update, no prune).
//! * [`upsert`] — keyed create-or-update, no prune (the API doesn't round-trip
//!   writes, so idempotency is a service-supplied predicate, not merge-equality).
//! * [`replace`] — whole-list structural replace (one endpoint owns the set).
//!
//! Genuinely bespoke hooks (multi-endpoint per item, sparse form singletons)
//! keep their own explicit `if execute` branch — that is irreducible, not slop.

use std::future::Future;

use serde_json::Value;

use crate::apply::Change;

/// The identity values already present server-side: the `field` (e.g. `"Name"`,
/// `"AppName"`) of each live item, skipping any that lack it.
pub fn present_keys(live: &[Value], field: &str) -> Vec<String> {
    live.iter()
        .filter_map(|i| i.get(field).and_then(Value::as_str).map(String::from))
        .collect()
}

/// Keyed "create-or-leave" reconcile: for each desired item, [`unchanged`] if a
/// live item with the same `key_field` already exists, else run `create`. No
/// update, no prune — mirrors the *arr "don't delete/modify what you didn't
/// make" caution for server-owned resources (Jellyfin libraries, api keys).
///
/// `create` performs the write and runs **only when `execute`** — the driver
/// owns the preview gate, so the closure can't accidentally write during a plan.
/// Returns one [`Change`] per desired item, in order.
///
/// [`unchanged`]: Change::unchanged
pub async fn create_only<F, Fut>(
    desired: &[Value],
    key_field: &str,
    present: &[String],
    execute: bool,
    create: F,
) -> anyhow::Result<Vec<Change>>
where
    F: Fn(String, Value) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut changes = Vec::with_capacity(desired.len());
    for cfg in desired {
        let key = cfg
            .get(key_field)
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("entry is missing `{key_field}`"))?;

        if present.iter().any(|k| k == key) {
            changes.push(Change::unchanged(key));
            continue;
        }
        if execute {
            create(key.to_string(), cfg.clone()).await?;
        }
        changes.push(Change::created(key));
    }
    Ok(changes)
}

/// Keyed "create-or-update" reconcile, no prune: match each desired item to a
/// live one by `key_field`; if none exists run `create`, else if `in_sync` says
/// it already matches leave it, else run `update`. The recurring shape behind
/// APIs whose collections don't round-trip a write — server-rewritten ids,
/// redacted secrets, enriched sub-arrays — so plain merge-equality would churn
/// forever and the caller supplies its own convergence predicate instead.
///
/// `desired` are already in **wire** form (see [`crate::engine::encode_config`]);
/// `in_sync(desired, live)` decides idempotency; `create(wire)` / `update(live,
/// wire)` perform the writes and run **only when `execute`** — the driver owns
/// the preview gate. `update` receives the live item so it can echo server-owned
/// fields the write must carry back (see [`echo`]) or derive the update path.
/// Returns one [`Change`] per desired item, keyed by `key_field`, in order.
pub async fn upsert<S, C, CFut, U, UFut>(
    desired: &[Value],
    live: &[Value],
    key_field: &str,
    in_sync: S,
    execute: bool,
    create: C,
    update: U,
) -> anyhow::Result<Vec<Change>>
where
    S: Fn(&Value, &Value) -> bool,
    C: Fn(Value) -> CFut,
    CFut: Future<Output = anyhow::Result<()>>,
    U: Fn(&Value, Value) -> UFut,
    UFut: Future<Output = anyhow::Result<()>>,
{
    fn key_of<'v>(v: &'v Value, field: &str) -> Option<&'v str> {
        v.get(field).and_then(Value::as_str)
    }
    let mut changes = Vec::with_capacity(desired.len());
    for wire in desired {
        let key = key_of(wire, key_field)
            .ok_or_else(|| anyhow::anyhow!("entry is missing `{key_field}`"))?;
        let change = match live.iter().find(|l| key_of(l, key_field) == Some(key)) {
            Some(l) if in_sync(wire, l) => Change::unchanged(key),
            Some(l) => {
                if execute {
                    update(l, wire.clone()).await?;
                }
                Change::updated(key)
            }
            None => {
                if execute {
                    create(wire.clone()).await?;
                }
                Change::created(key)
            }
        };
        changes.push(change);
    }
    Ok(changes)
}

/// Copy `key` from `from` into `wire` — a server-owned field an update must echo
/// back in its body (an id the API reads from the payload, a server-rewritten
/// identifier). No-op if `from` lacks `key` or `wire` isn't a JSON object.
pub fn echo(wire: &mut Value, key: &str, from: &Value) {
    if let (Some(obj), Some(v)) = (wire.as_object_mut(), from.get(key)) {
        obj.insert(key.to_string(), v.clone());
    }
}

/// Whole-list structural replace: an API with no per-item id where one endpoint
/// owns the entire set (Jellyfin `/Repositories`). Compares the `desired` set to
/// `live` by an order-insensitive `identity`, and if they differ runs `write`
/// (the full-list POST) — **only when `execute`**. Emits a single [`Change`]
/// under `label`: [`unchanged`](Change::unchanged) when the sets match, else
/// [`updated`](Change::updated).
///
/// `identity` maps one item (desired or live, already in the same wire shape) to
/// a comparable key; the comparison sorts both sides so it ignores order and JSON
/// key ordering.
pub async fn replace<K, F, Fut>(
    desired: &[Value],
    live: &[Value],
    label: &'static str,
    execute: bool,
    identity: impl Fn(&Value) -> K,
    write: F,
) -> anyhow::Result<Vec<Change>>
where
    K: Ord,
    F: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut want: Vec<K> = desired.iter().map(&identity).collect();
    let mut have: Vec<K> = live.iter().map(&identity).collect();
    want.sort();
    have.sort();

    if want == have {
        return Ok(vec![Change::unchanged(label)]);
    }
    if execute {
        write().await?;
    }
    Ok(vec![Change::updated(label)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::cell::Cell;

    #[tokio::test]
    async fn create_only_skips_present_and_honours_execute() {
        let desired = vec![json!({ "app": "a" }), json!({ "app": "b" })];
        let present = vec!["a".to_string()];
        let writes = Cell::new(0);

        let changes = create_only(&desired, "app", &present, true, |_k, _c| {
            writes.set(writes.get() + 1);
            async { Ok(()) }
        })
        .await
        .unwrap();

        assert_eq!(writes.get(), 1); // only "b" created
        assert_eq!(changes[0], Change::unchanged("a"));
        assert_eq!(changes[1], Change::created("b"));
    }

    #[tokio::test]
    async fn create_only_preview_writes_nothing() {
        let desired = vec![json!({ "app": "b" })];
        let writes = Cell::new(0);

        let changes = create_only(&desired, "app", &[], false, |_k, _c| {
            writes.set(writes.get() + 1);
            async { Ok(()) }
        })
        .await
        .unwrap();

        assert_eq!(writes.get(), 0);
        assert_eq!(changes, vec![Change::created("b")]);
    }

    #[tokio::test]
    async fn upsert_creates_updates_and_leaves() {
        // Idempotency: declared subset already present in live (extra live `id`).
        let in_sync = |w: &Value, l: &Value| {
            w.as_object()
                .unwrap()
                .iter()
                .all(|(k, v)| l.get(k) == Some(v))
        };
        let desired = vec![
            json!({ "name": "keep", "port": 1 }),
            json!({ "name": "drift", "port": 2 }),
            json!({ "name": "fresh", "port": 3 }),
        ];
        let live = vec![
            json!({ "name": "keep", "port": 1, "id": 10 }),
            json!({ "name": "drift", "port": 99, "id": 11 }),
        ];
        let creates = Cell::new(0);
        let updates = Cell::new(0);
        let changes = upsert(
            &desired,
            &live,
            "name",
            in_sync,
            true,
            |_w| {
                creates.set(creates.get() + 1);
                async { Ok(()) }
            },
            |_l, _w| {
                updates.set(updates.get() + 1);
                async { Ok(()) }
            },
        )
        .await
        .unwrap();

        assert_eq!(creates.get(), 1);
        assert_eq!(updates.get(), 1);
        assert_eq!(
            changes,
            vec![
                Change::unchanged("keep"),
                Change::updated("drift"),
                Change::created("fresh"),
            ]
        );
    }

    #[tokio::test]
    async fn upsert_preview_writes_nothing() {
        let desired = vec![json!({ "name": "fresh" }), json!({ "name": "drift" })];
        let live = vec![json!({ "name": "drift", "id": 1 })];
        let writes = Cell::new(0);
        let bump = || {
            writes.set(writes.get() + 1);
        };
        let changes = upsert(
            &desired,
            &live,
            "name",
            |_w, _l| false, // everything present is "drifted"
            false,
            |_w| {
                bump();
                async { Ok(()) }
            },
            |_l, _w| {
                bump();
                async { Ok(()) }
            },
        )
        .await
        .unwrap();
        assert_eq!(writes.get(), 0);
        assert_eq!(
            changes,
            vec![Change::created("fresh"), Change::updated("drift")]
        );
    }

    #[test]
    fn echo_copies_live_field_into_wire() {
        let mut wire = json!({ "name": "x" });
        echo(&mut wire, "id", &json!({ "id": 7, "name": "x" }));
        assert_eq!(wire["id"], json!(7));
        // Absent source key → no-op.
        echo(&mut wire, "missing", &json!({ "id": 7 }));
        assert_eq!(wire.get("missing"), None);
    }

    #[tokio::test]
    async fn replace_is_order_insensitive_and_gated() {
        let ident = |v: &Value| v.get("n").and_then(Value::as_i64).unwrap_or(0);
        let live = vec![json!({ "n": 1 }), json!({ "n": 2 })];

        // Same set, different order → unchanged, never writes.
        let same = vec![json!({ "n": 2 }), json!({ "n": 1 })];
        let wrote = Cell::new(false);
        let c = replace(&same, &live, "repos", true, ident, || {
            wrote.set(true);
            async { Ok(()) }
        })
        .await
        .unwrap();
        assert_eq!(c, vec![Change::unchanged("repos")]);
        assert!(!wrote.get());

        // Different set but preview → updated, still no write.
        let diff = vec![json!({ "n": 3 })];
        let wrote = Cell::new(false);
        let c = replace(&diff, &live, "repos", false, ident, || {
            wrote.set(true);
            async { Ok(()) }
        })
        .await
        .unwrap();
        assert_eq!(c, vec![Change::updated("repos")]);
        assert!(!wrote.get());
    }
}
