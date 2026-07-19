use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

use crate::types::CallbackQuery;

/// Maximum length of callback data (Telegram limit).
pub const MAX_CALLBACK_DATA_LEN: usize = 64;

/// Error type for callback data operations.
#[derive(Debug)]
pub enum CallbackDataError {
    /// The callback data is too long (exceeds 64 bytes).
    TooLong { len: usize, max: usize },
    /// The callback data could not be serialized.
    SerializationError(String),
    /// The callback data could not be deserialized.
    DeserializationError(String),
    /// The separator was found in the prefix.
    SeparatorInPrefix { separator: char, prefix: String },
}

impl Display for CallbackDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "Callback data is too long: {len} bytes (max {max})")
            }
            Self::SerializationError(e) => write!(f, "Failed to serialize callback data: {e}"),
            Self::DeserializationError(e) => {
                write!(f, "Failed to deserialize callback data: {e}")
            }
            Self::SeparatorInPrefix { separator, prefix } => {
                write!(f, "Separator '{separator}' cannot be used inside prefix '{prefix}'")
            }
        }
    }
}

impl Error for CallbackDataError {}

/// A trait for typed callback data.
///
/// This trait allows you to define callback data as a Rust enum/struct,
/// serialize it to a string for Telegram, and deserialize it back when
/// receiving a callback query.
///
/// # Example
///
/// ```rust
/// use teloxide_max::{prelude::*, utils::callback_data::CallbackData};
///
/// #[derive(Clone, CallbackData, Debug)]
/// #[callback_data(prefix = "btn")]
/// enum Action {
///     #[callback_data(suffix = "ok")]
///     Confirm,
///     #[callback_data(suffix = "no")]
///     Cancel,
///     #[callback_data(suffix = "page")]
///     Page { num: u32 },
/// }
///
/// // Serialize
/// let data = Action::Confirm;
/// assert_eq!(data.to_string(), "btn:ok");
///
/// let data = Action::Page { num: 42 };
/// assert_eq!(data.to_string(), "btn:page:42");
///
/// // Deserialize
/// let parsed: Action = "btn:ok".parse().unwrap();
/// assert!(matches!(parsed, Action::Confirm));
///
/// let parsed: Action = "btn:page:42".parse().unwrap();
/// assert!(matches!(parsed, Action::Page { num: 42 }));
/// ```
pub trait CallbackData: Sized + Clone + Debug + Send + Sync + 'static {
    /// Returns the prefix for all callback data variants.
    fn prefix() -> &'static str;

    /// Returns the separator character.
    fn separator() -> char {
        ':'
    }

    /// Serializes this callback data to a string.
    fn serialize(&self) -> Result<String, CallbackDataError>;

    /// Deserializes callback data from a string.
    fn deserialize(data: &str) -> Result<Self, CallbackDataError>;

    /// Returns the callback data as a string, suitable for use with
    /// `InlineKeyboardButton::callback`.
    fn into_callback_string(self) -> Result<String, CallbackDataError> {
        self.serialize()
    }
}

/// Extension trait for filtering callback queries by typed callback data.
pub trait CallbackDataExt {
    /// Filters this callback query by the given callback data type.
    ///
    /// If the callback query's data matches the prefix and can be deserialized
    /// into `T`, it returns `Some((CallbackQuery, T))`. Otherwise, returns
    /// `None`.
    fn filter_callback_data<T: CallbackData>(&self) -> Option<(CallbackQuery, T)>;
}

impl CallbackDataExt for CallbackQuery {
    fn filter_callback_data<T: CallbackData>(&self) -> Option<(CallbackQuery, T)> {
        let data = self.data.as_deref()?;
        if !data.starts_with(T::prefix()) {
            return None;
        }
        let parsed = T::deserialize(data).ok()?;
        Some((self.clone(), parsed))
    }
}

