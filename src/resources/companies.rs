use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::resources::ExportFile;
use crate::response::{Response, ResponseMeta};
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
        if let Some(ref s) = params.status {
            query.push(("status", s.clone()));
        }
        if let Some(ref lf) = params.legal_form {
            query.push(("legalForm", lf.clone()));
        }
        if let Some(cm) = params.capital_min {
            query.push(("capitalMin", cm.to_string()));
        }
        if let Some(cx) = params.capital_max {
            query.push(("capitalMax", cx.to_string()));
        }
        if let Some(ref ac) = params.auditor_category {
            query.push(("auditorCategory", ac.clone()));
        }
        if let Some(ref sb) = params.sort_by {
            query.push(("sortBy", sb.clone()));
        }
        if let Some(sd) = params.sort_desc {
            query.push(("sortDesc", sd.to_string()));
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

    pub async fn get_full(&self, uid: &str) -> Result<Response<CompanyFullResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/full"))
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

    pub async fn classification(&self, uid: &str) -> Result<Response<Classification>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/classification"))
            .await
    }

    pub async fn fingerprint(&self, uid: &str) -> Result<Response<Fingerprint>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/fingerprint"))
            .await
    }

    pub async fn structure(&self, uid: &str) -> Result<Response<CorporateStructure>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/structure"))
            .await
    }

    pub async fn acquisitions(&self, uid: &str) -> Result<Response<Vec<Acquisition>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/acquisitions"))
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

    // -- Notes --

    pub async fn notes(&self, uid: &str) -> Result<Response<Vec<Note>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/notes"))
            .await
    }

    pub async fn create_note(&self, uid: &str, req: &CreateNoteRequest) -> Result<Response<Note>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/companies/{uid}/notes"), req)
            .await
    }

    pub async fn update_note(
        &self,
        uid: &str,
        note_id: &str,
        req: &UpdateNoteRequest,
    ) -> Result<Response<Note>> {
        self.client
            .request_with_body(
                Method::PUT,
                &format!("/v1/companies/{uid}/notes/{note_id}"),
                req,
            )
            .await
    }

    pub async fn delete_note(&self, uid: &str, note_id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(
                Method::DELETE,
                &format!("/v1/companies/{uid}/notes/{note_id}"),
            )
            .await
    }

    // -- Tags --

    pub async fn tags(&self, uid: &str) -> Result<Response<Vec<Tag>>> {
        self.client
            .request(Method::GET, &format!("/v1/companies/{uid}/tags"))
            .await
    }

    pub async fn create_tag(&self, uid: &str, req: &CreateTagRequest) -> Result<Response<Tag>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/companies/{uid}/tags"), req)
            .await
    }

    pub async fn delete_tag(&self, uid: &str, tag_id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(
                Method::DELETE,
                &format!("/v1/companies/{uid}/tags/{tag_id}"),
            )
            .await
    }

    pub async fn all_tags(&self) -> Result<Response<Vec<TagSummary>>> {
        self.client.request(Method::GET, "/v1/tags").await
    }

    // -- Excel export --

    pub async fn export_excel(&self, req: &ExcelExportRequest) -> Result<ExportFile> {
        let (bytes, meta, content_type, filename) = self
            .client
            .request_bytes_with_body(Method::POST, "/v1/companies/export/excel", req)
            .await?;
        Ok(ExportFile {
            meta,
            bytes,
            content_type,
            filename,
        })
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

    #[tokio::test]
    async fn test_companies_get_full() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/full")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"company":{"uid":"CHE-100.023.968","name":"Test AG","canton":"ZH"},"persons":[{"personId":"p-1","firstName":"Hans","lastName":"Mueller","role":"CEO","since":"2020-01-01","until":null}],"recentChanges":[{"id":"c-1","companyUid":"CHE-100.023.968","changeType":"capital_change","fieldName":"share_capital","oldValue":"100000","newValue":"200000","detectedAt":"2026-03-01","sourceDate":null}],"relationships":[{"relatedUid":"CHE-200.000.001","relatedName":"Partner GmbH","relationshipType":"shared_person"}]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .get_full("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.company.uid, "CHE-100.023.968");
        assert_eq!(resp.data.persons.len(), 1);
        assert_eq!(resp.data.persons[0].role, "CEO");
        assert_eq!(resp.data.recent_changes.len(), 1);
        assert_eq!(
            resp.data.relationships[0].relationship_type,
            "shared_person"
        );
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_structure() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/structure")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"headOffices":[{"uid":"CHE-300.000.001","name":"Parent AG"}],"branchOffices":[],"acquisitions":[{"uid":"CHE-400.000.001","name":"Acquired GmbH"}],"acquiredBy":[]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .structure("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.head_offices.len(), 1);
        assert_eq!(resp.data.head_offices[0].name, "Parent AG");
        assert_eq!(resp.data.acquisitions.len(), 1);
        assert!(resp.data.branch_offices.is_empty());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_classification() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/classification")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"companyUid":"CHE-100.023.968","sectorCode":"FIN","sectorName":"Financial Services","groupCode":null,"groupName":null,"industryCode":"64","industryName":"Financial Services","subIndustryCode":null,"subIndustryName":null,"method":"KeywordMatch","classifiedAt":"2026-04-01T00:00:00Z","auditorCategory":"big4","isFinmaRegulated":true}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .classification("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        assert_eq!(resp.data.sector_code, Some("FIN".into()));
        assert!(resp.data.is_finma_regulated);
        assert_eq!(resp.data.method, "KeywordMatch");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_acquisitions() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/acquisitions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"acquirerUid":"CHE-100.023.968","acquiredUid":"CHE-200.000.001","acquirerName":"Test AG","acquiredName":"Target GmbH","createdAt":"2025-06-15T00:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .companies()
            .acquisitions("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].acquirer_uid, "CHE-100.023.968");
        assert_eq!(resp.data[0].acquired_name, Some("Target GmbH".into()));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_notes_crud() {
        let mut server = mockito::Server::new_async().await;
        let list_mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/notes")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"note-1","companyUid":"CHE-100.023.968","content":"Interesting pattern","noteType":"note","rating":null,"isPrivate":false,"createdAt":"2026-03-30T12:00:00Z","updatedAt":"2026-03-30T12:00:00Z"}]"#)
            .create_async()
            .await;
        let create_mock = server
            .mock("POST", "/v1/companies/CHE-100.023.968/notes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"note-2","companyUid":"CHE-100.023.968","content":"New note","noteType":"annotation","rating":4,"isPrivate":true,"createdAt":"2026-03-30T13:00:00Z","updatedAt":"2026-03-30T13:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let list_resp = client.companies().notes("CHE-100.023.968").await.unwrap();
        assert_eq!(list_resp.data.len(), 1);
        assert_eq!(list_resp.data[0].content, "Interesting pattern");
        assert!(!list_resp.data[0].is_private);

        let req = crate::CreateNoteRequest {
            content: "New note".into(),
            note_type: Some("annotation".into()),
            rating: Some(4),
            is_private: Some(true),
        };
        let create_resp = client
            .companies()
            .create_note("CHE-100.023.968", &req)
            .await
            .unwrap();
        assert_eq!(create_resp.data.id, "note-2");
        assert_eq!(create_resp.data.rating, Some(4));
        assert!(create_resp.data.is_private);

        list_mock.assert_async().await;
        create_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_tags_crud() {
        let mut server = mockito::Server::new_async().await;
        let list_mock = server
            .mock("GET", "/v1/companies/CHE-100.023.968/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r##"[{"id":"tag-1","companyUid":"CHE-100.023.968","tagName":"risk-high","color":"#FF0000","createdAt":"2026-03-30T12:00:00Z"}]"##)
            .create_async()
            .await;
        let create_mock = server
            .mock("POST", "/v1/companies/CHE-100.023.968/tags")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"tag-2","companyUid":"CHE-100.023.968","tagName":"audit-needed","color":null,"createdAt":"2026-03-30T13:00:00Z"}"#)
            .create_async()
            .await;
        let all_mock = server
            .mock("GET", "/v1/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"tagName":"risk-high","count":12},{"tagName":"audit-needed","count":5}]"#,
            )
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();

        let list_resp = client.companies().tags("CHE-100.023.968").await.unwrap();
        assert_eq!(list_resp.data.len(), 1);
        assert_eq!(list_resp.data[0].tag_name, "risk-high");
        assert_eq!(list_resp.data[0].color, Some("#FF0000".into()));

        let req = crate::CreateTagRequest {
            tag_name: "audit-needed".into(),
            color: None,
        };
        let create_resp = client
            .companies()
            .create_tag("CHE-100.023.968", &req)
            .await
            .unwrap();
        assert_eq!(create_resp.data.tag_name, "audit-needed");

        let all_resp = client.companies().all_tags().await.unwrap();
        assert_eq!(all_resp.data.len(), 2);
        assert_eq!(all_resp.data[0].count, 12);

        list_mock.assert_async().await;
        create_mock.assert_async().await;
        all_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_companies_export_excel() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/companies/export/excel")
            .with_status(200)
            .with_header("content-type", "text/csv")
            .with_header(
                "content-disposition",
                r#"attachment; filename="export.csv""#,
            )
            .with_body("uid,name\nCHE-100.023.968,Test AG\n")
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = crate::ExcelExportRequest {
            uids: Some(vec!["CHE-100.023.968".into()]),
            ..Default::default()
        };
        let file = client.companies().export_excel(&req).await.unwrap();
        assert_eq!(file.content_type, "text/csv");
        assert_eq!(file.filename, "export.csv");
        assert!(String::from_utf8_lossy(&file.bytes).contains("Test AG"));
        mock.assert_async().await;
    }
}
