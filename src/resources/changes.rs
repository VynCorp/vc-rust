use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Changes<'a> {
    client: &'a Client,
}

impl<'a> Changes<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List company changes with optional filtering and pagination.
    pub async fn list(
        &self,
        params: &ChangeListParams,
    ) -> Result<Response<PaginatedResponse<CompanyChange>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if let Some(ref uid) = params.company_uid {
            query.push(("companyUid", uid.clone()));
        }

        if query.is_empty() {
            self.client.request(Method::GET, "/changes").await
        } else {
            self.client
                .request_with_params(Method::GET, "/changes", &query)
                .await
        }
    }

    /// Get all changes for a specific company.
    pub async fn by_company(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/changes/{uid}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get change statistics.
    pub async fn statistics(&self) -> Result<Response<ChangeStatistics>> {
        self.client
            .request(Method::GET, "/changes/statistics")
            .await
    }

    /// Get changes by SOGC publication ID.
    pub async fn by_sogc(&self, sogc_id: &str) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/changes/sogc/{sogc_id}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Mark a change as reviewed.
    pub async fn review(
        &self,
        id: &str,
        req: &ReviewChangeRequest,
    ) -> Result<Response<ReviewChangeResponse>> {
        self.client
            .request_with_body(Method::PUT, &format!("/changes/{id}/review"), req)
            .await
    }

    /// Batch fetch changes for multiple companies.
    pub async fn batch(
        &self,
        req: &BatchChangeRequest,
    ) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request_with_body(Method::POST, "/changes/batch", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }
}
