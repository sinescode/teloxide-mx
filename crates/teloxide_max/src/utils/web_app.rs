//! Web App (Mini App) init data validation.
//!
//! Validates data received from Telegram Mini Apps using HMAC-SHA-256, as
//! described in the
//! [Telegram Bot API docs](https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app).
//!
//! Algorithm (matches aiogram / official Telegram docs):
//! 1. Parse query string into key/value pairs.
//! 2. Extract and remove `hash`.
//! 3. Sort remaining pairs by key and join as `key=value` lines with `\n`.
//! 4. Secret key = `HMAC_SHA256(key = "WebAppData", msg = bot_token)`.
//! 5. Expected hash = `HMAC_SHA256(key = secret, msg = data_check_string)`.
//! 6. Compare with constant-time equality.
//!
//! # Example
//!
//! ```rust,no_run
//! use teloxide_max::utils::web_app;
//!
//! let is_valid = web_app::validate_init_data(
//!     "YOUR_BOT_TOKEN",
//!     "query_id=...&user=...&auth_date=...&hash=...",
//! );
//! ```
//!
//! # Migration from aiogram
//!
//! | aiogram | teloxide_max |
//! |---------|--------------|
//! | `check_webapp_signature(token, init_data)` | `validate_init_data(token, init_data)` |
//! | `safe_parse_webapp_init_data(token, init_data)` | `safe_parse_init_data(token, init_data)` |
//! | `parse_webapp_init_data(init_data)` | `parse_init_data(init_data)` |

use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::utils::webhook_security::constant_time_eq;

type HmacSha256 = Hmac<Sha256>;

/// Validates the `initData` received from a Telegram Web App / Mini App.
///
/// Returns `true` if the HMAC signature is authentic.
///
/// This does **not** check `auth_date` freshness — callers should enforce
/// their own max-age policy when needed.
pub fn validate_init_data(bot_token: &str, init_data: &str) -> bool {
    let secret_key = compute_secret_key(bot_token);
    validate_with_secret(&secret_key, init_data)
}

/// Alias matching aiogram's `check_webapp_signature`.
#[inline]
pub fn check_webapp_signature(bot_token: &str, init_data: &str) -> bool {
    validate_init_data(bot_token, init_data)
}

/// Validates init data with a pre-computed secret key
/// (`HMAC_SHA256("WebAppData", bot_token)`).
pub fn validate_with_secret(secret_key: &[u8], init_data: &str) -> bool {
    let Some((hash, data_check_string)) = build_data_check_string(init_data) else {
        return false;
    };

    let mut mac = match HmacSha256::new_from_slice(secret_key) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(data_check_string.as_bytes());
    let computed = hex::encode(mac.finalize().into_bytes());

    constant_time_eq(computed.as_bytes(), hash.as_bytes())
}

/// Computes the WebApp secret key from the bot token.
///
/// `secret_key = HMAC_SHA256(key = "WebAppData", msg = bot_token)`
pub fn compute_secret_key(bot_token: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(b"WebAppData").expect("HMAC accepts any key length");
    mac.update(bot_token.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

/// Parses WebApp init data into a map of decoded string values.
///
/// This does **not** validate the signature. Prefer [`safe_parse_init_data`]
/// for untrusted input.
pub fn parse_init_data(init_data: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for pair in init_data.split('&') {
        if pair.is_empty() {
            continue;
        }
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        let decoded = urlencoding_decode(value);
        result.insert(key.to_string(), decoded);
    }
    result
}

/// Validates init data and returns parsed fields on success.
///
/// Returns `Err` with a descriptive message when the signature is invalid.
pub fn safe_parse_init_data(
    bot_token: &str,
    init_data: &str,
) -> Result<BTreeMap<String, String>, &'static str> {
    if !validate_init_data(bot_token, init_data) {
        return Err("Invalid init data signature");
    }
    Ok(parse_init_data(init_data))
}

/// Builds the Telegram data-check-string and extracts the provided hash.
///
/// Keys are sorted alphabetically (Telegram requirement). The `hash` field is
/// excluded from the check string; `signature` is kept (used by third-party
/// Ed25519 validation, not by this HMAC path).
fn build_data_check_string(init_data: &str) -> Option<(String, String)> {
    if init_data.is_empty() {
        return None;
    }

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut hash: Option<String> = None;

    for pair in init_data.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=')?;
        // Telegram uses the raw (still-encoded) values in the check string for
        // the pairs that appear in initData; parse_qsl in Python returns decoded
        // values. Official docs show: sort by key, join as "key=value" using the
        // values as received after URL-decoding (parse_qsl default).
        let decoded = urlencoding_decode(value);
        if key == "hash" {
            hash = Some(decoded);
        } else {
            pairs.push((key.to_string(), decoded));
        }
    }

    let hash = hash?;
    if hash.is_empty() {
        return None;
    }

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let data_check_string =
        pairs.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n");

    Some((hash, data_check_string))
}

