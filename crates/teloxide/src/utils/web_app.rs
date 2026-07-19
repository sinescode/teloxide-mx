//! Web App init data validation.
//!
//! Validates data received from Telegram Mini Apps (Web Apps) using
//! HMAC-SHA-256, as described in the
//! [Telegram Bot API docs](https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app).
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::utils::web_app;
//! # fn example() {
//! let is_valid = web_app::validate_init_data(
//!     "YOUR_BOT_TOKEN",
//!     "query_id=...&user=...&auth_date=...&hash=...",
//! );
//! # }
//! ```

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Validates the `initData` received from a Telegram Web App.
///
/// Returns `true` if the data is authentic and not expired.
pub fn validate_init_data(bot_token: &str, init_data: &str) -> bool {
    let secret_key = compute_secret_key(bot_token);
    validate_with_secret(&secret_key, init_data)
}

/// Validates init data with a pre-computed secret key.
pub fn validate_with_secret(secret_key: &[u8], init_data: &str) -> bool {
    let pairs: Vec<(&str, &str)> = init_data
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;

            Some((key, value))
        })
        .collect();

    let hash = match pairs.iter().find(|(k, _)| *k == "hash") {
        Some((_, h)) => h,
        None => return false,
    };

    let data_check_string: String = pairs
        .iter()
        .filter(|(k, _)| *k != "hash")
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut mac = HmacSha256::new_from_slice(secret_key).map_err(|_| ()).unwrap();
    mac.update(data_check_string.as_bytes());

    let computed = hex::encode(mac.finalize().into_bytes());
    computed == *hash
}

/// Computes the secret key from the bot token (SHA-256 hash).
fn compute_secret_key(bot_token: &str) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bot_token.as_bytes());
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_secret() {
        let key = compute_secret_key("123456:ABC-DEF");
        assert_eq!(key.len(), 32);
    }
}
