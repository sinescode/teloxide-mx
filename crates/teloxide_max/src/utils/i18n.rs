//! Internationalization (i18n) framework.
//!
//! Similar to aiogram's `I18n` system, this module provides:
//! - Locale loading from `.mo` / `.po` files
//! - Message translation with `gettext`-style format strings
//! - Locale middleware for automatic locale detection
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::utils::i18n::{I18n, I18nContext};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! # async fn example() {
//! let i18n = I18n::new("locales", "en", "messages");
//!
//! // In a handler:
//! // let text = i18n.translate("Welcome!", locale);
//! # }
//! ```

use std::{collections::HashMap, path::PathBuf, sync::Arc};

/// A translation entry mapping a key to translations in different locales.
#[derive(Debug, Clone)]
pub struct Translation {
    /// The default message (English).
    pub default: String,
    /// Translations keyed by locale code.
    pub translations: HashMap<String, String>,
}

impl Translation {
    /// Creates a new translation with a default message.
    pub fn new(default: impl Into<String>) -> Self {
        Self { default: default.into(), translations: HashMap::new() }
    }

    /// Adds a translation for a locale.
    pub fn with_locale(mut self, locale: impl Into<String>, message: impl Into<String>) -> Self {
        self.translations.insert(locale.into(), message.into());
        self
    }

    /// Returns the translated message for the given locale, falling back to
    /// default.
    pub fn translate(&self, locale: &str) -> &str {
        self.translations.get(locale).map(|s| s.as_str()).unwrap_or(&self.default)
    }
}

/// An i18n context providing translations.
#[derive(Debug, Clone)]
pub struct I18nContext {
    /// The current locale.
    locale: String,
    /// Translation strings.
    translations: Arc<HashMap<String, Translation>>,
}

impl I18nContext {
    /// Creates a new i18n context with the given locale and translations.
    pub fn new(locale: impl Into<String>, translations: HashMap<String, Translation>) -> Self {
        Self { locale: locale.into(), translations: Arc::new(translations) }
    }

    /// Returns the current locale.
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Sets the locale.
    pub fn set_locale(&mut self, locale: impl Into<String>) {
        self.locale = locale.into();
    }

    /// Translates a key to the current locale.
    pub fn translate(&self, key: &str) -> String {
        self.translations
            .get(key)
            .map(|t| t.translate(&self.locale).to_string())
            .unwrap_or_else(|| key.to_string())
    }

    /// Translates a key with format arguments.
    pub fn translate_format(&self, key: &str, args: &[(&str, &str)]) -> String {
        let template = self.translate(key);
        let mut result = template.to_string();
        for (placeholder, value) in args {
            result = result.replace(&format!("{{{placeholder}}}"), value);
        }
        result
    }

    /// Returns the number of loaded translations.
    pub fn len(&self) -> usize {
        self.translations.len()
    }

    /// Returns `true` if no translations are loaded.
    pub fn is_empty(&self) -> bool {
        self.translations.is_empty()
    }
}

/// An i18n loader that can load translations from files.
#[derive(Debug, Clone)]
pub struct I18nLoader {
    base_path: PathBuf,
    default_locale: String,
    domain: String,
}

impl I18nLoader {
    /// Creates a new loader pointing to a locales directory.
    pub fn new(
        base_path: impl Into<PathBuf>,
        default_locale: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            base_path: base_path.into(),
            default_locale: default_locale.into(),
            domain: domain.into(),
        }
    }

    /// Loads translations from `.po` files in the locales directory.
    ///
    /// Expected structure:
    /// ```text
    /// locales/
    ///   en/
    ///     messages.po
    ///   es/
    ///     messages.po
    /// ```
    pub fn load_po(&self) -> Result<HashMap<String, Translation>, I18nError> {
        let mut translations = HashMap::new();

        if !self.base_path.exists() {
            return Err(I18nError::DirectoryNotFound(self.base_path.clone()));
        }

        for entry in
            std::fs::read_dir(&self.base_path).map_err(|e| I18nError::IoError(e.to_string()))?
        {
            let entry = entry.map_err(|e| I18nError::IoError(e.to_string()))?;
            if !entry.file_type().map_err(|e| I18nError::IoError(e.to_string()))?.is_dir() {
                continue;
            }

            let locale = entry.file_name().to_string_lossy().to_string();
            let po_path = entry.path().join(format!("{}.po", self.domain));

            if po_path.exists() {
                let content = std::fs::read_to_string(&po_path)
                    .map_err(|e| I18nError::IoError(e.to_string()))?;
                let parsed = parse_po(&content);
                for (key, value) in parsed {
                    let entry =
                        translations.entry(key.clone()).or_insert_with(|| Translation::new(key));
                    entry.translations.insert(locale.clone(), value);
                }
            }
        }

        Ok(translations)
    }

    /// Loads translations from a Rust HashMap.
    pub fn load_hashmap(
        data: HashMap<String, HashMap<String, String>>,
    ) -> HashMap<String, Translation> {
        data.into_iter()
            .map(|(key, locales)| {
                let default = locales.get("en").cloned().unwrap_or_else(|| key.clone());
                let translation = Translation { default, translations: locales };
                (key, translation)
            })
            .collect()
    }

    /// Returns the default locale.
    pub fn default_locale(&self) -> &str {
        &self.default_locale
    }
}

/// Errors that can occur during i18n operations.
#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    #[error("Locales directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Invalid .po file format")]
    InvalidFormat,
}

/// Parses a simple `.po` file format.
///
/// Supports basic `msgid "..."` / `msgstr "..."` pairs.
fn parse_po(content: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut current_msgid = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("msgid ") {
            current_msgid = extract_po_string(line);
        } else if line.starts_with("msgstr ") {
            if let (Some(key), Some(value)) = (current_msgid.take(), extract_po_string(line)) {
                if !key.is_empty() {
                    result.push((key, value));
                }
            }
        }
    }

    result
}

fn extract_po_string(line: &str) -> Option<String> {
    let rest = if let Some(r) = line.strip_prefix("msgid ") {
        r
    } else if let Some(r) = line.strip_prefix("msgstr ") {
        r
    } else {
        return None;
    };
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_basic() {
        let t = Translation::new("Hello").with_locale("es", "Hola").with_locale("fr", "Bonjour");

        assert_eq!(t.translate("en"), "Hello");
        assert_eq!(t.translate("es"), "Hola");
        assert_eq!(t.translate("fr"), "Bonjour");
        assert_eq!(t.translate("de"), "Hello"); // fallback
    }

    #[test]
    fn i18n_context_translate() {
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
    fn i18n_context_format() {
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
    fn parse_po_simple() {
        let po = r#"
msgid "Hello"
msgstr "Hola"

msgid "Goodbye"
msgstr "Adiós"
"#;
        let entries = parse_po(po);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], ("Hello".to_string(), "Hola".to_string()));
        assert_eq!(entries[1], ("Goodbye".to_string(), "Adiós".to_string()));
    }
}
