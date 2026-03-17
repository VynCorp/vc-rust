use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Users<'a> {
    client: &'a Client,
}

impl<'a> Users<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the authenticated user's profile.
    pub async fn me(&self) -> Result<Response<UserProfile>> {
        self.client.request(Method::GET, "/auth/me").await
    }

    /// Update the authenticated user's profile.
    pub async fn update_profile(
        &self,
        req: &UpdateProfileRequest,
    ) -> Result<Response<UserProfile>> {
        self.client
            .request_with_body(Method::PUT, "/auth/profile", req)
            .await
    }
}
