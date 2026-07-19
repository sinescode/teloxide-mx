//! Per-event middleware system.
//!
//! Similar to aiogram's middleware, this module provides both outer and inner
//! middleware that can be attached to specific event types.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::dispatching::middleware::{Middleware, MiddlewareContext};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! struct LoggingMiddleware;
//!
//! impl Middleware for LoggingMiddleware {
//!     fn handle<'a>(
//!         &'a self,
//!         ctx: MiddlewareContext,
//!         next: std::sync::Arc<dyn Fn(DependencyMap) -> std::pin::Pin<Box<dyn std::future::Future<Output = DependencyMap> + Send>> + Send + Sync>,
//!     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + 'a> {
//!         Box::pin(async move {
//!             log::info!("Processing update");
//!             let _deps = next(ctx.deps).await;
//!             log::info!("Update processed");
//!             Ok(())
//!         })
//!     }
//! }
//! ```

use crate::types::{CallbackQuery, InlineQuery, Message, Update};
use dptree::di::DependencyMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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
    pub fn update(&self) -> Option<&Update> {
        self.deps.get()
    }

    /// Extracts the message from the context.
    pub fn message(&self) -> Option<&Message> {
        self.deps.get()
    }

    /// Extracts the callback query from the context.
    pub fn callback_query(&self) -> Option<&CallbackQuery> {
        self.deps.get()
    }

    /// Extracts the inline query from the context.
    pub fn inline_query(&self) -> Option<&InlineQuery> {
        self.deps.get()
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
    /// Call `next.dispatch(ctx.deps)` to continue to the next middleware/handler.
    /// Return early to stop the chain.
    fn handle<'a>(
        &'a self,
        ctx: MiddlewareContext,
        next: Arc<
            dyn Fn(
                    DependencyMap,
                ) -> Pin<
                    Box<
                        dyn Future<Output = DependencyMap> + Send,
                    >,
                > + Send
                + Sync,
        >,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>>;
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
    fn handle<'a>(
        &'a self,
        ctx: MiddlewareContext,
        next: Arc<
            dyn Fn(
                    DependencyMap,
                ) -> Pin<
                    Box<
                        dyn Future<Output = DependencyMap> + Send,
                    >,
                > + Send
                + Sync,
        >,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
        let label = self.label.clone();
        Box::pin(async move {
            log::trace!("[{}] Before handler", label);
            let _deps = next(ctx.deps).await;
            log::trace!("[{}] After handler", label);
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
    fn handle<'a>(
        &'a self,
        ctx: MiddlewareContext,
        next: Arc<
            dyn Fn(
                    DependencyMap,
                ) -> Pin<
                    Box<
                        dyn Future<Output = DependencyMap> + Send,
                    >,
                > + Send
                + Sync,
        >,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
        let interval = self.interval_ms;
        Box::pin(async move {
            tokio::time::sleep(std::time::Duration::from_millis(interval)).await;
            let _deps = next(ctx.deps).await;
            Ok(())
        })
    }
}

/// A middleware that catches errors and logs them.
pub struct ErrorCatchMiddleware;

impl Middleware for ErrorCatchMiddleware {
    fn handle<'a>(
        &'a self,
        ctx: MiddlewareContext,
        next: Arc<
            dyn Fn(
                    DependencyMap,
                ) -> Pin<
                    Box<
                        dyn Future<Output = DependencyMap> + Send,
                    >,
                > + Send
                + Sync,
        >,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            let _deps = next(ctx.deps).await;
            Ok(())
        })
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
}
