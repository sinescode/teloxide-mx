use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star
/// spendings.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// Current level of the user
    pub level: i32,
    /// Numerical value of the user's rating
    pub rating: i32,
    /// The rating value required to get the current level
    pub current_level_rating: i32,
    /// The rating value required to get to the next level; omitted if the
    /// maximum level was reached
    pub next_level_rating: Option<i32>,
}
