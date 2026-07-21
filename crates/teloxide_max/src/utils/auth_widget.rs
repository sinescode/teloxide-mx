//! Telegram Login Widget validation.
//!
//! Validates data received from Telegram Login Widget using HMAC-SHA-256,
//! as described in the
//! [Telegram Bot API docs](https://core.telegram.org/widgets/login#checking-authorization).
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::utils::auth_widget;
//! # fn example() {
//! let is_valid = auth_widget::check_signature(
//!     "YOUR_BOT_TOKEN",
//!     "abc123",
//!     &[("id".into(), "12345".into()), ("first_name".into(), "John".into())],
//! );
//! # }
//! ```
//!
//! ```rust,no_run
//! # use teloxide_max::utils::auth_widget;
//! # fn example() {
//! let data = serde_json::json!({
//!     "id": 12345,
//!     "first_name": "John",
//!     "hash": "abc123"
//! });
//! let is_valid = auth_widget::check_integrity("YOUR_BOT_TOKEN", &data);
//! # }
//! ```

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// Checks the Telegram Login Widget signature.
///
/// # Arguments
///
/// * `token` - The bot token used to compute the secret key.
/// * `hash` - The `hash` value from the widget callback data.
/// * `query` - Key-value pairs from the widget callback data (excluding `hash`).
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
pub fn check_signature(token: &str, hash: &str, query: &[(String, String)]) -> bool {
    if hash.is_empty() {
        return false;
    }

    // Sort by key, filter out hash
    let mut sorted: Vec<(&String, &String)> = query.iter().filter(|(k, _)| k != "hash").collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    // Build data check string
    let data_check_string: String = sorted.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n");

    // Compute HMAC-SHA256
    let secret_key = compute_secret_key(token);
    let mut mac = match HmacSha256::new_from_slice(&secret_key) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(data_check_string.as_bytes());

    let computed = hex::encode(mac.finalize().into_bytes());
    computed == hash
}

/// Checks the integrity of Telegram Login Widget data using a JSON value.
///
/// The JSON must contain a `"hash"` field. All other fields are used as
/// the query data for signature verification.
///
/// # Arguments
///
/// * `token` - The bot token.
/// * `data` - JSON value containing widget data with `"hash"` field.
///
/// # Returns
///
/// `true` if the data is authentic, `false` otherwise.
pub fn check_integrity(token: &str, data: &serde_json::Value) -> bool {
    let hash = match data.get("hash").and_then(|h| h.as_str()) {
        Some(h) => h,
        None => return false,
    };

    let query: Vec<(String, String)> = data
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter(|(k, _)| k.as_str() != "hash")
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    (k.clone(), val)
                })
                .collect()
        })
        .unwrap_or_default();

    check_signature(token, hash, &query)
}

/// Checks the integrity using aHashMap.
///
/// # Arguments
///
/// * `token` - The bot token.
/// * `data` - HashMap containing widget data with `"hash"` key.
///
/// # Returns
///
/// `true` if the data is authentic, `false` otherwise.
pub fn check_integrity_map(token: &str, data: &HashMap<String, String>) -> bool {
    let hash = match data.get("hash") {
        Some(h) => h.as_str(),
        None => return false,
    };

    let query: Vec<(String, String)> = data
        .iter()
        .filter(|(k, _)| k.as_str() != "hash")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    check_signature(token, hash, &query)
}

/// Computes the secret key from the bot token (SHA-256 hash).
fn compute_secret_key(bot_token: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bot_token.as_bytes());
    hasher.finalize().to_vec()
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
    fn check_signature_empty_hash() {
        assert!(!check_signature("token", "", &[("id".into(), "1".into())]));
    }

    #[test]
    fn check_signature_valid() {
        // Create a known signature
        let token = "test_token";
        let secret = compute_secret_key(token);
        let data_check = "id=12345\nname=John";
        let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
        mac.update(data_check.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        assert!(check_signature(
            token,
            &hash,
            &[("id".into(), "12345".into()), ("name".into(), "John".into())]
        ));
    }

    #[test]
    fn check_signature_invalid() {
        assert!(!check_signature(
            "token",
            "invalid_hash",
            &[("id".into(), "12345".into())]
        ));
    }

    #[test]
    fn check_integrity_json() {
        let token = "test_token";
        let secret = compute_secret_key(token);
        let data_check = "id=12345";
        let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
        mac.update(data_check.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        let data = serde_json::json!({
            "id": "12345",
            "hash": hash
        });

        assert!(check_integrity(token, &data));
    }

    #[test]
    fn check_integrity_missing_hash() {
        let data = serde_json::json!({
            "id": "12345"
        });
        assert!(!check_integrity("token", &data));
    }

    #[test]
    fn check_integrity_map_valid() {
        let token = "test_token";
        let secret = compute_secret_key(token);
        let data_check = "id=12345";
        let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
        mac.update(data_check.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        let mut data = HashMap::new();
        data.insert("id".to_string(), "12345".to_string());
        data.insert("hash".to_string(), hash);

        assert!(check_integrity_map(token, &data));
    }

    #[test]
    fn check_integrity_map_missing_hash() {
        let data = HashMap::new();
        assert!(!check_integrity_map("token", &data));
    }

    #[test]
    fn signature_verification_full_flow() {
        let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
        let secret = compute_secret_key(token);

        // Simulate widget callback data
        let auth_date = "1678886400";
        let first_name = "John";
        let id = "123456789";
        let username = "john_doe";

        let data_check_string = format!(
            "auth_date={auth_date}\nfirst_name={first_name}\nid={id}\nusername={username}"
        );

        let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
        mac.update(data_check_string.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        let query = vec![
            ("auth_date".into(), auth_date.into()),
            ("first_name".into(), first_name.into()),
            ("id".into(), id.into()),
            ("username".into(), username.into()),
        ];

        assert!(check_signature(token, &hash, &query));
    }
}
