//! Deep link utilities for Telegram bots.
//!
//! Create deep links that open your bot with pre-filled data.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::utils::deep_linking;
//! # fn example() {
//! let link = deep_linking::create_start_link("my_bot", "payload123");
//! // → https://t.me/my_bot?start=payload123
//!
//! let encoded = deep_linking::encode_payload(b"hello world");
//! let link = deep_linking::create_start_link("my_bot", &encoded);
//!
//! let decoded = deep_linking::decode_payload(&encoded);
//! assert_eq!(decoded, "hello world");
//! # }
//! ```

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

/// Creates a `t.me/<bot_username>?start=<payload>` deep link.
pub fn create_start_link(bot_username: &str, payload: &str) -> String {
    create_deep_link(bot_username, "start", payload)
}

/// Creates a `t.me/<bot_username>?startgroup=<payload>` deep link.
pub fn create_startgroup_link(bot_username: &str, payload: &str) -> String {
    create_deep_link(bot_username, "startgroup", payload)
}

/// Creates a `t.me/<bot_username>?startapp=<payload>` deep link.
pub fn create_startapp_link(bot_username: &str, payload: &str) -> String {
    create_deep_link(bot_username, "startapp", payload)
}

/// Creates a `t.me/<bot_username>?start=<payload>` deep link (alias).
pub fn create_deep_link(bot_username: &str, link_type: &str, payload: &str) -> String {
    format!("https://t.me/{bot_username}?{link_type}={payload}")
}

/// Encodes raw bytes into a URL-safe base64 string (Telegram-compatible).
pub fn encode_payload(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

/// Decodes a URL-safe base64 payload back to raw bytes.
pub fn decode_payload(encoded: &str) -> Result<String, base64::DecodeError> {
    let bytes = URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_link() {
        let link = create_start_link("mybot", "hello");
        assert_eq!(link, "https://t.me/mybot?start=hello");
    }

    #[test]
    fn startgroup_link() {
        let link = create_startgroup_link("mybot", "group_data");
        assert_eq!(link, "https://t.me/mybot?startgroup=group_data");
    }

    #[test]
    fn encode_decode_roundtrip() {
        let data = b"Hello, Telegram!";
        let encoded = encode_payload(data);
        let decoded = decode_payload(&encoded).unwrap();
        assert_eq!(decoded, "Hello, Telegram!");
    }
}
