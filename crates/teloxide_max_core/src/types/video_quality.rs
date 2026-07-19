use serde::{Deserialize, Serialize};

/// This object describes a video quality option.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Video width
    pub width: u32,
    /// Video height
    pub height: u32,
    /// File size in bytes
    pub file_size: Option<u64>,
}
