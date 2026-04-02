use std::time::Duration;

use reqwest::{header, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{ErrorBody, Result, VyncoError};
use crate::resources::*;
use crate::response::{Response, ResponseMeta};

const DEFAULT_BASE_URL: &str = "https://api.vynco.ch";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 2;

/// Builder for configuring and constructing a [`Client`].
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

impl ClientBuilder {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }

    /// Set the API base URL (default: `https://api.vynco.ch`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the request timeout (default: 30 seconds).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of retries on 429/5xx (default: 2).
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Build the client. Returns an error if the API key is empty.
    pub fn build(self) -> Result<Client> {
        if self.api_key.is_empty() {
            return Err(VyncoError::Config("API key must not be empty".into()));
        }

        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", self.api_key);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value)
                .map_err(|e| VyncoError::Config(format!("invalid API key: {e}")))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("vynco-rust/{}", env!("CARGO_PKG_VERSION")))
            .timeout(self.timeout)
            .build()
            .map_err(VyncoError::Http)?;

        Ok(Client {
            http,
            base_url: self.base_url,
            max_retries: self.max_retries,
        })
    }
}

/// Async client for the VynCo API.
///
/// Created via [`Client::builder`]. Safe to clone and share across tasks
/// (the underlying connection pool is shared).
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) max_retries: u32,
}

impl Client {
    /// Create a new [`ClientBuilder`] with the given API key.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    // -- Resource accessors --------------------------------------------------

