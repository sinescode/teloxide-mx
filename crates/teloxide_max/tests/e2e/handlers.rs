//! End-to-end tests for Class-based handlers.
//!
//! Tests the MessageHandler, CallbackQueryHandler, and ErrorHandler traits.

use teloxide_max::handlers::{
    CallbackQueryHandler, CallbackQueryHandlerExt, ErrorHandler as ClassBasedErrorHandler,
    HandlerResult, MessageHandler, MessageHandlerExt,
};
use teloxide_max::types::{CallbackQuery, Chat, ChatId, Message, MessageId, User, UserId};
use teloxide_max::Bot;

// Test message handler
struct TestMessageHandler;

#[async_trait::async_trait]
impl MessageHandler for TestMessageHandler {
    async fn handle(&self, _bot: Bot, msg: Message) -> HandlerResult {
        // Just verify we can access the message
        let _chat_id = msg.chat.id;
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

// Test callback handler
struct TestCallbackHandler;

#[async_trait::async_trait]
impl CallbackQueryHandler for TestCallbackHandler {
    async fn handle(&self, _bot: Bot, query: CallbackQuery) -> HandlerResult {
        // Just verify we can access the callback query
        let _query_id = &query.id;
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

// Test error handler
struct TestErrorHandler;

#[async_trait::async_trait]
impl ClassBasedErrorHandler for TestErrorHandler {
    async fn handle_error(&self, error: Box<dyn std::error::Error + Send + Sync>) -> HandlerResult {
        // Log the error
        log::error!("Error: {error}");
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

#[test]
fn test_message_handler_creation() {
    let _handler = TestMessageHandler::new();
}

#[test]
fn test_callback_handler_creation() {
    let _handler = TestCallbackHandler::new();
}

#[test]
fn test_error_handler_creation() {
    let _handler = TestErrorHandler::new();
}

#[test]
fn test_message_handler_endpoint() {
    let _endpoint = TestMessageHandler::endpoint();
}

#[test]
fn test_callback_handler_endpoint() {
    let _endpoint = TestCallbackHandler::endpoint();
}

#[test]
fn test_message_handler_ext_methods() {
    // Create a test message
    let msg = Message {
        id: MessageId(1),
        date: 1678886400,
        chat: Chat {
            id: ChatId(123),
            ..Default::default()
        },
        from: Some(User {
            id: UserId(456),
            is_bot: false,
            first_name: "Test".into(),
            ..Default::default()
        }),
        text: Some("Hello".into()),
        ..Default::default()
    };

    // Test MessageHandlerExt methods
    assert_eq!(msg.chat().id.0, 123);
    assert!(msg.from_user().is_some());
    assert_eq!(msg.from_user().unwrap().id.0, 456);
    assert_eq!(msg.text(), Some("Hello"));
}

#[test]
fn test_callback_query_handler_ext_methods() {
    // Create a test callback query
    let query = CallbackQuery {
        id: "test_query_id".into(),
        from: Some(User {
            id: UserId(789),
            is_bot: false,
            first_name: "TestUser".into(),
            ..Default::default()
        }),
        chat_instance: "test_instance".into(),
        data: Some("button:click".into()),
        ..Default::default()
    };

    // Test CallbackQueryHandlerExt methods
    assert!(query.from_user().is_some());
    assert_eq!(query.from_user().unwrap().id.0, 789);
    assert_eq!(query.callback_data(), Some("button:click"));
}

#[test]
fn test_message_handler_no_from_user() {
    let msg = Message {
        id: MessageId(1),
        date: 1678886400,
        chat: Chat {
            id: ChatId(123),
            ..Default::default()
        },
        from: None,
        ..Default::default()
    };

    assert!(msg.from_user().is_none());
}

#[test]
fn test_callback_query_no_data() {
    let query = CallbackQuery {
        id: "test_query_id".into(),
        from: Some(User {
            id: UserId(789),
            is_bot: false,
            first_name: "TestUser".into(),
            ..Default::default()
        }),
        chat_instance: "test_instance".into(),
        data: None,
        ..Default::default()
    };

    assert_eq!(query.callback_data(), None);
}
