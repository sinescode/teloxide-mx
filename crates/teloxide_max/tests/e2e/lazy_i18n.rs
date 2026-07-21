//! End-to-end tests for Lazy i18n.
//!
//! Tests the lazy translation system.

use teloxide_max::utils::lazy_i18n::{lazy_gettext, lazy_gettext_with_params, LazyTranslation};

#[test]
fn test_lazy_translation_basic() {
    let lt = lazy_gettext("hello");
    assert_eq!(lt.key(), "hello");
    assert!(lt.params().is_empty());
    // Without loader, returns the key itself
    assert_eq!(lt.to_string(), "hello");
}

#[test]
fn test_lazy_translation_with_params() {
    let lt = LazyTranslation::with_params(
        "greeting",
        vec![("name".into(), "Alice".into()), ("age".into(), "25".into())],
    );

    assert_eq!(lt.key(), "greeting");
    assert_eq!(lt.params().len(), 2);
    assert_eq!(lt.params()[0], ("name".to_string(), "Alice".to_string()));
    assert_eq!(lt.params()[1], ("age".to_string(), "25".to_string()));
}

#[test]
fn test_lazy_gettext_macro_basic() {
    let lt = lazy_gettext!("welcome_message");
    assert_eq!(lt.key(), "welcome_message");
}

#[test]
fn test_lazy_gettext_macro_with_params() {
    let lt = lazy_gettext!("greeting", "name" => "Bob", "count" => "5");
    assert_eq!(lt.key(), "greeting");
    assert_eq!(lt.params().len(), 2);
    assert!(lt.params().contains(&("name".to_string(), "Bob".to_string())));
    assert!(lt.params().contains(&("count".to_string(), "5".to_string())));
}

#[test]
fn test_lazy_gettext_with_params_fn() {
    let lt = lazy_gettext_with_params("test_key", &[("a", "1"), ("b", "2")]);
    assert_eq!(lt.key(), "test_key");
    assert_eq!(lt.params().len(), 2);
}

#[test]
fn test_lazy_translation_from_str() {
    let lt: LazyTranslation = "hello".into();
    assert_eq!(lt.key(), "hello");
}

#[test]
fn test_lazy_translation_from_string() {
    let lt: LazyTranslation = "hello_world".to_string().into();
    assert_eq!(lt.key(), "hello_world");
}

#[test]
fn test_lazy_translation_display_without_loader() {
    let lt = lazy_gettext!("fallback_key");
    assert_eq!(format!("{lt}"), "fallback_key");
}

#[test]
fn test_lazy_translation_clone() {
    let lt1 = lazy_gettext!("test");
    let lt2 = lt1.clone();
    assert_eq!(lt1.key(), lt2.key());
}

#[test]
fn test_lazy_translation_debug() {
    let lt = lazy_gettext!("debug_test");
    let debug_str = format!("{lt:?}");
    assert!(debug_str.contains("debug_test"));
}

#[test]
fn test_lazy_translation_multiple_params() {
    let params = vec![
        ("name".into(), "Alice".into()),
        ("city".into(), "NYC".into()),
        ("country".into(), "USA".into()),
    ];
    let lt = LazyTranslation::with_params("user_info", params);
    assert_eq!(lt.params().len(), 3);
}

#[test]
fn test_lazy_translation_empty_key() {
    let lt = lazy_gettext!("");
    assert_eq!(lt.key(), "");
    assert_eq!(lt.to_string(), "");
}
