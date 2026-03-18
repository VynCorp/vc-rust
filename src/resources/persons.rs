use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Persons<'a> {
    client: &'a Client,
}

impl<'a> Persons<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List persons with optional search and pagination.
    pub async fn list(&self, params: &PersonListParams) -> Result<Response<Vec<Person>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }

        let resp: Response<serde_json::Value> = if query.is_empty() {
            self.client.request(Method::GET, "/persons").await?
        } else {
            self.client
                .request_with_params(Method::GET, "/persons", &query)
                .await?
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get a person by their ID.
    pub async fn get(&self, id: &str) -> Result<Response<Person>> {
        self.client
            .request(Method::GET, &format!("/persons/{id}"))
            .await
    }

    /// Get all roles held by a person.
    pub async fn roles(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/persons/{id}/roles"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get the network of companies connected to a person.
    pub async fn connections(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/persons/{id}/connections"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get board members of a company.
    pub async fn board_members(&self, uid: &str) -> Result<Response<Vec<Person>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, &format!("/persons/board-members/{uid}"))
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get person network statistics.
    pub async fn network_stats(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/persons/network-stats")
            .await
    }
}
