//! Chat action auto-sender — keeps users informed during long operations.
//!
//! Similar to aiogram's `ChatActionSender`, this utility automatically sends
//! chat actions (typing, upload_photo, etc.) while a long operation is running.
//!
//! Sends the first action **immediately** (after optional `initial_sleep`),
//! then repeats every `interval` (default 5s) until dropped.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::types::ChatAction;
//! # use teloxide_max::utils::chat_action::ChatActionSender;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! async fn handle(bot: Bot, msg: Message) -> HandlerResult {
//!     let _action = ChatActionSender::typing(&bot, msg.chat.id);
//!
//!     // Long operation...
//!     let result = heavy_computation().await;
//!
//!     msg.answer(&bot, result).await?;
//!     Ok(())
//! }
//! # async fn heavy_computation() -> String { String::new() }
//! ```
//!
//! # Migration from aiogram
//!
//! ```python
//! # aiogram
//! async with ChatActionSender.typing(bot=bot, chat_id=message.chat.id):
//!     await do_work()
//! ```
//!
//! ```rust
//! // teloxide_max — RAII: drops when leaving scope
//! let _action = ChatActionSender::typing(&bot, msg.chat.id);
//! do_work().await;
//! ```

use std::time::Duration;

use tokio::time::sleep;

use teloxide_max_core::payloads::SendChatActionSetters;

use crate::{
    requests::Requester,
    types::{ChatAction, ChatId, ThreadId},
};

/// Default interval between chat action messages (5 seconds).
pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(5);

/// Default delay before the first action (0 — send immediately).
pub const DEFAULT_INITIAL_SLEEP: Duration = Duration::from_secs(0);

/// Automatically sends chat actions at regular intervals while alive.
///
/// When dropped (goes out of scope), the action loop is aborted.
pub struct ChatActionSender {
    _task: tokio::task::JoinHandle<()>,
}

/// Configuration for [`ChatActionSender`].
#[derive(Debug, Clone)]
pub struct ChatActionSenderConfig {
    /// Chat action to send.
    pub action: ChatAction,
    /// Interval between repeated sends.
    pub interval: Duration,
    /// Sleep before the first send (aiogram `initial_sleep`).
    pub initial_sleep: Duration,
    /// Optional forum topic thread id.
    pub message_thread_id: Option<ThreadId>,
}

impl Default for ChatActionSenderConfig {
    fn default() -> Self {
        Self {
            action: ChatAction::Typing,
            interval: DEFAULT_INTERVAL,
            initial_sleep: DEFAULT_INITIAL_SLEEP,
            message_thread_id: None,
        }
    }
}

impl ChatActionSender {
    /// Creates a sender with `action`, default interval (5s), immediate first
    /// send.
    pub fn new<R>(bot: &R, chat_id: ChatId, action: ChatAction) -> Self
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::SendChatAction: Send,
    {
        Self::with_config(bot, chat_id, ChatActionSenderConfig { action, ..Default::default() })
    }

    /// Convenience: typing indicator.
    pub fn typing<R>(bot: &R, chat_id: ChatId) -> Self
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::SendChatAction: Send,
    {
        Self::new(bot, chat_id, ChatAction::Typing)
    }

    /// Creates a sender with a custom interval (still sends first action
    /// immediately).
    pub fn with_interval<R>(
        bot: &R,
        chat_id: ChatId,
        action: ChatAction,
        interval: Duration,
    ) -> Self
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::SendChatAction: Send,
    {
        Self::with_config(
            bot,
            chat_id,
            ChatActionSenderConfig { action, interval, ..Default::default() },
        )
    }

    /// Full configuration (interval, initial_sleep, thread id).
    pub fn with_config<R>(bot: &R, chat_id: ChatId, config: ChatActionSenderConfig) -> Self
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::SendChatAction: Send,
    {
        let bot = bot.clone();
        let ChatActionSenderConfig { action, interval, initial_sleep, message_thread_id } = config;

        let task = tokio::spawn(async move {
            if !initial_sleep.is_zero() {
                sleep(initial_sleep).await;
            }
            loop {
                let mut req = bot.send_chat_action(chat_id, action);
                if let Some(thread_id) = message_thread_id {
                    req = req.message_thread_id(thread_id);
                }
                if req.await.is_err() {
                    break;
                }
                sleep(interval).await;
            }
        });
        Self { _task: task }
    }

    /// Stops the chat action sender manually.
    pub fn stop(self) {
        self._task.abort();
    }

    /// Returns `true` if the background task is still running.
    pub fn is_finished(&self) -> bool {
        self._task.is_finished()
    }
}

impl Drop for ChatActionSender {
    fn drop(&mut self) {
        self._task.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let cfg = ChatActionSenderConfig::default();
        assert_eq!(cfg.interval, DEFAULT_INTERVAL);
        assert_eq!(cfg.initial_sleep, DEFAULT_INITIAL_SLEEP);
        assert!(matches!(cfg.action, ChatAction::Typing));
        assert!(cfg.message_thread_id.is_none());
    }
}
