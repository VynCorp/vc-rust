use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Companies<'a> {
    client: &'a Client,
}

impl<'a> Companies<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List companies with optional filtering and pagination.
    pub async fn list(
        &self,
        params: &CompanyListParams,
    ) -> Result<Response<PaginatedResponse<Company>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref st) = params.status {
            query.push(("status", st.clone()));
        }
        if let Some(ref ac) = params.auditor_category {
            query.push(("auditorCategory", ac.clone()));
        }
        if let Some(ref sb) = params.sort_by {
            query.push(("sortBy", sb.clone()));
        }
        if let Some(sd) = params.sort_desc {
            query.push(("sortDesc", sd.to_string()));
        }
        if let Some(ref ts) = params.target_status {
            query.push(("targetStatus", ts.clone()));
        }

        if query.is_empty() {
            self.client.request(Method::GET, "/companies").await
        } else {
            self.client
                .request_with_params(Method::GET, "/companies", &query)
                .await
        }
    }

    /// Get a company by its Swiss UID (e.g. "CHE-100.023.968").
    pub async fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}"))
            .await
    }

    /// Get the count of companies matching optional filters.
    pub async fn count(
        &self,
        params: &CompanyCountParams,
    ) -> Result<Response<CompanyCount>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref s) = params.status {
            query.push(("status", s.clone()));
        }
        if let Some(ref ac) = params.auditor_category {
            query.push(("auditorCategory", ac.clone()));
        }

        if query.is_empty() {
            self.client.request(Method::GET, "/companies/count").await
        } else {
            self.client
                .request_with_params(Method::GET, "/companies/count", &query)
                .await
        }
    }

    /// Get aggregate statistics about companies.
    pub async fn statistics(&self) -> Result<Response<CompanyStatistics>> {
        self.client
            .request(Method::GET, "/companies/statistics")
            .await
    }

    /// Full-text search companies (FTS5).
    pub async fn search(
        &self,
        req: &CompanySearchRequest,
    ) -> Result<Response<Vec<Company>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request_with_body(Method::POST, "/companies/search", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Batch lookup up to 50 companies by UID.
    pub async fn batch(
        &self,
        req: &BatchCompanyRequest,
    ) -> Result<Response<Vec<Company>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request_with_body(Method::POST, "/companies/batch", req)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Compare two or more companies side-by-side.
    pub async fn compare(
        &self,
        req: &CompareCompaniesRequest,
    ) -> Result<Response<serde_json::Value>> {
        self.client
            .request_with_body(Method::POST, "/companies/compare", req)
            .await
    }
}
