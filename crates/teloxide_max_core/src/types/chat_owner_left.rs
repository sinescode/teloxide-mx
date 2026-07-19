use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes a service message about the chat owner leaving the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerleft).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerLeft {
    /// User that left and was the owner
    pub user: Option<User>,
}
