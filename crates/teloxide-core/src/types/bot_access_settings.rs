use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object describes the access settings of a bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// `true`, if only selected users can access the bot. The bot's owner can always access it
    pub is_access_restricted: bool,
    /// The list of other users who have access to the bot if the access is restricted
    pub added_users: Option<Vec<User>>,
}
