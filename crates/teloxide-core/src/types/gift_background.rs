use serde::{Deserialize, Serialize};

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB24 format
    pub center_color: u32,
    /// Edge color of the background in RGB24 format
    pub edge_color: u32,
    /// Text color in RGB24 format
    pub text_color: Option<u32>,
}
