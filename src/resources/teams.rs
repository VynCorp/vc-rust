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

    /// Get the current team.
    pub async fn me(&self) -> Result<Response<Team>> {
        self.client.request(Method::GET, "/teams/me").await
    }

    /// Create a new team.
    pub async fn create(&self, req: &CreateTeamRequest) -> Result<Response<Team>> {
        self.client
            .request_with_body(Method::POST, "/teams", req)
            .await
    }

    /// List team members.
    pub async fn members(&self) -> Result<Response<Vec<TeamMember>>> {
        let resp: Response<serde_json::Value> = self
            .client
            .request(Method::GET, "/teams/me/members")
            .await?;
        let data = Client::extract_list(resp.data)?;
        Ok(Response {
            data,
            meta: resp.meta,
        })
    }

    /// Invite a new team member.
    pub async fn invite_member(&self, req: &InviteMemberRequest) -> Result<Response<TeamMember>> {
        self.client
            .request_with_body(Method::POST, "/teams/me/members", req)
            .await
    }

    /// Update a team member's role.
    pub async fn update_member_role(
        &self,
        id: &str,
        req: &UpdateMemberRoleRequest,
    ) -> Result<Response<TeamMember>> {
        self.client
            .request_with_body(Method::PUT, &format!("/teams/me/members/{id}"), req)
            .await
    }

    /// Remove a team member.
    pub async fn remove_member(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .request_empty(Method::DELETE, &format!("/teams/me/members/{id}"))
            .await
    }

    /// Get team billing summary.
    pub async fn billing_summary(&self) -> Result<Response<BillingSummary>> {
        self.client
            .request(Method::GET, "/teams/me/billing-summary")
            .await
    }
}