/// Minimal URL-decoding (application/x-www-form-urlencoded).
fn urlencoding_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let h1 = from_hex(bytes[i + 1]);
                let h2 = from_hex(bytes[i + 2]);
                if let (Some(a), Some(b)) = (h1, h2) {
                    out.push((a << 4) | b);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_secret_key_length() {
        let key = compute_secret_key("123456:ABC-DEF");
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn compute_secret_key_differs_from_plain_sha256() {
        use sha2::Digest;
        let token = "123456:ABC-DEF";
        let hmac_key = compute_secret_key(token);
        let plain = {
            let mut hasher = Sha256::new();
            hasher.update(token.as_bytes());
            hasher.finalize().to_vec()
        };
        // Official algorithm uses HMAC("WebAppData", token), not SHA256(token).
        assert_ne!(hmac_key, plain);
    }

    #[test]
    fn validate_init_data_empty() {
        assert!(!validate_init_data("token", ""));
    }

    #[test]
    fn validate_init_data_no_hash() {
        assert!(!validate_init_data("token", "key=value&key2=value2"));
    }

    #[test]
    fn validate_init_data_invalid_hash() {
        let token = "test_token";
        let init_data = "query_id=123&user=%7B%22id%22%3A123%7D&auth_date=1678886400&hash=invalid";
        assert!(!validate_init_data(token, init_data));
    }

    #[test]
    fn full_webapp_flow_sorted_hmac() {
        let token = "123456:ABC-DEF";
        let secret_key = compute_secret_key(token);

        // Unsorted input order — algorithm must sort by key before hashing.
        let pairs = [("user", r#"{"id":123}"#), ("query_id", "AAE"), ("auth_date", "1678886400")];

        let mut sorted: Vec<(&str, &str)> = pairs.to_vec();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        let data_check_string =
            sorted.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n");

        let mut mac = HmacSha256::new_from_slice(&secret_key).unwrap();
        mac.update(data_check_string.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        // Build init_data in non-sorted order with URL-encoded user JSON.
        let init_data =
            format!("user=%7B%22id%22%3A123%7D&query_id=AAE&auth_date=1678886400&hash={hash}");
        assert!(validate_init_data(token, &init_data));
        assert!(check_webapp_signature(token, &init_data));

        let parsed = safe_parse_init_data(token, &init_data).unwrap();
        assert_eq!(parsed.get("query_id").map(String::as_str), Some("AAE"));
        assert_eq!(parsed.get("auth_date").map(String::as_str), Some("1678886400"));
    }

    #[test]
    fn safe_parse_rejects_invalid() {
        let err = safe_parse_init_data("token", "a=1&hash=deadbeef").unwrap_err();
        assert_eq!(err, "Invalid init data signature");
    }

    #[test]
    fn parse_init_data_decodes_values() {
        let map = parse_init_data("name=hello%20world&flag=1");
        assert_eq!(map.get("name").map(String::as_str), Some("hello world"));
        assert_eq!(map.get("flag").map(String::as_str), Some("1"));
    }

    #[test]
    fn validate_with_secret_empty_hash() {
        assert!(!validate_with_secret(b"secret", "key=value&hash="));
    }
}
