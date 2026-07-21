//! End-to-end tests for Deep Linking.
//!
//! Tests payload encoding/decoding and link creation.

use teloxide_max::utils::deep_linking;

#[test]
fn test_encode_payload() {
    let payload = "Hello World";
    let encoded = deep_linking::encode_payload(payload);
    assert!(!encoded.is_empty());
    // Encoded should be URL-safe
    assert!(!encoded.contains(' '));
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('='));
}

#[test]
fn test_decode_payload() {
    let payload = "Hello World";
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(payload.to_string()));
}

#[test]
fn test_encode_decode_roundtrip() {
    let original = "test_payload_123";
    let encoded = deep_linking::encode_payload(original);
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(original.to_string()));
}

#[test]
fn test_decode_invalid_base64() {
    let result = deep_linking::decode_payload("not_valid_base64!!!!");
    assert!(result.is_none());
}

#[test]
fn test_encode_empty_payload() {
    let encoded = deep_linking::encode_payload("");
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some("".to_string()));
}

#[test]
fn test_encode_special_characters() {
    let payload = "Hello!@#$%^&*()";
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(payload.to_string()));
}

#[test]
fn test_encode_unicode() {
    let payload = "Hello 世界";
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(payload.to_string()));
}

#[test]
fn test_encode_long_payload() {
    let payload = "a".repeat(1000);
    let encoded = deep_linking::encode_payload(payload.clone());
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(payload));
}

#[test]
fn test_encode_payload_length() {
    let payload = "short";
    let encoded = deep_linking::encode_payload(payload);
    // Base64 encoding increases length by ~33%
    assert!(encoded.len() <= payload.len() * 2);
}

#[test]
fn test_decode_empty_string() {
    let result = deep_linking::decode_payload("");
    assert_eq!(result, Some("".to_string()));
}

#[test]
fn test_encode_url_safe() {
    let payload = "test+with/special=chars";
    let encoded = deep_linking::encode_payload(payload);
    // Should not contain URL-unsafe characters
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('/'));
    assert!(!encoded.contains('='));
}

#[test]
fn test_roundtrip_complex_payload() {
    let payload = r#"{"user_id":123,"action":"start","data":"hello world"}"#;
    let encoded = deep_linking::encode_payload(payload);
    let decoded = deep_linking::decode_payload(&encoded);
    assert_eq!(decoded, Some(payload.to_string()));
}
