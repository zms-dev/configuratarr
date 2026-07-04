use core_macros::nested;

use core_lib::SecretValue;

use crate::resources::download_client_basic::DownloadClientBasic;
use crate::resources::download_client_rules::DownloadClientRules;

/// Client-specific settings nested under a [`DownloadClient`](crate::resources::download_client::DownloadClient).
#[nested(case = snake)]
pub struct DownloadClientSettings {
    /// API key (arr-style clients that authenticate by key rather than user/pass).
    pub apikey: Option<SecretValue>,
    /// HTTP basic-auth credentials, if the client's endpoint is protected.
    pub basic: Option<DownloadClientBasic>,
    /// Throughput/queue rules applied before pushing releases.
    pub rules: Option<DownloadClientRules>,
    /// Id of another download client to delegate to (proxy setups).
    pub external_download_client_id: Option<i32>,
}
