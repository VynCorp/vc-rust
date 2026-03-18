use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Health<'a> {
    client: &'a Client,
}

impl<'a> Health<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Check API health status.
    pub async fn check(&self) -> Result<Response<HealthResponse>> {
        self.client.request(Method::GET, "/health").await
    }
}