/// Helper function to filter callback data from a callback query.
///
/// This can be used with `dptree::filter_map` to create typed callback data
/// filters.
///
/// # Example
///
/// ```rust
/// use teloxide_max::{
///     prelude::*,
///     utils::callback_data::{filter_callback_data, CallbackData},
/// };
///
/// #[derive(Clone, CallbackData, Debug)]
/// #[callback_data(prefix = "btn")]
/// enum Action {
///     #[callback_data(suffix = "ok")]
///     Confirm,
///     #[callback_data(suffix = "no")]
///     Cancel,
/// }
///
/// async fn handle_confirm(bot: Bot, q: CallbackQuery, action: Action) -> ResponseResult<()> {
///     match action {
///         Action::Confirm => {
///             bot.answer_callback_query(&q.id).text("Confirmed!").await?;
///         }
///         Action::Cancel => {
///             bot.answer_callback_query(&q.id).text("Cancelled!").await?;
///         }
///     }
///     Ok(())
/// }
///
/// fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync>> {
///     Update::filter_callback_query()
///         .branch(dptree::filter_map(filter_callback_data::<Action>).endpoint(handle_confirm))
/// }
/// ```
pub fn filter_callback_data<T: CallbackData>(query: CallbackQuery) -> Option<(CallbackQuery, T)> {
    query.filter_callback_data()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct TestCallbackData {
        prefix: String,
        action: String,
        id: Option<u32>,
    }

    impl CallbackData for TestCallbackData {
        fn prefix() -> &'static str {
            "test"
        }

        fn serialize(&self) -> Result<String, CallbackDataError> {
            let mut result = format!("{}:{}", self.prefix, self.action);
            if let Some(id) = self.id {
                result.push(':');
                result.push_str(&id.to_string());
            }
            Ok(result)
        }

        fn deserialize(data: &str) -> Result<Self, CallbackDataError> {
            let prefix = Self::prefix();
            if !data.starts_with(prefix) {
                return Err(CallbackDataError::DeserializationError(format!(
                    "Data doesn't start with prefix '{prefix}'"
                )));
            }
            let rest = &data[prefix.len()..];
            if !rest.starts_with(':') {
                return Err(CallbackDataError::DeserializationError(
                    "Missing separator after prefix".to_string(),
                ));
            }
            let rest = &rest[1..];
            let parts: Vec<&str> = rest.split(':').collect();
            match parts.len() {
                1 => {
                    Ok(Self { prefix: prefix.to_string(), action: parts[0].to_string(), id: None })
                }
                2 => {
                    let id = parts[1]
                        .parse::<u32>()
                        .map_err(|e| CallbackDataError::DeserializationError(e.to_string()))?;
                    Ok(Self {
                        prefix: prefix.to_string(),
                        action: parts[0].to_string(),
                        id: Some(id),
                    })
                }
                _ => Err(CallbackDataError::DeserializationError("Too many parts".to_string())),
            }
        }
    }

    #[test]
    fn test_serialize() {
        let data = TestCallbackData {
            prefix: "test".to_string(),
            action: "confirm".to_string(),
            id: None,
        };
        assert_eq!(data.serialize().unwrap(), "test:confirm");
    }

    #[test]
    fn test_serialize_with_id() {
        let data = TestCallbackData {
            prefix: "test".to_string(),
            action: "page".to_string(),
            id: Some(42),
        };
        assert_eq!(data.serialize().unwrap(), "test:page:42");
    }

    #[test]
    fn test_deserialize() {
        let data = TestCallbackData::deserialize("test:confirm").unwrap();
        assert_eq!(data.action, "confirm");
        assert_eq!(data.id, None);
    }

    #[test]
    fn test_deserialize_with_id() {
        let data = TestCallbackData::deserialize("test:page:42").unwrap();
        assert_eq!(data.action, "page");
        assert_eq!(data.id, Some(42));
    }

    #[test]
    fn test_deserialize_wrong_prefix() {
        let result = TestCallbackData::deserialize("wrong:confirm");
        assert!(result.is_err());
    }

    #[test]
    fn test_into_callback_string() {
        let data = TestCallbackData {
            prefix: "test".to_string(),
            action: "confirm".to_string(),
            id: None,
        };
        assert_eq!(data.into_callback_string().unwrap(), "test:confirm");
    }
}
