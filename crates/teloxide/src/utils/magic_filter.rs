//! Magic filter DSL for ergonomic message filtering.
//!
//! This module provides a composable filter system inspired by aiogram's
//! `MagicFilter`. Filters can be combined with `&`, `|`, and `!` operators.
//!
//! # Example
//!
//! ```rust
//! use teloxide::prelude::*;
//! use teloxide::utils::magic_filter::{F, Filter};
//!
//! // Simple filters
//! let f = F::text;                          // Has text
//! let f = F::text.contains("hello");        // Text contains "hello"
//! let f = F::text.startswith("!");          // Text starts with "!"
//! let f = F::text.regexp(r"^\d+$");         // Text is all digits
//! let f = F::from_user.id(123);             // From specific user
//! let f = F::chat.is_private();             // In private chat
//! let f = F::chat.is_group();              // In group chat
//! let f = F::has_photo;                     // Has photo
//! let f = F::has_document;                  // Has document
//!
//! // Composed filters
//! let f = F::text.contains("admin") & F::from_user.id(123);
//! let f = F::text.startswith("!") | F::text.startswith("/");
//! let f = !F::from_user.is_bot;
//! ```

use crate::types::{ChatType, Message, UserId, ChatId};

/// Magic filter entry point.
pub struct F;

impl F {
    /// Filter on message text.
    pub const text: TextFilter = TextFilter;

    /// Filter on the user who sent the message.
    pub const from_user: UserFilter = UserFilter;

    /// Filter on the chat.
    pub const chat: ChatFilter = ChatFilter;

    /// Filter on message length.
    pub const len: LenFilter = LenFilter;
}

/// Filter that checks if message has text.
pub struct TextFilter;

impl TextFilter {
    /// Returns true if the message has text.
    pub fn matches(&self, msg: &Message) -> bool {
        msg.text().is_some()
    }

    /// Checks if text contains a substring.
    pub fn contains(self, s: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.contains(s)))
    }

    /// Checks if text starts with a prefix.
    pub fn startswith(self, s: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.starts_with(s)))
    }

    /// Checks if text ends with a suffix.
    pub fn endswith(self, s: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.ends_with(s)))
    }

    /// Checks if text matches a regex pattern.
    pub fn regexp(self, pattern: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| {
            msg.text().map_or(false, |t| {
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(t))
                    .unwrap_or(false)
            })
        })
    }

    /// Checks if text equals a value (case-insensitive).
    pub fn eq_ignore_case(self, s: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| {
            msg.text()
                .map_or(false, |t| t.to_lowercase() == s.to_lowercase())
        })
    }

    /// Checks if text is exactly equal to a value.
    pub fn eq(self, s: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text() == Some(s))
    }

    /// Checks if text length is greater than n.
    pub fn len_gt(self, n: usize) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.len() > n))
    }

    /// Checks if text length is less than n.
    pub fn len_lt(self, n: usize) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.len() < n))
    }
}

/// Filter that checks user properties.
pub struct UserFilter;

impl UserFilter {
    /// Checks if the user has a specific ID.
    pub fn id(self, user_id: u64) -> ComposedFilter {
        let uid = UserId(user_id);
        ComposedFilter::new(move |msg| msg.from.as_ref().map_or(false, |u| u.id == uid))
    }

    /// Checks if the sender is a bot.
    pub fn is_bot(self) -> ComposedFilter {
        ComposedFilter::new(|msg| msg.from.as_ref().map_or(false, |u| u.is_bot))
    }

    /// Checks if the user is a premium user.
    pub fn is_premium(self) -> ComposedFilter {
        ComposedFilter::new(|msg| msg.from.as_ref().map_or(false, |u| u.is_premium))
    }

    /// Checks if the user has a specific username.
    pub fn username(self, name: &'static str) -> ComposedFilter {
        ComposedFilter::new(move |msg| {
            msg.from
                .as_ref()
                .map_or(false, |u| u.username.as_deref() == Some(name))
        })
    }
}

/// Filter that checks chat properties.
pub struct ChatFilter;

impl ChatFilter {
    /// Checks if the chat is private.
    pub fn is_private(self) -> ComposedFilter {
        ComposedFilter::new(|msg| msg.chat.kind == ChatType::Private)
    }

    /// Checks if the chat is a group or supergroup.
    pub fn is_group(self) -> ComposedFilter {
        ComposedFilter::new(|msg| {
            matches!(
                msg.chat.kind,
                ChatType::Group | ChatType::Supergroup
            )
        })
    }

    /// Checks if the chat is a supergroup.
    pub fn is_supergroup(self) -> ComposedFilter {
        ComposedFilter::new(|msg| msg.chat.kind == ChatType::Supergroup)
    }

    /// Checks if the chat is a channel.
    pub fn is_channel(self) -> ComposedFilter {
        ComposedFilter::new(|msg| msg.chat.kind == ChatType::Channel)
    }

    /// Checks if the chat has a specific ID.
    pub fn id(self, chat_id: i64) -> ComposedFilter {
        let cid = ChatId(chat_id);
        ComposedFilter::new(move |msg| msg.chat.id == cid)
    }
}

