//! Callback answer helper — auto-answer callback queries.
//!
//! Similar to aiogram's `CallbackAnswer` utility, this provides a convenient
//! way to answer callback queries from within handlers.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::utils::callback_answer::CallbackAnswer;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! async fn handle(bot: Bot, q: CallbackQuery) -> HandlerResult {
//!     let answer = CallbackAnswer::new(&bot, &q.id);
//!     answer.text("Done!").show_alert(true).send().await?;
//!     Ok(())
//! }
//! ```

use crate::{payloads::AnswerCallbackQuerySetters, requests::Requester, types::CallbackQueryId};
use url::Url;

/// A builder for answering callback queries.
pub struct CallbackAnswer<'a, R: Requester> {
    bot: &'a R,
    callback_query_id: &'a CallbackQueryId,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<Url>,
    cache_time: Option<u32>,
}

impl<'a, R: Requester> CallbackAnswer<'a, R> {
    /// Creates a new `CallbackAnswer` for the given callback query.
    pub fn new(bot: &'a R, callback_query_id: &'a CallbackQueryId) -> Self {
        Self { bot, callback_query_id, text: None, show_alert: None, url: None, cache_time: None }
    }

    /// Sets the text to display in the notification.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets whether to show an alert instead of a notification.
    pub fn show_alert(mut self, show: bool) -> Self {
        self.show_alert = Some(show);
        self
    }

    /// Sets the URL to open when the button is pressed (for games).
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets how long (in seconds) the result is cacheable.
    pub fn cache_time(mut self, seconds: u32) -> Self {
        self.cache_time = Some(seconds);
        self
    }

    /// Sends the callback answer.
    pub async fn send(self) -> Result<(), R::Err>
    where
        R::AnswerCallbackQuery: AnswerCallbackQuerySetters,
    {
        let mut req = self.bot.answer_callback_query(self.callback_query_id.clone());
        if let Some(text) = self.text {
            req = req.text(text);
        }
        if let Some(show_alert) = self.show_alert {
            req = req.show_alert(show_alert);
        }
        if let Some(url) = self.url {
            req = req.url(url);
        }
        if let Some(cache_time) = self.cache_time {
            req = req.cache_time(cache_time);
        }
        req.await?;
        Ok(())
    }
}
