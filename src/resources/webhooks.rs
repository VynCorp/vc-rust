use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Webhooks<'a> {
    client: &'a Client,
}

impl<'a> Webhooks<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Response<Vec<WebhookSubscription>>> {
        self.client.request(Method::GET, "/v1/webhooks").await
    }

    pub async fn create(
        &self,
        req: &CreateWebhookRequest,
    ) -> Result<Response<CreateWebhookResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/webhooks", req)
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<WebhookSubscription>> {
        self.client
            .request_with_body(Method::PUT, &format!("/v1/webhooks/{id}"), req)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/webhooks/{id}"))
            .await
    }

    pub async fn test(&self, id: &str) -> Result<Response<TestDeliveryResponse>> {
        self.client
            .request_with_body(
                Method::POST,
                &format!("/v1/webhooks/{id}/test"),
                &serde_json::Value::Object(serde_json::Map::new()),
            )
            .await
    }

    pub async fn deliveries(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<Vec<WebhookDelivery>>> {
        let path = format!("/v1/webhooks/{id}/deliveries");
        if let Some(l) = limit {
            self.client
                .request_with_params(Method::GET, &path, &[("limit", l.to_string())])
                .await
        } else {
            self.client.request(Method::GET, &path).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateWebhookRequest, UpdateWebhookRequest};

    #[tokio::test]
    async fn test_webhooks_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/webhooks")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"webhook":{"id":"wh-123","url":"https://example.com/hook","description":"Test","eventFilters":["auditor_change"],"companyFilters":[],"status":"active","createdAt":"2026-03-30T12:00:00Z","updatedAt":"2026-03-30T12:00:00Z"},"signingSecret":"whsec_abc123"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateWebhookRequest {
            url: "https://example.com/hook".into(),
            description: Some("Test".into()),
            event_filters: Some(vec!["auditor_change".into()]),
            company_filters: None,
        };
        let resp = client.webhooks().create(&req).await.unwrap();
        assert_eq!(resp.data.webhook.id, "wh-123");
        assert_eq!(resp.data.signing_secret, "whsec_abc123");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_webhooks_update() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PUT", "/v1/webhooks/wh-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"wh-123","url":"https://example.com/hook","description":"Updated","eventFilters":[],"companyFilters":[],"status":"paused","createdAt":"2026-03-30T12:00:00Z","updatedAt":"2026-03-30T13:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = UpdateWebhookRequest {
            status: Some("paused".into()),
            description: Some("Updated".into()),
            ..Default::default()
        };
        let resp = client.webhooks().update("wh-123", &req).await.unwrap();
        assert_eq!(resp.data.status, "paused");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_webhooks_test_delivery() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/webhooks/wh-123/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":true,"httpStatus":200,"error":null}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.webhooks().test("wh-123").await.unwrap();
        assert!(resp.data.success);
        assert_eq!(resp.data.http_status, Some(200));
        mock.assert_async().await;
    }
}
