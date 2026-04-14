use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

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

    /// Screen up to 100 companies against sanctions lists in a single call.
    pub async fn batch(
        &self,
        req: &BatchScreeningRequest,
    ) -> Result<Response<BatchScreeningResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/screening/batch", req)
            .await
    }

    /// Browse SECO/OpenSanctions/FINMA sanctions databases with search and pagination.
    pub async fn browse_sanctions(
        &self,
        params: &SanctionsSearchParams,
    ) -> Result<Response<SanctionsListResponse>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref et) = params.entity_type {
            query.push(("entityType", et.clone()));
        }
        if let Some(ref p) = params.program {
            query.push(("program", p.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client.request(Method::GET, "/v1/sanctions").await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/sanctions", &query)
                .await
        }
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
            .with_body(r#"{"queryName":"Test Corp","queryUid":null,"screenedAt":"2026-03-30T12:00:00Z","hitCount":0,"riskLevel":"clear","hits":[],"sourcesChecked":["seco","opensanctions","finma"]}"#)
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
            .with_body(r#"{"queryName":"Bad Corp","queryUid":null,"screenedAt":"2026-03-30T12:00:00Z","hitCount":1,"riskLevel":"high","hits":[{"source":"seco","matchedName":"Bad Corp Ltd","entityType":"entity","score":0.95,"datasets":["seco-sanctions"],"details":{}}],"sourcesChecked":["seco"]}"#)
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
