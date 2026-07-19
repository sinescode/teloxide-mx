use serde::{Deserialize, Serialize};

use crate::types::User;

/// A rich text node.
///
/// [The official docs](https://core.telegram.org/bots/api#richtext).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    Plain { text: String },
    Bold { text: Box<RichText> },
    Italic { text: Box<RichText> },
    Underline { text: Box<RichText> },
    Strikethrough { text: Box<RichText> },
    Spoiler { text: Box<RichText> },
    Code { text: Box<RichText> },
    Subscript { text: Box<RichText> },
    Superscript { text: Box<RichText> },
    Marked { text: Box<RichText> },
    Url { text: Box<RichText>, url: String },
    EmailAddress { text: Box<RichText>, email: Option<String> },
    PhoneNumber { text: Box<RichText>, phone_number: Option<String> },
    BankCardNumber { text: Box<RichText> },
    Mention { text: Box<RichText> },
    Hashtag { text: Box<RichText> },
    Cashtag { text: Box<RichText> },
    BotCommand { text: Box<RichText> },
    TextMention { text: Box<RichText>, user: User },
    CustomEmoji { text: Box<RichText>, custom_emoji_id: String },
    DateTime { text: Box<RichText>, unix_time: Option<i64>, date_time_format: Option<String> },
    MathematicalExpression { text: Box<RichText>, expression: Option<String> },
    Anchor { name: String },
    AnchorLink { text: Box<RichText>, name: String },
    Reference { name: String },
    ReferenceLink { text: Box<RichText>, name: String },
    Concat { texts: Vec<RichText> },
}