/// Filter that checks message length.
pub struct LenFilter;

impl LenFilter {
    /// Checks if text length is greater than n.
    pub fn gt(self, n: usize) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.len() > n))
    }

    /// Checks if text length is less than n.
    pub fn lt(self, n: usize) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.len() < n))
    }

    /// Checks if text length equals n.
    pub fn eq(self, n: usize) -> ComposedFilter {
        ComposedFilter::new(move |msg| msg.text().map_or(false, |t| t.len() == n))
    }
}

/// A composable filter that can be combined with `&`, `|`, and `!`.
pub struct ComposedFilter {
    predicate: Box<dyn Fn(&Message) -> bool + Send + Sync>,
}

impl ComposedFilter {
    /// Creates a new filter from a predicate.
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(&Message) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Box::new(predicate),
        }
    }

    /// Tests if the message matches this filter.
    pub fn matches(&self, msg: &Message) -> bool {
        (self.predicate)(msg)
    }

    /// Combines two filters with AND.
    pub fn and(self, other: ComposedFilter) -> ComposedFilter {
        ComposedFilter::new(move |msg| {
            (self.predicate)(msg) && (other.predicate)(msg)
        })
    }

    /// Combines two filters with OR.
    pub fn or(self, other: ComposedFilter) -> ComposedFilter {
        ComposedFilter::new(move |msg| {
            (self.predicate)(msg) || (other.predicate)(msg)
        })
    }

    /// Negates this filter.
    pub fn not(self) -> ComposedFilter {
        ComposedFilter::new(move |msg| !(self.predicate)(msg))
    }
}

/// Allows composing filters with `&`.
impl std::ops::BitAnd for ComposedFilter {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

/// Allows composing filters with `|`.
impl std::ops::BitOr for ComposedFilter {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

/// Allows negating filters with `!`.
impl std::ops::Not for ComposedFilter {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

/// Helper trait for applying magic filters to messages.
pub trait FilterExt {
    /// Tests if this message matches the filter.
    fn matches(&self, filter: &ComposedFilter) -> bool;
}

impl FilterExt for Message {
    fn matches(&self, filter: &ComposedFilter) -> bool {
        filter.matches(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Chat, ChatPrivate, MessageCommon, MessageKind, MediaKind, MediaText, User};

    fn make_message(text: &str) -> Message {
        Message {
            id: crate::types::MessageId(1),
            chat: Chat {
                id: ChatId(1),
                kind: ChatType::Private,
                ..Default::default()
            },
            from: Some(User {
                id: UserId(1),
                is_bot: false,
                first_name: "Test".to_string(),
                last_name: None,
                username: None,
                language_code: Some("en".to_string()),
                is_premium: false,
                added_to_attachment_menu: false,
            }),
            text: Some(text.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn text_filter() {
        let msg = make_message("hello world");
        assert!(F::text.matches(&msg));
    }

    #[test]
    fn text_contains() {
        let msg = make_message("hello world");
        let filter = F::text.contains("world");
        assert!(filter.matches(&msg));

        let filter = F::text.contains("xyz");
        assert!(!filter.matches(&msg));
    }

    #[test]
    fn text_startswith() {
        let msg = make_message("/start bot");
        let filter = F::text.startswith("/");
        assert!(filter.matches(&msg));
    }

    #[test]
    fn text_eq() {
        let msg = make_message("hello");
        let filter = F::text.eq("hello");
        assert!(filter.matches(&msg));

        let filter = F::text.eq("world");
        assert!(!filter.matches(&msg));
    }

    #[test]
    fn user_id_filter() {
        let msg = make_message("hi");
        let filter = F::from_user.id(1);
        assert!(filter.matches(&msg));

        let filter = F::from_user.id(999);
        assert!(!filter.matches(&msg));
    }

    #[test]
    fn chat_is_private() {
        let msg = make_message("hi");
        let filter = F::chat.is_private();
        assert!(filter.matches(&msg));
    }

    #[test]
    fn and_filter() {
        let msg = make_message("hello");
        let filter = F::text.contains("hello") & F::from_user.id(1);
        assert!(filter.matches(&msg));

        let filter = F::text.contains("hello") & F::from_user.id(999);
        assert!(!filter.matches(&msg));
    }

    #[test]
    fn or_filter() {
        let msg = make_message("hello");
        let filter = F::text.eq("hello") | F::text.eq("world");
        assert!(filter.matches(&msg));

        let filter = F::text.eq("xyz") | F::text.eq("world");
        assert!(!filter.matches(&msg));
    }

    #[test]
    fn not_filter() {
        let msg = make_message("hello");
        let filter = !F::text.eq("hello");
        assert!(!filter.matches(&msg));

        let filter = !F::text.eq("world");
        assert!(filter.matches(&msg));
    }

    #[test]
    fn text_len_gt() {
        let msg = make_message("hello");
        let filter = F::len.gt(3);
        assert!(filter.matches(&msg));

        let filter = F::len.gt(10);
        assert!(!filter.matches(&msg));
    }
}
