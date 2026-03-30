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
}
