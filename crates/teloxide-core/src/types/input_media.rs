use std::iter;

use serde::Serialize;

use crate::types::{InputFile, MessageEntity, ParseMode, Seconds};

/// This object represents the content of a media message to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputMedia {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(crate::types::InputMediaLivePhoto),
    Sticker(InputMediaSticker),
    Location(InputMediaLocation),
    Venue(InputMediaVenue),
    Link(InputMediaLink),
}

/// Represents a photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaPhoto {
    /// File to send.
    pub media: InputFile,

    /// Caption of the photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Pass `true` if the photo needs to be covered with a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaPhoto {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaPhoto::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents a video to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVideo {
    // File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Cover for the video in the message. Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended), pass an HTTP URL for
    /// Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name
    pub cover: Option<InputFile>,

    /// Start timestamp for the video in the message
    pub start_timestamp: Option<Seconds>,

    /// Caption of the video to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Video width.
    pub width: Option<u16>,

    /// Video height.
    pub height: Option<u16>,

    /// Video duration.
    pub duration: Option<u16>,

    /// Pass `true`, if the uploaded video is suitable for streaming.
    pub supports_streaming: Option<bool>,

    /// Pass `true` if the video needs to be covered with a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaVideo {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            cover: None,
            start_timestamp: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: false,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumbnail(mut self, val: InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn cover(mut self, val: InputFile) -> Self {
        self.cover = Some(val);
        self
    }

    pub fn start_timestamp(mut self, val: Seconds) -> Self {
        self.start_timestamp = Some(val);
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
        self
    }

    pub const fn width(mut self, val: u16) -> Self {
        self.width = Some(val);
        self
    }

    pub const fn height(mut self, val: u16) -> Self {
        self.height = Some(val);
        self
    }

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub const fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaVideo::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without
/// sound) to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaAnimation {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the animation to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Animation width.
    pub width: Option<u16>,

    /// Animation height.
    pub height: Option<u16>,

    /// Animation duration.
    pub duration: Option<u16>,

    /// Pass `true` if the animation needs to be covered with a spoiler
    /// animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaAnimation {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumbnail(mut self, val: InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
        self
    }

    pub const fn width(mut self, val: u16) -> Self {
        self.width = Some(val);
        self
    }

    pub const fn height(mut self, val: u16) -> Self {
        self.height = Some(val);
        self
    }

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaAnimation::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents an audio file to be treated as music to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaAudio {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the audio to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Duration of the audio in seconds.
    pub duration: Option<u16>,

    /// Performer of the audio.
    pub performer: Option<String>,

    /// Title of the audio.
    pub title: Option<String>,
}

impl InputMediaAudio {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            performer: None,
            title: None,
            duration: None,
            caption_entities: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumbnail(mut self, val: InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn performer<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }
}

/// Represents a general file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediadocument).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaDocument {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the document to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Disables automatic server-side content type detection for files uploaded
    /// using multipart/form-data. Always true, if the document is sent as part
    /// of an album.
    pub disable_content_type_detection: Option<bool>,
}

impl InputMediaDocument {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            disable_content_type_detection: None,
            caption_entities: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumbnail(mut self, val: InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }
}

/// Represents a sticker to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediasticker).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaSticker {
    /// File to send.
    pub media: InputFile,

    /// UTF-8 text of the emoji.
    pub emoji: Option<String>,
}

impl InputMediaSticker {
    pub const fn new(media: InputFile) -> Self {
        Self { media, emoji: None }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn emoji<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.emoji = Some(val.into());
        self
    }
}

/// Represents a location to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialocation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLocation {
    /// Latitude of the location.
    pub latitude: f64,

    /// Longitude of the location.
    pub longitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500.
    pub horizontal_accuracy: Option<f64>,
}

impl InputMediaLocation {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude, horizontal_accuracy: None }
    }

    pub const fn horizontal_accuracy(mut self, val: f64) -> Self {
        self.horizontal_accuracy = Some(val);
        self
    }
}

/// Represents a venue to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavenue).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVenue {
    /// Latitude of the venue.
    pub latitude: f64,

    /// Longitude of the venue.
    pub longitude: f64,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue.
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue.
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,

    /// Google Places type of the venue.
    pub google_place_type: Option<String>,
}

impl InputMediaVenue {
    pub fn new(
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    pub fn foursquare_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    pub fn foursquare_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_type = Some(val.into());
        self
    }

    pub fn google_place_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.google_place_id = Some(val.into());
        self
    }

    pub fn google_place_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.google_place_type = Some(val.into());
        self
    }
}

/// Represents a link to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialink).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLink {
    /// The URL of the link.
    pub url: String,
}

impl InputMediaLink {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

impl From<InputMedia> for InputFile {
    fn from(media: InputMedia) -> InputFile {
        match media {
            InputMedia::Photo(InputMediaPhoto { media, .. })
            | InputMedia::Document(InputMediaDocument { media, .. })
            | InputMedia::Audio(InputMediaAudio { media, .. })
            | InputMedia::Animation(InputMediaAnimation { media, .. })
            | InputMedia::Video(InputMediaVideo { media, .. })
            | InputMedia::LivePhoto(crate::types::InputMediaLivePhoto { media, .. })
            | InputMedia::Sticker(InputMediaSticker { media, .. }) => media,
            InputMedia::Location(_) | InputMedia::Venue(_) | InputMedia::Link(_) => {
                unreachable!("InputMedia::Location, Venue, and Link do not contain an InputFile")
            }
        }
    }
}

impl InputMedia {
    /// Returns an iterator of all files in this input media
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputMedia::*;

        let (media, thumbnail, photo) = match self {
            Photo(InputMediaPhoto { media, .. }) => (media, None, None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_ref(), None),
            LivePhoto(m) => (&m.media, None, m.photo.as_ref()),
            Sticker(InputMediaSticker { media, .. }) => (media, None, None),
            Location(_) | Venue(_) | Link(_) => unreachable!(),
        };

        iter::once(media).chain(thumbnail).chain(photo)
    }

    /// Returns an iterator of all files in this input media
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputMedia::*;

        let (media, thumbnail, photo) = match self {
            Photo(InputMediaPhoto { media, .. }) => (media, None, None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_mut(), None),
            LivePhoto(m) => (&mut m.media, None, m.photo.as_mut()),
            Sticker(InputMediaSticker { media, .. }) => (media, None, None),
            Location(_) | Venue(_) | Link(_) => unreachable!(),
        };

        iter::once(media).chain(thumbnail).chain(photo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_serialize() {
        let expected_json = r#"{"type":"photo","media":"123456"}"#;
        let photo = InputMedia::Photo(InputMediaPhoto {
            media: InputFile::file_id("123456".into()),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&photo).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn video_serialize() {
        let expected_json = r#"{"type":"video","media":"123456"}"#;
        let video = InputMedia::Video(InputMediaVideo {
            media: InputFile::file_id("123456".into()),
            thumbnail: None,
            cover: None,
            start_timestamp: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn animation_serialize() {
        let expected_json = r#"{"type":"animation","media":"123456"}"#;
        let video = InputMedia::Animation(InputMediaAnimation {
            media: InputFile::file_id("123456".into()),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn audio_serialize() {
        let expected_json = r#"{"type":"audio","media":"123456"}"#;
        let video = InputMedia::Audio(InputMediaAudio {
            media: InputFile::file_id("123456".into()),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            caption_entities: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn document_serialize() {
        let expected_json = r#"{"type":"document","media":"123456"}"#;
        let video = InputMedia::Document(InputMediaDocument {
            media: InputFile::file_id("123456".into()),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
