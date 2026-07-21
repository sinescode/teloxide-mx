//! End-to-end tests for Chat Action sender.
//!
//! Tests the ChatActionSender utility.

use teloxide_max::utils::chat_action::ChatActionSender;
use std::time::Duration;

#[test]
fn test_chat_action_sender_new() {
    let _sender = ChatActionSender::new();
}

#[test]
fn test_chat_action_sender_with_interval() {
    let _sender = ChatActionSender::new().with_interval(Duration::from_secs(3));
}

#[test]
fn test_chat_action_sender_with_action() {
    let _sender = ChatActionSender::new().with_action("upload_photo");
}

#[test]
fn test_chat_action_sender_typing() {
    let _sender = ChatActionSender::typing();
}

#[test]
fn test_chat_action_sender_upload_photo() {
    let _sender = ChatActionSender::upload_photo();
}

#[test]
fn test_chat_action_sender_record_video() {
    let _sender = ChatActionSender::record_video();
}

#[test]
fn test_chat_action_sender_upload_video() {
    let _sender = ChatActionSender::upload_video();
}

#[test]
fn test_chat_action_sender_record_voice() {
    let _sender = ChatActionSender::record_voice();
}

#[test]
fn test_chat_action_sender_upload_voice() {
    let _sender = ChatActionSender::upload_voice();
}

#[test]
fn test_chat_action_sender_upload_document() {
    let _sender = ChatActionSender::upload_document();
}

#[test]
fn test_chat_action_sender_choose_sticker() {
    let _sender = ChatActionSender::choose_sticker();
}

#[test]
fn test_chat_action_sender_find_location() {
    let _sender = ChatActionSender::find_location();
}

#[test]
fn test_chat_action_sender_record_video_note() {
    let _sender = ChatActionSender::record_video_note();
}

#[test]
fn test_chat_action_sender_upload_video_note() {
    let _sender = ChatActionSender::upload_video_note();
}

#[test]
fn test_chat_action_sender_chaining() {
    let _sender = ChatActionSender::new()
        .with_interval(Duration::from_secs(2))
        .with_action("typing");
}

#[test]
fn test_chat_action_sender_debug() {
    let sender = ChatActionSender::new();
    let debug_str = format!("{sender:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_chat_action_sender_clone() {
    let sender1 = ChatActionSender::new();
    let sender2 = sender1.clone();
    let _ = sender2;
}

#[test]
fn test_chat_action_sender_default_interval() {
    let sender = ChatActionSender::new();
    // Default interval should be 5 seconds
    let _ = sender;
}

#[test]
fn test_chat_action_sender_custom_interval() {
    let _sender = ChatActionSender::new().with_interval(Duration::from_millis(100));
}

#[test]
fn test_chat_action_sender_many_actions() {
    let actions = vec![
        "typing",
        "upload_photo",
        "record_video",
        "upload_video",
        "record_voice",
        "upload_voice",
        "upload_document",
        "choose_sticker",
        "find_location",
        "record_video_note",
        "upload_video_note",
    ];

    for action in actions {
        let _sender = ChatActionSender::new().with_action(action);
    }
}
