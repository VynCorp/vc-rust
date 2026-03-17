/// Metadata extracted from VynCo API response headers.
#[derive(Debug, Clone, Default)]
pub struct ResponseMeta {
    /// Unique request identifier for tracing (`X-Request-Id`).
    pub request_id: Option<String>,
    /// Credits consumed by this request (`X-Credits-Used`).
    pub credits_used: Option<i64>,
    /// Remaining credit balance after this request (`X-Credits-Remaining`).
    pub credits_remaining: Option<i64>,
    /// Maximum requests per minute for the current tier (`X-Rate-Limit-Limit`).
    pub rate_limit_limit: Option<u32>,
    /// Data source for OGD compliance (`X-Data-Source`): "Zefix" or "LINDAS".
    pub data_source: Option<String>,
}

/// A response from the VynCo API, containing both the deserialized body and header metadata.
#[derive(Debug)]
pub struct Response<T> {
    pub data: T,
    pub meta: ResponseMeta,
}

impl ResponseMeta {
    pub(crate) fn from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        Self {
            request_id: headers
                .get("X-Request-Id")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            credits_used: headers
                .get("X-Credits-Used")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            credits_remaining: headers
                .get("X-Credits-Remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            rate_limit_limit: headers
                .get("X-Rate-Limit-Limit")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            data_source: headers
                .get("X-Data-Source")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
        }
    }
}
