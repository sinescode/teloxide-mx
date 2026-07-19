use serde::Serialize;

use crate::types::InputRichMessage;

/// Represents the content of an inline query result as a rich message.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagecontent).
#[derive(Clone, Debug, Serialize)]
pub struct InputRichMessageContent {
    pub rich_message: InputRichMessage,
}
