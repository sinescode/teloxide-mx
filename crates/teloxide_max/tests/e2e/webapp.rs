//! End-to-end tests for WebApp validation.
//!
//! Tests the Telegram Mini App init data validation.

use teloxide_max::utils::web_app;

#[test]
fn test_validate_init_data_basic() {
    // This is a basic test - in real usage, you'd need a valid signature
    let token = "test_token";
    let init_data = "query_id=123&user=%7B%22id%22%3A123%7D&auth_date=1678886400&hash=invalid";
    let result = web_app::validate_init_data(token, init_data);
    // Invalid hash should return false
    assert!(!result);
}

#[test]
fn test_validate_init_data_empty() {
    let result = web_app::validate_init_data("token", "");
    assert!(!result);
}

#[test]
fn test_validate_init_data_no_hash() {
    let result = web_app::validate_init_data("token", "key=value&key2=value2");
    assert!(!result);
}

#[test]
fn test_validate_with_secret() {
    let secret = b"test_secret_key";
    let result = web_app::validate_with_secret(secret, "key=value&hash=invalid");
    assert!(!result);
}

#[test]
fn test_validate_with_secret_empty() {
    let secret = b"test_secret";
    let result = web_app::validate_with_secret(secret, "");
    assert!(!result);
}

#[test]
fn test_full_webapp_flow() {
    use hmac::{Hmac, Mac};
    use sha2::{Digest, Sha256};

    let token = "123456:ABC-DEF";
    let secret_key = {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hasher.finalize().to_vec()
    };

    // Create valid init data
    let init_data = "query_id=123&user=%7B%22id%22%3A123%7D&auth_date=1678886400";

    // Compute hash
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&secret_key).unwrap();
    mac.update(init_data.as_bytes());
    let hash = hex::encode(mac.finalize().into_bytes());

    let full_init_data = format!("{init_data}&hash={hash}");
    let result = web_app::validate_init_data(token, &full_init_data);
    assert!(result);
}

#[test]
fn test_webapp_data_with_special_characters() {
    let init_data = "key=value%20with%20spaces&hash=invalid";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result); // Invalid hash
}

#[test]
fn test_webapp_data_unicode() {
    let init_data = "user=%7B%22name%22%3A%22%E4%B8%AD%E6%96%87%22%7D&hash=invalid";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result); // Invalid hash
}

#[test]
fn test_webapp_multiple_params() {
    let init_data = "a=1&b=2&c=3&hash=invalid";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result);
}

#[test]
fn test_webapp_empty_hash() {
    let init_data = "key=value&hash=";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result);
}
