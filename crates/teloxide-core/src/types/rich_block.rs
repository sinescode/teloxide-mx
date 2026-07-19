use serde::{Deserialize, Serialize};

use crate::types::{Animation, Audio, PhotoSize, RichText, Video, Voice};

/// A block element of a rich message.
///
/// [The official docs](https://core.telegram.org/bots/api#richblock).
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichBlock {
    Paragraph { rich_text: Option<RichText> },
    SectionHeading { rich_text: Option<RichText>, level: Option<u8> },
    Preformatted { text: Option<String>, language: Option<String> },
    Footer { rich_text: Option<RichText> },
    Divider {},
    MathematicalExpression { expression: Option<String> },
    Anchor { name: String },
    List { items: Option<Vec<RichBlockListItem>>, is_ordered: Option<bool> },
    BlockQuotation { blocks: Option<Vec<RichBlock>>, cite: Option<RichText> },
    PullQuotation { rich_text: Option<RichText>, cite: Option<RichText> },
    Collage { media: Option<Vec<RichBlockMedia>> },
    Slideshow { media: Option<Vec<RichBlockMedia>> },
    Table { cells: Option<Vec<Vec<RichBlockTableCell>>> },
    Details { summary: Option<RichText>, blocks: Option<Vec<RichBlock>> },
    Map { latitude: f64, longitude: f64, horizontal_accuracy: Option<f64> },
    Animation { animation: Option<Animation>, caption: Option<RichBlockCaption> },
    Audio { audio: Option<Audio>, caption: Option<RichBlockCaption> },
    Photo { photo: Option<Vec<PhotoSize>>, caption: Option<RichBlockCaption> },
    Video { video: Option<Video>, caption: Option<RichBlockCaption> },
    VoiceNote { voice: Option<Voice>, caption: Option<RichBlockCaption> },
    Thinking { rich_text: Option<RichText>, duration: Option<u32> },
}

/// An item in a rich block list.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockListItem {
    pub blocks: Option<Vec<RichBlock>>,
    pub rich_text: Option<RichText>,
}

/// A cell in a rich block table.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTableCell {
    pub blocks: Option<Vec<RichBlock>>,
    pub rich_text: Option<RichText>,
    pub colspan: Option<u8>,
    pub rowspan: Option<u8>,
    pub is_header: Option<bool>,
}

/// Caption for media in a rich block.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCaption {
    pub rich_text: Option<RichText>,
    pub show_caption_above_media: Option<bool>,
}

/// Media reference inside collage/slideshow rich blocks.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMedia {
    pub photo: Option<Vec<PhotoSize>>,
    pub video: Option<Video>,
    pub animation: Option<Animation>,
    pub caption: Option<RichBlockCaption>,
}
