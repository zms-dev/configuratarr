use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppType {
    Radarr,
    Sonarr,
    Prowlarr,
    Lidarr,
    Readarr,
}

pub struct StarrClient {
    pub app_type: AppType,
    pub base_url: String,
    pub api_key: Option<String>,
    pub client: reqwest::Client,
}

impl StarrClient {
    pub fn new(app_type: AppType, base_url: &str, api_key: Option<&str>) -> Self {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            app_type,
            base_url: base_url.to_string(),
            api_key: api_key.map(|s| s.to_string()),
            client,
        }
    }

    pub fn api_prefix(&self) -> &'static str {
        match self.app_type {
            AppType::Radarr | AppType::Sonarr => "/api/v3",
            AppType::Prowlarr | AppType::Lidarr | AppType::Readarr => "/api/v1",
        }
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        relative_path: &str,
        body: Option<Value>,
    ) -> Result<reqwest::Response, anyhow::Error> {
        let base = self.base_url.trim_end_matches('/');
        let prefix = self.api_prefix();

        let path = if relative_path.starts_with('/') {
            relative_path.to_string()
        } else {
            format!("/{}", relative_path)
        };

        let full_path = if path.starts_with(prefix) {
            path
        } else {
            format!("{}{}", prefix, path)
        };

        let url = format!("{}{}", base, full_path);
        let mut req = self.client.request(method, &url);

        if let Some(ref key) = self.api_key {
            req = req.header("X-Api-Key", key);
        }

        if let Some(payload) = body {
            req = req.json(&payload);
        }

        let res = req.send().await?;
        Ok(res)
    }

    pub async fn login(&self) -> Result<(), anyhow::Error> {
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{}/login", base);

        let res = self.client.post(&url).send().await?;
        if !res.status().is_success() {
            return Err(map_http_error("POST", "/login", res.status()));
        }
        Ok(())
    }

    pub async fn check_status(&self) -> Result<Value, anyhow::Error> {
        let res = self
            .send_request(reqwest::Method::GET, "/system/status", None)
            .await?;
        if !res.status().is_success() {
            return Err(map_http_error("GET", "/system/status", res.status()));
        }
        let body = res.json::<Value>().await?;
        Ok(body)
    }

    pub async fn wait_for_status(&self, timeout_sec: u64) -> Result<(), anyhow::Error> {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(timeout_sec);

        loop {
            match self.check_status().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if start.elapsed() >= timeout {
                        return Err(anyhow::anyhow!(
                            "Timed out waiting for status after {}s. Last error: {}",
                            timeout_sec,
                            e
                        ));
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        }
    }

    pub async fn get_resources(&self, path: &str) -> Result<Value, anyhow::Error> {
        let res = self.send_request(reqwest::Method::GET, path, None).await?;
        if !res.status().is_success() {
            return Err(map_http_error("GET", path, res.status()));
        }
        let body = res.json::<Value>().await?;
        Ok(body)
    }

    pub async fn create_resource(
        &self,
        path: &str,
        payload: Value,
    ) -> Result<Value, anyhow::Error> {
        let res = self
            .send_request(reqwest::Method::POST, path, Some(payload))
            .await?;
        if !res.status().is_success() {
            return Err(map_http_error("POST", path, res.status()));
        }
        let body = res.json::<Value>().await?;
        Ok(body)
    }

    pub async fn update_resource(
        &self,
        path: &str,
        id: i32,
        payload: Value,
    ) -> Result<Value, anyhow::Error> {
        let trimmed_path = path.trim_end_matches('/');
        let path_with_id = format!("{}/{}", trimmed_path, id);
        let res = self
            .send_request(reqwest::Method::PUT, &path_with_id, Some(payload))
            .await?;
        if !res.status().is_success() {
            return Err(map_http_error("PUT", &path_with_id, res.status()));
        }
        let body = res.json::<Value>().await?;
        Ok(body)
    }

    pub async fn delete_resource(&self, path: &str, id: i32) -> Result<(), anyhow::Error> {
        let trimmed_path = path.trim_end_matches('/');
        let path_with_id = format!("{}/{}", trimmed_path, id);
        let res = self
            .send_request(reqwest::Method::DELETE, &path_with_id, None)
            .await?;
        if !res.status().is_success() {
            return Err(map_http_error("DELETE", &path_with_id, res.status()));
        }
        Ok(())
    }

    pub async fn get_schema(&self, path: &str) -> Result<Value, anyhow::Error> {
        let schema_path = if path.ends_with("schema") {
            path.to_string()
        } else {
            let path_trimmed = path.trim_end_matches('/');
            format!("{}/schema", path_trimmed)
        };
        self.get_resources(&schema_path).await
    }
}

impl AppType {
    pub fn env_prefix(&self) -> &'static str {
        match self {
            Self::Radarr => "RADARR",
            Self::Sonarr => "SONARR",
            Self::Prowlarr => "PROWLARR",
            Self::Lidarr => "LIDARR",
            Self::Readarr => "READARR",
        }
    }
}

pub fn resolve_connection(
    app_type: AppType,
    config: Option<&crate::config::AppConfig>,
    cli_url: Option<&str>,
    cli_api_key: Option<&str>,
) -> Result<(String, Option<String>), anyhow::Error> {
    let mut url = if let Some(u) = cli_url {
        u.to_string()
    } else {
        let app_env = format!("CONFIGURATARR_{}_URL", app_type.env_prefix());
        std::env::var(&app_env)
            .or_else(|_| std::env::var("CONFIGURATARR_URL"))
            .or_else(|_| {
                config
                    .and_then(|c| c.host.as_ref())
                    .map(|h| h.url.clone())
                    .ok_or(anyhow::anyhow!("Connection URL not configured"))
            })?
    };

    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("http://{}", url);
    }

    let api_key = if let Some(key) = cli_api_key {
        Some(key.to_string())
    } else {
        let app_env = format!("CONFIGURATARR_{}_API_KEY", app_type.env_prefix());
        std::env::var(&app_env)
            .or_else(|_| std::env::var("CONFIGURATARR_API_KEY"))
            .ok()
            .or_else(|| {
                config
                    .and_then(|c| c.host.as_ref())
                    .and_then(|h| h.api_key.clone())
            })
    };

    Ok((url, api_key))
}

fn map_http_error(method: &str, path: &str, status: reqwest::StatusCode) -> anyhow::Error {
    let msg = match status {
        reqwest::StatusCode::UNAUTHORIZED => {
            format!(
                "{} '{}' failed: 401 Unauthorized. Please check if your API key is correct or configured.",
                method, path
            )
        }
        reqwest::StatusCode::FORBIDDEN => {
            format!(
                "{} '{}' failed: 403 Forbidden. Access is denied.",
                method, path
            )
        }
        reqwest::StatusCode::NOT_FOUND => {
            format!(
                "{} '{}' failed: 404 Not Found. Please verify the API endpoint exists.",
                method, path
            )
        }
        s => {
            format!("{} '{}' failed with HTTP status: {}", method, path, s)
        }
    };
    anyhow::anyhow!(msg)
}
