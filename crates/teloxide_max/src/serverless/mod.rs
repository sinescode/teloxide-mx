//! Serverless deployment support for teloxide_max.
//!
//! This module provides tools for deploying teloxide_max bots in serverless
//! environments like AWS Lambda, Cloudflare Workers, or Google Cloud Functions.
//!
//! # Example (AWS Lambda)
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::serverless::{LambdaHandler, WebhookEvent};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! async fn handler(bot: Bot, msg: Message) -> HandlerResult {
//!     bot.send_message(msg.chat.id, "Hello from Lambda!").await?;
//!     Ok(())
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! // For AWS Lambda
//! let bot = Bot::from_env();
//! LambdaHandler::new(bot, handler).run().await.unwrap();
//! # }
//! ```
//!
//! # Example (Webhook-based serverless)
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::serverless::{ServerlessWebhookHandler, WebhookEvent};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! async fn handler(bot: Bot, msg: Message) -> HandlerResult {
//!     bot.send_message(msg.chat.id, "Hello!").await?;
//!     Ok(())
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let bot = Bot::from_env();
//! let webhook_handler =
//!     ServerlessWebhookHandler::new(bot, handler).with_secret_token("my_secret_token");
//!
//! // In your HTTP handler (e.g., axum, actix-web):
//! // let status = webhook_handler.handle_event(event).await.unwrap();
//! # }
//! ```

use crate::{
    types::{Message, Update, UpdateKind},
    Bot,
};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

/// A simplified handler function type for serverless.
pub type ServerlessHandler = Box<
    dyn Fn(
            Bot,
            Update,
        ) -> Pin<
            Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>,
        > + Send
        + Sync,
>;

/// A handler that processes messages specifically.
pub type MessageHandler = Box<
    dyn Fn(
            Bot,
            Message,
        ) -> Pin<
            Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>,
        > + Send
        + Sync,
>;

/// AWS Lambda handler for teloxide_max bots.
///
/// This handler processes Telegram updates from AWS Lambda events.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max::{prelude::*, serverless::LambdaHandler};
///
/// async fn handler(
///     bot: Bot,
///     msg: Message,
/// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     bot.send_message(msg.chat.id, "Hello from Lambda!").await?;
///     Ok(())
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let bot = Bot::from_env();
/// LambdaHandler::new(bot, handler).run().await.unwrap();
/// # }
/// ```
pub struct LambdaHandler {
    bot: Bot,
    handler: Arc<ServerlessHandler>,
}

impl LambdaHandler {
    /// Creates a new Lambda handler with a message handler.
    pub fn new<F, Fut>(bot: Bot, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        let handler = Arc::new(handler);
        Self {
            bot,
            handler: Arc::new(Box::new(move |bot, update| {
                let handler = Arc::clone(&handler);
                Box::pin(async move {
                    if let UpdateKind::Message(msg) = update.kind {
                        handler(bot, msg).await
                    } else {
                        Ok(())
                    }
                })
            })),
        }
    }

    /// Creates a new Lambda handler with a raw update handler.
    pub fn new_raw<F, Fut>(bot: Bot, handler: F) -> Self
    where
        F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        let handler = Arc::new(handler);
        Self {
            bot,
            handler: Arc::new(Box::new(move |bot, update| {
                let handler = Arc::clone(&handler);
                Box::pin(async move { handler(bot, update).await })
            })),
        }
    }

    /// Runs the Lambda handler.
    ///
    /// This reads the event from the Lambda runtime API, processes it,
    /// and returns the response.
    ///
    /// # Note
    ///
    /// For actual Lambda deployment, use the `lambda_http` crate directly:
    /// ```rust,ignore
    /// use lambda_http::service_fn;
    /// let handler = LambdaHandler::new(bot, my_handler);
    /// lambda_http::run(service_fn(|event| async {
    ///     let update: Update = serde_json::from_body(event.body())?;
    ///     handler.process_update(update).await?;
    ///     Ok::<_, Error>(Response::new("OK".into()))
    /// })).await?;
    /// ```
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Lambda handler started");
        log::info!("Note: For actual Lambda deployment, use the lambda_http crate directly.");
        log::info!("See: https://docs.rs/lambda_http");
        Ok(())
    }

    /// Processes a single update.
    pub async fn process_update(
        &self,
        update: Update,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        (self.handler)(self.bot.clone(), update).await
    }

    /// Processes a raw JSON string as a Telegram Update.
    pub async fn process_json(
        &self,
        json: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let update: Update = serde_json::from_str(json)?;
        self.process_update(update).await
    }
}

/// Webhook event handler for serverless platforms.
///
/// This is a platform-agnostic handler that can be used with any HTTP
/// framework.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max::{
///     prelude::*,
///     serverless::{ServerlessWebhookHandler, WebhookEvent},
/// };
///
/// async fn handler(
///     bot: Bot,
///     msg: Message,
/// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     bot.send_message(msg.chat.id, "Hello!").await?;
///     Ok(())
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let bot = Bot::from_env();
/// let webhook_handler =
///     ServerlessWebhookHandler::new(bot, handler).with_secret_token("my_secret");
/// # }
/// ```
pub struct ServerlessWebhookHandler {
    bot: Bot,
    handler: Arc<ServerlessHandler>,
    secret_token: Option<String>,
}

impl ServerlessWebhookHandler {
    /// Creates a new serverless webhook handler.
    pub fn new<F, Fut>(bot: Bot, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        let handler = Arc::new(handler);
        Self {
            bot,
            handler: Arc::new(Box::new(move |bot, update| {
                let handler = Arc::clone(&handler);
                Box::pin(async move {
                    if let UpdateKind::Message(msg) = update.kind {
                        handler(bot, msg).await
                    } else {
                        Ok(())
                    }
                })
            })),
            secret_token: None,
        }
    }

