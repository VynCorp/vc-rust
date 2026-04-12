use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::{OwnershipRequest, OwnershipResponse};

/// Ownership trace operations.
///
/// For ultimate beneficial owner resolution use [`crate::Companies::ubo`] —
/// this resource exposes the lower-level ownership-chain trace endpoint
/// that walks head-office / branch-office / acquisition relationships
/// upward and detects circular ownership.
pub struct Ownership<'a> {
    client: &'a Client,
}

impl<'a> Ownership<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Trace the ownership chain upward from a company.
    ///
    /// Walks head-office / branch-office relationships up to
    /// `req.max_depth` levels (default 10 on the server, max 20),
    /// detecting circular ownership and identifying key persons.
    pub async fn trace(
        &self,
        uid: &str,
        req: &OwnershipRequest,
    ) -> Result<Response<OwnershipResponse>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/ownership/{uid}"), req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, OwnershipRequest};

    #[tokio::test]
    async fn test_ownership_trace() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/ownership/CHE-101.329.561")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"uid":"CHE-101.329.561","companyName":"UBS SA","ownershipChain":[],"ultimateParent":null,"keyPersons":[],"circularFlags":[],"riskLevel":"low","assessedAt":"2026-04-12T00:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = OwnershipRequest { max_depth: Some(5) };
        let resp = client
            .ownership()
            .trace("CHE-101.329.561", &req)
            .await
            .unwrap();
        assert_eq!(resp.data.uid, "CHE-101.329.561");
        assert_eq!(resp.data.risk_level, "low");
        mock.assert_async().await;
    }
}
