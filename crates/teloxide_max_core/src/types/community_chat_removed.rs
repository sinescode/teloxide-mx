use serde::{Deserialize, Serialize};

use crate::types::{Chat, Community};

/// Service message about a chat being removed from a community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatremoved).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatRemoved {
    /// Community the chat was removed from
    pub community: Option<Community>,
    /// Chat that was removed
    pub chat: Option<Chat>,
}
