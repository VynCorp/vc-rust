use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Reports<'a> {
    client: &'a Client,
}

impl<'a> Reports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all industries with available reports and company counts.
    pub async fn industries(&self) -> Result<Response<IndustryListResponse>> {
        self.client
            .request(Method::GET, "/v1/reports/industries")
            .await
    }

    /// Get a detailed industry report with analytics.
    pub async fn get(&self, industry: &str) -> Result<Response<IndustryReportResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/reports/industry/{industry}"))
            .await
    }

    /// Generate an AI-powered narrative industry report.
    pub async fn generate(&self, industry: &str) -> Result<Response<GeneratedIndustryReport>> {
        self.client
            .request(
                Method::POST,
                &format!("/v1/reports/industry/{industry}/generate"),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_reports_industries() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/reports/industries")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"industries":[{"industry":"Finance","companyCount":1200}],"total":1}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.reports().industries().await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert_eq!(resp.data.industries[0].industry, "Finance");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_reports_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/reports/industry/Finance")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"industry":"Finance","companyCount":1200,"avgCapital":500000.0,"medianCapital":250000.0,"topCompanies":[],"cantonDistribution":[],"recentChanges":42,"auditorConcentration":[],"statusDistribution":[],"generatedAt":"2026-04-01T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.reports().get("Finance").await.unwrap();
        assert_eq!(resp.data.industry, "Finance");
        assert_eq!(resp.data.company_count, 1200);
        assert_eq!(resp.data.recent_changes, 42);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_reports_generate() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/reports/industry/Finance/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"industry":"Finance","report":"The finance industry in Switzerland...","sources":["zefix","seco"],"generatedAt":"2026-04-01T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.reports().generate("Finance").await.unwrap();
        assert_eq!(resp.data.industry, "Finance");
        assert!(!resp.data.report.is_empty());
        assert_eq!(resp.data.sources.len(), 2);
        mock.assert_async().await;
    }
}
