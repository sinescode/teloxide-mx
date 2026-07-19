//! Message convenience methods — aiogram-style `message.answer()` /
//! `message.reply()`.
//!
//! These extension traits add ergonomic shortcuts directly on [`Message`], so
//! you can write `msg.answer("Hello!")` instead of
//! `bot.send_message(msg.chat.id, "Hello!")`.

use crate::types::{ChatId, InputFile, InputMedia, Message, MessageId, Recipient, ReplyParameters};
use teloxide_max_core::payloads::{
    SendAnimationSetters, SendAudioSetters, SendContactSetters, SendDiceSetters,
    SendDocumentSetters, SendMessageSetters, SendPhotoSetters, SendStickerSetters,
    SendVenueSetters, SendVideoSetters, SendVoiceSetters, SendLocationSetters,
    UnpinChatMessageSetters,
};

/// Extension trait providing [`Message::answer`] and [`Message::reply`] sugar.
///
/// This requires a `Bot` instance to be available (via dependency injection
/// or manually). The methods return opaque request builders identical to the
/// ones returned by `Bot::send_message`, etc.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::prelude::*;
/// # use teloxide_max::sugar::MessageExt;
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
/// async fn handle(bot: Bot, msg: Message) -> HandlerResult {
///     msg.answer(&bot, "Hello!").await?;
///     msg.reply(&bot, "Replying to you").await?;
///     Ok(())
/// }
/// ```
pub trait MessageExt {
    /// Sends a message to the same chat as this message (like aiogram's
    /// `message.answer()`).
    fn answer<B>(&self, bot: &B, text: impl Into<String>) -> B::SendMessage
    where
        B: crate::requests::Requester;

    /// Replies to this specific message (like aiogram's `message.reply()`).
    fn reply<B>(&self, bot: &B, text: impl Into<String>) -> B::SendMessage
    where
        B: crate::requests::Requester;

    /// Sends a photo to the same chat, replying to this message.
    fn answer_photo<B>(&self, bot: &B, photo: InputFile) -> B::SendPhoto
    where
        B: crate::requests::Requester;

    /// Sends a document to the same chat, replying to this message.
    fn answer_document<B>(&self, bot: &B, document: InputFile) -> B::SendDocument
    where
        B: crate::requests::Requester;

    /// Sends a video to the same chat, replying to this message.
    fn answer_video<B>(&self, bot: &B, video: InputFile) -> B::SendVideo
    where
        B: crate::requests::Requester;

    /// Sends audio to the same chat, replying to this message.
    fn answer_audio<B>(&self, bot: &B, audio: InputFile) -> B::SendAudio
    where
        B: crate::requests::Requester;

    /// Sends a voice message to the same chat, replying to this message.
    fn answer_voice<B>(&self, bot: &B, voice: InputFile) -> B::SendVoice
    where
        B: crate::requests::Requester;

    /// Sends an animation to the same chat, replying to this message.
    fn answer_animation<B>(&self, bot: &B, animation: InputFile) -> B::SendAnimation
    where
        B: crate::requests::Requester;

    /// Sends a sticker to the same chat, replying to this message.
    fn answer_sticker<B>(&self, bot: &B, sticker: InputFile) -> B::SendSticker
    where
        B: crate::requests::Requester;

    /// Sends dice to the same chat, replying to this message.
    fn answer_dice<B>(&self, bot: &B) -> B::SendDice
    where
        B: crate::requests::Requester;

    /// Sends a venue to the same chat, replying to this message.
    fn answer_venue<B>(
        &self,
        bot: &B,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> B::SendVenue
    where
        B: crate::requests::Requester;

    /// Sends a location to the same chat, replying to this message.
    fn answer_location<B>(&self, bot: &B, latitude: f64, longitude: f64) -> B::SendLocation
    where
        B: crate::requests::Requester;

    /// Sends a contact to the same chat, replying to this message.
    fn answer_contact<B>(
        &self,
        bot: &B,
        phone_number: String,
        first_name: String,
    ) -> B::SendContact
    where
        B: crate::requests::Requester;

    /// Forwards this message to another chat.
    fn forward<B>(&self, bot: &B, chat_id: Recipient) -> B::ForwardMessage
    where
        B: crate::requests::Requester;

    /// Edits the text of this message.
    fn edit_text<B>(&self, bot: &B, text: String) -> B::EditMessageText
    where
        B: crate::requests::Requester;

    /// Edits the media of this message.
    fn edit_media<B>(&self, bot: &B, media: InputMedia) -> B::EditMessageMedia
    where
        B: crate::requests::Requester;

    /// Edits the reply markup of this message.
    fn edit_reply_markup<B>(&self, bot: &B) -> B::EditMessageReplyMarkup
    where
        B: crate::requests::Requester;

    /// Deletes this message.
    fn delete<B>(&self, bot: &B) -> B::DeleteMessage
    where
        B: crate::requests::Requester;

    /// Pins this message in its chat.
    fn pin<B>(&self, bot: &B) -> B::PinChatMessage
    where
        B: crate::requests::Requester;

    /// Unpins this message from its chat.
    fn unpin<B>(&self, bot: &B) -> B::UnpinChatMessage
    where
        B: crate::requests::Requester;

    /// Returns the chat ID of this message.
    fn chat_id(&self) -> ChatId;

    /// Returns the message ID of this message.
    fn message_id(&self) -> MessageId;
}

impl MessageExt for Message {
    fn answer<B>(&self, bot: &B, text: impl Into<String>) -> B::SendMessage
    where
        B: crate::requests::Requester,
    {
        bot.send_message(self.chat.id, text)
    }

