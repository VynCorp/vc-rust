use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Watchlists<'a> {
    client: &'a Client,
}

impl<'a> Watchlists<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Response<Vec<WatchlistSummary>>> {
        self.client.request(Method::GET, "/v1/watchlists").await
    }

    pub async fn create(&self, req: &CreateWatchlistRequest) -> Result<Response<Watchlist>> {
        self.client
            .request_with_body(Method::POST, "/v1/watchlists", req)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/watchlists/{id}"))
            .await
    }

    pub async fn companies(&self, id: &str) -> Result<Response<WatchlistCompaniesResponse>> {
        self.client
            .request(Method::GET, &format!("/v1/watchlists/{id}/companies"))
            .await
    }

    pub async fn add_companies(
        &self,
        id: &str,
        req: &AddCompaniesRequest,
    ) -> Result<Response<AddCompaniesResponse>> {
        self.client
            .request_with_body(Method::POST, &format!("/v1/watchlists/{id}/companies"), req)
            .await
    }

    pub async fn remove_company(&self, id: &str, uid: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(
                Method::DELETE,
                &format!("/v1/watchlists/{id}/companies/{uid}"),
            )
            .await
    }

    pub async fn events(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<EventListResponse>> {
        let path = format!("/v1/watchlists/{id}/events");
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
    use crate::{AddCompaniesRequest, Client, CreateWatchlistRequest};

    #[tokio::test]
    async fn test_watchlists_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/watchlists")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"wl-123","name":"My Watchlist","description":"Test","created_at":"2026-03-30T12:00:00Z","updated_at":"2026-03-30T12:00:00Z"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = CreateWatchlistRequest {
            name: "My Watchlist".into(),
            description: Some("Test".into()),
        };
        let resp = client.watchlists().create(&req).await.unwrap();
        assert_eq!(resp.data.id, "wl-123");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/watchlists")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"wl-123","name":"My Watchlist","description":"","company_count":5,"created_at":"2026-03-30T12:00:00Z"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.watchlists().list().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].company_count, 5);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_add_companies() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/watchlists/wl-123/companies")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"added":2}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = AddCompaniesRequest {
            uids: vec!["CHE-100.023.968".into(), "CHE-105.805.080".into()],
        };
        let resp = client
            .watchlists()
            .add_companies("wl-123", &req)
            .await
            .unwrap();
        assert_eq!(resp.data.added, 2);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_watchlists_delete() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/v1/watchlists/wl-123")
            .with_status(204)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        client.watchlists().delete("wl-123").await.unwrap();
        mock.assert_async().await;
    }
}
