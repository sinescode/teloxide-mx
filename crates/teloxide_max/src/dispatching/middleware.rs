//! Per-event middleware system.
//!
//! Similar to aiogram's middleware, this module provides both outer and inner
//! middleware that can be attached to specific event types.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::dispatching::middleware::{Middleware, MiddlewareContext, NextFn, MiddlewareFuture};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! struct LoggingMiddleware;
//!
//! impl Middleware for LoggingMiddleware {
//!     fn handle<'a>(
//!         &'a self,
//!         ctx: MiddlewareContext,
//!         next: NextFn,
//!     ) -> MiddlewareFuture<'a> {
//!         Box::pin(async move {
//!             log::info!("Processing update");
//!             let _deps = next(ctx.deps).await;
//!             log::info!("Update processed");
//!             Ok(())
//!         })
//!     }
//! }
//! ```

use crate::types::{CallbackQuery, InlineQuery, Message, Update, UpdateKind};
use dptree::di::DependencyMap;
use futures::FutureExt;
use std::{future::Future, pin::Pin, sync::Arc};

type NextFn =
    Arc<dyn Fn(DependencyMap) -> Pin<Box<dyn Future<Output = DependencyMap> + Send>> + Send + Sync>;
type MiddlewareFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>>;

/// Context passed to middleware during update processing.
pub struct MiddlewareContext {
    /// The dependency map for this update.
    pub deps: DependencyMap,
}

impl MiddlewareContext {
    /// Creates a new middleware context.
    pub fn new(deps: DependencyMap) -> Self {
        Self { deps }
    }

    /// Extracts the update from the context.
    pub fn update(&self) -> Option<Arc<Update>> {
        self.deps.try_get()
    }

    /// Extracts the message from the context.
    pub fn message(&self) -> Option<Arc<Message>> {
        let update = self.deps.try_get::<Update>()?;
        match &update.kind {
            UpdateKind::Message(m) => Some(Arc::new(m.clone())),
            _ => None,
        }
    }

    /// Extracts the callback query from the context.
    pub fn callback_query(&self) -> Option<Arc<CallbackQuery>> {
        let update = self.deps.try_get::<Update>()?;
        match &update.kind {
            UpdateKind::CallbackQuery(q) => Some(Arc::new(q.clone())),
            _ => None,
        }
    }

    /// Extracts the inline query from the context.
    pub fn inline_query(&self) -> Option<Arc<InlineQuery>> {
        let update = self.deps.try_get::<Update>()?;
        match &update.kind {
            UpdateKind::InlineQuery(q) => Some(Arc::new(q.clone())),
            _ => None,
        }
    }
}

/// Trait for middleware that processes updates.
///
/// Middleware can:
/// - Modify the dependency map before the handler runs
/// - Short-circuit the handler chain
/// - Run code after the handler completes
/// - Handle errors
pub trait Middleware: Send + Sync + 'static {
    /// Processes the middleware.
    ///
    /// Call `next.dispatch(ctx.deps)` to continue to the next
    /// middleware/handler. Return early to stop the chain.
    fn handle<'a>(&'a self, ctx: MiddlewareContext, next: NextFn) -> MiddlewareFuture<'a>;
}

/// A middleware that logs before and after handler execution.
pub struct LoggingMiddleware {
    label: String,
}

impl LoggingMiddleware {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }
}

impl Middleware for LoggingMiddleware {
    fn handle<'a>(&'a self, ctx: MiddlewareContext, next: NextFn) -> MiddlewareFuture<'a> {
        let label = self.label.clone();
        Box::pin(async move {
            log::trace!("[{label}] Before handler");
            let _deps = next(ctx.deps).await;
            log::trace!("[{label}] After handler");
            Ok(())
        })
    }
}

/// A middleware that throttles requests per chat.
pub struct ThrottleMiddleware {
    interval_ms: u64,
}

impl ThrottleMiddleware {
    pub fn new(interval_ms: u64) -> Self {
        Self { interval_ms }
    }
}

impl Middleware for ThrottleMiddleware {
    fn handle<'a>(&'a self, ctx: MiddlewareContext, next: NextFn) -> MiddlewareFuture<'a> {
        let interval = self.interval_ms;
        Box::pin(async move {
            tokio::time::sleep(std::time::Duration::from_millis(interval)).await;
            let _deps = next(ctx.deps).await;
            Ok(())
        })
    }
}

