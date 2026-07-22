//! End-to-end tests for deep linking utilities.

use teloxide_max::utils::deep_linking;

#[test]
fn test_create_start_link() {
    let link = deep_linking::create_start_link("my_bot", "payload123");
    assert_eq!(link, "https://t.me/my_bot?start=payload123");
}

#[test]
fn test_create_startgroup_link() {
    let link = deep_linking::create_startgroup_link("my_bot", "group");
    assert_eq!(link, "https://t.me/my_bot?startgroup=group");
}

#[test]
fn test_create_startapp_link() {
    let link = deep_linking::create_startapp_link("my_bot", "app");
    assert_eq!(link, "https://t.me/my_bot?startapp=app");
}

#[test]
fn test_encode_payload() {
    let payload = b"hello world";
    let encoded = deep_linking::encode_payload(payload);
    assert!(!encoded.is_empty());
    // URL-safe base64 should not contain '+' or '/'
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('/'));
}

#[test]
fn test_decode_payload() {
    let payload = b"hello world";
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "hello world");
}

#[test]
fn test_encode_decode_roundtrip() {
    let original = b"test_payload_123";
    let encoded = deep_linking::encode_payload(original);
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "test_payload_123");
}

#[test]
fn test_decode_invalid() {
    // URL_SAFE_NO_PAD may still decode some strings; empty is ok
    let result = deep_linking::decode_payload("!!!");
    assert!(result.is_err());
}

#[test]
fn test_encode_empty() {
    let encoded = deep_linking::encode_payload(b"");
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn test_encode_unicode() {
    let payload = "你好世界".as_bytes();
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "你好世界");
}

#[test]
fn test_encode_special_chars() {
    let payload = b"hello+world/test=";
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "hello+world/test=");
}

#[test]
fn test_encode_string_owned() {
    let payload = String::from("owned");
    let encoded = deep_linking::encode_payload(payload.as_bytes());
    let decoded = deep_linking::decode_payload(&encoded).unwrap();
    assert_eq!(decoded, "owned");
}

#[test]
fn test_deep_link_with_encoded_payload() {
    let encoded = deep_linking::encode_payload(b"secret data");
    let link = deep_linking::create_start_link("mybot", &encoded);
    assert!(link.starts_with("https://t.me/mybot?start="));
    assert!(link.contains(&encoded));
}
