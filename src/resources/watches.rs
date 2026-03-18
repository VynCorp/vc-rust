use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Watches<'a> {
    client: &'a Client,
}

impl<'a> Watches<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all watched companies.
    pub async fn list(&self) -> Result<Response<Vec<CompanyWatch>>> {
        let resp: Response<serde_json::Value> =
            self.client.request(Method::GET, "/watches").await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Add a company watch.
    pub async fn create(
        &self,
        req: &CreateWatchRequest,
    ) -> Result<Response<CompanyWatch>> {
        self.client
            .request_with_body(Method::POST, "/watches", req)
            .await
    }

    /// Remove a company watch.
    pub async fn remove(&self, company_uid: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/watches/{company_uid}"))
            .await
    }

    /// List change notifications.
    pub async fn notifications(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<ChangeNotification>>> {
        let resp: Response<serde_json::Value> = match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client
                    .request_with_params(Method::GET, "/notifications", &params)
                    .await?
            }
            None => {
                self.client
                    .request(Method::GET, "/notifications")
                    .await?
            }
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }
}
