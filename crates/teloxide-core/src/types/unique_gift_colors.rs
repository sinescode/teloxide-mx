use serde::{Deserialize, Serialize};

/// This object contains information about the color scheme for a user's name
/// and messages, and link preview based on a unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the model of the unique gift
    pub model_custom_emoji_id: String,
    /// Custom emoji identifier of the symbol of the unique gift
    pub symbol_custom_emoji_id: String,
    /// Main color used in light themes, in RGB24 format
    pub light_theme_main_color: u32,
    /// Main color used in dark themes, in RGB24 format
    pub dark_theme_main_color: u32,
}
