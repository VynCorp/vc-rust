use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Credits<'a> {
    client: &'a Client,
}

impl<'a> Credits<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn balance(&self) -> Result<Response<CreditBalance>> {
        self.client
            .request(Method::GET, "/v1/credits/balance")
            .await
    }

    pub async fn usage(&self, since: Option<&str>) -> Result<Response<CreditUsage>> {
        if let Some(s) = since {
            self.client
                .request_with_params(
                    Method::GET,
                    "/v1/credits/usage",
                    &[("since", s.to_string())],
                )
                .await
        } else {
            self.client
                .request(Method::GET, "/v1/credits/usage")
                .await
        }
    }

    pub async fn history(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response<CreditHistory>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = offset {
            query.push(("offset", o.to_string()));
        }
        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/credits/history")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/credits/history", &query)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_credits_balance() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/credits/balance")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"balance":4500,"monthlyCredits":5000,"usedThisMonth":500,"tier":"pro","overageRate":0.05}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.credits().balance().await.unwrap();
        assert_eq!(resp.data.balance, 4500);
        assert_eq!(resp.data.monthly_credits, 5000);
        assert_eq!(resp.data.tier, "pro");
        mock.assert_async().await;
    }
}
