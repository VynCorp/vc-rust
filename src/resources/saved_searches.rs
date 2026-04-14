use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct SavedSearches<'a> {
    client: &'a Client,
}

impl<'a> SavedSearches<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all saved searches.
    pub async fn list(&self) -> Result<Response<Vec<SavedSearch>>> {
        self.client.request(Method::GET, "/v1/saved-searches").await
    }

    /// Create a new saved search.
    pub async fn create(&self, req: &CreateSavedSearchRequest) -> Result<Response<SavedSearch>> {
        self.client
            .request_with_body(Method::POST, "/v1/saved-searches", req)
            .await
    }

    /// Get a saved search by ID.
    pub async fn get(&self, id: &str) -> Result<Response<SavedSearch>> {
        self.client
            .request(Method::GET, &format!("/v1/saved-searches/{id}"))
            .await
    }

    /// Update a saved search.
    pub async fn update(
        &self,
        id: &str,
        req: &UpdateSavedSearchRequest,
    ) -> Result<Response<SavedSearch>> {
        self.client
            .request_with_body(Method::PUT, &format!("/v1/saved-searches/{id}"), req)
            .await
    }

    /// Delete a saved search.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/saved-searches/{id}"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateSavedSearchRequest};

    #[tokio::test]
    async fn test_saved_searches_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/saved-searches")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"ss-1","name":"ZH Pharma","description":null,"searchParams":{"canton":"ZH"},"isScheduled":false,"scheduleFrequency":null,"lastRunAt":null,"lastResultCount":null,"createdAt":"2026-04-01T00:00:00Z","updatedAt":"2026-04-01T00:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.saved_searches().list().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].name, "ZH Pharma");
        assert!(!resp.data[0].is_scheduled);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_saved_searches_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/saved-searches")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"ss-2","name":"New Search","description":"A test search","searchParams":{"canton":"BE"},"isScheduled":true,"scheduleFrequency":"daily","lastRunAt":null,"lastResultCount":null,"createdAt":"2026-04-01T00:00:00Z","updatedAt":"2026-04-01T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateSavedSearchRequest {
            name: "New Search".into(),
            search_params: serde_json::json!({"canton": "BE"}),
            description: Some("A test search".into()),
            is_scheduled: true,
            schedule_frequency: Some("daily".into()),
        };
        let resp = client.saved_searches().create(&req).await.unwrap();
        assert_eq!(resp.data.name, "New Search");
        assert!(resp.data.is_scheduled);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_saved_searches_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/saved-searches/ss-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"ss-1","name":"ZH Pharma","description":null,"searchParams":{"canton":"ZH"},"isScheduled":false,"scheduleFrequency":null,"lastRunAt":null,"lastResultCount":null,"createdAt":"2026-04-01T00:00:00Z","updatedAt":"2026-04-01T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.saved_searches().get("ss-1").await.unwrap();
        assert_eq!(resp.data.id, "ss-1");
        assert_eq!(resp.data.name, "ZH Pharma");
        mock.assert_async().await;
    }
}
