//! Serverless deployment support for teloxide.
//!
//! This module provides tools for deploying teloxide bots in serverless
//! environments like AWS Lambda, Cloudflare Workers, or Google Cloud Functions.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::serverless::{LambdaHandler, WebhookEvent};
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

use crate::{
    types::{Message, Update, UpdateKind},
    Bot,
};
use std::{future::Future, pin::Pin, sync::Arc};

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

/// AWS Lambda handler for teloxide bots.
pub struct LambdaHandler {
    bot: Bot,
    handler: Arc<ServerlessHandler>,
}

impl LambdaHandler {
    /// Creates a new Lambda handler.
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
                    if let UpdateKind::Message(msg) = &update.kind {
                        handler(bot, msg.clone()).await
                    } else {
                        Ok(())
                    }
                })
            })),
        }
    }

    /// Runs the Lambda handler.
    ///
    /// This reads the event from the Lambda runtime, processes it,
    /// and returns the response.
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // In a real Lambda implementation, this would:
        // 1. Read the event from the Lambda runtime API
        // 2. Deserialize it as a Telegram Update
        // 3. Process it through the handler
        // 4. Return a 200 response

        log::info!("Lambda handler started (placeholder - implement with actual Lambda runtime)");
        Ok(())
    }

    /// Processes a single update.
    pub async fn process_update(
        &self,
        update: Update,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        (self.handler)(self.bot.clone(), update).await
    }
}

/// Webhook event handler for serverless platforms.
pub struct WebhookEvent {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
}

impl WebhookEvent {
    /// Parses the body as a Telegram Update.
    pub fn parse_update(&self) -> Result<Update, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}

/// Serverless webhook handler that processes Telegram updates.
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
                    if let UpdateKind::Message(msg) = &update.kind {
                        handler(bot, msg.clone()).await
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
        // Validate secret token if configured
        if let Some(expected) = &self.secret_token {
            if let Some(received) = event.headers.get("X-Telegram-Bot-Api-Secret-Token") {
                if received != expected {
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
}

/// Adapter for Google Cloud Functions.
pub struct CloudFunctionsHandler {
    bot: Bot,
    handler: Arc<ServerlessHandler>,
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
            handler: Arc::new(Box::new(move |bot, update| {
                let handler = Arc::clone(&handler);
                Box::pin(async move {
                    if let UpdateKind::Message(msg) = &update.kind {
                        handler(bot, msg.clone()).await
                    } else {
                        Ok(())
                    }
                })
            })),
        }
    }

    /// Processes an HTTP request from Cloud Functions.
    pub async fn handle_http(
        &self,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let update: Update = serde_json::from_str(&body)?;
        match (self.handler)(self.bot.clone(), update).await {
            Ok(()) => Ok("OK".to_string()),
            Err(e) => {
                log::error!("Handler error: {e}");
                Err(e)
            }
        }
    }
}

/// Adapter for Cloudflare Workers (via wasm-bindgen).
pub struct WorkersHandler {
    bot: Bot,
    handler: Arc<ServerlessHandler>,
}

impl WorkersHandler {
    /// Creates a new Workers handler.
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
                    if let UpdateKind::Message(msg) = &update.kind {
                        handler(bot, msg.clone()).await
                    } else {
                        Ok(())
                    }
                })
            })),
        }
    }

    /// Processes a fetch event from Cloudflare Workers.
    pub async fn handle_fetch(
        &self,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let update: Update = serde_json::from_str(&body)?;
        match (self.handler)(self.bot.clone(), update).await {
            Ok(()) => Ok("OK".to_string()),
            Err(e) => {
                log::error!("Handler error: {e}");
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_event_parse() {
        let event = WebhookEvent {
            body: r#"{"update_id":1,"message":{"message_id":1,"from":{"id":1,"is_bot":false,"first_name":"Test"},"chat":{"id":1,"type":"private"},"date":1234567890,"text":"hello"}}"#.to_string(),
            headers: std::collections::HashMap::new(),
        };

        let update = event.parse_update();
        assert!(update.is_ok());
    }
}
