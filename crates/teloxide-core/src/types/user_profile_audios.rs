use serde::{Deserialize, Serialize};

use crate::types::Audio;

/// This object represents the list of profile audios of a user.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    /// Total number of profile audios available
    pub total_count: u32,
    /// Requested profile audios
    pub audios: Vec<Audio>,
}
