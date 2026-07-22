//! End-to-end tests for class-based handlers.

use teloxide_max::{
    handlers::{
        CallbackQueryHandler, CallbackQueryHandlerExt, ErrorHandler as ClassBasedErrorHandler,
        HandlerResult, MessageHandler, MessageHandlerExt,
    },
    types::{CallbackQuery, CallbackQueryId, Message, User, UserId},
    Bot,
};

struct TestMessageHandler;

#[async_trait::async_trait]
impl MessageHandler for TestMessageHandler {
    async fn handle(&self, _bot: Bot, msg: Message) -> HandlerResult {
        let _chat_id = msg.chat.id;
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

struct TestCallbackHandler;

#[async_trait::async_trait]
impl CallbackQueryHandler for TestCallbackHandler {
    async fn handle(&self, _bot: Bot, query: CallbackQuery) -> HandlerResult {
        let _query_id = &query.id;
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

struct TestErrorHandler;

#[async_trait::async_trait]
impl ClassBasedErrorHandler for TestErrorHandler {
    async fn handle_error(&self, error: Box<dyn std::error::Error + Send + Sync>) -> HandlerResult {
        log::error!("Error: {error}");
        Ok(())
    }

    fn new() -> Self {
        Self
    }
}

fn sample_message() -> Message {
    serde_json::from_value(serde_json::json!({
        "message_id": 1,
        "date": 1678886400,
        "chat": { "id": 123, "type": "private", "first_name": "Test" },
        "from": { "id": 456, "is_bot": false, "first_name": "Test" },
        "text": "Hello"
    }))
    .expect("valid message json")
}

fn sample_callback(data: Option<&str>) -> CallbackQuery {
    CallbackQuery {
        id: CallbackQueryId("test_query_id".into()),
        from: User {
            id: UserId(789),
            is_bot: false,
            first_name: "TestUser".into(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
            has_topics_enabled: false,
            allows_users_to_create_topics: false,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_guest_queries: None,
            supports_inline_queries: None,
            can_connect_to_business: None,
            has_main_web_app: None,
            can_manage_bots: None,
            supports_join_request_queries: None,
        },
        chat_instance: "test_instance".into(),
        data: data.map(str::to_string),
        game_short_name: None,
        message: None,
        inline_message_id: None,
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
    let msg = sample_message();
    assert_eq!(msg.chat().id.0, 123);
    assert!(msg.sender().is_some());
    assert_eq!(msg.sender().unwrap().id.0, 456);
    assert_eq!(msg.text(), Some("Hello"));
}

#[test]
fn test_callback_query_handler_ext_methods() {
    let query = sample_callback(Some("button:click"));
    assert_eq!(query.sender().id.0, 789);
    assert_eq!(query.callback_data(), Some("button:click"));
}

#[test]
fn test_message_handler_no_from_user() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1,
        "date": 1678886400,
        "chat": { "id": 123, "type": "private", "first_name": "Test" },
        "text": "Hello"
    }))
    .unwrap();
    assert!(msg.sender().is_none());
}

#[test]
fn test_callback_query_no_data() {
    let query = sample_callback(None);
    assert_eq!(query.callback_data(), None);
}
