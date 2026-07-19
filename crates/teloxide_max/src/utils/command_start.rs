//! Filters for `/start` commands.
//!
//! This module provides a [`CommandStart`] struct that represents a `/start`
//! command with an optional payload. It can be used to handle deep links
//! and bot startup flows.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::utils::command_start::CommandStart;
//!
//! let start = CommandStart::parse("/start", "mybot").unwrap();
//! assert_eq!(start.payload, None);
//!
//! let start = CommandStart::parse("/start@mybot hello", "mybot").unwrap();
//! assert_eq!(start.payload.as_deref(), Some("hello"));
//!
//! let start = CommandStart::parse("/start deep_link_data", "mybot").unwrap();
//! assert_eq!(start.payload.as_deref(), Some("deep_link_data"));
//! ```

use crate::dispatching::DpHandlerDescription;
use dptree::Handler;
use teloxide_max_core::types::{Me, Message};

/// Represents a `/start` command with an optional payload.
///
/// This is useful for handling Telegram deep links where users can start
/// the bot with pre-filled data via `https://t.me/botname?start=<payload>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStart {
    /// The optional payload passed after `/start`.
    ///
    /// For `/start`, this is `None`.
    /// For `/start hello`, this is `Some("hello")`.
    pub payload: Option<String>,
}

impl CommandStart {
    /// Parses a text message into a `CommandStart`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text of the message to parse.
    /// * `bot_name` - The username of the bot (without `@`).
    ///
    /// # Returns
    ///
    /// `Some(CommandStart)` if the text is a valid `/start` command,
    /// `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::command_start::CommandStart;
    ///
    /// assert!(CommandStart::parse("/start", "mybot").is_some());
    /// assert!(CommandStart::parse("/start@mybot", "mybot").is_some());
    /// assert!(CommandStart::parse("/start payload", "mybot").is_some());
    /// assert!(CommandStart::parse("/start@mybot payload", "mybot").is_some());
    /// assert!(CommandStart::parse("/help", "mybot").is_none());
    /// assert!(CommandStart::parse("hello", "mybot").is_none());
    /// ```
    pub fn parse(text: &str, bot_name: &str) -> Option<Self> {
        let text = text.strip_prefix('/')?;

        // Split into command part and rest (split on first space)
        let (cmd_part, rest) = match text.split_once(char::is_whitespace) {
            Some((cmd, rest)) => (cmd, rest.trim()),
            None => (text, ""),
        };

        // Split command part on @ to check bot name
        let (command, mentioned_bot) = match cmd_part.split_once('@') {
            Some((cmd, bot)) => (cmd, Some(bot)),
            None => (cmd_part, None),
        };

        if command != "start" {
            return None;
        }

        // Check bot mention if present
        if let Some(mentioned) = mentioned_bot {
            if !mentioned.eq_ignore_ascii_case(bot_name) {
                return None;
            }
        }

        let payload = if rest.is_empty() { None } else { Some(rest.to_string()) };
        Some(CommandStart { payload })
    }

    /// Returns a dptree handler that filters for `/start` commands.
    ///
    /// The handler extracts the [`CommandStart`] and injects it as a
    /// dependency for downstream handlers.
    ///
    /// # Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use teloxide_max::{prelude::*, utils::command_start::CommandStart};
    ///
    /// async fn handle_start(bot: Bot, msg: Message, start: CommandStart) -> ResponseResult<()> {
    ///     let payload = start.payload.unwrap_or_default();
    ///     bot.send_message(msg.chat.id, format!("Started with: {payload}")).await?;
    ///     Ok(())
    /// }
    ///
    /// // In your handler chain:
    /// // CommandStart::filter().endpoint(handle_start)
    /// ```
    #[must_use]
    pub fn filter() -> Handler<'static, DpHandlerDescription, DpHandlerDescription> {
        dptree::filter_map(|message: Message, me: Me| {
            let bot_name = me.user.username.expect("Bots must have a username");
            message
                .text()
                .or_else(|| message.caption())
                .and_then(|text| Self::parse(text, &bot_name))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_start_no_payload() {
        let start = CommandStart::parse("/start", "mybot").unwrap();
        assert_eq!(start, CommandStart { payload: None });
    }

    #[test]
    fn parse_start_with_payload() {
        let start = CommandStart::parse("/start hello", "mybot").unwrap();
        assert_eq!(start, CommandStart { payload: Some("hello".to_string()) });
    }

    #[test]
    fn parse_start_with_bot_mention() {
        let start = CommandStart::parse("/start@mybot payload", "mybot").unwrap();
        assert_eq!(start, CommandStart { payload: Some("payload".to_string()) });
    }

    #[test]
    fn parse_start_wrong_bot() {
        assert!(CommandStart::parse("/start@otherbot payload", "mybot").is_none());
    }

    #[test]
    fn parse_not_start() {
        assert!(CommandStart::parse("/help", "mybot").is_none());
    }

    #[test]
    fn parse_no_prefix() {
        assert!(CommandStart::parse("start", "mybot").is_none());
    }

    #[test]
    fn parse_empty_text() {
        assert!(CommandStart::parse("", "mybot").is_none());
    }

    #[test]
    fn parse_start_with_spaces() {
        let start = CommandStart::parse("/start   payload with spaces  ", "mybot").unwrap();
        assert_eq!(start, CommandStart { payload: Some("payload with spaces".to_string()) });
    }
}
