use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::{Alert, CreateAlertRequest};

/// Saved alerts — persistent saved queries that trigger notifications
/// (optionally via webhook) when matching companies or events appear.
pub struct Alerts<'a> {
    client: &'a Client,
}

impl<'a> Alerts<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all alerts for the authenticated user.
    pub async fn list(&self) -> Result<Response<Vec<Alert>>> {
        self.client.request(Method::GET, "/v1/alerts").await
    }

    /// Create a new alert.
    ///
    /// `frequency` accepts `hourly`, `daily`, or `weekly` (default `daily`
    /// on the server). `query_params` is an arbitrary JSON filter.
    pub async fn create(&self, req: &CreateAlertRequest) -> Result<Response<Alert>> {
        self.client
            .request_with_body(Method::POST, "/v1/alerts", req)
            .await
    }

    /// Delete an alert.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/alerts/{id}"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateAlertRequest};

    #[tokio::test]
    async fn test_alerts_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/alerts")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"alert-1","name":"Zurich big capital","queryParams":{"canton":"ZH"},"webhookUrl":null,"frequency":"daily","isActive":true,"savedSearchId":null,"lastTriggeredAt":null,"lastResultCount":null,"triggerCount":0,"createdAt":"2026-04-12T00:00:00Z","updatedAt":"2026-04-12T00:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.alerts().list().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].name, "Zurich big capital");
        assert!(resp.data[0].is_active);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_alerts_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/alerts")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"alert-2","name":"ZH fintech","queryParams":{"canton":"ZH","industry":"fintech"},"webhookUrl":"https://example.com/hook","frequency":"hourly","isActive":true,"savedSearchId":null,"lastTriggeredAt":null,"lastResultCount":null,"triggerCount":0,"createdAt":"2026-04-12T00:00:00Z","updatedAt":"2026-04-12T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateAlertRequest {
            name: "ZH fintech".into(),
            query_params: serde_json::json!({"canton": "ZH", "industry": "fintech"}),
            webhook_url: Some("https://example.com/hook".into()),
            frequency: Some("hourly".into()),
            saved_search_id: None,
        };
        let resp = client.alerts().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "alert-2");
        assert_eq!(resp.data.frequency, "hourly");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_alerts_delete() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/v1/alerts/alert-2")
            .with_status(204)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        client.alerts().delete("alert-2").await.unwrap();
        mock.assert_async().await;
    }
}
