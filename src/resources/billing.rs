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
        tier: &str,
    ) -> Result<Response<CheckoutSessionResponse>> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            tier: &'a str,
        }
        self.client
            .request_with_body(Method::POST, "/billing/checkout", &Body { tier })
            .await
    }

    /// Create a Stripe billing portal session for managing the subscription.
    pub async fn create_portal(&self) -> Result<Response<PortalSessionResponse>> {
        self.client
            .request_with_body(Method::POST, "/billing/portal", &serde_json::json!({}))
            .await
    }
}
