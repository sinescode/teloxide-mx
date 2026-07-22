//! Media group builder — construct media groups (albums) conveniently.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::utils::media_group::MediaGroupBuilder;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! async fn send_album(bot: &Bot, chat_id: ChatId) -> HandlerResult {
//!     let media = MediaGroupBuilder::new()
//!         .photo("https://example.com/photo1.jpg")
//!         .photo("https://example.com/photo2.jpg")
//!         .caption("Check these out!")
//!         .build();
//!     bot.send_media_group(chat_id, media).await?;
//!     Ok(())
//! }
//! ```

use crate::types::{
    FileId, InputFile, InputMedia, InputMediaAudio, InputMediaDocument, InputMediaPhoto,
    InputMediaVideo, MessageEntity, ParseMode,
};

/// Maximum number of media in a group (Telegram limit).
pub const MAX_MEDIA_GROUP_SIZE: usize = 10;

/// A builder for constructing media groups (albums).
#[derive(Clone, Debug, Default)]
pub struct MediaGroupBuilder {
    media: Vec<InputMedia>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    parse_mode: Option<ParseMode>,
}

impl MediaGroupBuilder {
    /// Creates a new empty `MediaGroupBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a shared caption for all media in the group.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets caption entities for the shared caption.
    pub fn caption_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(entities);
        self
    }

    /// Sets parse mode for the shared caption.
    pub fn parse_mode(mut self, mode: ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Adds a photo identified by file id or URL string.
    pub fn photo(mut self, media: impl AsRef<str>) -> Self {
        let file = file_from_str(media.as_ref());
        self.media.push(InputMedia::Photo(InputMediaPhoto::new(file)));
        self
    }

    /// Adds a video identified by file id or URL string.
    pub fn video(mut self, media: impl AsRef<str>) -> Self {
        let file = file_from_str(media.as_ref());
        self.media.push(InputMedia::Video(InputMediaVideo::new(file)));
        self
    }

    /// Adds a document identified by file id or URL string.
    pub fn document(mut self, media: impl AsRef<str>) -> Self {
        let file = file_from_str(media.as_ref());
        self.media.push(InputMedia::Document(InputMediaDocument::new(file)));
        self
    }

    /// Adds an audio file identified by file id or URL string.
    pub fn audio(mut self, media: impl AsRef<str>) -> Self {
        let file = file_from_str(media.as_ref());
        self.media.push(InputMedia::Audio(InputMediaAudio::new(file)));
        self
    }

    /// Adds any [`InputMedia`] to the media group.
    pub fn add_media(mut self, media: InputMedia) -> Self {
        self.media.push(media);
        self
    }

    /// Returns the number of media items.
    pub fn len(&self) -> usize {
        self.media.len()
    }

    /// Returns `true` if the media group is empty.
    pub fn is_empty(&self) -> bool {
        self.media.is_empty()
    }

    /// Builds the final list of [`InputMedia`].
    ///
    /// If a caption, parse_mode, or caption_entities were set, they are applied
    /// to the first media item in the group (as required by the Telegram API).
    pub fn build(mut self) -> Vec<InputMedia> {
        if let Some(first) = self.media.first_mut() {
            apply_caption(first, self.caption, self.caption_entities, self.parse_mode);
        }
        self.media
    }
}

fn file_from_str(s: &str) -> InputFile {
    if s.starts_with("http://") || s.starts_with("https://") {
        InputFile::url(s.parse().expect("valid URL"))
    } else {
        InputFile::file_id(FileId(s.to_string()))
    }
}

fn apply_caption(
    media: &mut InputMedia,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    parse_mode: Option<ParseMode>,
) {
    use InputMedia::*;
    match media {
        Photo(p) => {
            if caption.is_some() {
                p.caption = caption;
            }
            if caption_entities.is_some() {
                p.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                p.parse_mode = parse_mode;
            }
        }
        Video(v) => {
            if caption.is_some() {
                v.caption = caption;
            }
            if caption_entities.is_some() {
                v.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                v.parse_mode = parse_mode;
            }
        }
        Animation(a) => {
            if caption.is_some() {
                a.caption = caption;
            }
            if caption_entities.is_some() {
                a.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                a.parse_mode = parse_mode;
            }
        }
        Audio(a) => {
            if caption.is_some() {
                a.caption = caption;
            }
            if caption_entities.is_some() {
                a.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                a.parse_mode = parse_mode;
            }
        }
        Document(d) => {
            if caption.is_some() {
                d.caption = caption;
            }
            if caption_entities.is_some() {
                d.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                d.parse_mode = parse_mode;
            }
        }
        LivePhoto(lp) => {
            if caption.is_some() {
                lp.caption = caption;
            }
            if caption_entities.is_some() {
                lp.caption_entities = caption_entities;
            }
            if parse_mode.is_some() {
                lp.parse_mode = parse_mode;
            }
        }
        Sticker(_) | Location(_) | Venue(_) | Link(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_builder() {
        let builder = MediaGroupBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.len(), 0);
    }

    #[test]
    fn photo_from_file_id() {
        let media = MediaGroupBuilder::new().photo("AgACAgIAAxkBAAI").build();
        assert_eq!(media.len(), 1);
        assert!(matches!(media[0], InputMedia::Photo(_)));
    }
}
