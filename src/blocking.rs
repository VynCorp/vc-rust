//! Synchronous (blocking) client for the VynCo API.
//!
//! Enabled via the `blocking` feature flag. Wraps the async [`crate::Client`]
//! with an internal single-threaded Tokio runtime.
//!
//! **Important:** Do not use from within an async context — the internal
//! `block_on()` will panic if a Tokio runtime is already running.

use std::time::Duration;

use crate::error::{Result, VyncoError};
use crate::response::{Response, ResponseMeta};
use crate::types::*;

/// Builder for configuring and constructing a blocking [`Client`].
pub struct ClientBuilder {
    api_key: String,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl ClientBuilder {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
            timeout: None,
            max_retries: None,
        }
    }

    /// Set the API base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries on 429/5xx.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the blocking client.
    pub fn build(self) -> Result<Client> {
        let mut builder = crate::Client::builder(self.api_key);
        if let Some(url) = self.base_url {
            builder = builder.base_url(url);
        }
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(max_retries) = self.max_retries {
            builder = builder.max_retries(max_retries);
        }
        let inner = builder.build()?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| VyncoError::Config(format!("failed to create runtime: {e}")))?;

        Ok(Client { inner, rt })
    }
}

/// Synchronous client for the VynCo API.
///
/// Wraps the async client with a single-threaded Tokio runtime.
pub struct Client {
    inner: crate::Client,
    rt: tokio::runtime::Runtime,
}

impl Client {
    /// Create a new [`ClientBuilder`] with the given API key.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        self.rt.block_on(f)
    }

    pub fn companies(&self) -> Companies<'_> {
        Companies { client: self }
    }

    pub fn persons(&self) -> Persons<'_> {
        Persons { client: self }
    }

    pub fn dossiers(&self) -> Dossiers<'_> {
        Dossiers { client: self }
    }

    pub fn changes(&self) -> Changes<'_> {
        Changes { client: self }
    }

    pub fn analytics(&self) -> Analytics<'_> {
        Analytics { client: self }
    }

    pub fn api_keys(&self) -> ApiKeys<'_> {
        ApiKeys { client: self }
    }

    pub fn credits(&self) -> Credits<'_> {
        Credits { client: self }
    }

    pub fn billing(&self) -> Billing<'_> {
        Billing { client: self }
    }

    pub fn watches(&self) -> Watches<'_> {
        Watches { client: self }
    }

    pub fn news(&self) -> News<'_> {
        News { client: self }
    }

    pub fn reports(&self) -> Reports<'_> {
        Reports { client: self }
    }

    pub fn relationships(&self) -> Relationships<'_> {
        Relationships { client: self }
    }

    pub fn teams(&self) -> Teams<'_> {
        Teams { client: self }
    }

    pub fn health(&self) -> Health<'_> {
        Health { client: self }
    }
}

// ---------------------------------------------------------------------------
// Resource wrappers
// ---------------------------------------------------------------------------

pub struct Companies<'a> {
    client: &'a Client,
}

impl Companies<'_> {
    pub fn list(
        &self,
        params: &CompanyListParams,
    ) -> Result<Response<PaginatedResponse<Company>>> {
        self.client
            .block_on(self.client.inner.companies().list(params))
    }

    pub fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client
            .block_on(self.client.inner.companies().get(uid))
    }

    pub fn count(&self, params: &CompanyCountParams) -> Result<Response<CompanyCount>> {
        self.client
            .block_on(self.client.inner.companies().count(params))
    }

    pub fn statistics(&self) -> Result<Response<CompanyStatistics>> {
        self.client
            .block_on(self.client.inner.companies().statistics())
    }

    pub fn search(&self, req: &CompanySearchRequest) -> Result<Response<Vec<Company>>> {
        self.client
            .block_on(self.client.inner.companies().search(req))
    }

    pub fn batch(&self, req: &BatchCompanyRequest) -> Result<Response<Vec<Company>>> {
        self.client
            .block_on(self.client.inner.companies().batch(req))
    }

    pub fn compare(&self, req: &CompareCompaniesRequest) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.companies().compare(req))
    }
}

