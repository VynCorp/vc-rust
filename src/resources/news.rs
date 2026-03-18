use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct News<'a> {
    client: &'a Client,
}

impl<'a> News<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get news for a specific company.
    pub async fn for_company(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<CompanyNewsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client
                    .request_with_params(Method::GET, &format!("/companies/{uid}/news"), &params)
                    .await
            }
            None => {
                self.client
                    .request(Method::GET, &format!("/companies/{uid}/news"))
                    .await
            }
        }
    }

    /// Get recent news across all companies.
    pub async fn recent(&self, limit: Option<u32>) -> Result<Response<RecentNewsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client
                    .request_with_params(Method::GET, "/news/recent", &params)
                    .await
            }
            None => self.client.request(Method::GET, "/news/recent").await,
        }
    }
}
