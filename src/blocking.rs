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

    pub fn api_keys(&self) -> ApiKeys<'_> {
        ApiKeys { client: self }
    }

    pub fn credits(&self) -> Credits<'_> {
        Credits { client: self }
    }

    pub fn billing(&self) -> Billing<'_> {
        Billing { client: self }
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    pub fn teams(&self) -> Teams<'_> {
        Teams { client: self }
    }

    pub fn users(&self) -> Users<'_> {
        Users { client: self }
    }

    pub fn settings(&self) -> Settings<'_> {
        Settings { client: self }
    }
}

// ---------------------------------------------------------------------------
// Resource wrappers
// ---------------------------------------------------------------------------

pub struct Companies<'a> {
    client: &'a Client,
}

impl Companies<'_> {
    pub fn search(
        &self,
        params: &CompanySearchParams,
    ) -> Result<Response<PaginatedResponse<Company>>> {
        self.client
            .block_on(self.client.inner.companies().search(params))
    }

    pub fn get(&self, uid: &str) -> Result<Response<Company>> {
        self.client
            .block_on(self.client.inner.companies().get(uid))
    }

    pub fn count(&self, params: &CompanySearchParams) -> Result<Response<CompanyCount>> {
        self.client
            .block_on(self.client.inner.companies().count(params))
    }

    pub fn statistics(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.companies().statistics())
    }

    pub fn changes(&self, uid: &str) -> Result<Response<Vec<CompanyChange>>> {
        self.client
            .block_on(self.client.inner.companies().changes(uid))
    }

    pub fn persons(&self, uid: &str) -> Result<Response<Vec<PersonRole>>> {
        self.client
            .block_on(self.client.inner.companies().persons(uid))
    }

    pub fn dossier(&self, uid: &str) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.companies().dossier(uid))
    }
}

pub struct Persons<'a> {
    client: &'a Client,
}

impl Persons<'_> {
    pub fn get(&self, id: &str) -> Result<Response<Person>> {
        self.client
            .block_on(self.client.inner.persons().get(id))
    }

    pub fn search(&self, params: &PersonSearchParams) -> Result<Response<Vec<Person>>> {
        self.client
            .block_on(self.client.inner.persons().search(params))
    }
}

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl Dossiers<'_> {
    pub fn generate(
        &self,
        uid: &str,
        req: &GenerateDossierRequest,
    ) -> Result<Response<Dossier>> {
        self.client
            .block_on(self.client.inner.dossiers().generate(uid, req))
    }
}

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl ApiKeys<'_> {
    pub fn list(&self) -> Result<Response<Vec<ApiKeyInfo>>> {
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
    ) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.credits().history(limit, offset))
    }
}

pub struct Billing<'a> {
    client: &'a Client,
}

impl Billing<'_> {
    pub fn create_checkout(&self, tier: &str) -> Result<Response<CheckoutSessionResponse>> {
        self.client
            .block_on(self.client.inner.billing().create_checkout(tier))
    }

    pub fn create_portal(&self) -> Result<Response<PortalSessionResponse>> {
        self.client
            .block_on(self.client.inner.billing().create_portal())
    }
}

pub struct Webhooks<'a> {
    client: &'a Client,
}

impl Webhooks<'_> {
    pub fn list(&self) -> Result<Response<Vec<Webhook>>> {
        self.client
            .block_on(self.client.inner.webhooks().list())
    }

    pub fn create(&self, req: &CreateWebhookRequest) -> Result<Response<WebhookCreated>> {
        self.client
            .block_on(self.client.inner.webhooks().create(req))
    }

    pub fn get(&self, id: &str) -> Result<Response<Webhook>> {
        self.client
            .block_on(self.client.inner.webhooks().get(id))
    }

    pub fn update(
        &self,
        id: &str,
        req: &UpdateWebhookRequest,
    ) -> Result<Response<Webhook>> {
        self.client
            .block_on(self.client.inner.webhooks().update(id, req))
    }

    pub fn delete(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.webhooks().delete(id))
    }

    pub fn test(&self, id: &str) -> Result<ResponseMeta> {
        self.client
            .block_on(self.client.inner.webhooks().test(id))
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
}

pub struct Users<'a> {
    client: &'a Client,
}

impl Users<'_> {
    pub fn me(&self) -> Result<Response<UserProfile>> {
        self.client.block_on(self.client.inner.users().me())
    }

    pub fn update_profile(&self, req: &UpdateProfileRequest) -> Result<Response<UserProfile>> {
        self.client
            .block_on(self.client.inner.users().update_profile(req))
    }
}

pub struct Settings<'a> {
    client: &'a Client,
}

impl Settings<'_> {
    pub fn get(&self) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.settings().get())
    }

    pub fn update(&self, preferences: &serde_json::Value) -> Result<Response<serde_json::Value>> {
        self.client
            .block_on(self.client.inner.settings().update(preferences))
    }
}
