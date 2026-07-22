//! Class-based handlers for aiogram-style handler patterns.
//!
//! Similar to aiogram's `MessageHandler`, `CallbackQueryHandler`, and
//! `ErrorHandler` classes, this module provides structured handler types
//! that encapsulate common handler logic.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::handlers::{MessageHandler, CallbackQueryHandler, ErrorHandler};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! // Class-based message handler
//! struct GreetingHandler;
//!
//! #[async_trait::async_trait]
//! impl MessageHandler for GreetingHandler {
//!     async fn handle(&self, bot: Bot, msg: Message) -> HandlerResult {
//!         bot.send_message(msg.chat.id, "Hello!").await?;
//!         Ok(())
//!     }
//! }
//!
//! // Use with router
//! // router.add_message_handler(GreetingHandler::endpoint());
//! ```

use crate::{
    types::{CallbackQuery, Chat, Message, User},
    Bot,
};

/// Result type for handlers.
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// A class-based handler for message events.
///
/// Similar to aiogram's `MessageHandler` class, this trait provides
/// a structured way to handle messages with convenient accessors.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::prelude::*;
/// # use teloxide_max::handlers::MessageHandler;
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
///
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl MessageHandler for MyHandler {
///     async fn handle(&self, bot: Bot, msg: Message) -> HandlerResult {
///         if let Some(user) = msg.from() {
///             bot.send_message(msg.chat.id, format!("Hello {}!", user.first_name)).await?;
///         }
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait MessageHandler: Send + Sync + 'static {
    /// Handles a message event.
    async fn handle(&self, bot: Bot, msg: Message) -> HandlerResult;

    /// Creates a handler function that can be used with dptree.
    fn endpoint() -> impl Fn(
        Bot,
        Message,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static,
    {
        let handler = std::sync::Arc::new(Self::new());
        move |bot: Bot, msg: Message| {
            let handler = std::sync::Arc::clone(&handler);
            Box::pin(async move { handler.handle(bot, msg).await })
        }
    }

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for callback query events.
///
/// Similar to aiogram's `CallbackQueryHandler` class.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::prelude::*;
/// # use teloxide_max::handlers::CallbackQueryHandler;
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
///
/// struct ButtonHandler;
///
/// #[async_trait::async_trait]
/// impl CallbackQueryHandler for ButtonHandler {
///     async fn handle(&self, bot: Bot, query: CallbackQuery) -> HandlerResult {
///         if let Some(data) = &query.data {
///             bot.answer_callback_query(&query.id).text(format!("You clicked: {data}")).await?;
///         }
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait CallbackQueryHandler: Send + Sync + 'static {
    /// Handles a callback query event.
    async fn handle(&self, bot: Bot, query: CallbackQuery) -> HandlerResult;

    /// Creates a handler function that can be used with dptree.
    fn endpoint() -> impl Fn(
        Bot,
        CallbackQuery,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static,
    {
        let handler = std::sync::Arc::new(Self::new());
        move |bot: Bot, query: CallbackQuery| {
            let handler = std::sync::Arc::clone(&handler);
            Box::pin(async move { handler.handle(bot, query).await })
        }
    }

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for error events.
///
/// Similar to aiogram's `ErrorHandler` class.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::prelude::*;
/// # use teloxide_max::handlers::ErrorHandler;
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
///
/// struct MyErrorHandler;
///
/// #[async_trait::async_trait]
/// impl ErrorHandler for MyErrorHandler {
///     async fn handle_error(
///         &self,
///         error: Box<dyn std::error::Error + Send + Sync>,
///     ) -> HandlerResult {
///         log::error!("Error occurred: {error}");
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait ErrorHandler: Send + Sync + 'static {
    /// Handles an error event.
    async fn handle_error(&self, error: Box<dyn std::error::Error + Send + Sync>) -> HandlerResult;

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for inline query events.
#[async_trait::async_trait]
pub trait InlineQueryHandler: Send + Sync + 'static {
    /// Handles an inline query event.
    async fn handle(&self, bot: Bot, query: crate::types::InlineQuery) -> HandlerResult;

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for chosen inline result events.
#[async_trait::async_trait]
pub trait ChosenInlineResultHandler: Send + Sync + 'static {
    /// Handles a chosen inline result event.
    async fn handle(&self, bot: Bot, result: crate::types::ChosenInlineResult) -> HandlerResult;

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for shipping query events.
#[async_trait::async_trait]
pub trait ShippingQueryHandler: Send + Sync + 'static {
    /// Handles a shipping query event.
    async fn handle(&self, bot: Bot, query: crate::types::ShippingQuery) -> HandlerResult;

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// A class-based handler for pre-checkout query events.
#[async_trait::async_trait]
pub trait PreCheckoutQueryHandler: Send + Sync + 'static {
    /// Handles a pre-checkout query event.
    async fn handle(&self, bot: Bot, query: crate::types::PreCheckoutQuery) -> HandlerResult;

    /// Creates a new instance of this handler.
    fn new() -> Self;
}

/// Extension trait for converting message class-based handlers to dptree
/// handlers.
pub trait MessageHandlerEndpoint {
    /// Converts this handler into a dptree-compatible endpoint function.
    fn into_message_endpoint(
        self,
    ) -> impl Fn(
        Bot,
        Message,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static;
}

impl<T: MessageHandler + 'static> MessageHandlerEndpoint for T {
    fn into_message_endpoint(
        self,
    ) -> impl Fn(
        Bot,
        Message,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static,
    {
        let handler = std::sync::Arc::new(self);
        move |bot: Bot, msg: Message| {
            let handler = std::sync::Arc::clone(&handler);
            Box::pin(async move { handler.handle(bot, msg).await })
        }
    }
}

/// Extension trait for converting callback class-based handlers to dptree
/// handlers.
pub trait CallbackQueryHandlerEndpoint {
    /// Converts this handler into a dptree-compatible endpoint function.
    fn into_callback_endpoint(
        self,
    ) -> impl Fn(
        Bot,
        CallbackQuery,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static;
}

impl<T: CallbackQueryHandler + 'static> CallbackQueryHandlerEndpoint for T {
    fn into_callback_endpoint(
        self,
    ) -> impl Fn(
        Bot,
        CallbackQuery,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send>>
           + Send
           + Sync
           + 'static
    where
        Self: Sized + 'static,
    {
        let handler = std::sync::Arc::new(self);
        move |bot: Bot, query: CallbackQuery| {
            let handler = std::sync::Arc::clone(&handler);
            Box::pin(async move { handler.handle(bot, query).await })
        }
    }
}

/// Backward-compatible alias for [`MessageHandlerEndpoint`].
pub use MessageHandlerEndpoint as HandlerExt;

/// Convenience methods for Message.
pub trait MessageHandlerExt {
    /// Returns the chat for this message.
    fn chat(&self) -> &Chat;

    /// Returns the sender user for this message, if any.
    fn sender(&self) -> Option<&User>;

    /// Returns the text content of this message, if any.
    fn text_content(&self) -> Option<&str>;
}

impl MessageHandlerExt for Message {
    fn chat(&self) -> &Chat {
        &self.chat
    }

    fn sender(&self) -> Option<&User> {
        self.from.as_ref()
    }

    fn text_content(&self) -> Option<&str> {
        self.text()
    }
}

/// Convenience methods for CallbackQuery.
pub trait CallbackQueryHandlerExt {
    /// Returns the sender user for this callback query.
    fn sender(&self) -> &User;

    /// Returns the callback data, if any.
    fn callback_data(&self) -> Option<&str>;

    /// Returns the regular message associated with this callback query, if any.
    ///
    /// Returns `None` when the message is inaccessible to the bot.
    fn message(&self) -> Option<&Message>;
}

impl CallbackQueryHandlerExt for CallbackQuery {
    fn sender(&self) -> &User {
        &self.from
    }

    fn callback_data(&self) -> Option<&str> {
        self.data.as_deref()
    }

    fn message(&self) -> Option<&Message> {
        self.message.as_ref().and_then(|m| m.regular_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler;

    #[async_trait::async_trait]
    impl MessageHandler for TestHandler {
        async fn handle(&self, _bot: Bot, _msg: Message) -> HandlerResult {
            Ok(())
        }
        fn new() -> Self {
            Self
        }
    }

    #[test]
    fn test_handler_creation() {
        let _handler = TestHandler::new();
    }

    #[test]
    fn test_endpoint_creation() {
        let _endpoint = TestHandler::endpoint();
    }
}
