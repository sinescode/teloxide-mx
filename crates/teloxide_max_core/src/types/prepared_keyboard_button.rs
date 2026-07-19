use serde::{Deserialize, Serialize};

/// Describes a prepared keyboard button that can be shared with Mini Apps.
///
/// [The official docs](https://core.telegram.org/bots/api#preparedkeyboardbutton).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the prepared button
    pub id: String,
    /// Expiration date of the prepared button, in Unix time
    pub expiration_date: i64,
}
