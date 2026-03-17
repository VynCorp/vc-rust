use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Teams<'a> {
    client: &'a Client,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the current authenticated user's team.
    pub async fn me(&self) -> Result<Response<Team>> {
        self.client.request(Method::GET, "/teams/me").await
    }

    /// Create a new team.
    pub async fn create(&self, req: &CreateTeamRequest) -> Result<Response<Team>> {
        self.client
            .request_with_body(Method::POST, "/teams", req)
            .await
    }
}
