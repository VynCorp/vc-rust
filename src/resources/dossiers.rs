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

    /// List all generated dossiers.
    pub async fn list(&self) -> Result<Response<Vec<Dossier>>> {
        let resp: Response<serde_json::Value> =
            self.client.request(Method::GET, "/dossiers").await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Get the most recent dossier for a company.
    pub async fn get(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .request(Method::GET, &format!("/dossiers/{uid}"))
            .await
    }

    /// Generate an AI dossier for a company.
    ///
    /// Types: "standard" (40 credits) or "comprehensive" (100 credits).
    pub async fn generate(
        &self,
        uid: &str,
        req: &GenerateDossierRequest,
    ) -> Result<Response<Dossier>> {
        self.client
            .request_with_body(Method::POST, &format!("/dossiers/{uid}/generate"), req)
            .await
    }

    /// Get dossier generation statistics.
    pub async fn statistics(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .request(Method::GET, "/dossiers/statistics")
            .await
    }
}
