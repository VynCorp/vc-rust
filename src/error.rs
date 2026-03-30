use std::fmt;

use serde::Deserialize;

/// RFC 7807 Problem Details error body returned by the VynCo API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    #[serde(default, rename = "type")]
    pub error_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
}

impl fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref d) = self.detail {
            if !d.is_empty() {
                return write!(f, "{}", d);
            }
        }
        if !self.title.is_empty() {
            write!(f, "{}", self.title)
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

    #[error("conflict: {0}")]
    Conflict(ErrorBody),

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
