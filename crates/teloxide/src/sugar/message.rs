//! Message convenience methods — aiogram-style `message.answer()` / `message.reply()`.
//!
//! These extension traits add ergonomic shortcuts directly on [`Message`], so
//! you can write `msg.answer("Hello!")` instead of
//! `bot.send_message(msg.chat.id, "Hello!")`.

use crate::types::{ChatId, Message, MessageId, ReplyParameters};

/// Extension trait providing [`Message::answer`] and [`Message::reply`] sugar.
///
/// This requires a `Bot` instance to be available (via dependency injection
/// or manually). The methods return opaque request builders identical to the
/// ones returned by `Bot::send_message`, etc.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide::prelude::*;
/// # use teloxide::sugar::MessageExt;
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

    #[test]
    fn message_chat_id() {
        let msg = Message {
            id: MessageId(42),
            chat: Chat {
                id: ChatId(123),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(msg.chat_id(), ChatId(123));
        assert_eq!(msg.message_id(), MessageId(42));
    }
}
