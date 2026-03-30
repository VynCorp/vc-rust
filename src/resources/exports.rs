use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Exports<'a> {
    client: &'a Client,
}

impl<'a> Exports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, req: &CreateExportRequest) -> Result<Response<ExportJob>> {
        self.client
            .request_with_body(Method::POST, "/v1/exports", req)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Response<ExportDownload>> {
        self.client
            .request(Method::GET, &format!("/v1/exports/{id}"))
            .await
    }

    pub async fn download(&self, id: &str) -> Result<ExportFile> {
        let (bytes, meta, content_type, filename) = self
            .client
            .request_bytes(Method::GET, &format!("/v1/exports/{id}/download"))
            .await?;
        Ok(ExportFile {
            meta,
            bytes,
            content_type,
            filename,
        })
    }
}

/// Downloaded export file with raw bytes and metadata.
#[derive(Debug)]
pub struct ExportFile {
    pub meta: ResponseMeta,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateExportRequest};

    #[tokio::test]
    async fn test_exports_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/exports")
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"exp-123","status":"pending","format":"ndjson","total_rows":null,"file_size_bytes":null,"error_message":null,"created_at":"2026-03-30T12:00:00Z","completed_at":null,"expires_at":null}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateExportRequest {
            format: Some("ndjson".into()),
            canton: Some("ZH".into()),
            ..Default::default()
        };
        let resp = client.exports().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "exp-123");
        assert_eq!(resp.data.status, "pending");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exports_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/exports/exp-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"job":{"id":"exp-123","status":"completed","format":"ndjson","total_rows":100,"file_size_bytes":5000,"created_at":"2026-03-30T12:00:00Z","completed_at":"2026-03-30T12:01:00Z"},"data":null}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.exports().get("exp-123").await.unwrap();
        assert_eq!(resp.data.job.status, "completed");
        assert_eq!(resp.data.job.total_rows, Some(100));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exports_download() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/exports/exp-123/download")
            .with_status(200)
            .with_header("content-type", "application/x-ndjson; charset=utf-8")
            .with_header(
                "content-disposition",
                r#"attachment; filename="export-exp-123.ndjson""#,
            )
            .with_body("{\"uid\":\"CHE-100.023.968\",\"name\":\"Test AG\"}\n")
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let file = client.exports().download("exp-123").await.unwrap();
        assert!(file.content_type.contains("ndjson"));
        assert_eq!(file.filename, "export-exp-123.ndjson");
        let content = String::from_utf8(file.bytes).unwrap();
        assert!(content.contains("CHE-100.023.968"));
        mock.assert_async().await;
    }
}
