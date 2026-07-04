use core_macros::nested;

/// Outbound proxy settings (`settings-proxy-*`).
///
/// The proxy `type` (`socks5` / `http`, or unset to disable) is also settable —
/// declare it as `type:` under `proxy:`; it is forwarded verbatim (it is not a
/// struct field here only because `type` is a Rust keyword).
#[nested(case = snake)]
pub struct Proxy {
    /// Proxy host.
    pub url: Option<String>,
    /// Proxy port.
    pub port: Option<String>,
    /// Proxy username.
    pub username: Option<String>,
    /// Proxy password.
    pub password: Option<String>,
    /// Hosts to reach directly, bypassing the proxy.
    pub exclude: Vec<String>,
}
