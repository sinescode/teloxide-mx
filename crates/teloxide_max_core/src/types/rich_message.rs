use serde::{Deserialize, Serialize};

use crate::types::RichBlock;

/// Represents a rich formatted message received in a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// List of rich blocks the message is composed of
    pub blocks: Vec<RichBlock>,
}
