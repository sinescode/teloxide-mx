use serde::{Deserialize, Serialize};

use crate::types::MessageId;

/// Describes a message sent as a reply to a guest query.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent message
    pub message_id: MessageId,
    /// Identifier of the inline message, if sent as inline
    pub inline_message_id: Option<String>,
}
