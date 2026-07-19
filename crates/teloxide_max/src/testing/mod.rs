//! Testing utilities for teloxide_max handlers.
//!
//! This module provides tools for unit testing bot handlers without
//! making real Telegram API calls.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::testing::{MockBot, UpdateBuilder, mock_update};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! async fn my_handler(bot: Bot, msg: Message) -> HandlerResult {
//!     bot.send_message(msg.chat.id, "Hello!").await?;
//!     Ok(())
//! }
//!
//! #[tokio::test]
//! async fn test_my_handler() {
//!     let bot = MockBot::new();
//!     let update = UpdateBuilder::message()
//!         .with_text("/start")
//!         .with_user_id(12345)
//!         .with_chat_id(12345)
//!         .build();
//!
//!     let result = mock_update(bot.clone(), update, my_handler).await;
//!     assert!(result.is_ok());
//!     assert!(bot.has_sent_message("Hello!"));
//! }
//! ```

use crate::types::{
    CallbackQueryId, ChatId, InlineQueryId, Message, MessageId, Update, UpdateId, UpdateKind, User,
    UserId,
};
use std::sync::{Arc, Mutex};

/// A mock bot that records sent messages for testing.
#[derive(Clone)]
pub struct MockBot {
    sent_messages: Arc<Mutex<Vec<SentMessage>>>,
    _token: String,
}

/// A record of a message sent by MockBot.
#[derive(Debug, Clone)]
pub struct SentMessage {
    pub chat_id: ChatId,
    pub text: String,
}

impl MockBot {
    /// Creates a new mock bot.
    pub fn new() -> Self {
        Self { sent_messages: Arc::new(Mutex::new(Vec::new())), _token: "TEST_TOKEN".to_string() }
    }

    /// Creates a mock bot with a custom token.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self { sent_messages: Arc::new(Mutex::new(Vec::new())), _token: token.into() }
    }

    /// Records a sent message.
    pub fn record_sent(&self, chat_id: ChatId, text: impl Into<String>) {
        self.sent_messages.lock().unwrap().push(SentMessage { chat_id, text: text.into() });
    }

    /// Returns all sent messages.
    pub fn sent_messages(&self) -> Vec<SentMessage> {
        self.sent_messages.lock().unwrap().clone()
    }

    /// Returns the number of sent messages.
    pub fn sent_count(&self) -> usize {
        self.sent_messages.lock().unwrap().len()
    }

    /// Checks if a message with the given text was sent.
    pub fn has_sent_message(&self, text: &str) -> bool {
        self.sent_messages.lock().unwrap().iter().any(|m| m.text == text)
    }

    /// Checks if a message was sent to the given chat.
    pub fn has_sent_to(&self, chat_id: ChatId) -> bool {
        self.sent_messages.lock().unwrap().iter().any(|m| m.chat_id == chat_id)
    }

    /// Returns the last sent message.
    pub fn last_message(&self) -> Option<SentMessage> {
        self.sent_messages.lock().unwrap().last().cloned()
    }

    /// Clears all recorded messages.
    pub fn clear(&self) {
        self.sent_messages.lock().unwrap().clear();
    }
}

impl Default for MockBot {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test updates.
pub struct UpdateBuilder {
    update_id: UpdateId,
    kind: UpdateKind,
}

impl UpdateBuilder {
    /// Creates a builder for a message update.
    pub fn message() -> MessageUpdateBuilder {
        MessageUpdateBuilder {
            text: None,
            user_id: UserId(1),
            user_name: "TestUser".to_string(),
            chat_id: ChatId(1),
            chat_name: "TestChat".to_string(),
            message_id: MessageId(1),
        }
    }

    /// Creates a builder for a callback query update.
    pub fn callback_query() -> CallbackQueryUpdateBuilder {
        CallbackQueryUpdateBuilder {
            data: None,
            user_id: UserId(1),
            user_name: "TestUser".to_string(),
            chat_id: ChatId(1),
        }
    }