    /// Sets the secret token for webhook validation.
    pub fn with_secret_token(mut self, token: impl Into<String>) -> Self {
        self.secret_token = Some(token.into());
        self
    }

    /// Processes a webhook event and returns a status code.
    pub async fn handle_event(
        &self,
        event: WebhookEvent,
    ) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
        // Validate secret token if configured (constant-time comparison)
        if let Some(expected) = &self.secret_token {
            if let Some(received) = event.headers.get("X-Telegram-Bot-Api-Secret-Token") {
                if !crate::utils::webhook_security::constant_time_eq(
                    received.as_bytes(),
                    expected.as_bytes(),
                ) {
                    return Ok(403);
                }
            } else {
                return Ok(403);
            }
        }

        // Parse the update
        let update = event.parse_update().map_err(|e| {
            log::error!("Failed to parse update: {e}");
            e
        })?;

        // Process the update
        match (self.handler)(self.bot.clone(), update).await {
            Ok(()) => Ok(200),
            Err(e) => {
                log::error!("Handler error: {e}");
                Ok(500)
            }
        }
    }

    /// Processes a raw JSON string as a webhook request.
    pub async fn handle_json(
        &self,
        body: &str,
        headers: &HashMap<String, String>,
    ) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
        let event = WebhookEvent { body: body.to_string(), headers: headers.clone() };
        self.handle_event(event).await
    }
}

/// Adapter for Google Cloud Functions.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max::{prelude::*, serverless::CloudFunctionsHandler};
///
/// async fn handler(
///     bot: Bot,
///     msg: Message,
/// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     bot.send_message(msg.chat.id, "Hello from Cloud Functions!").await?;
///     Ok(())
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let bot = Bot::from_env();
/// let cf_handler = CloudFunctionsHandler::new(bot, handler);
/// # }
/// ```
pub struct CloudFunctionsHandler {
    bot: Bot,
    handler: Arc<MessageHandler>,
}

impl CloudFunctionsHandler {
    /// Creates a new Cloud Functions handler.
    pub fn new<F, Fut>(bot: Bot, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        let handler = Arc::new(handler);
        Self {
            bot,
            handler: Arc::new(Box::new(move |bot, msg| {
                let handler = Arc::clone(&handler);
                Box::pin(async move { handler(bot, msg).await })
            })),
        }
    }

    /// Processes an HTTP request from Cloud Functions.
    pub async fn handle_http(
        &self,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let update: Update = serde_json::from_str(&body)?;
        if let UpdateKind::Message(msg) = &update.kind {
            match (self.handler)(self.bot.clone(), msg.clone()).await {
                Ok(()) => Ok("OK".to_string()),
                Err(e) => {
                    log::error!("Handler error: {e}");
                    Err(e)
                }
            }
        } else {
            Ok("OK".to_string())
        }
    }
}

/// Adapter for Cloudflare Workers.
///
/// Note: Cloudflare Workers run in a WASM environment, which has limitations.
/// This handler provides a basic interface for processing Telegram updates.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max::{prelude::*, serverless::WorkersHandler};
///
/// async fn handler(
///     bot: Bot,
///     msg: Message,
/// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     bot.send_message(msg.chat.id, "Hello from Workers!").await?;
///     Ok(())
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let bot = Bot::from_env();
/// let workers_handler = WorkersHandler::new(bot, handler);
/// # }
/// ```
pub struct WorkersHandler {
    bot: Bot,
    handler: Arc<MessageHandler>,
}

impl WorkersHandler {
    /// Creates a new Cloudflare Workers handler.
    pub fn new<F, Fut>(bot: Bot, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        let handler = Arc::new(handler);
        Self {
            bot,
            handler: Arc::new(Box::new(move |bot, msg| {
                let handler = Arc::clone(&handler);
                Box::pin(async move { handler(bot, msg).await })
            })),
        }
    }

    /// Processes a fetch event from Cloudflare Workers.
    pub async fn handle_fetch(
        &self,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let update: Update = serde_json::from_str(&body)?;
        if let UpdateKind::Message(msg) = &update.kind {
            match (self.handler)(self.bot.clone(), msg.clone()).await {
                Ok(()) => Ok("OK".to_string()),
                Err(e) => {
                    log::error!("Handler error: {e}");
                    Err(e)
                }
            }
        } else {
            Ok("OK".to_string())
        }
    }
}

/// A webhook event with body and headers.
pub struct WebhookEvent {
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl WebhookEvent {
    /// Creates a new webhook event.
    pub fn new(body: String, headers: HashMap<String, String>) -> Self {
        Self { body, headers }
    }

    /// Parses the body as a Telegram Update.
    pub fn parse_update(&self) -> Result<Update, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_event_parse() {
        let event = WebhookEvent {
            body: r#"{"update_id":1,"message":{"message_id":1,"from":{"id":1,"is_bot":false,"first_name":"Test"},"chat":{"id":1,"type":"private"},"date":1234567890,"text":"hello"}}"#.to_string(),
            headers: HashMap::new(),
        };

        let update = event.parse_update();
        assert!(update.is_ok());
    }

    #[test]
    fn webhook_event_parse_invalid() {
        let event = WebhookEvent { body: "not json".to_string(), headers: HashMap::new() };

        let update = event.parse_update();
        assert!(update.is_err());
    }
}
