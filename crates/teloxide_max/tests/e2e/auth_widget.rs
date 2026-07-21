//! End-to-end tests for Auth Widget validation.
//!
//! Tests the Telegram Login Widget signature verification.

use teloxide_max::utils::auth_widget;

#[test]
fn test_check_signature_valid() {
    // Simulate a valid widget callback
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let secret = sha256(token);
    let data_check = "auth_date=1678886400\nfirst_name=John\nid=123456789\nusername=john_doe";
    let hash = hmac_sha256(&secret, data_check);

    let query = vec![
        ("auth_date".into(), "1678886400".into()),
        ("first_name".into(), "John".into()),
        ("id".into(), "123456789".into()),
        ("username".into(), "john_doe".into()),
    ];

    assert!(auth_widget::check_signature(token, &hash, &query));
}

#[test]
fn test_check_signature_invalid_hash() {
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let query = vec![("id".into(), "12345".into())];

    assert!(!auth_widget::check_signature(token, "invalid_hash", &query));
}

#[test]
fn test_check_signature_empty_hash() {
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let query = vec![("id".into(), "12345".into())];

    assert!(!auth_widget::check_signature(token, "", &query));
}

#[test]
fn test_check_signature_wrong_token() {
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let secret = sha256(token);
    let data_check = "id=12345";
    let hash = hmac_sha256(&secret, data_check);

    // Use wrong token
    let wrong_token = "999999:WRONG-TOKEN";
    let query = vec![("id".into(), "12345".into())];

    assert!(!auth_widget::check_signature(wrong_token, &hash, &query));
}

#[test]
fn test_check_integrity_json_valid() {
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let secret = sha256(token);
    let data_check = "id=12345";
    let hash = hmac_sha256(&secret, data_check);

    let data = serde_json::json!({
        "id": "12345",
        "hash": hash
    });

    assert!(auth_widget::check_integrity(token, &data));
}

#[test]
fn test_check_integrity_json_missing_hash() {
    let data = serde_json::json!({
        "id": "12345"
    });

    assert!(!auth_widget::check_integrity("token", &data));
}

#[test]
fn test_check_integrity_json_wrong_hash() {
    let data = serde_json::json!({
        "id": "12345",
        "hash": "wrong_hash"
    });

    assert!(!auth_widget::check_integrity("token", &data));
}

#[test]
fn test_check_integrity_map_valid() {
    let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
    let secret = sha256(token);
    let data_check = "id=12345";
    let hash = hmac_sha256(&secret, data_check);

    let mut data = std::collections::HashMap::new();
    data.insert("id".to_string(), "12345".to_string());
    data.insert("hash".to_string(), hash);

    assert!(auth_widget::check_integrity_map(token, &data));
}

#[test]
fn test_check_integrity_map_missing_hash() {
    let data = std::collections::HashMap::new();
    assert!(!auth_widget::check_integrity_map("token", &data));
}

#[test]
fn test_full_widget_flow() {
    let token = "bot_token_123";
    let secret = sha256(token);

    // Simulate complete widget data
    let auth_date = "1678886400";
    let id = "987654321";
    let first_name = "Alice";
    let last_name = "Smith";
    let username = "alice_smith";
    let photo_url = "https://example.com/photo.jpg";

    let data_check_string = format!(
        "auth_date={auth_date}\nfirst_name={first_name}\nid={id}\nlast_name={last_name}\nphoto_url={photo_url}\nusername={username}"
    );

    let hash = hmac_sha256(&secret, &data_check_string);

    let query = vec![
        ("auth_date".into(), auth_date.into()),
        ("first_name".into(), first_name.into()),
        ("id".into(), id.into()),
        ("last_name".into(), last_name.into()),
        ("photo_url".into(), photo_url.into()),
        ("username".into(), username.into()),
    ];

    assert!(auth_widget::check_signature(token, &hash, &query));
}

#[test]
fn test_signature_sorted_keys() {
    let token = "test_token";
    let secret = sha256(token);

    // Keys should be sorted alphabetically
    let data_check = "b=2\na=1\nc=3";
    let hash = hmac_sha256(&secret, data_check);

    // Pass in random order
    let query = vec![
        ("c".into(), "3".into()),
        ("a".into(), "1".into()),
        ("b".into(), "2".into()),
    ];

    assert!(auth_widget::check_signature(token, &hash, &query));
}

// Helper functions
fn sha256(input: &str) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

fn hmac_sha256(key: &[u8], message: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}
