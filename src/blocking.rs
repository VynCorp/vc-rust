//! Synchronous (blocking) client for the VynCo API.
//!
//! Enabled via the `blocking` feature flag. Wraps the async [`crate::Client`]
//! with an internal single-threaded Tokio runtime.
//!
//! **Important:** Do not use from within an async context — the internal
//! `block_on()` will panic if a Tokio runtime is already running.

use std::time::Duration;

use crate::error::{Result, VyncoError};
use crate::resources::ExportFile;
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

    pub fn health(&self) -> Health<'_> {
        Health { client: self }
    }

    pub fn companies(&self) -> Companies<'_> {
        Companies { client: self }
    }

    pub fn auditors(&self) -> Auditors<'_> {
        Auditors { client: self }
    }

    pub fn dashboard(&self) -> Dashboard<'_> {
        Dashboard { client: self }
    }

    pub fn screening(&self) -> Screening<'_> {
        Screening { client: self }
    }

    pub fn watchlists(&self) -> Watchlists<'_> {
        Watchlists { client: self }
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    pub fn exports(&self) -> Exports<'_> {
        Exports { client: self }
    }

    pub fn ai(&self) -> Ai<'_> {
        Ai { client: self }
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

    pub fn teams(&self) -> Teams<'_> {
        Teams { client: self }
    }

    pub fn changes(&self) -> Changes<'_> {
        Changes { client: self }
    }

    pub fn persons(&self) -> Persons<'_> {
        Persons { client: self }
    }

    pub fn analytics(&self) -> Analytics<'_> {
        Analytics { client: self }
    }

    pub fn dossiers(&self) -> Dossiers<'_> {
        Dossiers { client: self }
    }

    pub fn graph(&self) -> Graph<'_> {
        Graph { client: self }
    }
}

// ---------------------------------------------------------------------------
// Resource wrappers
// ---------------------------------------------------------------------------

pub struct Health<'a> {
    client: &'a Client,
}

impl Health<'_> {
    pub fn check(&self) -> Result<Response<HealthResponse>> {
        self.client.block_on(self.client.inner.health().check())
    }
}

pub struct Companies<'a> {
    client: &'a Client,
}

impl Companies<'_> {
    pub fn list(&self, params: &CompanyListParams) -> Result<Response<PagedResponse<Company>>> {
        self.client
            .block_on(self.client.inner.companies().list(params))
    }

    pub fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client.block_on(self.client.inner.companies().get(uid))
    }

    pub fn get_full(&self, uid: &str) -> Result<Response<CompanyFullResponse>> {
        self.client
            .block_on(self.client.inner.companies().get_full(uid))
    }

    pub fn count(&self) -> Result<Response<CompanyCount>> {
        self.client.block_on(self.client.inner.companies().count())
    }

    pub fn events(&self, uid: &str, limit: Option<u32>) -> Result<Response<EventListResponse>> {
        self.client
            .block_on(self.client.inner.companies().events(uid, limit))
    }

    pub fn statistics(&self) -> Result<Response<CompanyStatistics>> {
        self.client
            .block_on(self.client.inner.companies().statistics())
    }

    pub fn compare(&self, req: &CompareRequest) -> Result<Response<CompareResponse>> {
        self.client
            .block_on(self.client.inner.companies().compare(req))
    }

    pub fn news(&self, uid: &str) -> Result<Response<Vec<NewsItem>>> {
        self.client
            .block_on(self.client.inner.companies().news(uid))
    }

    pub fn reports(&self, uid: &str) -> Result<Response<Vec<CompanyReport>>> {
        self.client
            .block_on(self.client.inner.companies().reports(uid))
    }

    pub fn relationships(&self, uid: &str) -> Result<Response<Vec<Relationship>>> {
        self.client
            .block_on(self.client.inner.companies().relationships(uid))
    }

    pub fn hierarchy(&self, uid: &str) -> Result<Response<HierarchyResponse>> {
        self.client
            .block_on(self.client.inner.companies().hierarchy(uid))
    }

    pub fn fingerprint(&self, uid: &str) -> Result<Response<Fingerprint>> {
        self.client
            .block_on(self.client.inner.companies().fingerprint(uid))
    }

    pub fn structure(&self, uid: &str) -> Result<Response<CorporateStructure>> {
        self.client
            .block_on(self.client.inner.companies().structure(uid))
    }

    pub fn acquisitions(&self, uid: &str) -> Result<Response<Vec<Acquisition>>> {
        self.client
            .block_on(self.client.inner.companies().acquisitions(uid))
    }

    pub fn nearby(&self, params: &NearbyParams) -> Result<Response<Vec<NearbyCompany>>> {
        self.client
            .block_on(self.client.inner.companies().nearby(params))
    }

    pub fn notes(&self, uid: &str) -> Result<Response<Vec<Note>>> {
        self.client
            .block_on(self.client.inner.companies().notes(uid))
    }

    pub fn create_note(&self, uid: &str, req: &CreateNoteRequest) -> Result<Response<Note>> {
        self.client
            .block_on(self.client.inner.companies().create_note(uid, req))
    }

    pub fn update_note(
        &self,
        uid: &str,
        note_id: &str,
        req: &UpdateNoteRequest,
    ) -> Result<Response<Note>> {
        self.client
            .block_on(self.client.inner.companies().update_note(uid, note_id, req))
    }

    pub fn delete_note(&self, uid: &str, note_id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.companies().delete_note(uid, note_id))
    }

    pub fn tags(&self, uid: &str) -> Result<Response<Vec<Tag>>> {
        self.client
            .block_on(self.client.inner.companies().tags(uid))
    }

    pub fn create_tag(&self, uid: &str, req: &CreateTagRequest) -> Result<Response<Tag>> {
        self.client
            .block_on(self.client.inner.companies().create_tag(uid, req))
    }

    pub fn delete_tag(&self, uid: &str, tag_id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.companies().delete_tag(uid, tag_id))
    }

    pub fn all_tags(&self) -> Result<Response<Vec<TagSummary>>> {
        self.client
            .block_on(self.client.inner.companies().all_tags())
    }

    pub fn export_excel(&self, req: &ExcelExportRequest) -> Result<ExportFile> {
        self.client
            .block_on(self.client.inner.companies().export_excel(req))
    }
}

