use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::resources::exports::ExportFile;
use crate::response::Response;
use crate::types::*;

pub struct Graph<'a> {
    client: &'a Client,
}

impl<'a> Graph<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, uid: &str) -> Result<Response<GraphResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/graph/{uid}"))
            .await
    }

    pub async fn export(&self, uid: &str, format: &str) -> Result<ExportFile> {
        let (bytes, meta, content_type, filename) = self
            .client
            .request_bytes(
                Method::GET,
                &format!("/v1/graph/{uid}/export?format={format}"),
            )
            .await?;
        Ok(ExportFile {
            meta,
            bytes,
            content_type,
            filename,
        })
    }

    pub async fn analyze(
        &self,
        req: &NetworkAnalysisRequest,
    ) -> Result<Response<NetworkAnalysisResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/network/analyze", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_graph_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/graph/CHE-100.023.968")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"nodes":[{"id":"n1","name":"Test AG","uid":"CHE-100.023.968","type":"company","capital":100000.0,"canton":"ZH","status":"active","role":null,"personId":null},{"id":"n2","name":"Hans Mueller","uid":"","type":"person","capital":null,"canton":null,"status":null,"role":"Director","personId":"p-1"}],"links":[{"source":"n2","target":"n1","type":"board_member","label":"Director"}]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.graph().get("CHE-100.023.968").await.unwrap();
        assert_eq!(resp.data.nodes.len(), 2);
        assert_eq!(resp.data.links.len(), 1);
        assert_eq!(resp.data.nodes[0].name, "Test AG");
        assert_eq!(resp.data.links[0].link_type, "board_member");
        mock.assert_async().await;
    }
}