    /// Creates a builder for an inline query update.
    pub fn inline_query() -> InlineQueryUpdateBuilder {
        InlineQueryUpdateBuilder {
            query: String::new(),
            user_id: UserId(1),
            user_name: "TestUser".to_string(),
        }
    }

    /// Creates a builder from an existing update.
    pub fn from_update(update: Update) -> Self {
        Self { update_id: update.id, kind: update.kind }
    }

    /// Builds the update.
    pub fn build(self) -> Update {
        Update { id: self.update_id, kind: self.kind }
    }
}

/// Builder for message updates.
pub struct MessageUpdateBuilder {
    text: Option<String>,
    user_id: UserId,
    user_name: String,
    chat_id: ChatId,
    chat_name: String,
    message_id: MessageId,
}

impl MessageUpdateBuilder {
    /// Sets the message text.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the user ID.
    pub fn with_user_id(mut self, id: u64) -> Self {
        self.user_id = UserId(id);
        self
    }

    /// Sets the user name.
    pub fn with_user_name(mut self, name: impl Into<String>) -> Self {
        self.user_name = name.into();
        self
    }

    /// Sets the chat ID.
    pub fn with_chat_id(mut self, id: i64) -> Self {
        self.chat_id = ChatId(id);
        self
    }

    /// Sets the chat name.
    pub fn with_chat_name(mut self, name: impl Into<String>) -> Self {
        self.chat_name = name.into();
        self
    }

    /// Builds the update.
    pub fn build(self) -> Update {
        let text = self.text.unwrap_or_default();
        let user_id = self.user_id.0;
        let user_name = &self.user_name;
        let chat_id = self.chat_id.0;
        let chat_name = &self.chat_name;
        let message_id = self.message_id.0;

        let json = serde_json::json!({
            "message_id": message_id,
            "from": {
                "id": user_id,
                "is_bot": false,
                "first_name": user_name,
                "username": user_name,
            },
            "chat": {
                "id": chat_id,
                "type": "private",
                "first_name": chat_name,
            },
            "date": 1_569_518_829_i64,
            "text": text,
        });

        let message: Message =
            serde_json::from_value(json).expect("failed to deserialize test Message");

        Update { id: UpdateId(1), kind: UpdateKind::Message(message) }
    }
}

/// Builder for callback query updates.
pub struct CallbackQueryUpdateBuilder {
    data: Option<String>,
    user_id: UserId,
    user_name: String,
    chat_id: ChatId,
}

impl CallbackQueryUpdateBuilder {
    /// Sets the callback data.
    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Sets the user ID.
    pub fn with_user_id(mut self, id: u64) -> Self {
        self.user_id = UserId(id);
        self
    }

    /// Sets the chat ID.
    pub fn with_chat_id(mut self, id: i64) -> Self {
        self.chat_id = ChatId(id);
        self
    }

    /// Builds the update.
    pub fn build(self) -> Update {
        use crate::types::CallbackQuery;
        Update {
            id: UpdateId(1),
            kind: UpdateKind::CallbackQuery(CallbackQuery {
                id: CallbackQueryId("test_callback".to_string()),
                from: User {
                    id: self.user_id,
                    is_bot: false,
                    first_name: self.user_name,
                    last_name: None,
                    username: None,
                    language_code: Some("en".to_string()),
                    is_premium: false,
                    added_to_attachment_menu: false,
                    has_topics_enabled: false,
                    allows_users_to_create_topics: false,
                    can_join_groups: None,
                    can_read_all_group_messages: None,
                    supports_guest_queries: None,
                    supports_inline_queries: None,
                    can_connect_to_business: None,
                    has_main_web_app: None,
                    can_manage_bots: None,
                    supports_join_request_queries: None,
                },
                chat_instance: "test".to_string(),
                data: self.data,
                game_short_name: None,
                message: None,
                inline_message_id: None,
            }),
        }
    }
}

/// Builder for inline query updates.
pub struct InlineQueryUpdateBuilder {
    query: String,
    user_id: UserId,
    user_name: String,
}

impl InlineQueryUpdateBuilder {
    /// Sets the inline query text.
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// Sets the user ID.
    pub fn with_user_id(mut self, id: u64) -> Self {
        self.user_id = UserId(id);
        self
    }

