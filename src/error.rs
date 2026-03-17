use std::fmt;

use serde::Deserialize;

/// RFC 7807 Problem Details error body returned by the VynCo API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub status: u16,
}

impl fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.detail.is_empty() {
            write!(f, "{}", self.detail)
        } else if !self.message.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "HTTP {}", self.status)
        }
    }
}

/// All errors that can occur when using the VynCo SDK.
#[derive(Debug, thiserror::Error)]
pub enum VyncoError {
    #[error("authentication error: {0}")]
    Authentication(ErrorBody),

    #[error("insufficient credits: {0}")]
    InsufficientCredits(ErrorBody),

    #[error("forbidden: {0}")]
    Forbidden(ErrorBody),

    #[error("not found: {0}")]
    NotFound(ErrorBody),

    #[error("validation error: {0}")]
    Validation(ErrorBody),

    #[error("rate limited: {0}")]
    RateLimit(ErrorBody),

    #[error("server error: {0}")]
    Server(ErrorBody),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("{0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, VyncoError>;
