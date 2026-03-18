use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Relationships<'a> {
    client: &'a Client,
}

impl<'a> Relationships<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get company relationships (parent, subsidiaries, board overlaps).
    pub async fn for_company(
        &self,
        uid: &str,
    ) -> Result<Response<RelationshipResponse>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}/relationships"))
            .await
    }

    /// Get corporate hierarchy (full recursive parent/subsidiary tree).
    pub async fn hierarchy(
        &self,
        uid: &str,
    ) -> Result<Response<RelationshipResponse>> {
        self.client
            .request(Method::GET, &format!("/companies/{uid}/hierarchy"))
            .await
    }
}
