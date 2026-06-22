//! Resolver traits passed to the two resolve phases.
//!
//! The split into [`StaticEnv`] and [`RefSource`] mirrors the two-phase apply
//! pipeline: static resolution (env vars + file reads + literal templates)
//! happens once up front, ref resolution happens per resource in topological
//! order as the engine learns ids from POST/PUT responses.

/// Read-only access to the static side of resolution.
///
/// `StaticEnv` is consulted during the static resolve phase, before any HTTP
/// I/O. It does not see resource ids — those are produced during apply and
/// flow through [`RefSource`] instead.
pub trait StaticEnv {
    /// Read an environment variable. `None` means unset; callers decide
    /// whether that's an error (e.g. template references it) or fine
    /// (defaulted).
    fn env(&self, name: &str) -> Option<&str>;

    /// Read a file's contents. Used for `${file./path}` templates. Errors
    /// propagate (e.g. permission denied, not found) so the user sees the
    /// real cause.
    fn file(&self, path: &str) -> anyhow::Result<String>;
}

/// The default [`StaticEnv`]: process environment + filesystem.
///
/// Environment is snapshotted at construction (the trait hands back `&str`, so
/// the values must outlive the call); files are read fresh on each lookup.
pub struct SystemEnv {
    vars: std::collections::HashMap<String, String>,
}

impl SystemEnv {
    /// Snapshot the current process environment.
    pub fn new() -> Self {
        Self {
            vars: std::env::vars().collect(),
        }
    }
}

impl Default for SystemEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticEnv for SystemEnv {
    fn env(&self, name: &str) -> Option<&str> {
        self.vars.get(name).map(String::as_str)
    }

    fn file(&self, path: &str) -> anyhow::Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading `${{file.{path}}}`: {e}"))
    }
}

/// Read-only access to the live id map built during apply.
///
/// `RefSource` is consulted during the ref-resolve phase, just before
/// encoding each resource. The engine populates it from each create/update
/// response: after `Tag { label: "4k" }` is POSTed and the server returns
/// `id: 3`, the engine inserts `("tag", "4k") -> 3` so downstream resources
/// referencing `${ref.tag.4k}` can resolve.
pub trait RefSource {
    /// Look up an id by (type, key). `None` means the target hasn't been
    /// applied yet — which is a bug if topo ordering is correct, since the
    /// dep graph guarantees deps come first.
    fn lookup(&self, type_name: &str, key: &str) -> Option<i32>;
}
