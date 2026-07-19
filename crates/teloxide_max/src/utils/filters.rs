//! MagicFilter-like DSL for ergonomic filtering.
//!
//! This module provides a builder-pattern filter system similar to aiogram's
//! `MagicFilter`. Instead of a runtime magic filter, this uses Rust's type
//! system to provide compile-time checked, ergonomic filtering.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::{prelude::*, utils::filters::Filter};
//!
//! // Build complex filters with the builder pattern
//! let filter = Filter::message().text().startswith("hello").from_user(12345);
//!
//! // Or use the shorthand macros
//! let filter = f!(text.starts_with("hi") & from_user.id == 123);
//! ```

use crate::types::{ChatId, ChatKind, Message, Update, UpdateKind, UserId};

type MessagePredicate = Box<dyn Fn(&Message) -> bool + Send + Sync>;

/// A filter builder for ergonomic message matching.
pub struct FilterBuilder {
    conditions: Vec<MessagePredicate>,
}

impl FilterBuilder {
    /// Creates a new filter builder.
    pub fn new() -> Self {
        Self { conditions: Vec::new() }
    }

    /// Filters messages that have text.
    pub fn text(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.text().is_some()));
        self
    }

    /// Filters messages that start with the given prefix.
    pub fn startswith(mut self, prefix: &'static str) -> Self {
        self.conditions
            .push(Box::new(move |msg| msg.text().is_some_and(|t| t.starts_with(prefix))));
        self
    }

    /// Filters messages that end with the given suffix.
    pub fn endswith(mut self, suffix: &'static str) -> Self {
        self.conditions.push(Box::new(move |msg| msg.text().is_some_and(|t| t.ends_with(suffix))));
        self
    }

    /// Filters messages that contain the given substring.
    pub fn contains(mut self, substring: &'static str) -> Self {
        self.conditions
            .push(Box::new(move |msg| msg.text().is_some_and(|t| t.contains(substring))));
        self
    }

    /// Filters messages from a specific user.
    pub fn from_user(mut self, user_id: UserId) -> Self {
        self.conditions
            .push(Box::new(move |msg| msg.from.as_ref().is_some_and(|u| u.id == user_id)));
        self
    }

    /// Filters messages in a specific chat.
    pub fn in_chat(mut self, chat_id: ChatId) -> Self {
        self.conditions.push(Box::new(move |msg| msg.chat.id == chat_id));
        self
    }

    /// Filters messages with photos.
    pub fn has_photo(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.photo().is_some()));
        self
    }

    /// Filters messages with documents.
    pub fn has_document(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.document().is_some()));
        self
    }

    /// Filters messages with audio.
    pub fn has_audio(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.audio().is_some()));
        self
    }

    /// Filters messages with video.
    pub fn has_video(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.video().is_some()));
        self
    }

    /// Filters messages with voice.
    pub fn has_voice(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.voice().is_some()));
        self
    }

    /// Filters messages with animation (GIF).
    pub fn has_animation(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.animation().is_some()));
        self
    }

    /// Filters messages with stickers.
    pub fn has_sticker(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.sticker().is_some()));
        self
    }

    /// Filters messages with contacts.
    pub fn has_contact(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.contact().is_some()));
        self
    }

    /// Filters messages with location.
    pub fn has_location(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.location().is_some()));
        self
    }

    /// Filters messages with venue.
    pub fn has_venue(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.venue().is_some()));
        self
    }

    /// Filters messages with dice.
    pub fn has_dice(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.dice().is_some()));
        self
    }

    /// Filters messages with polls.
    pub fn has_poll(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.poll().is_some()));
        self
    }

    /// Filters messages that are commands (start with /).
    pub fn is_command(mut self) -> Self {
        self.conditions.push(Box::new(|msg| msg.text().is_some_and(|t| t.starts_with('/'))));
        self
    }

    /// Filters messages that are private (not in groups).
    pub fn is_private(mut self) -> Self {
        self.conditions.push(Box::new(|msg| matches!(msg.chat.kind, ChatKind::Private(_))));
        self
    }

    /// Filters messages that are in groups.
    pub fn is_group(mut self) -> Self {
        self.conditions.push(Box::new(|msg| {
            matches!(
                &msg.chat.kind,
                ChatKind::Public(p) if matches!(p.kind, crate::types::PublicChatKind::Group | crate::types::PublicChatKind::Supergroup(_))
            )
        }));
        self
    }

    /// Filters messages in channels.
    pub fn is_channel(mut self) -> Self {
        self.conditions.push(Box::new(|msg| {
            matches!(
                &msg.chat.kind,
                ChatKind::Public(p) if matches!(p.kind, crate::types::PublicChatKind::Channel(_))
            )
        }));
        self
    }

    /// Adds a custom condition.
    pub fn custom(mut self, f: impl Fn(&Message) -> bool + Send + Sync + 'static) -> Self {
        self.conditions.push(Box::new(f));
        self
    }

    /// Adds an AND condition (both filters must match).
    pub fn and(mut self, other: FilterBuilder) -> Self {
        self.conditions.extend(other.conditions);
        self
    }

    /// Builds the filter into a predicate function.
    pub fn build(self) -> impl Fn(&Message) -> bool {
        move |msg| self.conditions.iter().all(|c| c(msg))
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Shortcut for creating a filter builder.
pub fn message_filter() -> FilterBuilder {
    FilterBuilder::new()
}

type UpdatePredicate = Box<dyn Fn(&Update) -> bool + Send + Sync>;

/// A filter for Update types.
pub struct UpdateFilter {
    conditions: Vec<UpdatePredicate>,
}

impl UpdateFilter {
    pub fn new() -> Self {
        Self { conditions: Vec::new() }
    }

    /// Filters updates that are messages.
    pub fn is_message(mut self) -> Self {
        self.conditions.push(Box::new(|u| matches!(u.kind, UpdateKind::Message(_))));
        self
    }

    /// Filters updates that are callback queries.
    pub fn is_callback_query(mut self) -> Self {
        self.conditions.push(Box::new(|u| matches!(u.kind, UpdateKind::CallbackQuery(_))));
        self
    }

    /// Filters updates that are inline queries.
    pub fn is_inline_query(mut self) -> Self {
        self.conditions.push(Box::new(|u| matches!(u.kind, UpdateKind::InlineQuery(_))));
        self
    }

    /// Filters updates from a specific user.
    pub fn from_user(mut self, user_id: UserId) -> Self {
        self.conditions.push(Box::new(move |u| u.from().is_some_and(|f| f.id == user_id)));
        self
    }

    /// Builds the filter into a predicate function.
    pub fn build(self) -> impl Fn(&Update) -> bool {
        move |update| self.conditions.iter().all(|c| c(update))
    }
}

impl Default for UpdateFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Chat, ChatKind, ChatPrivate, MediaKind, MediaText, MessageCommon, MessageId, MessageKind,
        User,
    };

    fn make_message(text: &str, user_id: u64) -> Message {
        Message {
            id: MessageId(1),
            thread_id: None,
            direct_messages_topic: None,
            from: Some(User {
                id: UserId(user_id),
                is_bot: false,
                first_name: "Test".to_string(),
                last_name: None,
                username: None,
                language_code: None,
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
            }),
            sender_chat: None,
            date: chrono::Utc::now(),
            chat: Chat {
                id: ChatId(1),
                kind: ChatKind::Private(ChatPrivate {
                    username: None,
                    first_name: None,
                    last_name: None,
                }),
            },
            is_topic_message: false,
            suggested_post_info: None,
            is_paid_post: false,
            via_bot: None,
            sender_business_bot: None,
            kind: MessageKind::Common(MessageCommon {
                sender_tag: None,
                receiver_user: None,
                ephemeral_message_id: None,
                guest_bot_caller_user: None,
                guest_bot_caller_chat: None,
                guest_query_id: None,
                reply_to_poll_option_id: None,
                author_signature: None,
                paid_star_count: None,
                effect_id: None,
                forward_origin: None,
                reply_to_message: None,
                external_reply: None,
                quote: None,
                reply_to_story: None,
                reply_to_checklist_task_id: None,
                sender_boost_count: None,
                edit_date: None,
                media_kind: MediaKind::Text(MediaText {
                    text: text.to_string(),
                    entities: vec![],
                    link_preview_options: None,
                }),
                reply_markup: None,
                is_automatic_forward: false,
                has_protected_content: false,
                is_from_offline: false,
                business_connection_id: None,
            }),
        }
    }

    #[test]
    fn filter_text() {
        let filter = FilterBuilder::new().text().build();
        let msg = make_message("hello", 1);
        assert!(filter(&msg));
    }

    #[test]
    fn filter_startswith() {
        let filter = FilterBuilder::new().text().startswith("hello").build();
        let msg = make_message("hello world", 1);
        assert!(filter(&msg));

        let msg2 = make_message("goodbye", 1);
        assert!(!filter(&msg2));
    }

    #[test]
    fn filter_from_user() {
        let filter = FilterBuilder::new().from_user(UserId(123)).build();
        let msg = make_message("hi", 123);
        assert!(filter(&msg));

        let msg2 = make_message("hi", 456);
        assert!(!filter(&msg2));
    }

    #[test]
    fn filter_and() {
        let filter = FilterBuilder::new().text().startswith("hi").from_user(UserId(1)).build();

        let msg = make_message("hi there", 1);
        assert!(filter(&msg));

        let msg2 = make_message("hi there", 2);
        assert!(!filter(&msg2));

        let msg3 = make_message("bye", 1);
        assert!(!filter(&msg3));
    }

    #[test]
    fn filter_command() {
        let filter = FilterBuilder::new().is_command().build();
        let msg = make_message("/start", 1);
        assert!(filter(&msg));

        let msg2 = make_message("hello", 1);
        assert!(!filter(&msg2));
    }

    #[test]
    fn filter_is_private() {
        let filter = FilterBuilder::new().is_private().build();
        let msg = make_message("hi", 1);
        assert!(filter(&msg));
    }
}
