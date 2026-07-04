use core_macros::nested;

/// An external filter check — autobrr calls a webhook or runs a command and
/// gates the release on the result.
#[nested(case = snake)]
pub struct ExternalFilter {
    /// Server-assigned id.
    pub id: Option<i32>,
    /// Display name.
    pub name: Option<String>,
    /// Evaluation order among external checks.
    pub index: Option<i32>,
    /// Check kind: `EXEC` or `WEBHOOK`.
    #[wire(name = "type")]
    pub external_type: Option<String>,
    /// Whether the check is active.
    pub enabled: Option<bool>,
    /// Webhook URL (`WEBHOOK` type).
    pub webhook_host: Option<String>,
    /// HTTP method for the webhook.
    pub webhook_method: Option<String>,
    /// Request body sent to the webhook.
    pub webhook_data: Option<String>,
    /// HTTP status the webhook must return to pass.
    pub webhook_expect_status: Option<i32>,
    /// Command to run (`EXEC` type).
    pub exec_cmd: Option<String>,
    /// Arguments passed to the command.
    pub exec_args: Option<String>,
    /// Exit status the command must return to pass.
    pub exec_expect_status: Option<i32>,
}
