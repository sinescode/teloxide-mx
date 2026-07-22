//! End-to-end tests for Error Filters.
//!
//! Tests ExceptionTypeFilter, ExceptionMessageFilter, and ErrorFilterChain.

use teloxide_max::dispatching::middleware::{
    ErrorEvent, ErrorFilterChain, ExceptionMessageFilter, ExceptionTypeFilter,
};

#[test]
fn test_exception_type_filter_match() {
    let filter = ExceptionTypeFilter::new("TelegramBadRequest");
    assert!(filter.matches("TelegramBadRequest: bad request"));
    assert!(!filter.matches("SomeOtherError: something"));
}

#[test]
fn test_exception_type_filter_case_sensitive() {
    let filter = ExceptionTypeFilter::new("Error");
    assert!(filter.matches("SomeError occurred"));
    assert!(!filter.matches("someerror occurred")); // case sensitive
}

#[test]
fn test_exception_type_filter_many() {
    let filter = ExceptionTypeFilter::new_many(vec![
        "TelegramForbiddenError".into(),
        "TelegramNotFound".into(),
        "TelegramBadRequest".into(),
    ]);

    assert!(filter.matches("TelegramForbiddenError: bot blocked"));
    assert!(filter.matches("TelegramNotFound: user not found"));
    assert!(filter.matches("TelegramBadRequest: invalid query"));
    assert!(!filter.matches("SomeOtherError: unknown"));
}

#[test]
fn test_exception_type_filter_partial_match() {
    let filter = ExceptionTypeFilter::new("Telegram");
    assert!(filter.matches("TelegramBadRequest: error"));
    assert!(filter.matches("TelegramForbiddenError: error"));
    assert!(!filter.matches("NetworkError: timeout"));
}

#[test]
fn test_exception_message_filter_exact() {
    let filter = ExceptionMessageFilter::new("timed out");
    assert!(filter.matches("Connection timed out"));
    assert!(filter.matches("Request timed out after 30s"));
    assert!(!filter.matches("Connection refused"));
}

#[test]
fn test_exception_message_filter_regex() {
    let filter = ExceptionMessageFilter::regex(r"(?i)error \d{3}");
    assert!(filter.matches("Got error 404"));
    assert!(filter.matches("Error 500 occurred"));
    assert!(!filter.matches("No error here"));
    assert!(!filter.matches("error abc")); // not 3 digits
}

#[test]
fn test_exception_message_filter_regex_invalid_pattern() {
    // Invalid regex should not match anything
    let filter = ExceptionMessageFilter::regex(r"[invalid");
    assert!(!filter.matches("anything"));
}

#[test]
fn test_error_filter_chain_empty() {
    let chain = ErrorFilterChain::new();
    assert!(!chain.matches("any error"));
}

#[test]
fn test_error_filter_chain_single() {
    let chain = ErrorFilterChain::new().add_filter(ExceptionTypeFilter::new("TelegramError"));
    assert!(chain.matches("TelegramError: something"));
    assert!(!chain.matches("OtherError: something"));
}

#[test]
fn test_error_filter_chain_multiple() {
    let chain = ErrorFilterChain::new()
        .add_filter(ExceptionTypeFilter::new("TelegramForbiddenError"))
        .add_filter(ExceptionTypeFilter::new("TelegramNotFound"))
        .add_message_filter(ExceptionMessageFilter::new("timed out"));

    assert!(chain.matches("TelegramForbiddenError: blocked"));
    assert!(chain.matches("TelegramNotFound: user"));
    assert!(chain.matches("Connection timed out"));
    assert!(!chain.matches("Unknown error"));
}

#[test]
fn test_error_filter_chain_mixed_types() {
    let chain = ErrorFilterChain::new()
        .add_filter(ExceptionTypeFilter::new("TelegramBadRequest"))
        .add_message_filter(ExceptionMessageFilter::regex(r"rate limit \d+"));

    assert!(chain.matches("TelegramBadRequest: invalid"));
    assert!(chain.matches("rate limit 429"));
    assert!(!chain.matches("other error"));
}

#[test]
fn test_error_event_creation_from_error() {
    let error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let event = ErrorEvent::new(&error);

    assert!(event.error.contains("file not found"));
    assert!(!event.error_type.is_empty());
}

#[test]
fn test_error_event_from_string() {
    let event = ErrorEvent::from_string("test error message");
    assert_eq!(event.error, "test error message");
    assert_eq!(event.error_type, "unknown");
}

#[test]
fn test_error_event_debug() {
    let event = ErrorEvent::from_string("debug test");
    let debug_str = format!("{event:?}");
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_error_event_clone() {
    let event1 = ErrorEvent::from_string("clone test");
    let event2 = event1.clone();
    assert_eq!(event1.error, event2.error);
    assert_eq!(event1.error_type, event2.error_type);
}

#[test]
fn test_error_filter_chain_default() {
    let chain = ErrorFilterChain::default();
    assert!(!chain.matches("any error"));
}

#[test]
fn test_exception_message_filter_empty_pattern() {
    let filter = ExceptionMessageFilter::new("");
    assert!(filter.matches("anything")); // empty pattern matches everything
}

#[test]
fn test_exception_type_filter_empty() {
    let filter = ExceptionTypeFilter::new("");
    assert!(filter.matches("anything")); // empty pattern matches everything
}

#[test]
fn test_error_filter_chain_order_matters() {
    let chain = ErrorFilterChain::new()
        .add_filter(ExceptionTypeFilter::new("Error1"))
        .add_filter(ExceptionTypeFilter::new("Error2"));

    assert!(chain.matches("Error1 occurred"));
    assert!(chain.matches("Error2 occurred"));
    assert!(!chain.matches("Error3 occurred"));
}

#[test]
fn test_error_filter_chain_len() {
    let chain = ErrorFilterChain::new()
        .add_filter(ExceptionTypeFilter::new("Error1"))
        .add_message_filter(ExceptionMessageFilter::new("pattern"));
    assert_eq!(chain.len(), 2);
}

#[test]
fn test_error_filter_chain_is_empty() {
    let chain = ErrorFilterChain::new();
    assert!(chain.is_empty());
}

#[test]
fn test_error_filter_chain_not_empty() {
    let chain = ErrorFilterChain::new().add_filter(ExceptionTypeFilter::new("Error"));
    assert!(!chain.is_empty());
}
