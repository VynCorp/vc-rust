use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl<'a> ApiKeys<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all API keys for the current team.
    pub async fn list(&self) -> Result<Response<Vec<ApiKeyInfo>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, "/api-keys")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Create a new API key.
    pub async fn create(&self, req: &CreateApiKeyRequest) -> Result<Response<ApiKeyCreated>> {
        self.client
            .request_with_body(Method::POST, "/api-keys", req)
            .await
    }

    /// Revoke (delete) an API key by its ID.
    pub async fn revoke(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/api-keys/{id}"))
            .await
    }
}
