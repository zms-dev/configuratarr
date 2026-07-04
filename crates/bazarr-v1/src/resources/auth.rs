use core_macros::nested;

/// Web-UI authentication settings (`settings-auth-*`).
///
/// `password` is stored md5-hashed server-side; the reconcile hook hashes the
/// declared plaintext before diffing, so it stays idempotent. The `auth.apikey`
/// is intentionally **not** modelled — rotating the key mid-apply would break the
/// very connection configuratarr is authenticating with.
///
/// The auth `type` (unset = none, `basic`, or `form`) is settable — declare it as
/// `type:` under `auth:`; it is forwarded verbatim (not a struct field here only
/// because `type` is a Rust keyword).
#[nested(case = snake)]
pub struct Auth {
    /// Login username.
    pub username: Option<String>,
    /// Login password (sent in plaintext; bazarr stores it md5-hashed).
    pub password: Option<String>,
}
