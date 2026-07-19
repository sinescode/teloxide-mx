use serde::{Deserialize, Serialize};

use crate::types::{FileId, FileUniqueId, PhotoSize};

/// This object represents a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Identifier for the video file which can be used to download or reuse the
    /// file
    pub file_id: FileId,
    /// Unique identifier for the video file
    pub file_unique_id: FileUniqueId,
    /// Video width as defined by the sender
    pub width: u32,
    /// Video height as defined by the sender
    pub height: u32,
    /// Duration of the video in seconds as defined by the sender
    pub duration: u32,
    /// Available sizes of the corresponding static photo
    pub photo: Option<Vec<PhotoSize>>,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes
    pub file_size: Option<u64>,
}
