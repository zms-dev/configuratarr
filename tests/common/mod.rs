#[allow(dead_code)]
use serde_json::{Value, json};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub struct StarrMockServer {
    server: MockServer,
}

#[allow(dead_code)]
impl StarrMockServer {
    pub async fn start() -> Self {
        Self {
            server: MockServer::start().await,
        }
    }

    pub fn url(&self) -> String {
        self.server.uri()
    }

    pub async fn mock_status(&self, api_prefix: &str, app_name: &str, status_code: u16) {
        Mock::given(method("GET"))
            .and(path(format!("{}/system/status", api_prefix)))
            .respond_with(ResponseTemplate::new(status_code).set_body_json(json!({
                "appName": app_name,
                "version": "1.0.0"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_login(&self, status_code: u16) {
        Mock::given(method("POST"))
            .and(path("/login"))
            .respond_with(
                ResponseTemplate::new(status_code)
                    .insert_header("Set-Cookie", "sid=mock_cookie_val"),
            )
            .mount(&self.server)
            .await;
    }

    pub async fn mock_get(&self, mock_path: &str, response: Value) {
        Mock::given(method("GET"))
            .and(path(mock_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_post(&self, mock_path: &str, expected_body: Value, response: Value) {
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(201).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_put(&self, mock_path: &str, expected_body: Value, response: Value) {
        Mock::given(method("PUT"))
            .and(path(mock_path))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_delete(&self, mock_path: &str) {
        Mock::given(method("DELETE"))
            .and(path(mock_path))
            .respond_with(ResponseTemplate::new(200))
            .mount(&self.server)
            .await;
    }
}

#[test]
fn dummy_test() {}
