use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl<'a> Dossiers<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Generate an AI dossier for a company.
    ///
    /// Levels: "summary" (20 credits), "standard" (50 credits),
    /// "comprehensive" (100 credits).
    pub async fn generate(
        &self,
        uid: &str,
        req: &GenerateDossierRequest,
    ) -> Result<Response<Dossier>> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            uid: &'a str,
            level: &'a str,
        }
        let body = Body {
            uid,
            level: &req.level,
        };
        self.client
            .request_with_body(Method::POST, "/dossiers", &body)
            .await
    }
}
