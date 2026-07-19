use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{Chat, Sticker};

/// This object represent a list of gifts.
///
/// [The official docs](https://core.telegram.org/bots/api#gifts).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Vec<Gift>,
}

/// A unique identifier of the gift.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct GiftId(pub String);

/// This object represents a gift that can be sent by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#gift).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: GiftId,

    /// The sticker that represents the gift
    pub sticker: Sticker,

    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: u32,

    /// The number of Telegram Stars that must be paid to upgrade the gift to a
    /// unique one
    pub upgrade_star_count: Option<u32>,

    /// The total number of the gifts of this type that can be sent; for limited
    /// gifts only
    pub total_count: Option<u32>,

    /// The number of remaining gifts of this type that can be sent; for limited
    /// gifts only
    pub remaining_count: Option<u32>,

    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,

    /// The total number of gifts of this type that can be sent by the user; for
    /// limited gifts only. TBA 9.3+
    pub personal_total_count: Option<u32>,
    /// The number of remaining gifts of this type that can be sent by the user;
    /// for limited gifts only. TBA 9.3+
    pub personal_remaining_count: Option<u32>,
    /// True, if the gift is for Telegram Premium subscribers only. TBA 9.3+
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_premium: bool,
    /// True, if the gift has unique colors. TBA 9.3+
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_colors: bool,
    /// Number of unique gift variants. TBA 9.3+
    pub unique_gift_variant_count: Option<u32>,
    /// Background of the gift. TBA 9.3+
    pub gift_background: Option<crate::types::GiftBackground>,
}

impl Gift {
    pub fn is_limited(&self) -> bool {
        self.total_count.is_some()
    }

    /// Returns [`None`] if Gift isn't limited or a tuple where first element is
    /// [`Self::remaining_count`] and second is [`Self::total_count`]
    pub fn limited_count(&self) -> Option<(u32, u32)> {
        match (self.remaining_count, self.total_count) {
            (Some(remaining_count), Some(total_count)) => Some((remaining_count, total_count)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser() {
        let gift_id = S { gift_id: GiftId("id".to_owned()) };
        let json = r#"{"gift_id":"id"}"#;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct S {
            gift_id: GiftId,
        }

        assert_eq!(serde_json::to_string(&gift_id).unwrap(), json);
        assert_eq!(gift_id, serde_json::from_str(json).unwrap());
    }
}
