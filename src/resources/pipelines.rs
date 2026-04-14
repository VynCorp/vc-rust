use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Pipelines<'a> {
    client: &'a Client,
}

impl<'a> Pipelines<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all pipelines.
    pub async fn list(&self) -> Result<Response<Vec<Pipeline>>> {
        self.client.request(Method::GET, "/v1/pipelines").await
    }

    /// Create a new pipeline with optional custom stages.
    pub async fn create(&self, req: &CreatePipelineRequest) -> Result<Response<Pipeline>> {
        self.client
            .request_with_body(Method::POST, "/v1/pipelines", req)
            .await
    }

    /// Get a pipeline with all its entries.
    pub async fn get(&self, id: &str) -> Result<Response<PipelineWithEntries>> {
        self.client
            .request(Method::GET, &format!("/v1/pipelines/{id}"))
            .await
    }

    /// Delete a pipeline.
    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/pipelines/{id}"))
            .await
    }

    /// Add a company to a pipeline.
    pub async fn add_entry(
        &self,
        id: &str,
        req: &AddEntryRequest,
    ) -> Result<Response<PipelineEntry>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/pipelines/{id}/entries"), req)
            .await
    }

    /// Update a pipeline entry.
    pub async fn update_entry(
        &self,
        id: &str,
        entry_id: &str,
        req: &UpdateEntryRequest,
    ) -> Result<Response<PipelineEntry>> {
        self.client
            .request_with_body(
                Method::PUT,
                &format!("/v1/pipelines/{id}/entries/{entry_id}"),
                req,
            )
            .await
    }

    /// Remove an entry from a pipeline.
    pub async fn remove_entry(&self, id: &str, entry_id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(
                Method::DELETE,
                &format!("/v1/pipelines/{id}/entries/{entry_id}"),
            )
            .await
    }

    /// Get aggregate statistics for a pipeline.
    pub async fn stats(&self, id: &str) -> Result<Response<PipelineStats>> {
        self.client
            .request(Method::GET, &format!("/v1/pipelines/{id}/stats"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreatePipelineRequest};

    #[tokio::test]
    async fn test_pipelines_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/pipelines")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"pipe-1","teamId":"team-1","name":"Sales","stages":["Lead","Qualified","Won"],"createdAt":1700000000}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.pipelines().list().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].name, "Sales");
        assert_eq!(resp.data[0].stages.len(), 3);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_pipelines_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/pipelines")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"pipe-2","teamId":"team-1","name":"New Pipeline","stages":["Stage1"],"createdAt":1700000000}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreatePipelineRequest {
            name: "New Pipeline".into(),
            stages: Some(vec!["Stage1".into()]),
        };
        let resp = client.pipelines().create(&req).await.unwrap();
        assert_eq!(resp.data.name, "New Pipeline");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_pipelines_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/pipelines/pipe-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"pipe-1","teamId":"team-1","name":"Sales","stages":["Lead","Won"],"createdAt":1700000000,"entries":[],"totalEntries":0}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.pipelines().get("pipe-1").await.unwrap();
        assert_eq!(resp.data.id, "pipe-1");
        assert_eq!(resp.data.total_entries, 0);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_pipelines_stats() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/pipelines/pipe-1/stats")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"byStage":{"Lead":5,"Won":3},"byTier":{"1":2,"2":6},"total":8}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.pipelines().stats("pipe-1").await.unwrap();
        assert_eq!(resp.data.total, 8);
        assert_eq!(*resp.data.by_stage.get("Lead").unwrap(), 5);
        mock.assert_async().await;
    }
}
