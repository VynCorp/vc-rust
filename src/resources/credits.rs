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

    /// Get the current credit balance and tier information.
    pub async fn balance(&self) -> Result<Response<CreditBalance>> {
        self.client.request(Method::GET, "/credits/balance").await
    }

    /// Get credit usage breakdown by operation type.
    ///
    /// Optionally filter by start date (ISO 8601 string).
    pub async fn usage(&self, since: Option<&str>) -> Result<Response<UsageBreakdown>> {
        match since {
            Some(s) => {
                let params = [("since", s.to_string())];
                self.client
                    .request_with_params(Method::GET, "/credits/usage", &params)
                    .await
            }
            None => self.client.request(Method::GET, "/credits/usage").await,
        }
    }

    /// Get credit ledger history.
    pub async fn history(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Response<Vec<CreditLedgerEntry>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = offset {
            query.push(("offset", o.to_string()));
        }

        let resp: Response<serde_json::Value> = if query.is_empty() {
            self.client.request(Method::GET, "/credits/history").await?
        } else {
            self.client
                .request_with_params(Method::GET, "/credits/history", &query)
                .await?
        };
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }
}
