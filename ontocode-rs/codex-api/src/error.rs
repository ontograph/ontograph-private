use crate::rate_limits::RateLimitError;
use http::StatusCode;
use ontocode_client::TransportError;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("api error {status}: {message}")]
    Api { status: StatusCode, message: String },
    #[error("stream error: {0}")]
    Stream(String),
    #[error("context window exceeded")]
    ContextWindowExceeded,
    #[error("quota exceeded")]
    QuotaExceeded,
    #[error("usage not included")]
    UsageNotIncluded,
    #[error("retryable error: {message}")]
    Retryable {
        message: String,
        delay: Option<Duration>,
    },
    #[error("rate limit: {0}")]
    RateLimit(String),
    #[error("invalid request: {message}")]
    InvalidRequest { message: String },
    #[error("cyber policy: {message}")]
    CyberPolicy { message: String },
    #[error("server overloaded")]
    ServerOverloaded,
}

impl From<RateLimitError> for ApiError {
    fn from(err: RateLimitError) -> Self {
        Self::RateLimit(err.to_string())
    }
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct ErrorPayload {
    pub r#type: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub plan_type: Option<String>,
    pub resets_at: Option<i64>,
}

impl ErrorPayload {
    pub fn is_context_window_error(&self) -> bool {
        self.code.as_deref() == Some("context_length_exceeded")
    }

    pub fn is_quota_exceeded_error(&self) -> bool {
        self.code.as_deref() == Some("insufficient_quota")
    }

    pub fn is_usage_not_included(&self) -> bool {
        self.code.as_deref() == Some("usage_not_included")
    }

    pub fn is_invalid_prompt_error(&self) -> bool {
        self.code.as_deref() == Some("invalid_prompt")
    }

    pub fn is_cyber_policy_error(&self) -> bool {
        self.code.as_deref() == Some("cyber_policy")
    }

    pub fn is_server_overloaded_error(&self) -> bool {
        self.code.as_deref() == Some("server_is_overloaded")
    }
}
