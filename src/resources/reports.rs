use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Reports<'a> {
    client: &'a Client,
}

impl<'a> Reports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get financial reports for a company.
    pub async fn for_company(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<CompanyReportsResponse>> {
        match limit {
            Some(l) => {
                let params = [("limit", l.to_string())];
                self.client
                    .request_with_params(
                        Method::GET,
                        &format!("/companies/{uid}/reports"),
                        &params,
                    )
                    .await
            }
            None => {
                self.client
                    .request(Method::GET, &format!("/companies/{uid}/reports"))
                    .await
            }
        }
    }
}
