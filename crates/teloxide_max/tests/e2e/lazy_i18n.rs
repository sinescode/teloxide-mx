//! End-to-end tests for Lazy i18n.

use teloxide_max::{
    lazy_gettext,
    utils::lazy_i18n::{lazy_gettext, lazy_gettext_with_params, LazyTranslation},
};

#[test]
fn test_lazy_translation_basic() {
    let lt = lazy_gettext("hello");
    assert_eq!(lt.key(), "hello");
    assert!(lt.params().is_empty());
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
fn test_lazy_with_context() {
    use std::{collections::HashMap, sync::Arc};
    use teloxide_max::utils::i18n::{I18nContext, Translation};

    let mut map = HashMap::new();
    map.insert("hello".into(), Translation::new("Hello").with_locale("es", "Hola"));
    let ctx = Arc::new(I18nContext::new("es", map));
    let lt = LazyTranslation::new("hello").with_context(ctx);
    assert_eq!(lt.to_string(), "Hola");
}
