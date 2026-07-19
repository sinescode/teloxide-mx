use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a managed bot.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request
    pub request_id: i32,
    /// Pass true to request a bot that supports guest queries
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub supports_guest_queries: bool,
}
