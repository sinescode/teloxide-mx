//! Convenient error handling.
//!
//! This module provides error handlers for different error types, similar
//! to aiogram's error handler system with `ErrorEvent` context.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::error_handlers::*;
//! # use teloxide::error_types::TelegramError;
//!
//! // Log errors
//! let handler = LoggingErrorHandler::new();
//!
//! // Ignore specific error types
//! let handler = IgnoringErrorHandler::new();
//!
//! // Filter errors by type
//! // handler.filter(|e: &TelegramError| e.error_code == 400);
//! ```

use futures::future::BoxFuture;
use std::{collections::HashMap, convert::Infallible, fmt::Debug, future::Future, sync::Arc};

/// An asynchronous handler of an error.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait ErrorHandler<E> {
    #[must_use]
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()>;
}

impl<E, F, Fut> ErrorHandler<E> for F
where
    F: Fn(E) -> Fut + Send + Sync + 'static,
    E: Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        Box::pin(async move { self(error).await })
    }
}

/// Something that can be handled by an error handler.
///
/// ## Examples
/// ```
/// use teloxide::error_handlers::OnError;
///
/// # #[tokio::main]
/// # async fn main_() {
/// // Prints nothing
/// let ok: Result<i32, i32> = Ok(200);
/// ok.log_on_error().await;
///
/// // Prints "Error: 404"
/// let err: Result<i32, i32> = Err(404);
/// err.log_on_error().await;
/// # }
/// ```
///
/// Use an arbitrary error handler:
/// ```
/// use teloxide::error_handlers::{IgnoringErrorHandler, OnError};
///
/// # #[tokio::main]
/// # async fn main_() {
/// let err: Result<i32, i32> = Err(404);
/// err.on_error(IgnoringErrorHandler::new()).await;
/// # }
/// ```
pub trait OnError<E> {
    #[must_use]
    fn on_error<'a, Eh>(self, eh: Arc<Eh>) -> BoxFuture<'a, ()>
    where
        Self: 'a,
        Eh: ErrorHandler<E> + Send + Sync,
        Arc<Eh>: 'a;

    /// A shortcut for `.on_error(LoggingErrorHandler::new())`.
    #[must_use]
    fn log_on_error<'a>(self) -> BoxFuture<'a, ()>
    where
        Self: Sized + 'a,
        E: Debug,
    {
        self.on_error(LoggingErrorHandler::new())
    }
}

impl<T, E> OnError<E> for Result<T, E>
where
    T: Send,
    E: Send,
{
    fn on_error<'a, Eh>(self, eh: Arc<Eh>) -> BoxFuture<'a, ()>
    where
        Self: 'a,
        Eh: ErrorHandler<E> + Send + Sync,
        Arc<Eh>: 'a,
    {
        Box::pin(async move {
            if let Err(error) = self {
                eh.handle_error(error).await;
            }
        })
    }
}

/// A handler that silently ignores all errors.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::error_handlers::{ErrorHandler, IgnoringErrorHandler};
///
/// IgnoringErrorHandler::new().handle_error(()).await;
/// IgnoringErrorHandler::new().handle_error(404).await;
/// IgnoringErrorHandler::new().handle_error("error").await;
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct IgnoringErrorHandler;

impl IgnoringErrorHandler {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl<E> ErrorHandler<E> for IgnoringErrorHandler {
    fn handle_error(self: Arc<Self>, _: E) -> BoxFuture<'static, ()> {
        Box::pin(async {})
    }
}

/// A handler that silently ignores all errors that can never happen (e.g.:
/// [`!`] or [`Infallible`]).
///
/// ## Examples
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use std::convert::{Infallible, TryInto};
///
/// use teloxide::error_handlers::{ErrorHandler, IgnoringErrorHandlerSafe};
///
/// let result: Result<String, Infallible> = "str".try_into();
/// match result {
///     Ok(string) => println!("{}", string),
///     Err(inf) => IgnoringErrorHandlerSafe::new().handle_error(inf).await,
/// }
///
/// IgnoringErrorHandlerSafe::new().handle_error(return).await; // return type of `return` is `!` (aka never)
/// # }
/// ```
///
/// ```compile_fail
/// use teloxide::dispatching::{ErrorHandler, IgnoringErrorHandlerSafe};
///
/// IgnoringErrorHandlerSafe.handle_error(0);
/// ```
///
/// [`!`]: https://doc.rust-lang.org/std/primitive.never.html
/// [`Infallible`]: std::convert::Infallible
#[derive(Clone, Copy)]
pub struct IgnoringErrorHandlerSafe;