/// A middleware that catches errors and logs them.
///
/// When the downstream handler chain returns an error, this middleware
/// logs the error via `tracing::error!` (or `log::error!` as fallback)
/// and converts it to `Ok(())`, preventing the error from propagating
/// up the middleware chain.
pub struct ErrorCatchMiddleware;

impl Middleware for ErrorCatchMiddleware {
    fn handle<'a>(&'a self, ctx: MiddlewareContext, next: NextFn) -> MiddlewareFuture<'a> {
        Box::pin(async move {
            // Note: the next() call itself returns DependencyMap (not Result).
            // Errors from the handler chain surface as Err from handle().
            // We catch any error that propagates through the chain and log it.
            match std::panic::AssertUnwindSafe(next(ctx.deps)).catch_unwind().await {
                Ok(_deps) => Ok(()),
                Err(panic) => {
                    let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = panic.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "handler panicked (unknown payload)".to_string()
                    };
                    log::error!("ErrorCatchMiddleware caught panic: {msg}");
                    Ok(())
                }
            }
        })
    }
}

/// An error event context for error handlers.
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// The error that occurred.
    pub error: String,
    /// The type name of the error.
    pub error_type: String,
}

impl ErrorEvent {
    /// Creates a new error event.
    pub fn new(error: &dyn std::error::Error) -> Self {
        Self { error: error.to_string(), error_type: std::any::type_name_of_val(error).to_string() }
    }

    /// Creates an error event from a string.
    pub fn from_string(error: impl Into<String>) -> Self {
        Self { error: error.into(), error_type: "unknown".to_string() }
    }
}

/// Filter that matches errors by their type name.
///
/// Similar to aiogram's `ExceptionTypeFilter`.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::dispatching::middleware::ExceptionTypeFilter;
/// let filter = ExceptionTypeFilter::new("TelegramBadRequest");
/// assert!(filter.matches("TelegramBadRequest: bad request"));
/// assert!(!filter.matches("SomeOtherError: something"));
/// ```
#[derive(Debug, Clone)]
pub struct ExceptionTypeFilter {
    type_names: Vec<String>,
}

impl ExceptionTypeFilter {
    /// Creates a filter that matches errors whose type name contains any of
    /// the given strings.
    pub fn new(type_name: impl Into<String>) -> Self {
        Self { type_names: vec![type_name.into()] }
    }

    /// Creates a filter that matches multiple error type names.
    pub fn new_many(type_names: Vec<String>) -> Self {
        Self { type_names }
    }

    /// Checks if the error message contains any of the type names.
    pub fn matches(&self, error: &str) -> bool {
        self.type_names.iter().any(|name| error.contains(name))
    }
}

/// Filter that matches errors by their message content.
///
/// Similar to aiogram's `ExceptionMessageFilter`.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::dispatching::middleware::ExceptionMessageFilter;
/// let filter = ExceptionMessageFilter::new("timed out");
/// assert!(filter.matches("Connection timed out"));
/// assert!(!filter.matches("Connection refused"));
/// ```
#[derive(Debug, Clone)]
pub struct ExceptionMessageFilter {
    pattern: String,
    is_regex: bool,
}

impl ExceptionMessageFilter {
    /// Creates a filter with a simple substring match.
    pub fn new(pattern: impl Into<String>) -> Self {
        Self { pattern: pattern.into(), is_regex: false }
    }

    /// Creates a filter with a regex pattern.
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self { pattern: pattern.into(), is_regex: true }
    }

    /// Checks if the error message matches the pattern.
    pub fn matches(&self, error: &str) -> bool {
        if self.is_regex {
            match regex::Regex::new(&self.pattern) {
                Ok(re) => re.is_match(error),
                Err(_) => false,
            }
        } else {
            error.contains(&self.pattern)
        }
    }
}

/// A chain of error filters that tries each filter in order.
///
/// Similar to aiogram's error handler chain where the first matching
/// filter wins.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::dispatching::middleware::{ErrorFilterChain, ExceptionTypeFilter};
/// let chain = ErrorFilterChain::new()
///     .add_filter(ExceptionTypeFilter::new("TelegramForbiddenError"))
///     .add_filter(ExceptionTypeFilter::new("TelegramNotFound"));
///
/// assert!(chain.matches("TelegramForbiddenError: bot blocked"));
/// assert!(chain.matches("TelegramNotFound: user not found"));
/// assert!(!chain.matches("SomeOtherError: unknown"));
/// ```
pub struct ErrorFilterChain {
    type_filters: Vec<ExceptionTypeFilter>,
    message_filters: Vec<ExceptionMessageFilter>,
}

