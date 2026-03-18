use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Billing<'a> {
    client: &'a Client,
}

impl<'a> Billing<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a Stripe checkout session for upgrading to a tier.
    pub async fn create_checkout(
        &self,
        req: &CheckoutRequest,
    ) -> Result<Response<SessionUrlResponse>> {
        self.client
            .request_with_body(Method::POST, "/billing/checkout-session", req)
            .await
    }

    /// Create a Stripe billing portal session for managing the subscription.
    pub async fn create_portal(&self) -> Result<Response<SessionUrlResponse>> {
        self.client
            .request_with_body(
                Method::POST,
                "/billing/portal-session",
                &serde_json::json!({}),
            )
            .await
    }
}