impl IgnoringErrorHandlerSafe {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

#[allow(unreachable_code)]
impl ErrorHandler<Infallible> for IgnoringErrorHandlerSafe {
    fn handle_error(self: Arc<Self>, _: Infallible) -> BoxFuture<'static, ()> {
        Box::pin(async {})
    }
}

/// A handler that log all errors passed into it.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::error_handlers::{ErrorHandler, LoggingErrorHandler};
///
/// LoggingErrorHandler::new().handle_error(()).await;
/// LoggingErrorHandler::with_custom_text("Omg1").handle_error(404).await;
/// LoggingErrorHandler::with_custom_text("Omg2").handle_error("Invalid data type!").await;
/// # }
/// ```
pub struct LoggingErrorHandler {
    text: String,
}

impl LoggingErrorHandler {
    /// Creates `LoggingErrorHandler` with a meta text before a log.
    ///
    /// The logs will be printed in this format: `{text}: {:?}`.
    #[must_use]
    pub fn with_custom_text<T>(text: T) -> Arc<Self>
    where
        T: Into<String>,
    {
        Arc::new(Self { text: text.into() })
    }

    /// A shortcut for
    /// `LoggingErrorHandler::with_custom_text("Error".to_owned())`.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_custom_text("Error".to_owned())
    }
}

impl<E> ErrorHandler<E> for LoggingErrorHandler
where
    E: Debug,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        log::error!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}

/// A handler that routes errors to different handlers based on error type.
///
/// Similar to aiogram's `ErrorEvent` with `ExceptionTypeFilter`.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide::prelude::*;
/// # use teloxide::error_handlers::*;
/// # use teloxide::error_types::TelegramError;
///
/// let handler = ErrorRouter::new()
///     .on(400, |e: TelegramError| async move {
///         log::warn!("Bad request: {}", e.description);
///     })
///     .on(429, |e: TelegramError| async move {
///         log::warn!("Rate limited, retry after {:?}", e.retry_after());
///     })
///     .default(LoggingErrorHandler::new());
/// ```
pub struct ErrorRouter<E> {
    code_handlers: HashMap<i64, Arc<dyn ErrorHandler<E> + Send + Sync>>,
    default_handler: Option<Arc<dyn ErrorHandler<E> + Send + Sync>>,
}

impl<E> ErrorRouter<E> {
    /// Creates a new error router.
    pub fn new() -> Self {
        Self { code_handlers: HashMap::new(), default_handler: None }
    }

    /// Registers a handler for a specific Telegram error code.
    ///
    /// When an error with a matching `error_code` is routed, this handler
    /// will be invoked instead of the default.
    pub fn on(
        mut self,
        error_code: i64,
        handler: impl ErrorHandler<E> + Send + Sync + 'static,
    ) -> Self {
        self.code_handlers.insert(error_code, Arc::new(handler));
        self
    }

    /// Sets the default fallback handler, used when no specific error code
    /// handler matches.
    pub fn default(mut self, handler: impl ErrorHandler<E> + Send + Sync + 'static) -> Self {
        self.default_handler = Some(Arc::new(handler));
        self
    }

    /// Returns the router ready for use.
    pub fn build(self) -> Self {
        self
    }

    /// Routes an error to the appropriate handler by error code.
    ///
    /// Looks up the error code in registered handlers; falls back to the
    /// default handler if no match is found.
    pub async fn route(&self, error_code: i64, error: E)
    where
        E: Send + 'static,
    {
        if let Some(handler) = self.code_handlers.get(&error_code) {
            let handler = Arc::clone(handler);
            handler.handle_error(error).await;
        } else if let Some(handler) = &self.default_handler {
            let handler = Arc::clone(handler);
            handler.handle_error(error).await;
        }
    }
}

impl<E> Default for ErrorRouter<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Context wrapper for errors that includes the original update.
pub struct ErrorEvent<E> {
    /// The error that occurred.
    pub error: E,
    /// The update that caused the error, if available.
    pub update: Option<Arc<crate::types::Update>>,
}

impl<E> ErrorEvent<E> {
    /// Creates a new error event.
    pub fn new(error: E) -> Self {
        Self { error, update: None }
    }

    /// Creates a new error event with an update context.
    pub fn with_update(error: E, update: Arc<crate::types::Update>) -> Self {
        Self { error, update: Some(update) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn error_event_creation() {
        let event = ErrorEvent::new("test error");
        assert_eq!(event.error, "test error");
        assert!(event.update.is_none());
    }
}
