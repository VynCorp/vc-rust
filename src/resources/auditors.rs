use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Auditors<'a> {
    client: &'a Client,
}

impl<'a> Auditors<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn history(&self, uid: &str) -> Result<Response<AuditorHistoryResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/auditor-history"))
            .await
    }

    pub async fn tenures(
        &self,
        params: &AuditorTenureParams,
    ) -> Result<Response<PagedResponse<AuditorTenure>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(y) = params.min_years {
            query.push(("min_years", y.to_string()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client
                .request(Method::GET, "/v1/auditor-tenures")
                .await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/auditor-tenures", &query)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_auditor_history() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/auditor-history")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"company_uid":"CHE-100.023.968","company_name":"Test AG","current_auditor":{"id":"t1","company_uid":"CHE-100.023.968","company_name":"Test AG","auditor_name":"KPMG AG","appointed_at":"2020-01-01","tenure_years":6.2,"is_current":true,"source":"ZefixRest"},"history":[]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .auditors()
            .history("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        let current = resp.data.current_auditor.unwrap();
        assert_eq!(current.auditor_name, "KPMG AG");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_auditor_tenures() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/v1/auditor-tenures?min_years=10&page=1&pageSize=50",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"items":[{"id":"t1","company_uid":"CHE-100.023.968","company_name":"Test AG","auditor_name":"KPMG AG","tenure_years":11.2,"is_current":true,"source":"ZefixRest"}],"total":1,"page":1,"page_size":50}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let params = crate::AuditorTenureParams {
            min_years: Some(10.0),
            page: Some(1),
            page_size: Some(50),
            ..Default::default()
        };
        let resp = client.auditors().tenures(&params).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert!(resp.data.items[0].tenure_years.unwrap() > 10.0);
        mock.assert_async().await;
    }
}
