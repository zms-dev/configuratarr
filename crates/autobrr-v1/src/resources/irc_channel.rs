use core_lib::SecretValue;
use core_macros::nested;

/// One channel an [`IrcNetwork`](crate::resources::irc_network::IrcNetwork) joins.
/// Runtime state autobrr attaches on read (`id`, `detached`, `monitoring`, …) is
/// not modelled — only the declared identity + key.
#[nested(case = snake)]
pub struct IrcChannel {
    /// Channel name (e.g. `#announce`).
    pub name: String,
    /// Channel key/password, where the channel is protected (write-only).
    pub password: Option<SecretValue>,
}