pub struct Auditors<'a> {
    client: &'a Client,
}

impl Auditors<'_> {
    pub fn history(&self, uid: &str) -> Result<Response<AuditorHistoryResponse>> {
        self.client
            .block_on(self.client.inner.auditors().history(uid))
    }

    pub fn tenures(
        &self,
        params: &AuditorTenureParams,
    ) -> Result<Response<PagedResponse<AuditorTenure>>> {
        self.client
            .block_on(self.client.inner.auditors().tenures(params))
    }
}

pub struct Dashboard<'a> {
    client: &'a Client,
}

impl Dashboard<'_> {
    pub fn get(&self) -> Result<Response<DashboardResponse>> {
        self.client.block_on(self.client.inner.dashboard().get())
    }
}

pub struct Screening<'a> {
    client: &'a Client,
}

impl Screening<'_> {
    pub fn screen(&self, req: &ScreeningRequest) -> Result<Response<ScreeningResponse>> {
        self.client
            .block_on(self.client.inner.screening().screen(req))
    }
}

pub struct Watchlists<'a> {
    client: &'a Client,
}

impl Watchlists<'_> {
    pub fn list(&self) -> Result<Response<Vec<WatchlistSummary>>> {
        self.client.block_on(self.client.inner.watchlists().list())
    }

    pub fn create(&self, req: &CreateWatchlistRequest) -> Result<Response<Watchlist>> {
        self.client
            .block_on(self.client.inner.watchlists().create(req))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.watchlists().delete(id))
    }

    pub fn companies(&self, id: &str) -> Result<Response<WatchlistCompaniesResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().companies(id))
    }

    pub fn add_companies(
        &self,
        id: &str,
        req: &AddCompaniesRequest,
    ) -> Result<Response<AddCompaniesResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().add_companies(id, req))
    }

    pub fn remove_company(&self, id: &str, uid: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.watchlists().remove_company(id, uid))
    }

    pub fn events(&self, id: &str, limit: Option<u32>) -> Result<Response<EventListResponse>> {
        self.client
            .block_on(self.client.inner.watchlists().events(id, limit))
    }
}

pub struct Webhooks<'a> {
    client: &'a Client,
}

impl Webhooks<'_> {
    pub fn list(&self) -> Result<Response<Vec<WebhookSubscription>>> {
        self.client.block_on(self.client.inner.webhooks().list())
    }

    pub fn create(&self, req: &CreateWebhookRequest) -> Result<Response<CreateWebhookResponse>> {
        self.client
            .block_on(self.client.inner.webhooks().create(req))
    }

    pub fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<WebhookSubscription>> {
        self.client
            .block_on(self.client.inner.webhooks().update(id, req))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.webhooks().delete(id))
    }

    pub fn test(&self, id: &str) -> Result<Response<TestDeliveryResponse>> {
        self.client.block_on(self.client.inner.webhooks().test(id))
    }

    pub fn deliveries(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Response<Vec<WebhookDelivery>>> {
        self.client
            .block_on(self.client.inner.webhooks().deliveries(id, limit))
    }
}

pub struct Exports<'a> {
    client: &'a Client,
}

impl Exports<'_> {
    pub fn create(&self, req: &CreateExportRequest) -> Result<Response<ExportJob>> {
        self.client
            .block_on(self.client.inner.exports().create(req))
    }

    pub fn get(&self, id: &str) -> Result<Response<ExportDownload>> {
        self.client.block_on(self.client.inner.exports().get(id))
    }

    pub fn download(&self, id: &str) -> Result<ExportFile> {
        self.client
            .block_on(self.client.inner.exports().download(id))
    }
}

pub struct Ai<'a> {
    client: &'a Client,
}

