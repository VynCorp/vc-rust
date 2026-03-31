use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::DashboardResponse;

pub struct Dashboard<'a> {
    client: &'a Client,
}

impl<'a> Dashboard<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Response<DashboardResponse>> {
        self.client.request(Method::GET, "/v1/dashboard").await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_dashboard_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/dashboard")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"generatedAt":"2026-03-30T12:00:00Z","data":{"totalCompanies":507000,"withCanton":500000,"withStatus":495000,"withLegalForm":490000,"withCapital":300000,"withIndustry":200000,"withAuditor":150000,"completenessPct":72.5},"pipelines":[],"auditorTenures":{"totalTenures":45000,"longTenures7plus":3200,"avgTenureYears":5.8,"maxTenureYears":42.0}}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.dashboard().get().await.unwrap();
        assert_eq!(resp.data.data.total_companies, 507000);
        assert_eq!(resp.data.auditor_tenures.long_tenures_7plus, 3200);
        mock.assert_async().await;
    }
}
