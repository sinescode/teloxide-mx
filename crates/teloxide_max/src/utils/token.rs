//! Bot token validation and helpers.
//!
//! Telegram bot tokens have the form `{bot_id}:{secret}` where `bot_id` is
//! numeric and `secret` is a non-empty string without whitespace.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::utils::token::{extract_bot_id, validate_token};
//!
//! assert!(validate_token("123456:ABC-DEF").is_ok());
//! assert_eq!(extract_bot_id("123456:ABC-DEF").unwrap(), 123456);
//! assert!(validate_token("not-a-token").is_err());
//! ```
//!
//! # Migration from aiogram
//!
//! | aiogram | teloxide_max |
//! |---------|--------------|
//! | `validate_token(token)` | `validate_token(token)` |
//! | `extract_bot_id(token)` | `extract_bot_id(token)` |
//! | `TokenValidationError` | `TokenValidationError` |

use thiserror::Error;

/// Error raised when a bot token fails validation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TokenValidationError {
    /// Token is empty.
    #[error("Token is invalid! It must not be empty.")]
    Empty,

    /// Token contains whitespace.
    #[error("Token is invalid! It can't contain spaces.")]
    ContainsWhitespace,

    /// Token does not match `{numeric_id}:{secret}` form.
    #[error("Token is invalid! Expected format '{{bot_id}}:{{secret}}'.")]
    InvalidFormat,
}

/// Validate a Telegram bot token.
///
/// Returns `Ok(true)` when the token has a valid shape. This does **not**
/// check that the token is accepted by Telegram's servers.
pub fn validate_token(token: &str) -> Result<bool, TokenValidationError> {
    if token.is_empty() {
        return Err(TokenValidationError::Empty);
    }
    if token.chars().any(|c| c.is_whitespace()) {
        return Err(TokenValidationError::ContainsWhitespace);
    }

    let mut parts = token.splitn(2, ':');
    let left = parts.next().unwrap_or("");
    let right = parts.next().unwrap_or("");

    if left.is_empty() || right.is_empty() || !left.chars().all(|c| c.is_ascii_digit()) {
        return Err(TokenValidationError::InvalidFormat);
    }

    Ok(true)
}

/// Extract the numeric bot id from a Telegram bot token.
///
/// Equivalent to parsing the left side of `token.split_once(':')`.
pub fn extract_bot_id(token: &str) -> Result<u64, TokenValidationError> {
    validate_token(token)?;
    let left = token.split_once(':').map(|(l, _)| l).unwrap_or("");
    left.parse::<u64>().map_err(|_| TokenValidationError::InvalidFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_token() {
        assert!(validate_token("123456:ABC-DEF_ghi").is_ok());
        assert_eq!(extract_bot_id("123456:ABC-DEF_ghi").unwrap(), 123456);
    }

    #[test]
    fn empty_token() {
        assert_eq!(validate_token(""), Err(TokenValidationError::Empty));
    }

    #[test]
    fn whitespace_token() {
        assert_eq!(validate_token("123 456:ABC"), Err(TokenValidationError::ContainsWhitespace));
    }

    #[test]
    fn missing_colon() {
        assert_eq!(validate_token("123456ABC"), Err(TokenValidationError::InvalidFormat));
    }

    #[test]
    fn non_numeric_id() {
        assert_eq!(validate_token("abc:DEF"), Err(TokenValidationError::InvalidFormat));
    }

    #[test]
    fn empty_secret() {
        assert_eq!(validate_token("123456:"), Err(TokenValidationError::InvalidFormat));
    }

    #[test]
    fn large_bot_id() {
        assert_eq!(
            extract_bot_id("7123456789:AAHdqTcvCH1vGWJxfSeofSAs0K5PALDsaw").unwrap(),
            7123456789
        );
    }
}
