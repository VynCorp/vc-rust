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

    /// Run K-Means clustering on companies.
    pub async fn cluster(&self, req: &ClusterRequest) -> Result<Response<serde_json::Value>> {
        self.client
            .request_with_body(Method::POST, "/analytics/cluster", req)
            .await
    }

    /// Run on-demand anomaly detection.
    pub async fn anomalies(&self, req: &AnomalyRequest) -> Result<Response<serde_json::Value>> {
        self.client
            .request_with_body(Method::POST, "/analytics/anomalies", req)
            .await
    }

    /// Get cohort analytics.
    pub async fn cohorts(&self, params: &CohortParams) -> Result<Response<serde_json::Value>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref g) = params.group_by {
            query.push(("groupBy", g.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }

        if query.is_empty() {
            self.client.request(Method::GET, "/analytics/cohorts").await
        } else {
            self.client
                .request_with_params(Method::GET, "/analytics/cohorts", &query)
                .await
        }
    }

    /// Get canton analytics.
    pub async fn cantons(&self) -> Result<Response<serde_json::Value>> {
        self.client.request(Method::GET, "/analytics/cantons").await
    }

    /// Get auditor analytics.
    pub async fn auditors(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/analytics/auditors")
            .await
    }

    /// Get RFM segmentation.
    pub async fn rfm_segments(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/analytics/rfm-segments")
            .await
    }

    /// Get change velocity analytics.
    pub async fn velocity(&self, days: Option<u32>) -> Result<Response<serde_json::Value>> {
        match days {
            Some(d) => {
                let params = [("days", d.to_string())];
                self.client
                    .request_with_params(Method::GET, "/analytics/velocity", &params)
                    .await
            }
            None => {
                self.client
                    .request(Method::GET, "/analytics/velocity")
                    .await
            }
        }
    }
}
