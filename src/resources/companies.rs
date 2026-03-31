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

    pub async fn statistics(&self) -> Result<Response<CompanyStatistics>> {
        self.client
            .request(Method::GET, "/v1/companies/statistics")
            .await
    }

    pub async fn compare(&self, req: &CompareRequest) -> Result<Response<CompareResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/companies/compare", req)
            .await
    }

    pub async fn news(&self, uid: &str) -> Result<Response<Vec<NewsItem>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/news"))
            .await
    }

    pub async fn reports(&self, uid: &str) -> Result<Response<Vec<CompanyReport>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/reports"))
            .await
    }

    pub async fn relationships(&self, uid: &str) -> Result<Response<Vec<Relationship>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/relationships"))
            .await
    }

    pub async fn hierarchy(&self, uid: &str) -> Result<Response<HierarchyResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/hierarchy"))
            .await
    }

    pub async fn fingerprint(&self, uid: &str) -> Result<Response<Fingerprint>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/fingerprint"))
            .await
    }

    pub async fn nearby(&self, params: &NearbyParams) -> Result<Response<Vec<NearbyCompany>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("lat", params.lat.to_string()));
        query.push(("lng", params.lng.to_string()));
        if let Some(r) = params.radius_km {
            query.push(("radiusKm", r.to_string()));
        }
        if let Some(l) = params.limit {
            query.push(("limit", l.to_string()));
        }
        self.client
            .request_with_params(Method::GET, "/v1/companies/nearby", &query)
            .await
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
    async fn test_companies_compare() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/companies/compare")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"uids":["CHE-100.023.968","CHE-200.000.001"],"names":["Test AG","Example GmbH"],"dimensions":[{"field":"canton","label":"Canton","values":["ZH","BE"]},{"field":"legalForm","label":"Legal Form","values":["AG","GmbH"]}]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = crate::CompareRequest {
            uids: vec!["CHE-100.023.968".into(), "CHE-200.000.001".into()],
        };
        let resp = client.companies().compare(&req).await.unwrap();
        assert_eq!(resp.data.uids.len(), 2);
        assert_eq!(resp.data.dimensions.len(), 2);
        assert_eq!(resp.data.dimensions[0].field, "canton");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_fingerprint() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/fingerprint")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"companyUid":"CHE-100.023.968","name":"Test AG","industrySector":"Finance","industryGroup":null,"industry":null,"sizeCategory":"large","employeeCountEstimate":500,"capitalAmount":100000.0,"capitalCurrency":"CHF","revenue":null,"netIncome":null,"auditorTier":"big4","changeFrequency":5,"boardSize":7,"companyAge":25,"canton":"ZH","legalForm":"AG","hasParentCompany":false,"subsidiaryCount":3,"generatedAt":"2026-03-30T12:00:00Z","fingerprintVersion":"1.0"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .fingerprint("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        assert_eq!(resp.data.board_size, 7);
        assert_eq!(resp.data.canton, "ZH");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_events() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/events?limit=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"events":[{"id":"evt-1","ceType":"company.auditor.changed","ceSource":"https://api.vynco.ch","ceTime":"2026-03-01T00:00:00Z","companyUid":"CHE-100.023.968","companyName":"Test AG","category":"auditor_change","severity":"medium","summary":"Auditor changed","detailJson":{},"createdAt":"2026-03-01T00:00:00Z"}],"count":1}"#)
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
