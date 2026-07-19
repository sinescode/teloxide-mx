//! Chat action auto-sender — keeps users informed during long operations.
//!
//! Similar to aiogram's `ChatActionSender`, this utility automatically sends
//! chat actions (typing, upload_photo, etc.) while a long operation is running.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::utils::chat_action::ChatActionSender;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! async fn handle(bot: Bot, msg: Message) -> HandlerResult {
//!     let _action = ChatActionSender::new(&bot, msg.chat.id, "typing");
//!
//!     // Long operation...
//!     let result = heavy_computation().await;
//!
//!     msg.answer(&bot, result).await?;
//!     Ok(())
//! }
//! ```

use std::time::Duration;

use tokio::time::sleep;

use crate::requests::Requester;
use crate::types::{ChatAction, ChatId};

/// Default interval between chat action messages (5 seconds).
pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(5);

/// Automatically sends chat actions at regular intervals while alive.
///
/// When dropped (goes out of scope), the action loop stops.
pub struct ChatActionSender {
    _task: tokio::task::JoinHandle<()>,
}

impl ChatActionSender {
    /// Creates a new `ChatActionSender` that sends the specified action
    /// every `interval` seconds.
    pub fn new<R>(bot: &R, chat_id: ChatId, action: ChatAction) -> Self
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::SendChatAction: Send,
    {
        let bot = bot.clone();
        let task = tokio::spawn(async move {
            loop {
                sleep(DEFAULT_INTERVAL).await;
                if let Err(_) = bot.send_chat_action(chat_id, action.clone()).await {
                    break;
                }
            }
        });
        Self { _task: task }
    }

    /// Creates a new sender with a custom interval.
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
        let bot = bot.clone();
        let task = tokio::spawn(async move {
            loop {
                sleep(interval).await;
                if let Err(_) = bot.send_chat_action(chat_id, action.clone()).await {
                    break;
                }
            }
        });
        Self { _task: task }
    }

    /// Stops the chat action sender manually.
    pub fn stop(self) {
        self._task.abort();
    }
}

impl Drop for ChatActionSender {
    fn drop(&mut self) {
        self._task.abort();
    }
}
