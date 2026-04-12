use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Analytics<'a> {
    client: &'a Client,
}

impl<'a> Analytics<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn cantons(&self) -> Result<Response<Vec<CantonDistribution>>> {
        self.client
            .request(Method::GET, "/v1/analytics/cantons")
            .await
    }

    pub async fn auditors(&self) -> Result<Response<Vec<AuditorMarketShare>>> {
        self.client
            .request(Method::GET, "/v1/analytics/auditors")
            .await
    }

    pub async fn cluster(&self, req: &ClusterRequest) -> Result<Response<ClusterResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/analytics/cluster", req)
            .await
    }

    pub async fn anomalies(&self, req: &AnomalyRequest) -> Result<Response<AnomalyResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/analytics/anomalies", req)
            .await
    }

    pub async fn rfm_segments(&self) -> Result<Response<RfmSegmentsResponse>> {
        self.client
            .request(Method::GET, "/v1/analytics/rfm-segments")
            .await
    }

    pub async fn cohorts(&self, params: &CohortParams) -> Result<Response<CohortResponse>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref g) = params.group_by {
            query.push(("groupBy", g.clone()));
        }
        if let Some(ref m) = params.metric {
            query.push(("metric", m.clone()));
        }
        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/analytics/cohorts")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/analytics/cohorts", &query)
                .await
        }
    }

    pub async fn candidates(
        &self,
        params: &CandidateParams,
    ) -> Result<Response<PagedResponse<AuditCandidate>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.sort_by {
            query.push(("sortBy", s.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/analytics/candidates")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/analytics/candidates", &query)
                .await
        }
    }

    /// Market flow analytics — registrations and dissolutions over time.
    ///
    /// `params.period`: `monthly` (default), `quarterly`, `yearly`.
    /// `params.group_by`: `canton` (default), `industry`, `legalForm`.
    pub async fn flows(&self, params: &FlowsParams) -> Result<Response<FlowsResponse>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref p) = params.period {
            query.push(("period", p.clone()));
        }
        if let Some(ref s) = params.since {
            query.push(("since", s.clone()));
        }
        if let Some(ref g) = params.group_by {
            query.push(("groupBy", g.clone()));
        }
        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/analytics/flows")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/analytics/flows", &query)
                .await
        }
    }

    /// Canton migration analytics — companies moving their legal seat.
    pub async fn migrations(&self, since: Option<&str>) -> Result<Response<MigrationResponse>> {
        if let Some(s) = since {
            self.client
                .request_with_params(
                    Method::GET,
                    "/v1/analytics/migrations",
                    &[("since", s.to_string())],
                )
                .await
        } else {
            self.client
                .request(Method::GET, "/v1/analytics/migrations")
                .await
        }
    }

    /// Benchmark a company against its industry peers.
    ///
    /// Returns percentile ranks for dimensions such as capital, board_size,
    /// change_frequency, and company_age.
    pub async fn benchmark(&self, params: &BenchmarkParams) -> Result<Response<BenchmarkResponse>> {
        let mut query: Vec<(&str, String)> = vec![("uid", params.uid.clone())];
        if let Some(ref d) = params.dimensions {
            query.push(("dimensions", d.clone()));
        }
        self.client
            .request_with_params(Method::GET, "/v1/analytics/benchmark", &query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_analytics_cantons() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/analytics/cantons")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"canton":"ZH","count":80000,"percentage":15.8},{"canton":"BE","count":60000,"percentage":11.8}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.analytics().cantons().await.unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].canton, "ZH");
        assert_eq!(resp.data[0].count, 80000);
        mock.assert_async().await;
    }
}
