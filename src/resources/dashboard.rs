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
            .with_body(r#"{"generatedAt":"2026-03-30T12:00:00Z","data":{"totalCompanies":507000,"enrichedCompanies":490000,"companiesWithIndustry":200000,"companiesWithGeo":300000,"totalPersons":150000,"totalChanges":800000,"totalSogcPublications":1200000},"pipelines":[{"id":"zefix-bulk","status":"idle","itemsProcessed":507000,"lastCompletedAt":"2026-03-30T10:00:00Z"}],"auditorTenures":{"totalTracked":45000,"currentAuditors":38000,"tenuresOver10Years":1500,"tenuresOver7Years":3200,"avgTenureYears":5.8,"longestTenure":{"companyUid":"CHE-100.000.001","companyName":"Old Corp AG","auditorName":"KPMG SA","tenureYears":42.0}}}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.dashboard().get().await.unwrap();
        assert_eq!(resp.data.data.total_companies, 507000);
        assert_eq!(resp.data.data.enriched_companies, 490000);
        assert_eq!(resp.data.auditor_tenures.tenures_over_7_years, 3200);
        assert_eq!(resp.data.pipelines[0].id, "zefix-bulk");
        let lt = resp.data.auditor_tenures.longest_tenure.as_ref().unwrap();
        assert_eq!(lt.tenure_years, 42.0);
        mock.assert_async().await;
    }
}
