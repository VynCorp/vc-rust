use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Persons<'a> {
    client: &'a Client,
}

impl<'a> Persons<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn board_members(&self, uid: &str) -> Result<Response<Vec<BoardMember>>> {
        self.client
            .request(Method::GET, &format!("/v1/persons/board-members/{uid}"))
            .await
    }

    pub async fn search(
        &self,
        params: &PersonSearchParams,
    ) -> Result<Response<PagedResponse<PersonSearchResult>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = params.q {
            query.push(("q", q.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client.request(Method::GET, "/v1/persons/search").await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/persons/search", &query)
                .await
        }
    }

    pub async fn get(&self, id: &str) -> Result<Response<PersonDetail>> {
        self.client
            .request(Method::GET, &format!("/v1/persons/{id}"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_persons_board_members() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/persons/board-members/CHE-100.023.968")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":"p-1","firstName":"Hans","lastName":"Mueller","role":"Director","roleCategory":"board","origin":"Switzerland","residence":"Zurich","signingAuthority":"sole","since":"2020-01-01"},{"id":"p-2","firstName":"Anna","lastName":"Schmidt","role":"Secretary","roleCategory":"management","origin":"Germany","residence":"Basel","signingAuthority":"collective","since":"2021-06-15"}]"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client
            .persons()
            .board_members("CHE-100.023.968")
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].last_name, Some("Mueller".into()));
        assert_eq!(resp.data[1].role, "Secretary");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_persons_search() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/persons/search?q=Mueller&page=1&pageSize=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"items":[{"id":"p-1","fullName":"Hans Mueller","firstName":"Hans","lastName":"Mueller","placeOfOrigin":"Zürich","nationality":"CH","roleCount":3}],"total":1,"page":1,"pageSize":10}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let params = crate::PersonSearchParams {
            q: Some("Mueller".into()),
            page: Some(1),
            page_size: Some(10),
        };
        let resp = client.persons().search(&params).await.unwrap();
        assert_eq!(resp.data.total, 1);
        assert_eq!(resp.data.items[0].full_name, "Hans Mueller");
        assert_eq!(resp.data.items[0].role_count, Some(3));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_persons_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/persons/p-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"p-1","fullName":"Hans Mueller","firstName":"Hans","lastName":"Mueller","placeOfOrigin":"Zürich","residence":"Zürich","nationality":"CH","roles":[{"companyUid":"CHE-100.023.968","companyName":"Test AG","roleFunction":"Director","roleCategory":"board","signingAuthority":"sole","startDate":"2020-01-01","endDate":null,"changeAction":null,"isCurrent":true}]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.persons().get("p-1").await.unwrap();
        assert_eq!(resp.data.full_name, "Hans Mueller");
        assert_eq!(resp.data.roles.len(), 1);
        assert_eq!(resp.data.roles[0].company_uid, "CHE-100.023.968");
        assert_eq!(resp.data.roles[0].is_current, Some(true));
        mock.assert_async().await;
    }
}