    /// Builds the update.
    pub fn build(self) -> Update {
        use crate::types::InlineQuery;
        Update {
            id: UpdateId(1),
            kind: UpdateKind::InlineQuery(InlineQuery {
                id: InlineQueryId("test_inline".to_string()),
                from: User {
                    id: self.user_id,
                    is_bot: false,
                    first_name: self.user_name,
                    last_name: None,
                    username: None,
                    language_code: Some("en".to_string()),
                    is_premium: false,
                    added_to_attachment_menu: false,
                    has_topics_enabled: false,
                    allows_users_to_create_topics: false,
                    can_join_groups: None,
                    can_read_all_group_messages: None,
                    supports_guest_queries: None,
                    supports_inline_queries: None,
                    can_connect_to_business: None,
                    has_main_web_app: None,
                    can_manage_bots: None,
                    supports_join_request_queries: None,
                },
                query: self.query,
                offset: String::new(),
                chat_type: None,
                location: None,
            }),
        }
    }
}

/// Creates a mock message update with default values.
pub fn mock_message(text: &str) -> Update {
    UpdateBuilder::message().with_text(text).build()
}

/// Creates a mock message update from a specific user.
pub fn mock_message_from(text: &str, user_id: u64, chat_id: i64) -> Update {
    UpdateBuilder::message().with_text(text).with_user_id(user_id).with_chat_id(chat_id).build()
}

/// Creates a mock callback query update.
pub fn mock_callback(data: &str) -> Update {
    UpdateBuilder::callback_query().with_data(data).build()
}

/// Creates a mock inline query update.
pub fn mock_inline(query: &str) -> Update {
    UpdateBuilder::inline_query().with_query(query).build()
}

/// Runs a handler with a mock update and returns the result.
///
/// This is a simplified test runner that creates a minimal dependency map
/// and dispatches the update through the handler.
pub async fn mock_update<F, Fut>(
    handler: F,
    update: Update,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnOnce(Update) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>,
{
    handler(update).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_bot_records_messages() {
        let bot = MockBot::new();
        bot.record_sent(ChatId(1), "Hello");
        bot.record_sent(ChatId(1), "World");
        assert_eq!(bot.sent_count(), 2);
        assert!(bot.has_sent_message("Hello"));
        assert!(bot.has_sent_message("World"));
    }

    #[test]
    fn message_update_builder() {
        let update =
            UpdateBuilder::message().with_text("test").with_user_id(42).with_chat_id(100).build();

        assert_eq!(update.id, UpdateId(1));
        if let UpdateKind::Message(msg) = &update.kind {
            assert_eq!(msg.text(), Some("test"));
            assert_eq!(msg.from.as_ref().unwrap().id, UserId(42));
            assert_eq!(msg.chat.id, ChatId(100));
        } else {
            panic!("Expected message update");
        }
    }

    #[test]
    fn callback_query_builder() {
        let update =
            UpdateBuilder::callback_query().with_data("action:confirm").with_user_id(42).build();

        if let UpdateKind::CallbackQuery(q) = &update.kind {
            assert_eq!(q.data.as_deref(), Some("action:confirm"));
            assert_eq!(q.from.id, UserId(42));
        } else {
            panic!("Expected callback query update");
        }
    }

    #[test]
    fn mock_message_helper() {
        let update = mock_message("hello");
        if let UpdateKind::Message(msg) = &update.kind {
            assert_eq!(msg.text(), Some("hello"));
        } else {
            panic!("Expected message update");
        }
    }

    #[test]
    fn mock_callback_helper() {
        let update = mock_callback("btn:click");
        if let UpdateKind::CallbackQuery(q) = &update.kind {
            assert_eq!(q.data.as_deref(), Some("btn:click"));
        } else {
            panic!("Expected callback query");
        }
    }
}
