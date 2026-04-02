use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl<'a> Dossiers<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, req: &CreateDossierRequest) -> Result<Response<Dossier>> {
        self.client
            .request_with_body(Method::POST, "/v1/dossiers", req)
            .await
    }

    pub async fn list(&self) -> Result<Response<Vec<DossierSummary>>> {
        self.client.request(Method::GET, "/v1/dossiers").await
    }

    pub async fn get(&self, id_or_uid: &str) -> Result<Response<Dossier>> {
        self.client
            .request(Method::GET, &format!("/v1/dossiers/{id_or_uid}"))
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/dossiers/{id}"))
            .await
    }

    pub async fn generate(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .request_with_body(
                Method::POST,
                &format!("/v1/dossiers/{uid}/generate"),
                &serde_json::json!({}),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CreateDossierRequest};

    #[tokio::test]
    async fn test_dossiers_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/dossiers")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"dos-1","userId":"usr-1","companyUid":"CHE-100.023.968","companyName":"Test AG","level":"standard","content":"Company overview...","sources":["sogc","zefix"],"createdAt":"2026-03-30T12:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateDossierRequest {
            uid: "CHE-100.023.968".into(),
            level: Some("standard".into()),
        };
        let resp = client.dossiers().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "dos-1");
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        assert_eq!(resp.data.level, "standard");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_dossiers_generate() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/dossiers/CHE-100.023.968/generate")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"dos-gen-1","userId":"usr-1","companyUid":"CHE-100.023.968","companyName":"Test AG","level":"summary","content":"Auto-generated dossier...","sources":["zefix","sogc"],"createdAt":"2026-03-30T14:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.dossiers().generate("CHE-100.023.968").await.unwrap();
        assert_eq!(resp.data.id, "dos-gen-1");
        assert_eq!(resp.data.company_uid, "CHE-100.023.968");
        assert_eq!(resp.data.level, "summary");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_dossiers_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/dossiers")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"dos-1","companyUid":"CHE-100.023.968","companyName":"Test AG","level":"standard","createdAt":"2026-03-30T12:00:00Z"},{"id":"dos-2","companyUid":"CHE-200.000.001","companyName":"Example GmbH","level":"detailed","createdAt":"2026-03-29T12:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.dossiers().list().await.unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].id, "dos-1");
        assert_eq!(resp.data[1].company_name, "Example GmbH");
        mock.assert_async().await;
    }
}
