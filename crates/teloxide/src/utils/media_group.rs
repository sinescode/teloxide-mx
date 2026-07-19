//! Media group builder — construct media groups (albums) conveniently.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::utils::media_group::MediaGroupBuilder;
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

use crate::types::{InputMedia, MessageEntity};

/// Maximum number of media in a group (Telegram limit).
pub const MAX_MEDIA_GROUP_SIZE: usize = 10;

/// A builder for constructing media groups (albums).
pub struct MediaGroupBuilder {
    media: Vec<InputMedia>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
}

impl MediaGroupBuilder {
    /// Creates a new empty `MediaGroupBuilder`.
    pub fn new() -> Self {
        Self { media: Vec::new(), caption: None, caption_entities: None }
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

    /// Adds a photo to the media group.
    pub fn photo(mut self, media: impl Into<InputMedia>) -> Self {
        self.media.push(media.into());
        self
    }

    /// Adds a video to the media group.
    pub fn video(mut self, media: impl Into<InputMedia>) -> Self {
        self.media.push(media.into());
        self
    }

    /// Adds a document to the media group.
    pub fn document(mut self, media: impl Into<InputMedia>) -> Self {
        self.media.push(media.into());
        self
    }

    /// Adds an audio file to the media group.
    pub fn audio(mut self, media: impl Into<InputMedia>) -> Self {
        self.media.push(media.into());
        self
    }

    /// Adds any [`InputMedia`] to the media group.
    pub fn add(mut self, media: InputMedia) -> Self {
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
    /// If a caption or caption_entities were set, they are applied to the first
    /// media item in the group (as required by the Telegram API).
    pub fn build(mut self) -> Vec<InputMedia> {
        if let Some(first) = self.media.first_mut() {
            use crate::types::InputMedia::*;

            match first {
                Photo(p) => {
                    if self.caption.is_some() {
                        p.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        p.caption_entities = self.caption_entities;
                    }
                }
                Video(v) => {
                    if self.caption.is_some() {
                        v.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        v.caption_entities = self.caption_entities;
                    }
                }
                Animation(a) => {
                    if self.caption.is_some() {
                        a.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        a.caption_entities = self.caption_entities;
                    }
                }
                Audio(a) => {
                    if self.caption.is_some() {
                        a.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        a.caption_entities = self.caption_entities;
                    }
                }
                Document(d) => {
                    if self.caption.is_some() {
                        d.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        d.caption_entities = self.caption_entities;
                    }
                }
                LivePhoto(lp) => {
                    if self.caption.is_some() {
                        lp.caption = self.caption;
                    }
                    if self.caption_entities.is_some() {
                        lp.caption_entities = self.caption_entities;
                    }
                }
                // Sticker, Location, Venue, Link don't have caption fields
                Sticker(_) | Location(_) | Venue(_) | Link(_) => {}
            }
        }
        self.media
    }
}

impl Default for MediaGroupBuilder {
    fn default() -> Self {
        Self::new()
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
}