    pub fn health(&self) -> Health<'_> {
        Health::new(self)
    }

    pub fn companies(&self) -> Companies<'_> {
        Companies::new(self)
    }

    pub fn auditors(&self) -> Auditors<'_> {
        Auditors::new(self)
    }

    pub fn dashboard(&self) -> Dashboard<'_> {
        Dashboard::new(self)
    }

    pub fn screening(&self) -> Screening<'_> {
        Screening::new(self)
    }

    pub fn watchlists(&self) -> Watchlists<'_> {
        Watchlists::new(self)
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks::new(self)
    }

    pub fn exports(&self) -> Exports<'_> {
        Exports::new(self)
    }

    pub fn ai(&self) -> Ai<'_> {
        Ai::new(self)
    }

    pub fn api_keys(&self) -> ApiKeys<'_> {
        ApiKeys::new(self)
    }

    pub fn credits(&self) -> Credits<'_> {
        Credits::new(self)
    }

    pub fn teams(&self) -> Teams<'_> {
        Teams::new(self)
    }

    pub fn billing(&self) -> Billing<'_> {
        Billing::new(self)
    }

    pub fn changes(&self) -> Changes<'_> {
        Changes::new(self)
    }

    pub fn persons(&self) -> Persons<'_> {
        Persons::new(self)
    }

    pub fn analytics(&self) -> Analytics<'_> {
        Analytics::new(self)
    }

    pub fn dossiers(&self) -> Dossiers<'_> {
        Dossiers::new(self)
    }

    pub fn graph(&self) -> Graph<'_> {
        Graph::new(self)
    }

    // -- Internal request methods --------------------------------------------

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Send a request with no body and deserialize the JSON response.
    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<Response<T>> {
        self.execute(self.http.request(method.clone(), self.url(path)))
            .await
    }

    /// Send a request with a JSON body and deserialize the JSON response.
    pub(crate) async fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<Response<T>> {
        self.execute(self.http.request(method.clone(), self.url(path)).json(body))
            .await
    }

    /// Send a request with query parameters and deserialize the JSON response.
    pub(crate) async fn request_with_params<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<Response<T>> {
        self.execute(
            self.http
                .request(method.clone(), self.url(path))
                .query(params),
        )
        .await
    }

    /// Send a request that returns no body (e.g. DELETE). Returns only metadata.
    pub(crate) async fn request_empty(&self, method: Method, path: &str) -> Result<ResponseMeta> {
        let resp = self
            .execute_raw(self.http.request(method.clone(), self.url(path)))
            .await?;
        let meta = ResponseMeta::from_headers(resp.headers());
        let status = resp.status();
        if status.is_success() {
            Ok(meta)
        } else {
            Err(self.map_error(status, resp).await)
        }
    }

    /// Send a request and return raw bytes (e.g. file downloads).
    /// Returns `(bytes, meta, content_type, filename)`.
    pub(crate) async fn request_bytes(
        &self,
        method: Method,
        path: &str,
    ) -> Result<(Vec<u8>, ResponseMeta, String, String)> {
        let resp = self
            .execute_raw(self.http.request(method.clone(), self.url(path)))
            .await?;
        let meta = ResponseMeta::from_headers(resp.headers());
        let status = resp.status();

        let content_type = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let filename = resp
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split("filename=")
                    .nth(1)
                    .map(|f| f.trim_matches('"').to_string())
            })
            .unwrap_or_default();

        if !status.is_success() {
            return Err(self.map_error(status, resp).await);
        }

        let bytes = resp.bytes().await.map_err(VyncoError::Http)?.to_vec();
        Ok((bytes, meta, content_type, filename))
    }

    /// Send a request with a JSON body and return raw bytes (e.g. CSV exports).
    /// Returns `(bytes, meta, content_type, filename)`.
    pub(crate) async fn request_bytes_with_body<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<(Vec<u8>, ResponseMeta, String, String)> {
        let builder = self.http.request(method.clone(), self.url(path)).json(body);
        let resp = self.execute_raw(builder).await?;
        let meta = ResponseMeta::from_headers(resp.headers());
        let status = resp.status();

        let content_type = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let filename = resp
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split("filename=")
                    .nth(1)
                    .map(|f| f.trim_matches('"').to_string())
            })
            .unwrap_or_default();

        if !status.is_success() {
            return Err(self.map_error(status, resp).await);
        }

        let bytes = resp.bytes().await.map_err(VyncoError::Http)?.to_vec();
        Ok((bytes, meta, content_type, filename))
    }

    /// Execute a request with retry logic, returning the deserialized body + metadata.
    async fn execute<T: DeserializeOwned>(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<Response<T>> {
        let resp = self.execute_with_retry(builder).await?;
        let meta = ResponseMeta::from_headers(resp.headers());
        let status = resp.status();

        if !status.is_success() {
            return Err(self.map_error(status, resp).await);
        }

        let data: T = resp.json().await.map_err(VyncoError::Http)?;
        Ok(Response { data, meta })
    }

    /// Execute a request with retry logic, returning the raw response.
    async fn execute_raw(&self, builder: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        self.execute_with_retry(builder).await
    }

    /// Core retry logic: retries on 429 and 5xx with exponential backoff.
    async fn execute_with_retry(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let mut last_err: Option<VyncoError> = None;

        for attempt in 0..=self.max_retries {
            let request = builder
                .try_clone()
                .ok_or_else(|| VyncoError::Config("request cannot be cloned for retry".into()))?
                .build()
                .map_err(VyncoError::Http)?;

            match self.http.execute(request).await {
                Ok(resp) => {
                    let status = resp.status();
                    if (status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error())
                        && attempt < self.max_retries
                    {
                        let retry_after = resp
                            .headers()
                            .get("Retry-After")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok());

                        let delay = retry_after
                            .map(Duration::from_secs)
                            .unwrap_or_else(|| Duration::from_millis(500 * 2u64.pow(attempt)));

                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Ok(resp);
                }
                Err(e) => {
                    last_err = Some(VyncoError::Http(e));
                    if attempt < self.max_retries {
                        let delay = Duration::from_millis(500 * 2u64.pow(attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_err.unwrap_or_else(|| VyncoError::Config("request failed".into())))
    }

    /// Map an HTTP error status to a typed VyncoError.
    async fn map_error(&self, status: StatusCode, resp: reqwest::Response) -> VyncoError {
        let body = resp
            .json::<ErrorBody>()
            .await
            .unwrap_or_else(|_| ErrorBody {
                error_type: String::new(),
                title: String::new(),
                status: status.as_u16(),
                detail: Some(format!("HTTP {}", status.as_u16())),
                instance: None,
            });

        match status {
            StatusCode::UNAUTHORIZED => VyncoError::Authentication(body),
            StatusCode::PAYMENT_REQUIRED => VyncoError::InsufficientCredits(body),
            StatusCode::FORBIDDEN => VyncoError::Forbidden(body),
            StatusCode::NOT_FOUND => VyncoError::NotFound(body),
            StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                VyncoError::Validation(body)
            }
            StatusCode::CONFLICT => VyncoError::Conflict(body),
            StatusCode::TOO_MANY_REQUESTS => VyncoError::RateLimit(body),
            s if s.is_server_error() => VyncoError::Server(body),
            _ => VyncoError::Server(body),
        }
    }
}