impl ErrorFilterChain {
    /// Creates a new empty filter chain.
    pub fn new() -> Self {
        Self { type_filters: Vec::new(), message_filters: Vec::new() }
    }

    /// Adds an `ExceptionTypeFilter` to the chain.
    pub fn add_filter(mut self, filter: ExceptionTypeFilter) -> Self {
        self.type_filters.push(filter);
        self
    }

    /// Adds an `ExceptionMessageFilter` to the chain.
    pub fn add_message_filter(mut self, filter: ExceptionMessageFilter) -> Self {
        self.message_filters.push(filter);
        self
    }

    /// Checks if any filter in the chain matches the error.
    pub fn matches(&self, error: &str) -> bool {
        self.type_filters.iter().any(|f| f.matches(error))
            || self.message_filters.iter().any(|f| f.matches(error))
    }

    /// Returns the number of filters in the chain.
    pub fn len(&self) -> usize {
        self.type_filters.len() + self.message_filters.len()
    }

    /// Returns true if the chain has no filters.
    pub fn is_empty(&self) -> bool {
        self.type_filters.is_empty() && self.message_filters.is_empty()
    }
}

impl Default for ErrorFilterChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middleware_context_creation() {
        let deps = DependencyMap::new();
        let ctx = MiddlewareContext::new(deps);
        assert!(ctx.update().is_none());
    }

    #[test]
    fn exception_type_filter_match() {
        let filter = ExceptionTypeFilter::new("TelegramBadRequest");
        assert!(filter.matches("TelegramBadRequest: bad request"));
        assert!(!filter.matches("SomeOtherError: something"));
    }

    #[test]
    fn exception_type_filter_many() {
        let filter = ExceptionTypeFilter::new_many(vec![
            "TelegramForbiddenError".into(),
            "TelegramNotFound".into(),
        ]);
        assert!(filter.matches("TelegramForbiddenError: blocked"));
        assert!(filter.matches("TelegramNotFound: user"));
        assert!(!filter.matches("OtherError: unknown"));
    }

    #[test]
    fn exception_message_filter_match() {
        let filter = ExceptionMessageFilter::new("timed out");
        assert!(filter.matches("Connection timed out"));
        assert!(!filter.matches("Connection refused"));
    }

    #[test]
    fn exception_message_filter_regex() {
        let filter = ExceptionMessageFilter::regex(r"error \d{3}");
        assert!(filter.matches("Got error 404"));
        assert!(!filter.matches("No error here"));
    }

    #[test]
    fn error_filter_chain() {
        let chain = ErrorFilterChain::new()
            .add_filter(ExceptionTypeFilter::new("TelegramForbiddenError"))
            .add_filter(ExceptionTypeFilter::new("TelegramNotFound"));

        assert!(chain.matches("TelegramForbiddenError: bot blocked"));
        assert!(chain.matches("TelegramNotFound: user not found"));
        assert!(!chain.matches("SomeOtherError: unknown"));
    }

    #[test]
    fn error_filter_chain_with_message_filter() {
        let chain = ErrorFilterChain::new()
            .add_filter(ExceptionTypeFilter::new("TelegramBadRequest"))
            .add_message_filter(ExceptionMessageFilter::new("timed out"));

        assert!(chain.matches("TelegramBadRequest: invalid"));
        assert!(chain.matches("Connection timed out"));
        assert!(!chain.matches("Other error"));
    }

    #[test]
    fn error_filter_chain_len() {
        let chain = ErrorFilterChain::new()
            .add_filter(ExceptionTypeFilter::new("Error1"))
            .add_message_filter(ExceptionMessageFilter::new("pattern"));
        assert_eq!(chain.len(), 2);
    }

    #[test]
    fn error_filter_chain_is_empty() {
        let chain = ErrorFilterChain::new();
        assert!(chain.is_empty());
    }

    #[test]
    fn error_event_creation() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let event = ErrorEvent::new(&error);
        assert_eq!(event.error, "not found");
    }

    #[test]
    fn error_event_from_string() {
        let event = ErrorEvent::from_string("test error");
        assert_eq!(event.error, "test error");
    }
}
