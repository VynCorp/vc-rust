use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::{ScreeningRequest, ScreeningResponse};

pub struct Screening<'a> {
    client: &'a Client,
}

impl<'a> Screening<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn screen(&self, req: &ScreeningRequest) -> Result<Response<ScreeningResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/screening", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, ScreeningRequest};

    #[tokio::test]
    async fn test_screening_clear() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/screening")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"query_name":"Test Corp","query_uid":null,"screened_at":"2026-03-30T12:00:00Z","hit_count":0,"risk_level":"clear","hits":[],"sources_checked":["seco","opensanctions","finma"]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = ScreeningRequest {
            name: "Test Corp".into(),
            uid: None,
            sources: None,
        };
        let resp = client.screening().screen(&req).await.unwrap();
        assert_eq!(resp.data.risk_level, "clear");
        assert_eq!(resp.data.hit_count, 0);
        assert_eq!(resp.data.sources_checked.len(), 3);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_screening_with_hits() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/screening")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"query_name":"Bad Corp","query_uid":null,"screened_at":"2026-03-30T12:00:00Z","hit_count":1,"risk_level":"high","hits":[{"source":"seco","matched_name":"Bad Corp Ltd","entity_type":"entity","score":0.95,"datasets":["seco-sanctions"],"details":{}}],"sources_checked":["seco"]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = ScreeningRequest {
            name: "Bad Corp".into(),
            uid: None,
            sources: Some(vec!["seco".into()]),
        };
        let resp = client.screening().screen(&req).await.unwrap();
        assert_eq!(resp.data.risk_level, "high");
        assert_eq!(resp.data.hits.len(), 1);
        assert!(resp.data.hits[0].score > 0.9);
        mock.assert_async().await;
    }
}
