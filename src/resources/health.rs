use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::HealthResponse;

pub struct Health<'a> {
    client: &'a Client,
}

impl<'a> Health<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn check(&self) -> Result<Response<HealthResponse>> {
        self.client.request(Method::GET, "/health").await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_health_check() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("X-Request-Id", "req_abc123")
            .with_body(
                r#"{"status":"ok","database":"connected","redis":"connected","version":"1.5.0"}"#,
            )
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.health().check().await.unwrap();
        assert_eq!(resp.data.status, "ok");
        assert_eq!(resp.data.database, "connected");
        assert_eq!(resp.data.version, "1.5.0");
        assert_eq!(resp.meta.request_id.as_deref(), Some("req_abc123"));
        mock.assert_async().await;
    }
}
