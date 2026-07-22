//! Lazy translation support.
//!
//! Similar to aiogram's `lazy_gettext`, this module provides a lazy translation
//! proxy that defers translation until the string is actually used.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::utils::lazy_i18n::{LazyTranslation, lazy_gettext};
//! # use std::sync::Arc;
//! # use teloxide_max::utils::i18n::I18nLoader;
//!
//! # fn example() {
//! // Create a lazy translation
//! let text = lazy_gettext("welcome_message");
//!
//! // Or with parameters
//! let text = lazy_gettext!("greeting", "name" => "Alice");
//!
//! // Translation happens when Display is called
//! println!("{}", text); // Translates at this point
//! # }
//! ```

use std::{fmt, sync::Arc};

use crate::utils::i18n::I18nContext;

/// A lazy translation proxy that translates on display, not at creation.
///
/// This is useful when you need to store a translation key that will be
/// translated later, for example in callback data or keyboard buttons.
#[derive(Debug, Clone)]
pub struct LazyTranslation {
    key: String,
    params: Vec<(String, String)>,
    context: Option<Arc<I18nContext>>,
}

impl LazyTranslation {
    /// Creates a new lazy translation with just a key.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into(), params: Vec::new(), context: None }
    }

    /// Creates a lazy translation with format parameters.
    pub fn with_params(key: impl Into<String>, params: Vec<(String, String)>) -> Self {
        Self { key: key.into(), params, context: None }
    }

    /// Sets the i18n context used for translation at display time.
    pub fn with_context(mut self, context: Arc<I18nContext>) -> Self {
        self.context = Some(context);
        self
    }

    /// Alias for [`with_context`] for aiogram-style naming.
    pub fn with_loader(self, context: Arc<I18nContext>) -> Self {
        self.with_context(context)
    }

    /// Returns the translation key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the format parameters.
    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }

    /// Forces translation using the stored context.
    ///
    /// If no context is set, returns the key itself as fallback.
    pub fn force_translate(&self) -> String {
        match &self.context {
            Some(ctx) => {
                let mut result = ctx.translate(&self.key);
                for (placeholder, value) in &self.params {
                    result = result.replace(&format!("{{{placeholder}}}"), value);
                }
                result
            }
            None => self.key.clone(),
        }
    }
}

impl fmt::Display for LazyTranslation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.force_translate())
    }
}

impl From<String> for LazyTranslation {
    fn from(key: String) -> Self {
        Self::new(key)
    }
}

impl From<&str> for LazyTranslation {
    fn from(key: &str) -> Self {
        Self::new(key)
    }
}

/// Creates a lazy translation from a key string.
pub fn lazy_gettext(key: impl Into<String>) -> LazyTranslation {
    LazyTranslation::new(key)
}

/// Creates a lazy translation with format parameters.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::utils::lazy_i18n::lazy_gettext_with_params;
/// let text = lazy_gettext_with_params("greeting", &[("name", "Alice"), ("age", "25")]);
/// ```
pub fn lazy_gettext_with_params(key: &str, params: &[(&str, &str)]) -> LazyTranslation {
    LazyTranslation::with_params(
        key,
        params.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
    )
}

/// Macro for creating lazy translations with inline parameters.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::utils::lazy_i18n::lazy_gettext;
/// let text = lazy_gettext!("welcome", "name" => "Alice", "count" => "5");
/// ```
#[macro_export]
macro_rules! lazy_gettext {
    ($key:expr) => {
        $crate::utils::lazy_i18n::LazyTranslation::new($key)
    };
    ($key:expr, $( $param:expr => $value:expr ),+ $(,)?) => {
        $crate::utils::lazy_i18n::LazyTranslation::with_params(
            $key,
            vec![$(($param.to_string(), $value.to_string()),)+],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lazy_translation_basic() {
        let lt = LazyTranslation::new("hello");
        assert_eq!(lt.key(), "hello");
        assert!(lt.params().is_empty());
        // Without loader, returns key
        assert_eq!(lt.to_string(), "hello");
    }

    #[test]
    fn lazy_translation_with_params() {
        let lt = LazyTranslation::with_params("greeting", vec![("name".into(), "Alice".into())]);
        assert_eq!(lt.key(), "greeting");
        assert_eq!(lt.params().len(), 1);
    }

    #[test]
    fn lazy_gettext_macro() {
        let lt = lazy_gettext!("test_key");
        assert_eq!(lt.key(), "test_key");
    }

    #[test]
    fn lazy_gettext_macro_with_params() {
        let lt = lazy_gettext!("greeting", "name" => "Bob");
        assert_eq!(lt.key(), "greeting");
        assert_eq!(lt.params().len(), 1);
        assert_eq!(lt.params()[0], ("name".to_string(), "Bob".to_string()));
    }

    #[test]
    fn lazy_translation_from_string() {
        let lt: LazyTranslation = "hello".into();
        assert_eq!(lt.key(), "hello");
    }

    #[test]
    fn lazy_translation_from_string_owned() {
        let lt: LazyTranslation = "hello".to_string().into();
        assert_eq!(lt.key(), "hello");
    }

    #[test]
    fn lazy_translation_display_without_loader() {
        let lt = lazy_gettext!("fallback_key");
        assert_eq!(format!("{lt}"), "fallback_key");
    }

    #[test]
    fn lazy_gettext_with_params_fn() {
        let lt = lazy_gettext_with_params("test", &[("a", "1"), ("b", "2")]);
        assert_eq!(lt.key(), "test");
        assert_eq!(lt.params().len(), 2);
    }
}
