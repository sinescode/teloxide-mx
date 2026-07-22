//! End-to-end tests for WebApp validation.
//!
//! Verifies the official Telegram Mini App HMAC algorithm:
//! secret = HMAC_SHA256(key="WebAppData", msg=bot_token)
//! hash   = HMAC_SHA256(key=secret, msg=sorted data_check_string)

use teloxide_max::utils::web_app;

#[test]
fn test_validate_init_data_basic() {
    let token = "test_token";
    let init_data = "query_id=123&user=%7B%22id%22%3A123%7D&auth_date=1678886400&hash=invalid";
    let result = web_app::validate_init_data(token, init_data);
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
    let secret = b"test_secret_key_32_bytes_padding!";
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
fn test_full_webapp_flow_official_algorithm() {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let token = "123456:ABC-DEF";
    let secret_key = web_app::compute_secret_key(token);

    // Official algorithm: HMAC_SHA256(key="WebAppData", msg=token)
    let mut expected_secret = HmacSha256::new_from_slice(b"WebAppData").unwrap();
    expected_secret.update(token.as_bytes());
    assert_eq!(secret_key, expected_secret.finalize().into_bytes().to_vec());

    // Sorted check string (keys alphabetical): auth_date, query_id, user
    let data_check_string = "auth_date=1678886400\nquery_id=AAE\nuser={\"id\":123}";

    let mut mac = HmacSha256::new_from_slice(&secret_key).unwrap();
    mac.update(data_check_string.as_bytes());
    let hash = hex::encode(mac.finalize().into_bytes());

    // Unsorted query order + URL-encoded user JSON — must still validate.
    let full_init_data =
        format!("user=%7B%22id%22%3A123%7D&query_id=AAE&auth_date=1678886400&hash={hash}");

    assert!(web_app::validate_init_data(token, &full_init_data));
    assert!(web_app::check_webapp_signature(token, &full_init_data));

    let parsed = web_app::safe_parse_init_data(token, &full_init_data).unwrap();
    assert_eq!(parsed.get("query_id").map(String::as_str), Some("AAE"));
    assert_eq!(parsed.get("auth_date").map(String::as_str), Some("1678886400"));
}

#[test]
fn test_safe_parse_rejects_invalid() {
    let err = web_app::safe_parse_init_data("token", "a=1&hash=deadbeef").unwrap_err();
    assert_eq!(err, "Invalid init data signature");
}

#[test]
fn test_parse_init_data_decodes() {
    let map = web_app::parse_init_data("name=hello%20world&flag=1");
    assert_eq!(map.get("name").map(String::as_str), Some("hello world"));
    assert_eq!(map.get("flag").map(String::as_str), Some("1"));
}

#[test]
fn test_webapp_data_with_special_characters() {
    let init_data = "key=value%20with%20spaces&hash=invalid";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result);
}

#[test]
fn test_webapp_data_unicode() {
    let init_data = "user=%7B%22name%22%3A%22%E4%B8%AD%E6%96%87%22%7D&hash=invalid";
    let result = web_app::validate_with_secret(b"secret", init_data);
    assert!(!result);
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

#[test]
fn test_plain_sha256_is_not_accepted() {
    // Regression: old teloxide code used SHA256(token) as secret — must fail.
    use hmac::{Hmac, Mac};
    use sha2::{Digest, Sha256};

    type HmacSha256 = Hmac<Sha256>;

    let token = "123456:ABC-DEF";
    let wrong_secret = {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hasher.finalize().to_vec()
    };

    let data_check_string = "auth_date=1\nquery_id=x";
    let mut mac = HmacSha256::new_from_slice(&wrong_secret).unwrap();
    mac.update(data_check_string.as_bytes());
    let hash = hex::encode(mac.finalize().into_bytes());

    let init_data = format!("query_id=x&auth_date=1&hash={hash}");
    assert!(!web_app::validate_init_data(token, &init_data));
}
