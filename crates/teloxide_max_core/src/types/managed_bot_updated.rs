use serde::{Deserialize, Serialize};

use crate::types::User;

/// Service message: a managed bot was updated.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotupdated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotUpdated {
    /// The updated managed bot
    pub bot: Option<User>,
}
