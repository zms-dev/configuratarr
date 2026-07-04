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
