use reqwest::Method;

use crate::client::Client;
use crate::error::Result;
use crate::response::{Response, ResponseMeta};
use crate::types::*;

pub struct Teams<'a> {
    client: &'a Client,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn me(&self) -> Result<Response<Team>> {
        self.client.request(Method::GET, "/v1/teams/me").await
    }

    pub async fn create(&self, req: &CreateTeamRequest) -> Result<Response<Team>> {
        self.client
            .request_with_body(Method::POST, "/v1/teams", req)
            .await
    }

    pub async fn members(&self) -> Result<Response<Vec<TeamMember>>> {
        self.client
            .request(Method::GET, "/v1/teams/me/members")
            .await
    }

    pub async fn invite_member(&self, req: &InviteMemberRequest) -> Result<Response<Invitation>> {
        self.client
            .request_with_body(Method::POST, "/v1/teams/me/members", req)
            .await
    }

    pub async fn update_member_role(
        &self,
        id: &str,
        req: &UpdateMemberRoleRequest,
    ) -> Result<Response<TeamMember>> {
        self.client
            .request_with_body(Method::PUT, &format!("/v1/teams/me/members/{id}"), req)
            .await
    }

    pub async fn remove_member(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/v1/teams/me/members/{id}"))
            .await
    }

    pub async fn billing_summary(&self) -> Result<Response<BillingSummary>> {
        self.client
            .request(Method::GET, "/v1/teams/me/billing-summary")
            .await
    }

    pub async fn join(&self, req: &JoinTeamRequest) -> Result<Response<JoinTeamResponse>> {
        self.client
            .request_with_body(Method::POST, "/v1/teams/join", req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_teams_me() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/teams/me")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"team-1","name":"Acme Corp","slug":"acme-corp","tier":"pro","creditBalance":4500,"monthlyCredits":5000}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.teams().me().await.unwrap();
        assert_eq!(resp.data.id, "team-1");
        assert_eq!(resp.data.name, "Acme Corp");
        assert_eq!(resp.data.tier, "pro");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_teams_join() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/teams/join")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"teamId":"team-1","teamName":"Acme Corp","role":"member"}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let req = crate::JoinTeamRequest {
            token: "inv-token-123".into(),
        };
        let resp = client.teams().join(&req).await.unwrap();
        assert_eq!(resp.data.team_id, "team-1");
        assert_eq!(resp.data.team_name, "Acme Corp");
        assert_eq!(resp.data.role, "member");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_teams_billing_summary() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/teams/me/billing-summary")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"tier":"pro","creditBalance":4500,"monthlyCredits":5000,"usedThisMonth":500,"members":[{"userId":"usr-1","name":"Alice","creditsUsed":300}]}"#)
            .create_async()
            .await;
        let client = Client::builder("vc_test_key")
            .base_url(server.url())
            .build()
            .unwrap();
        let resp = client.teams().billing_summary().await.unwrap();
        assert_eq!(resp.data.tier, "pro");
        assert_eq!(resp.data.used_this_month, 500);
        assert_eq!(resp.data.members.len(), 1);
        assert_eq!(resp.data.members[0].name, "Alice");
        mock.assert_async().await;
    }
}
