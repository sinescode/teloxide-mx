//! E2E tests for ContentType classification and MagicData / logic filters.

use teloxide_max::{
    types::Message,
    utils::{
        content_type::{ContentType, MessageContentTypeExt},
        magic_data::{and_all, and_f, invert_f, magic_data, or_any, or_f},
    },
};

fn text_message(text: &str) -> Message {
    let json = serde_json::json!({
        "message_id": 1,
        "from": { "id": 1, "is_bot": false, "first_name": "Test" },
        "chat": { "id": 1, "type": "private", "first_name": "Test" },
        "date": 1_569_518_829_i64,
        "text": text,
    });
    serde_json::from_value(json).expect("text Message")
}

fn sticker_message() -> Message {
    let json = serde_json::json!({
        "message_id": 3,
        "from": { "id": 1, "is_bot": false, "first_name": "Test" },
        "chat": { "id": 1, "type": "private", "first_name": "Test" },
        "date": 1_569_518_829_i64,
        "sticker": {
            "width": 512,
            "height": 512,
            "emoji": "😡",
            "set_name": "AdvenTimeAnim",
            "is_animated": true,
            "is_video": false,
            "type": "regular",
            "file_id": "CAACAgIAAxkBAAESLdBjMImep-J0W8XaTN6S_Lz1-j1QIQACIwADsND4DGmmygHGlyggKQQ",
            "file_unique_id": "AgADIwADsND4DA",
            "file_size": 16639
        }
    });
    serde_json::from_value(json).expect("sticker Message")
}

#[test]
fn content_type_text() {
    let msg = text_message("hello");
    assert_eq!(msg.content_type(), ContentType::Text);
    assert_eq!(ContentType::of(&msg).as_str(), "text");
    assert!(ContentType::Text.predicate()(&msg));
}

#[test]
fn content_type_sticker() {
    let msg = sticker_message();
    assert_eq!(ContentType::of(&msg), ContentType::Sticker);
    assert_eq!(ContentType::Sticker.as_str(), "sticker");
}

#[test]
fn content_type_any_matches() {
    assert!(ContentType::Text.matches(ContentType::Any));
    assert!(ContentType::Any.matches(ContentType::Photo));
    assert!(!ContentType::Text.matches(ContentType::Photo));
}

#[test]
fn magic_data_logic() {
    assert!(magic_data(true));
    assert!(!magic_data(false));
    assert!(and_f(true, true));
    assert!(!and_f(true, false));
    assert!(or_f(false, true));
    assert!(!or_f(false, false));
    assert!(invert_f(false));
    assert!(and_all([true, true]));
    assert!(or_any([false, true]));
}
