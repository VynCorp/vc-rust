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

    /// Get a person by their ID.
    pub async fn get(&self, id: &str) -> Result<Response<Person>> {
        self.client
            .request(Method::GET, &format!("/persons/{id}"))
            .await
    }

    /// Search for persons by name.
    pub async fn search(&self, params: &PersonSearchParams) -> Result<Response<Vec<Person>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request_with_body(Method::POST, "/persons/search", params)
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }
}
