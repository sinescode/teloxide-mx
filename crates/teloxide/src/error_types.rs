//! Exception/error types with detailed messages and documentation links.
//!
//! Similar to aiogram's rich exception hierarchy, this module provides
//! structured error types that map to Telegram API error codes.

use std::fmt;

use crate::types::ResponseParameters;

/// Base error type for all Telegram API errors.
#[derive(Debug, Clone)]
pub struct TelegramError {
    /// The error code from Telegram.
    pub error_code: i64,
    /// Human-readable description of the error.
    pub description: String,
    /// Additional parameters (e.g., retry_after, migrate_to_chat_id).
    pub parameters: Option<ResponseParameters>,
    /// Link to Telegram documentation about this error.
    pub doc_url: Option<&'static str>,
}

impl fmt::Display for TelegramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Telegram API error {}: {}", self.error_code, self.description)?;
        if let Some(url) = self.doc_url {
            write!(f, "\n(See: {url})")?;
        }
        Ok(())
    }
}

impl std::error::Error for TelegramError {}

impl TelegramError {
    pub fn new(error_code: i64, description: String) -> Self {
        Self { error_code, description, parameters: None, doc_url: None }
    }

    pub fn with_parameters(mut self, params: ResponseParameters) -> Self {
        self.parameters = Some(params);
        self
    }

    pub fn with_doc_url(mut self, url: &'static str) -> Self {
        self.doc_url = Some(url);
        self
    }

    /// Returns the number of seconds to wait before retrying, if applicable.
    pub fn retry_after(&self) -> Option<u32> {
        self.parameters.as_ref().and_then(|p| match p {
            ResponseParameters::RetryAfter(secs) => Some(secs.seconds()),
            _ => None,
        })
    }

    /// Returns the new chat ID if the group has migrated, if applicable.
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        self.parameters.as_ref().and_then(|p| match p {
            ResponseParameters::MigrateToChatId(id) => Some(id.0),
            _ => None,
        })
    }
}

/// Specialized error types for common Telegram API errors.
impl TelegramError {
    /// 403: The bot was kicked from the group/supergroup.
    pub fn bot_blocked(description: impl Into<String>) -> Self {
        Self::new(403, description.into())
            .with_doc_url("https://core.telegram.org/bots/api#making-requests")
    }

    /// 400: Bad request.
    pub fn bad_request(description: impl Into<String>) -> Self {
        Self::new(400, description.into())
    }

    /// 401: Unauthorized (invalid token).
    pub fn unauthorized(description: impl Into<String>) -> Self {
        Self::new(401, description.into())
    }

    /// 404: Not found (bot or chat).
    pub fn not_found(description: impl Into<String>) -> Self {
        Self::new(404, description.into())
    }

    /// 429: Too many requests (flood control).
    pub fn retry_after_error(seconds: u32) -> Self {
        Self::new(429, format!("Too Many Requests: retry after {seconds} seconds"))
    }

    /// 409: Conflict (another bot instance is getting updates).
    pub fn conflict(description: impl Into<String>) -> Self {
        Self::new(409, description.into())
    }
}
