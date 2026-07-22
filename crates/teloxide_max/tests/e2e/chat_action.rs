//! End-to-end tests for Chat Action sender API surface.

use std::time::Duration;

use teloxide_max::{
    types::{ChatAction, ChatId},
    utils::chat_action::{
        ChatActionSender, ChatActionSenderConfig, DEFAULT_INITIAL_SLEEP, DEFAULT_INTERVAL,
    },
    Bot,
};

#[test]
fn test_default_interval() {
    assert_eq!(DEFAULT_INTERVAL, Duration::from_secs(5));
    assert_eq!(DEFAULT_INITIAL_SLEEP, Duration::from_secs(0));
}

#[test]
fn test_config_defaults() {
    let cfg = ChatActionSenderConfig::default();
    assert!(matches!(cfg.action, ChatAction::Typing));
    assert_eq!(cfg.interval, DEFAULT_INTERVAL);
    assert_eq!(cfg.initial_sleep, DEFAULT_INITIAL_SLEEP);
    assert!(cfg.message_thread_id.is_none());
}

#[test]
fn test_chat_action_variants() {
    // Ensure TBA chat actions used by aiogram parity helpers exist.
    let _ = ChatAction::Typing;
    let _ = ChatAction::UploadPhoto;
    let _ = ChatAction::RecordVideo;
    let _ = ChatAction::UploadVideo;
    let _ = ChatAction::RecordVoice;
    let _ = ChatAction::UploadVoice;
    let _ = ChatAction::UploadDocument;
    let _ = ChatAction::FindLocation;
    let _ = ChatAction::RecordVideoNote;
    let _ = ChatAction::UploadVideoNote;
}

#[tokio::test]
async fn test_chat_action_sender_stops() {
    let bot = Bot::new("1:test");
    let sender = ChatActionSender::with_interval(
        &bot,
        ChatId(1),
        ChatAction::Typing,
        Duration::from_millis(50),
    );
    // Immediately stop — should not hang.
    sender.stop();
}

#[tokio::test]
async fn test_chat_action_sender_drop() {
    let bot = Bot::new("1:test");
    {
        let _sender = ChatActionSender::new(&bot, ChatId(1), ChatAction::UploadPhoto);
        // Drop aborts the task.
    }
}

#[tokio::test]
async fn test_chat_action_typing_helper() {
    let bot = Bot::new("1:test");
    let sender = ChatActionSender::typing(&bot, ChatId(1));
    assert!(!sender.is_finished());
    sender.stop();
}

#[tokio::test]
async fn test_chat_action_with_config() {
    let bot = Bot::new("1:test");
    let sender = ChatActionSender::with_config(
        &bot,
        ChatId(1),
        ChatActionSenderConfig {
            action: ChatAction::UploadDocument,
            interval: Duration::from_millis(100),
            initial_sleep: Duration::from_millis(0),
            message_thread_id: None,
        },
    );
    sender.stop();
}
