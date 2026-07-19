use serde::{Deserialize, Serialize};

use crate::types::{Chat, Community};

/// Service message about a chat being added to a community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatAdded {
    /// Community the chat was added to
    pub community: Option<Community>,
    /// Chat that was added
    pub chat: Option<Chat>,
}
