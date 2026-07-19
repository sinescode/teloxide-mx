use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// Describes a service message about an option deleted from a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptiondeleted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    /// Unique identifier of the deleted option
    pub option_persistent_id: String,
    /// Option text
    pub option_text: String,
    /// Message containing the poll from which the option was deleted, if known
    pub poll_message: Option<MaybeInaccessibleMessage>,
    /// Special entities that appear in the option text
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
