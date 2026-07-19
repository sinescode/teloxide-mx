use serde::Serialize;

use crate::types::{InputFile, ParseMode};

/// Describes a rich message to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputRichMessage {
    /// HTML or Markdown content of the rich message (legacy / simple form)
    pub content: Option<String>,
    /// Mode for parsing entities in the content
    pub parse_mode: Option<ParseMode>,
    /// List of rich blocks the message is composed of
    pub blocks: Option<Vec<InputRichBlock>>,
    /// List of media items attached to the message
    pub media: Option<Vec<InputRichMessageMedia>>,
}

/// A media item attached to a rich message to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputRichMessageMedia {
    /// Type of the media
    #[serde(rename = "type")]
    pub type_field: String,
    /// Media file
    pub media: InputFile,
    /// Caption
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
}

/// An input rich block element to be sent.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph {
        text: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    SectionHeading {
        text: Option<String>,
        parse_mode: Option<ParseMode>,
        level: Option<u8>,
    },
    Preformatted {
        text: Option<String>,
        language: Option<String>,
    },
    Footer {
        text: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Divider {},
    MathematicalExpression {
        expression: Option<String>,
    },
    Anchor {
        name: String,
    },
    List {
        items: Option<Vec<InputRichBlockListItem>>,
        is_ordered: Option<bool>,
    },
    BlockQuotation {
        blocks: Option<Vec<InputRichBlock>>,
    },
    PullQuotation {
        text: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Collage {
        media: Option<Vec<InputRichMessageMedia>>,
    },
    Slideshow {
        media: Option<Vec<InputRichMessageMedia>>,
    },
    Table {
        /// Flattened representation of table content as nested blocks
        rows: Option<Vec<Vec<InputRichBlock>>>,
    },
    Details {
        summary: Option<String>,
        blocks: Option<Vec<InputRichBlock>>,
    },
    Map {
        latitude: f64,
        longitude: f64,
    },
    Animation {
        media: InputFile,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Audio {
        media: InputFile,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Photo {
        media: InputFile,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Video {
        media: InputFile,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    VoiceNote {
        media: InputFile,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Thinking {
        text: Option<String>,
        duration: Option<u32>,
    },
}

/// An item in an input rich block list.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputRichBlockListItem {
    pub text: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub blocks: Option<Vec<InputRichBlock>>,
}
