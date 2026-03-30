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

    pub async fn create(&self, req: &CreateApiKeyRequest) -> Result<Response<ApiKeyCreated>> {
        self.client
            .request_with_body(Method::POST, "/v1/api-keys", req)
            .await
    }

    pub async fn list(&self) -> Result<Response<Vec<ApiKey>>> {
        self.client.request(Method::GET, "/v1/api-keys").await
    }

    pub async fn revoke(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/api-keys/{id}"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_api_keys_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/api-keys")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"key-1","name":"My Key","prefix":"vc_test_","environment":"sandbox","scopes":["read"],"status":"active","expiresAt":null,"createdAt":"2026-03-01T00:00:00Z","lastUsedAt":null},{"id":"key-2","name":"Prod Key","prefix":"vc_live_","environment":"production","scopes":["read","write"],"status":"active","expiresAt":null,"createdAt":"2026-03-15T00:00:00Z","lastUsedAt":"2026-03-29T00:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.api_keys().list().await.unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].id, "key-1");
        assert_eq!(resp.data[1].name, "Prod Key");
        mock.assert_async().await;
    }
}
