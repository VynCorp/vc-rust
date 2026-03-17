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

    /// List all webhooks.
    pub async fn list(&self) -> Result<Response<Vec<Webhook>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, "/webhooks")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Create a new webhook subscription.
    pub async fn create(&self, req: &CreateWebhookRequest) -> Result<Response<WebhookCreated>> {
        self.client
            .request_with_body(Method::POST, "/webhooks", req)
            .await
    }

    /// Get a webhook by its ID.
    pub async fn get(&self, id: &str) -> Result<Response<Webhook>> {
        self.client
            .request(Method::GET, &format!("/webhooks/{id}"))
            .await
    }

    /// Update an existing webhook.
    pub async fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<Webhook>> {
        self.client
            .request_with_body(Method::PUT, &format!("/webhooks/{id}"), req)
            .await
    }

    /// Delete a webhook by its ID.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/webhooks/{id}"))
            .await
    }

    /// Send a test event to a webhook.
    pub async fn test(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::POST, &format!("/webhooks/{id}/test"))
            .await
    }
}
