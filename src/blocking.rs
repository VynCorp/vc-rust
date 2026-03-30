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

    pub fn count(&self) -> Result<Response<CompanyCount>> {
        self.client.block_on(self.client.inner.companies().count())
    }

    pub fn events(&self, uid: &str, limit: Option<u32>) -> Result<Response<EventListResponse>> {
        self.client
            .block_on(self.client.inner.companies().events(uid, limit))
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
        self.client
            .block_on(self.client.inner.watchlists().list())
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
        self.client
            .block_on(self.client.inner.webhooks().test(id))
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
        self.client
            .block_on(self.client.inner.exports().get(id))
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
        self.client
            .block_on(self.client.inner.ai().risk_score(req))
    }
}