pub struct Persons<'a> {
    client: &'a Client,
}

impl Persons<'_> {
    pub fn list(&self, params: &PersonListParams) -> Result<Response<Vec<Person>>> {
        self.client
            .block_on(self.client.inner.persons().list(params))
    }

    pub fn get(&self, id: &str) -> Result<Response<Person>> {
        self.client
            .block_on(self.client.inner.persons().get(id))
    }

    pub fn roles(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        self.client
            .block_on(self.client.inner.persons().roles(id))
    }

    pub fn connections(&self, id: &str) -> Result<Response<Vec<serde_json::Value>>> {
        self.client
            .block_on(self.client.inner.persons().connections(id))
    }

    pub fn board_members(&self, uid: &str) -> Result<Response<Vec<Person>>> {
        self.client
            .block_on(self.client.inner.persons().board_members(uid))
    }

    pub fn network_stats(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.persons().network_stats())
    }
}

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl Dossiers<'_> {
    pub fn list(&self) -> Result<Response<Vec<Dossier>>> {
        self.client
            .block_on(self.client.inner.dossiers().list())
    }

    pub fn get(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().get(uid))
    }

    pub fn generate(
        &self,
        uid: &str,
        req: &GenerateDossierRequest,
    ) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().generate(uid, req))
    }

    pub fn statistics(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.dossiers().statistics())
    }
}

pub struct Changes<'a> {
    client: &'a Client,
}

impl Changes<'_> {
    pub fn list(
        &self,
        params: &ChangeListParams,
    ) -> Result<Response<PaginatedResponse<CompanyChange>>> {
        self.client
            .block_on(self.client.inner.changes().list(params))
    }

    pub fn by_company(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        self.client
            .block_on(self.client.inner.changes().by_company(uid))
    }

    pub fn statistics(&self) -> Result<Response<ChangeStatistics>> {
        self.client
            .block_on(self.client.inner.changes().statistics())
    }

    pub fn by_sogc(&self, sogc_id: &str) -> Result<Response<Vec<CompanyChange>>> {
        self.client
            .block_on(self.client.inner.changes().by_sogc(sogc_id))
    }

    pub fn review(
        &self,
        id: &str,
        req: &ReviewChangeRequest,
    ) -> Result<Response<ReviewChangeResponse>> {
        self.client
            .block_on(self.client.inner.changes().review(id, req))
    }

    pub fn batch(&self, req: &BatchChangeRequest) -> Result<Response<Vec<CompanyChange>>> {
        self.client
            .block_on(self.client.inner.changes().batch(req))
    }
}

pub struct Analytics<'a> {
    client: &'a Client,
}

impl Analytics<'_> {
    pub fn cluster(&self, req: &ClusterRequest) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().cluster(req))
    }

    pub fn anomalies(&self, req: &AnomalyRequest) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().anomalies(req))
    }

    pub fn cohorts(&self, params: &CohortParams) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().cohorts(params))
    }

    pub fn cantons(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().cantons())
    }

    pub fn auditors(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().auditors())
    }

    pub fn rfm_segments(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().rfm_segments())
    }

    pub fn velocity(&self, days: Option<u32>) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.analytics().velocity(days))
    }
}

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl ApiKeys<'_> {
    pub fn list(&self) -> Result<Response<Vec<ApiKey>>> {
        self.client
            .block_on(self.client.inner.api_keys().list())
    }

    pub fn create(&self, req: &CreateApiKeyRequest) -> Result<Response<ApiKeyCreated>> {
        self.client
            .block_on(self.client.inner.api_keys().create(req))
    }

    pub fn revoke(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.api_keys().revoke(id))
    }
}

pub struct Credits<'a> {
    client: &'a Client,
}

