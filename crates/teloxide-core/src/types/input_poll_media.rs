use serde::Serialize;

use crate::types::InputFile;

/// Input media to attach to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollMedia {
    Photo { media: InputFile },
    Video { media: InputFile },
    Animation { media: InputFile },
    Audio { media: InputFile },
    Document { media: InputFile },
    Sticker { media: InputFile },
    Location { latitude: f64, longitude: f64 },
    Venue {
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    },
}
