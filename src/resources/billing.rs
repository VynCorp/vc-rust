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

    pub async fn create_checkout(&self, req: &CheckoutRequest) -> Result<Response<SessionUrl>> {
        self.client
            .request_with_body(Method::POST, "/v1/billing/checkout-session", req)
            .await
    }

    pub async fn create_portal(&self) -> Result<Response<SessionUrl>> {
        self.client
            .request_with_body(
                Method::POST,
                "/v1/billing/portal-session",
                &serde_json::Value::Object(serde_json::Map::new()),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckoutRequest, Client};

    #[tokio::test]
    async fn test_billing_checkout() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/billing/checkout-session")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"url":"https://checkout.stripe.com/session/cs_test_123"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CheckoutRequest {
            tier: "pro".into(),
        };
        let resp = client.billing().create_checkout(&req).await.unwrap();
        assert!(resp.data.url.contains("stripe.com"));
        mock.assert_async().await;
    }
}