impl Ai<'_> {
    pub fn dossier(&self, req: &DossierRequest) -> Result<Response<DossierResponse>> {
        self.client.block_on(self.client.inner.ai().dossier(req))
    }

    pub fn search(&self, req: &AiSearchRequest) -> Result<Response<AiSearchResponse>> {
        self.client.block_on(self.client.inner.ai().search(req))
    }

    pub fn risk_score(&self, req: &RiskScoreRequest) -> Result<Response<RiskScoreResponse>> {
        self.client.block_on(self.client.inner.ai().risk_score(req))
    }
}

// ---------------------------------------------------------------------------
// New resource wrappers (9 modules)
// ---------------------------------------------------------------------------

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl ApiKeys<'_> {
    pub fn create(&self, req: &CreateApiKeyRequest) -> Result<Response<ApiKeyCreated>> {
        self.client
            .block_on(self.client.inner.api_keys().create(req))
    }

    pub fn list(&self) -> Result<Response<Vec<ApiKey>>> {
        self.client.block_on(self.client.inner.api_keys().list())
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
        self.client.block_on(self.client.inner.credits().balance())
    }

    pub fn usage(&self, since: Option<&str>) -> Result<Response<CreditUsage>> {
        self.client
            .block_on(self.client.inner.credits().usage(since))
    }

    pub fn history(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response<CreditHistory>> {
        self.client
            .block_on(self.client.inner.credits().history(limit, offset))
    }
}

pub struct Billing<'a> {
    client: &'a Client,
}

impl Billing<'_> {
    pub fn create_checkout(&self, req: &CheckoutRequest) -> Result<Response<SessionUrl>> {
        self.client
            .block_on(self.client.inner.billing().create_checkout(req))
    }

    pub fn create_portal(&self) -> Result<Response<SessionUrl>> {
        self.client
            .block_on(self.client.inner.billing().create_portal())
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
        self.client.block_on(self.client.inner.teams().create(req))
    }

    pub fn members(&self) -> Result<Response<Vec<TeamMember>>> {
        self.client.block_on(self.client.inner.teams().members())
    }

    pub fn invite_member(&self, req: &InviteMemberRequest) -> Result<Response<Invitation>> {
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

    pub fn join(&self, req: &JoinTeamRequest) -> Result<Response<JoinTeamResponse>> {
        self.client.block_on(self.client.inner.teams().join(req))
    }
}

pub struct Changes<'a> {
    client: &'a Client,
}

impl Changes<'_> {
    pub fn list(
        &self,
        params: &ChangeListParams,
    ) -> Result<Response<PagedResponse<CompanyChange>>> {
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
}

pub struct Persons<'a> {
    client: &'a Client,
}

impl Persons<'_> {
    pub fn board_members(&self, uid: &str) -> Result<Response<Vec<BoardMember>>> {
        self.client
            .block_on(self.client.inner.persons().board_members(uid))
    }
}

pub struct Analytics<'a> {
    client: &'a Client,
}

impl Analytics<'_> {
    pub fn cantons(&self) -> Result<Response<Vec<CantonDistribution>>> {
        self.client
            .block_on(self.client.inner.analytics().cantons())
    }

    pub fn auditors(&self) -> Result<Response<Vec<AuditorMarketShare>>> {
        self.client
            .block_on(self.client.inner.analytics().auditors())
    }

    pub fn cluster(&self, req: &ClusterRequest) -> Result<Response<ClusterResponse>> {
        self.client
            .block_on(self.client.inner.analytics().cluster(req))
    }

    pub fn anomalies(&self, req: &AnomalyRequest) -> Result<Response<AnomalyResponse>> {
        self.client
            .block_on(self.client.inner.analytics().anomalies(req))
    }

    pub fn rfm_segments(&self) -> Result<Response<RfmSegmentsResponse>> {
        self.client
            .block_on(self.client.inner.analytics().rfm_segments())
    }

    pub fn cohorts(&self, params: &CohortParams) -> Result<Response<CohortResponse>> {
        self.client
            .block_on(self.client.inner.analytics().cohorts(params))
    }

    pub fn candidates(
        &self,
        params: &CandidateParams,
    ) -> Result<Response<PagedResponse<AuditCandidate>>> {
        self.client
            .block_on(self.client.inner.analytics().candidates(params))
    }
}

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl Dossiers<'_> {
    pub fn create(&self, req: &CreateDossierRequest) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().create(req))
    }

    pub fn list(&self) -> Result<Response<Vec<DossierSummary>>> {
        self.client.block_on(self.client.inner.dossiers().list())
    }

    pub fn get(&self, id_or_uid: &str) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().get(id_or_uid))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.dossiers().delete(id))
    }

    pub fn generate(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().generate(uid))
    }
}

pub struct Graph<'a> {
    client: &'a Client,
}

impl Graph<'_> {
    pub fn get(&self, uid: &str) -> Result<Response<GraphResponse>> {
        self.client.block_on(self.client.inner.graph().get(uid))
    }

    pub fn export(&self, uid: &str, format: &str) -> Result<ExportFile> {
        self.client
            .block_on(self.client.inner.graph().export(uid, format))
    }

    pub fn analyze(
        &self,
        req: &NetworkAnalysisRequest,
    ) -> Result<Response<NetworkAnalysisResponse>> {
        self.client.block_on(self.client.inner.graph().analyze(req))
    }
}
