use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, LivePhoto, Location, PhotoSize, Sticker, Venue, Video,
};

/// Media attached to a poll, quiz explanation, or poll option.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    /// Media is an animation
    pub animation: Option<Animation>,
    /// Media is an audio file
    pub audio: Option<Audio>,
    /// Media is a general file
    pub document: Option<Document>,
    /// Media is a live photo
    pub live_photo: Option<LivePhoto>,
    /// Media is a shared location
    pub location: Option<Location>,
    /// Media is a photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Media is a sticker
    pub sticker: Option<Sticker>,
    /// Media is a venue
    pub venue: Option<Venue>,
    /// Media is a video
    pub video: Option<Video>,
}
