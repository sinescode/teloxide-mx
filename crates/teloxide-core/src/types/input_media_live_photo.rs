use serde::Serialize;

use crate::types::{InputFile, MessageEntity, ParseMode};

/// Represents a live photo to be sent as part of a media group or paid media.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialivephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLivePhoto {
    /// Type of the result, must be live_photo
    #[serde(rename = "type")]
    pub type_field: String,
    /// File to send (video component)
    pub media: InputFile,
    /// Static photo corresponding to the live photo
    pub photo: Option<InputFile>,
    /// Caption
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
}

impl InputMediaLivePhoto {
    pub fn new(media: impl Into<InputFile>) -> Self {
        Self {
            type_field: "live_photo".into(),
            media: media.into(),
            photo: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
        }
    }
}
