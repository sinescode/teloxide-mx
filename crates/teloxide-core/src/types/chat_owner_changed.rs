use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes a service message about an ownership change in the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerchanged).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerChanged {
    /// User that is the new owner
    pub user: Option<User>,
    /// Previous owner
    pub old_owner: Option<User>,
}
