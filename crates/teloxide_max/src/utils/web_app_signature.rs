//! Third-party Web App signature validation (Ed25519).
//!
//! Validates Mini App init data **without** the bot token, using only the bot
//! id and Telegram's published Ed25519 public keys.
//!
//! Source:
//! <https://core.telegram.org/bots/webapps#validating-data-for-third-party-use>
//!
//! # Example
//!
//! ```rust,no_run
//! use teloxide_max::utils::web_app_signature::{check_webapp_signature, PRODUCTION_PUBLIC_KEY};
//!
//! let ok = check_webapp_signature(
//!     123456789, // bot id
//!     "query_id=...&user=...&auth_date=...&signature=...&hash=...",
//!     PRODUCTION_PUBLIC_KEY,
//! );
//! ```
//!
//! # Migration from aiogram
//!
//! | aiogram | teloxide_max |
//! |---------|--------------|
//! | `check_webapp_signature(bot_id, init_data)` | `check_webapp_signature(bot_id, init_data, PRODUCTION_PUBLIC_KEY)` |
//! | `safe_check_webapp_init_data_from_signature(...)` | `safe_check_webapp_init_data_from_signature(...)` |

use std::collections::BTreeMap;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Telegram production Ed25519 public key for third-party WebApp validation.
pub const PRODUCTION_PUBLIC_KEY: [u8; 32] = [
    0xe7, 0xbf, 0x03, 0xa2, 0xfa, 0x46, 0x02, 0xaf, 0x45, 0x80, 0x70, 0x3d, 0x88, 0xdd, 0xa5, 0xbb,
    0x59, 0xf3, 0x2e, 0xd8, 0xb0, 0x2a, 0x56, 0xc1, 0x87, 0xfe, 0x7d, 0x34, 0xca, 0xed, 0x24, 0x2d,
];

/// Telegram test-environment Ed25519 public key.
pub const TEST_PUBLIC_KEY: [u8; 32] = [
    0x40, 0x05, 0x50, 0x58, 0xa4, 0xee, 0x38, 0x15, 0x6a, 0x06, 0x56, 0x2e, 0x52, 0xee, 0xce, 0x92,
    0xa7, 0x71, 0xbc, 0xd8, 0x34, 0x6a, 0x8c, 0x46, 0x15, 0xcb, 0x73, 0x76, 0xed, 0xdf, 0x72, 0xec,
];

/// Check incoming WebApp init data signature using only bot id (no bot token).
///
/// # Arguments
///
/// * `bot_id` — numeric bot id (left side of the bot token before `:`).
/// * `init_data` — raw `initData` query string from the Mini App frontend.
/// * `public_key_bytes` — 32-byte Ed25519 public key ([`PRODUCTION_PUBLIC_KEY`]
///   or [`TEST_PUBLIC_KEY`]).
pub fn check_webapp_signature(bot_id: u64, init_data: &str, public_key_bytes: [u8; 32]) -> bool {
    let Some((message, signature_bytes)) = build_third_party_message(bot_id, init_data) else {
        return false;
    };

    let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key_bytes) else {
        return false;
    };

    // Signature is 64 bytes for Ed25519.
    let Ok(sig_array) = <[u8; 64]>::try_from(signature_bytes.as_slice()) else {
        return false;
    };
    let signature = Signature::from_bytes(&sig_array);

    verifying_key.verify(message.as_bytes(), &signature).is_ok()
}

/// Validate raw WebApp init data using bot id + Ed25519 and return parsed
/// fields.
///
/// Returns `Err` when the signature is invalid.
pub fn safe_check_webapp_init_data_from_signature(
    bot_id: u64,
    init_data: &str,
    public_key_bytes: [u8; 32],
) -> Result<BTreeMap<String, String>, &'static str> {
    if !check_webapp_signature(bot_id, init_data, public_key_bytes) {
        return Err("Invalid init data signature");
    }
    Ok(crate::utils::web_app::parse_init_data(init_data))
}

/// Builds `{bot_id}:WebAppData\n` + sorted `key=value` lines (excluding
/// `hash` and `signature`), plus the decoded signature bytes.
fn build_third_party_message(bot_id: u64, init_data: &str) -> Option<(String, Vec<u8>)> {
    if init_data.is_empty() {
        return None;
    }

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut signature_b64: Option<String> = None;

    for pair in init_data.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=')?;
        let decoded = urlencoding_decode(value);
        match key {
            "signature" => signature_b64 = Some(decoded),
            "hash" => {
                // Drop hash for third-party validation.
            }
            _ => pairs.push((key.to_string(), decoded)),
        }
    }

    let signature_b64 = signature_b64?;
    let signature = decode_base64url(&signature_b64)?;

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let body = pairs.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n");
    let message = format!("{bot_id}:WebAppData\n{body}");

    Some((message, signature))
}

fn decode_base64url(input: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    // Pad to multiple of 4 (Telegram omits padding).
    let pad = (4 - input.len() % 4) % 4;
    let padded = format!("{input}{}", "=".repeat(pad));
    base64::engine::general_purpose::URL_SAFE.decode(padded).ok()
}

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
    fn public_keys_are_32_bytes() {
        assert_eq!(PRODUCTION_PUBLIC_KEY.len(), 32);
        assert_eq!(TEST_PUBLIC_KEY.len(), 32);
    }

    #[test]
    fn rejects_empty_init_data() {
        assert!(!check_webapp_signature(1, "", PRODUCTION_PUBLIC_KEY));
    }

    #[test]
    fn rejects_missing_signature() {
        assert!(!check_webapp_signature(1, "auth_date=1&hash=abc", PRODUCTION_PUBLIC_KEY));
    }

    #[test]
    fn rejects_invalid_signature_bytes() {
        assert!(!check_webapp_signature(
            1,
            "auth_date=1&signature=not-valid-base64!!!&hash=abc",
            PRODUCTION_PUBLIC_KEY
        ));
    }

    #[test]
    fn safe_check_returns_err_on_invalid() {
        let err = safe_check_webapp_init_data_from_signature(
            1,
            "a=1&signature=YQ&hash=x",
            PRODUCTION_PUBLIC_KEY,
        )
        .unwrap_err();
        assert_eq!(err, "Invalid init data signature");
    }

    #[test]
    fn base64url_padding() {
        // "hi" in base64url without padding is "aGk"
        let decoded = decode_base64url("aGk").unwrap();
        assert_eq!(decoded, b"hi");
    }
}