impl Credits<'_> {
    pub fn balance(&self) -> Result<Response<CreditBalance>> {
        self.client
            .block_on(self.client.inner.credits().balance())
    }

    pub fn usage(&self, since: Option<&str>) -> Result<Response<UsageBreakdown>> {
        self.client
            .block_on(self.client.inner.credits().usage(since))
    }

    pub fn history(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Response<Vec<CreditLedgerEntry>>> {
        self.client
            .block_on(self.client.inner.credits().history(limit, offset))
    }
}

pub struct Billing<'a> {
    client: &'a Client,
}

impl Billing<'_> {
    pub fn create_checkout(&self, req: &CheckoutRequest) -> Result<Response<SessionUrlResponse>> {
        self.client
            .block_on(self.client.inner.billing().create_checkout(req))
    }

    pub fn create_portal(&self) -> Result<Response<SessionUrlResponse>> {
        self.client
            .block_on(self.client.inner.billing().create_portal())
    }
}

pub struct Watches<'a> {
    client: &'a Client,
}

impl Watches<'_> {
    pub fn list(&self) -> Result<Response<Vec<CompanyWatch>>> {
        self.client
            .block_on(self.client.inner.watches().list())
    }

    pub fn create(&self, req: &CreateWatchRequest) -> Result<Response<CompanyWatch>> {
        self.client
            .block_on(self.client.inner.watches().create(req))
    }

    pub fn remove(&self, company_uid: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.watches().remove(company_uid))
    }

    pub fn notifications(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<ChangeNotification>>> {
        self.client
            .block_on(self.client.inner.watches().notifications(limit))
    }
}

pub struct News<'a> {
    client: &'a Client,
}

impl News<'_> {
    pub fn for_company(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<CompanyNewsResponse>> {
        self.client
            .block_on(self.client.inner.news().for_company(uid, limit))
    }

    pub fn recent(&self, limit: Option<u32>) -> Result<Response<RecentNewsResponse>> {
        self.client
            .block_on(self.client.inner.news().recent(limit))
    }
}

pub struct Reports<'a> {
    client: &'a Client,
}

impl Reports<'_> {
    pub fn for_company(
        &self,
        uid: &str,
        limit: Option<u32>,
    ) -> Result<Response<CompanyReportsResponse>> {
        self.client
            .block_on(self.client.inner.reports().for_company(uid, limit))
    }
}

pub struct Relationships<'a> {
    client: &'a Client,
}

impl Relationships<'_> {
    pub fn for_company(&self, uid: &str) -> Result<Response<RelationshipResponse>> {
        self.client
            .block_on(self.client.inner.relationships().for_company(uid))
    }

    pub fn hierarchy(&self, uid: &str) -> Result<Response<RelationshipResponse>> {
        self.client
            .block_on(self.client.inner.relationships().hierarchy(uid))
    }
}

pub struct Teams<'a> {
    client: &'a Client,
}

impl Teams<'_> {
    pub fn me(&self) -> Result<Response<Team>> {
        self.client.block_on(self.client.inner.teams().me())
    }

    pub fn create(&self, req: &CreateTeamRequest) -> Result<Response<Team>> {
        self.client
            .block_on(self.client.inner.teams().create(req))
    }

    pub fn members(&self) -> Result<Response<Vec<TeamMember>>> {
        self.client
            .block_on(self.client.inner.teams().members())
    }

    pub fn invite_member(&self, req: &InviteMemberRequest) -> Result<Response<TeamMember>> {
        self.client
            .block_on(self.client.inner.teams().invite_member(req))
    }

    pub fn update_member_role(
        &self,
        id: &str,
        req: &UpdateMemberRoleRequest,
    ) -> Result<Response<TeamMember>> {
        self.client
            .block_on(self.client.inner.teams().update_member_role(id, req))
    }

    pub fn remove_member(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.teams().remove_member(id))
    }

    pub fn billing_summary(&self) -> Result<Response<BillingSummary>> {
        self.client
            .block_on(self.client.inner.teams().billing_summary())
    }
}

pub struct Health<'a> {
    client: &'a Client,
}

impl Health<'_> {
    pub fn check(&self) -> Result<Response<HealthResponse>> {
        self.client
            .block_on(self.client.inner.health().check())
    }
}