    fn reply<B>(&self, bot: &B, text: impl Into<String>) -> B::SendMessage
    where
        B: crate::requests::Requester,
    {
        bot.send_message(self.chat.id, text).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_photo<B>(&self, bot: &B, photo: InputFile) -> B::SendPhoto
    where
        B: crate::requests::Requester,
    {
        bot.send_photo(self.chat.id, photo).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_document<B>(&self, bot: &B, document: InputFile) -> B::SendDocument
    where
        B: crate::requests::Requester,
    {
        bot.send_document(self.chat.id, document).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_video<B>(&self, bot: &B, video: InputFile) -> B::SendVideo
    where
        B: crate::requests::Requester,
    {
        bot.send_video(self.chat.id, video).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_audio<B>(&self, bot: &B, audio: InputFile) -> B::SendAudio
    where
        B: crate::requests::Requester,
    {
        bot.send_audio(self.chat.id, audio).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_voice<B>(&self, bot: &B, voice: InputFile) -> B::SendVoice
    where
        B: crate::requests::Requester,
    {
        bot.send_voice(self.chat.id, voice).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_animation<B>(&self, bot: &B, animation: InputFile) -> B::SendAnimation
    where
        B: crate::requests::Requester,
    {
        bot.send_animation(self.chat.id, animation)
            .reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_sticker<B>(&self, bot: &B, sticker: InputFile) -> B::SendSticker
    where
        B: crate::requests::Requester,
    {
        bot.send_sticker(self.chat.id, sticker)
            .reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_dice<B>(&self, bot: &B) -> B::SendDice
    where
        B: crate::requests::Requester,
    {
        bot.send_dice(self.chat.id).reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_venue<B>(
        &self,
        bot: &B,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> B::SendVenue
    where
        B: crate::requests::Requester,
    {
        bot.send_venue(self.chat.id, latitude, longitude, title, address)
            .reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_location<B>(&self, bot: &B, latitude: f64, longitude: f64) -> B::SendLocation
    where
        B: crate::requests::Requester,
    {
        bot.send_location(self.chat.id, latitude, longitude)
            .reply_parameters(ReplyParameters::new(self.id))
    }

    fn answer_contact<B>(
        &self,
        bot: &B,
        phone_number: String,
        first_name: String,
    ) -> B::SendContact
    where
        B: crate::requests::Requester,
    {
        bot.send_contact(self.chat.id, phone_number, first_name)
            .reply_parameters(ReplyParameters::new(self.id))
    }

    fn forward<B>(&self, bot: &B, chat_id: Recipient) -> B::ForwardMessage
    where
        B: crate::requests::Requester,
    {
        bot.forward_message(chat_id, self.chat.id, self.id)
    }

    fn edit_text<B>(&self, bot: &B, text: String) -> B::EditMessageText
    where
        B: crate::requests::Requester,
    {
        bot.edit_message_text(self.chat.id, self.id, text)
    }

    fn edit_media<B>(&self, bot: &B, media: InputMedia) -> B::EditMessageMedia
    where
        B: crate::requests::Requester,
    {
        bot.edit_message_media(self.chat.id, self.id, media)
    }

    fn edit_reply_markup<B>(&self, bot: &B) -> B::EditMessageReplyMarkup
    where
        B: crate::requests::Requester,
    {
        bot.edit_message_reply_markup(self.chat.id, self.id)
    }

    fn delete<B>(&self, bot: &B) -> B::DeleteMessage
    where
        B: crate::requests::Requester,
    {
        bot.delete_message(self.chat.id, self.id)
    }

    fn pin<B>(&self, bot: &B) -> B::PinChatMessage
    where
        B: crate::requests::Requester,
    {
        bot.pin_chat_message(self.chat.id, self.id)
    }

    fn unpin<B>(&self, bot: &B) -> B::UnpinChatMessage
    where
        B: crate::requests::Requester,
    {
        bot.unpin_chat_message(self.chat.id).message_id(self.id)
    }

    fn chat_id(&self) -> ChatId {
        self.chat.id
    }

    fn message_id(&self) -> MessageId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_message(chat_id: i64, message_id: i32) -> Message {
        let json = serde_json::json!({
            "message_id": message_id,
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "Test",
            },
            "chat": {
                "id": chat_id,
                "type": "private",
                "first_name": "Test",
            },
            "date": 1_569_518_829_i64,
        });
        serde_json::from_value(json).expect("failed to deserialize test Message")
    }

    #[test]
    fn message_chat_id() {
        let msg = make_message(123, 42);
        assert_eq!(msg.chat_id(), ChatId(123));
        assert_eq!(msg.message_id(), MessageId(42));
    }
}
