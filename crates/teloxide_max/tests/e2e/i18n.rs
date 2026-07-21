//! End-to-end tests for I18n system.
//!
//! Tests translation, i18n context, and i18n loader.

use std::collections::HashMap;
use teloxide_max::utils::i18n::{I18nContext, I18nLoader, Translation};

#[test]
fn test_translation_basic() {
    let t = Translation::new("Hello");
    assert_eq!(t.translate("en"), "Hello");
    assert_eq!(t.translate("de"), "Hello"); // fallback
}

#[test]
fn test_translation_with_locales() {
    let t = Translation::new("Hello")
        .with_locale("es", "Hola")
        .with_locale("fr", "Bonjour");

    assert_eq!(t.translate("en"), "Hello");
    assert_eq!(t.translate("es"), "Hola");
    assert_eq!(t.translate("fr"), "Bonjour");
    assert_eq!(t.translate("de"), "Hello"); // fallback
}

#[test]
fn test_translation_debug() {
    let t = Translation::new("test");
    let debug_str = format!("{t:?}");
    assert!(debug_str.contains("test"));
}

#[test]
fn test_translation_clone() {
    let t1 = Translation::new("test").with_locale("es", "prueba");
    let t2 = t1.clone();
    assert_eq!(t2.translate("en"), "test");
    assert_eq!(t2.translate("es"), "prueba");
}

#[test]
fn test_i18n_context_basic() {
    let ctx = I18nContext::new("en", HashMap::new());
    assert_eq!(ctx.locale(), "en");
}

#[test]
fn test_i18n_context_translate() {
    let mut translations = HashMap::new();
    translations.insert(
        "welcome".to_string(),
        Translation::new("Welcome!")
            .with_locale("es", "¡Bienvenido!")
            .with_locale("fr", "Bienvenue!"),
    );

    let ctx = I18nContext::new("es", translations);
    assert_eq!(ctx.translate("welcome"), "¡Bienvenido!");
}

#[test]
fn test_i18n_context_translate_fallback() {
    let mut translations = HashMap::new();
    translations.insert("welcome".to_string(), Translation::new("Welcome!"));

    let ctx = I18nContext::new("de", translations);
    // Should fallback to default (English)
    assert_eq!(ctx.translate("welcome"), "Welcome!");
}

#[test]
fn test_i18n_context_translate_missing_key() {
    let ctx = I18nContext::new("en", HashMap::new());
    // Missing key returns the key itself
    assert_eq!(ctx.translate("missing"), "missing");
}

#[test]
fn test_i18n_context_format() {
    let mut translations = HashMap::new();
    translations.insert(
        "greeting".to_string(),
        Translation::new("Hello, {name}!").with_locale("es", "¡Hola, {name}!"),
    );

    let ctx = I18nContext::new("es", translations);
    let result = ctx.translate_format("greeting", &[("name", "World")]);
    assert_eq!(result, "¡Hola, World!");
}

#[test]
fn test_i18n_context_format_multiple_params() {
    let mut translations = HashMap::new();
    translations.insert(
        "user_info".to_string(),
        Translation::new("{name} is {age} years old"),
    );

    let ctx = I18nContext::new("en", translations);
    let result = ctx.translate_format("user_info", &[("name", "Alice"), ("age", "25")]);
    assert_eq!(result, "Alice is 25 years old");
}

#[test]
fn test_i18n_context_set_locale() {
    let mut ctx = I18nContext::new("en", HashMap::new());
    ctx.set_locale("es");
    assert_eq!(ctx.locale(), "es");
}

#[test]
fn test_i18n_context_len() {
    let mut translations = HashMap::new();
    translations.insert("key1".to_string(), Translation::new("val1"));
    translations.insert("key2".to_string(), Translation::new("val2"));

    let ctx = I18nContext::new("en", translations);
    assert_eq!(ctx.len(), 2);
}

#[test]
fn test_i18n_context_is_empty() {
    let ctx = I18nContext::new("en", HashMap::new());
    assert!(ctx.is_empty());
}

#[test]
fn test_i18n_loader_new() {
    let _loader = I18nLoader::new("locales", "en", "messages");
}

#[test]
fn test_i18n_loader_default_locale() {
    let loader = I18nLoader::new("locales", "fr", "messages");
    assert_eq!(loader.default_locale(), "fr");
}

#[test]
fn test_i18n_loader_load_hashmap() {
    let mut data = HashMap::new();
    data.insert(
        "welcome".to_string(),
        HashMap::from([
            ("en".to_string(), "Welcome!".to_string()),
            ("es".to_string(), "¡Bienvenido!".to_string()),
        ]),
    );

    let translations = I18nLoader::load_hashmap(data);
    assert_eq!(translations.len(), 1);

    let welcome = translations.get("welcome").unwrap();
    assert_eq!(welcome.translate("en"), "Welcome!");
    assert_eq!(welcome.translate("es"), "¡Bienvenido!");
}

#[test]
fn test_i18n_loader_load_hashmap_fallback() {
    let mut data = HashMap::new();
    data.insert(
        "test".to_string(),
        HashMap::from([("es".to_string(), "prueba".to_string())]),
    );

    let translations = I18nLoader::load_hashmap(data);
    let test = translations.get("test").unwrap();
    // Default should be the key itself since no "en" entry
    assert_eq!(test.translate("en"), "test");
    assert_eq!(test.translate("es"), "prueba");
}

#[test]
fn test_i18n_context_debug() {
    let ctx = I18nContext::new("en", HashMap::new());
    let debug_str = format!("{ctx:?}");
    assert!(debug_str.contains("en"));
}

#[test]
fn test_i18n_context_clone() {
    let mut translations = HashMap::new();
    translations.insert("key".to_string(), Translation::new("value"));

    let ctx1 = I18nContext::new("en", translations);
    let ctx2 = ctx1.clone();
    assert_eq!(ctx2.locale(), "en");
    assert_eq!(ctx2.translate("key"), "value");
}
