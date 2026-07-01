use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use secrecy::{ExposeSecret, SecretString};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    inner: reqwest::Client,
}

pub struct HttpClientBuilder {
    base_url: String,
    headers: Vec<(&'static str, SecretString)>,
    timeout: Duration,
    insecure: bool,
}

impl HttpClient {
    pub fn builder(base_url: impl Into<String>) -> HttpClientBuilder {
        HttpClientBuilder {
            base_url: base_url.into(),
            headers: Vec::new(),
            timeout: Duration::from_secs(30),
            insecure: false,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        self.parse_json(resp, &url).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .post(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        self.parse_json(resp, &url).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .put(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PUT {url}"))?;
        self.parse_json(resp, &url).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .patch(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PATCH {url}"))?;
        self.parse_json(resp, &url).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .delete(&url)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;
        self.check_status(resp, &url).await
    }

    async fn parse_json<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
        url: &str,
    ) -> Result<T> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status}: {body}");
        }
        resp.json::<T>()
            .await
            .with_context(|| format!("deserializing response from {url}"))
    }

    async fn check_status(&self, resp: reqwest::Response, _url: &str) -> Result<()> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status}: {body}");
        }
        Ok(())
    }
}

/// Build the rustls config reqwest uses (`use_preconfigured_tls`). We drive TLS
/// ourselves so the trust anchors are the **bundled** webpki roots rather than
/// the OS trust store — reqwest 0.13's default (`rustls-platform-verifier`)
/// reads the system store, which is absent in the CA-less nix build sandbox and
/// aborts client construction. The ring provider avoids aws-lc/cmake.
///
/// `insecure` swaps root verification for a no-op verifier — the opt-in escape
/// hatch for a self-signed *arr instance (the old `danger_accept_invalid_certs`).
fn tls_config(insecure: bool) -> rustls::ClientConfig {
    let builder = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("ring provider supports the default protocol versions");

    if insecure {
        builder
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(danger::NoVerifier))
            .with_no_client_auth()
    } else {
        let mut roots = rustls::RootCertStore::empty();
        roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        builder.with_root_certificates(roots).with_no_client_auth()
    }
}

/// Certificate verifier that accepts anything — only wired in when the caller
/// asks for `insecure`. Isolated in its own module so the `dangerous` surface is
/// obvious and greppable.
mod danger {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use rustls::{DigitallySignedStruct, Error, SignatureScheme};

    #[derive(Debug)]
    pub struct NoVerifier;

    impl ServerCertVerifier for NoVerifier {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp_response: &[u8],
            _now: UnixTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            vec![
                SignatureScheme::RSA_PKCS1_SHA256,
                SignatureScheme::RSA_PKCS1_SHA384,
                SignatureScheme::RSA_PKCS1_SHA512,
                SignatureScheme::ECDSA_NISTP256_SHA256,
                SignatureScheme::ECDSA_NISTP384_SHA384,
                SignatureScheme::RSA_PSS_SHA256,
                SignatureScheme::RSA_PSS_SHA384,
                SignatureScheme::RSA_PSS_SHA512,
                SignatureScheme::ED25519,
            ]
        }
    }
}

impl HttpClientBuilder {
    pub fn header(mut self, name: &'static str, value: SecretString) -> Self {
        self.headers.push((name, value));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn insecure(mut self) -> Self {
        self.insecure = true;
        self
    }

    pub fn build(self) -> Result<HttpClient> {
        let mut headers = HeaderMap::new();
        for (name, value) in &self.headers {
            let key = HeaderName::from_bytes(name.as_bytes())
                .with_context(|| format!("invalid header name: {name}"))?;
            let mut val = HeaderValue::from_str(value.expose_secret())
                .with_context(|| format!("invalid header value for {name}"))?;
            val.set_sensitive(true);
            headers.insert(key, val);
        }

        let inner = reqwest::Client::builder()
            .timeout(self.timeout)
            .default_headers(headers)
            .use_preconfigured_tls(tls_config(self.insecure))
            .build()
            .context("building HTTP client")?;

        Ok(HttpClient {
            base_url: self.base_url,
            inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let client = HttpClient::builder("http://localhost:7878")
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_with_header() {
        let client = HttpClient::builder("http://localhost:7878")
            .header("X-Api-Key", SecretString::new("test-key".into()))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_insecure() {
        let client = HttpClient::builder("https://localhost:7878")
            .insecure()
            .build()
            .unwrap();
        assert_eq!(client.base_url, "https://localhost:7878");
    }

    #[test]
    fn builder_custom_timeout() {
        let client = HttpClient::builder("http://localhost:7878")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_invalid_header_name_errors() {
        let result = HttpClient::builder("http://localhost:7878")
            .header("invalid header!", SecretString::new("val".into()))
            .build();
        assert!(result.is_err());
    }
}
