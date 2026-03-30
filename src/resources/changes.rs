use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::Response;
use crate::types::*;

pub struct Changes<'a> {
    client: &'a Client,
}

impl<'a> Changes<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        params: &ChangeListParams,
    ) -> Result<Response<PagedResponse<CompanyChange>>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref t) = params.change_type {
            query.push(("type", t.clone()));
        }
        if let Some(ref s) = params.since {
            query.push(("since", s.clone()));
        }
        if let Some(ref u) = params.until {
            query.push(("until", u.clone()));
        }
        if let Some(ref cs) = params.company_search {
            query.push(("companySearch", cs.clone()));
        }
        if let Some(p) = params.page {
            query.push(("page", p.to_string()));
        }
        if let Some(ps) = params.page_size {
            query.push(("pageSize", ps.to_string()));
        }
        if query.is_empty() {
            self.client.request(Method::GET, "/v1/changes").await
        } else {
            self.client
                .request_with_params(Method::GET, "/v1/changes", &query)
                .await
        }
    }

    pub async fn by_company(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        self.client
            .request(Method::GET, &format!("/v1/changes/{uid}"))
            .await
    }

    pub async fn statistics(&self) -> Result<Response<ChangeStatistics>> {
        self.client
            .request(Method::GET, "/v1/changes/statistics")
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_changes_statistics() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/changes/statistics")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"totalChanges":12345,"changesThisWeek":234,"changesThisMonth":1050,"byType":{"name_change":500,"auditor_change":300}}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.changes().statistics().await.unwrap();
        assert_eq!(resp.data.total_changes, 12345);
        assert_eq!(resp.data.changes_this_week, 234);
        mock.assert_async().await;
    }
}
