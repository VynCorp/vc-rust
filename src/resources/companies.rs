use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Companies<'a> {
    client: &'a Client,
}

impl<'a> Companies<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        params: &CompanyListParams,
    ) -> Result<Response<PagedResponse<Company>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.search {
            query.push(("search", s.clone()));
        }
        if let Some(ref c) = params.canton {
            query.push(("canton", c.clone()));
        }
        if let Some(ref cs) = params.changed_since {
            query.push(("changed_since", cs.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client.request(Method::GET, "/v1/companies").await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/companies", &query)
                .await
        }
    }

    pub async fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}"))
            .await
    }

    pub async fn count(&self) -> Result<Response<CompanyCount>> {
        self.client
            .request(Method::GET, "/v1/companies/count")
            .await
    }

    pub async fn events(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<EventListResponse>> {
        let path = format!("/v1/companies/{uid}/events");
        if let Some(l) = limit {
            self.client
                .request_with_params(Method::GET, &path, &[("limit", l.to_string())])
                .await
        } else {
            self.client.request(Method::GET, &path).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_companies_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies?canton=ZH&page=1&pageSize=20")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"items":[{"uid":"CHE-100.023.968","name":"Test AG","canton":"ZH","status":"active"}],"total":1,"page":1,"pageSize":20}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let params = crate::CompanyListParams {
            canton: Some("ZH".into()),
            page: Some(1),
            page_size: Some(20),
            ..Default::default()
        };
        let resp = client.companies().list(&params).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert_eq!(resp.data.items[0].uid, "CHE-100.023.968");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"uid":"CHE-100.023.968","name":"Test AG","canton":"ZH","status":"active","shareCapital":100000.0}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.companies().get("CHE-100.023.968").await.unwrap();
        assert_eq!(resp.data.name, "Test AG");
        assert_eq!(resp.data.share_capital, Some(100000.0));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_get_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-000.000.000")
            .with_status(404)
            .with_header("content-type", "application/problem+json")
            .with_body(
                r#"{"type":"not-found","title":"Not Found","status":404,"detail":"Company not found"}"#,
            )
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .max_retries(0)
            .build()
            .unwrap();
        let err = client.companies().get("CHE-000.000.000").await.unwrap_err();
        assert!(matches!(err, crate::VyncoError::NotFound(_)));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_count() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/count")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count":507234}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.companies().count().await.unwrap();
        assert_eq!(resp.data.count, 507234);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_events() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/events?limit=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"events":[{"id":"evt-1","ce_type":"company.auditor.changed","ce_source":"https://api.vynco.ch","ce_time":"2026-03-01T00:00:00Z","company_uid":"CHE-100.023.968","company_name":"Test AG","category":"auditor_change","severity":"medium","summary":"Auditor changed","detail_json":{},"created_at":"2026-03-01T00:00:00Z"}],"count":1}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .events("CHE-100.023.968", Some(10))
            .await
            .unwrap();
        assert_eq!(resp.data.count, 1);
        assert_eq!(resp.data.events[0].category, "auditor_change");
        mock.assert_async().await;
    }
}
