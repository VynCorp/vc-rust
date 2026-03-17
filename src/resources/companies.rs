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

    /// Search companies with filters and pagination.
    pub async fn search(
        &self,
        params: &CompanySearchParams,
    ) -> Result<Response<PaginatedResponse<Company>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref lf) = params.legal_form {
            query.push(("legalForm", lf.clone()));
        }
        if let Some(ref st) = params.status {
            query.push(("status", st.clone()));
        }
        if let Some(ref sb) = params.sort_by {
            query.push(("sortBy", sb.clone()));
        }
        if let Some(sd) = params.sort_desc {
            query.push(("sortDesc", sd.to_string()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
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

    /// Get the total count of companies matching optional filters.
    pub async fn count(
        &self,
        params: &CompanySearchParams,
    ) -> Result<Response<CompanyCount>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref lf) = params.legal_form {
            query.push(("legalForm", lf.clone()));
        }
        if let Some(ref st) = params.status {
            query.push(("status", st.clone()));
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
    pub async fn statistics(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/companies/statistics")
            .await
    }

    /// Get the change history for a specific company.
    pub async fn changes(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/companies/{uid}/changes"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get the board members / persons for a specific company.
    pub async fn persons(&self, uid: &str) -> Result<Response<Vec<PersonRole>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/companies/{uid}/persons"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get the AI-generated dossier for a specific company (if available).
    pub async fn dossier(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}/dossier"))
            .await
    }
}
