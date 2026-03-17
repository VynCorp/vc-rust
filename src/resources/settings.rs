use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;

pub struct Settings<'a> {
    client: &'a Client,
}

impl<'a> Settings<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get the user's preferences.
    pub async fn get(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/settings/preferences")
            .await
    }

    /// Update the user's preferences.
    pub async fn update(
        &self,
        preferences: &serde_json::Value,
    ) -> Result<Response<serde_json::Value>> {
        self.client
            .request_with_body(Method::PUT, "/settings/preferences", preferences)
            .await
    }
}
