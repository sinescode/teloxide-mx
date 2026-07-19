use serde::{Deserialize, Serialize};

use crate::types::User;
use chrono::{DateTime, Utc};

/// Describes a change to a user payment subscription toward the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botsubscriptionupdated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotSubscriptionUpdated {
    /// User that the subscription belongs to
    pub user: User,
    /// True, if the subscription is currently active
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_active: bool,
    /// Point in time when the subscription was last updated
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "u64"))]
    pub date: DateTime<Utc>,
    /// Expiration date of the subscription, if any
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<u64>"))]
    pub expiration_date: Option<DateTime<Utc>>,
}
