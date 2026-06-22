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
            .danger_accept_invalid_certs(self.insecure)
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
