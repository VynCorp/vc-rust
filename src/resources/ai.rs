use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Ai<'a> {
    client: &'a Client,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn dossier(&self, req: &DossierRequest) -> Result<Response<DossierResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/dossier", req)
            .await
    }

    pub async fn search(&self, req: &AiSearchRequest) -> Result<Response<AiSearchResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/search", req)
            .await
    }

    pub async fn risk_score(&self, req: &RiskScoreRequest) -> Result<Response<RiskScoreResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/ai/risk-score", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{AiSearchRequest, Client, DossierRequest, RiskScoreRequest};

    #[tokio::test]
    async fn test_ai_dossier() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/ai/dossier")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"uid":"CHE-100.023.968","companyName":"Test AG","dossier":"Test AG is a Swiss company...","sources":["zefix","seco"],"generatedAt":"2026-03-30T12:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = DossierRequest {
            uid: "CHE-100.023.968".into(),
            depth: Some("summary".into()),
        };
        let resp = client.ai().dossier(&req).await.unwrap();
        assert_eq!(resp.data.company_name, "Test AG");
        assert!(!resp.data.dossier.is_empty());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_ai_search() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/ai/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"query":"pharma in Zurich","explanation":"Searching for pharma in ZH","filtersApplied":{},"results":[{"uid":"CHE-100.023.968","name":"Test Pharma AG"}],"total":1}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = AiSearchRequest {
            query: "pharma in Zurich".into(),
        };
        let resp = client.ai().search(&req).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert!(!resp.data.explanation.is_empty());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_ai_risk_score() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/ai/risk-score")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"uid":"CHE-100.023.968","companyName":"Test AG","overallScore":25,"riskLevel":"low","breakdown":[{"factor":"Sanctions","score":0,"weight":0.35,"description":"No hits"}],"assessedAt":"2026-03-30T12:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = RiskScoreRequest {
            uid: "CHE-100.023.968".into(),
        };
        let resp = client.ai().risk_score(&req).await.unwrap();
        assert_eq!(resp.data.overall_score, 25);
        assert_eq!(resp.data.risk_level, "low");
        assert!(!resp.data.breakdown.is_empty());
        mock.assert_async().await;
    }
}
