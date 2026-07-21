//! End-to-end tests for Keyboard Builders.
//!
//! Tests InlineKeyboardBuilder and ReplyKeyboardBuilder.

use teloxide_max::utils::keyboard::{InlineKeyboardBuilder, ReplyKeyboardBuilder};
use teloxide_max::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
    ReplyKeyboardMarkup,
};

#[test]
fn test_inline_keyboard_builder_empty() {
    let builder = InlineKeyboardBuilder::new();
    let markup = builder.build();
    assert!(markup.inline_keyboard.is_empty());
}

#[test]
fn test_inline_keyboard_builder_single_button() {
    let mut builder = InlineKeyboardBuilder::new();
    builder.button("Click me", InlineKeyboardButton::callback("Click me", "action:click"));
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard.len(), 1);
    assert_eq!(markup.inline_keyboard[0].len(), 1);
}

#[test]
fn test_inline_keyboard_builder_multiple_buttons() {
    let mut builder = InlineKeyboardBuilder::new();
    builder.button("Btn1", InlineKeyboardButton::callback("Btn1", "btn1"));
    builder.button("Btn2", InlineKeyboardButton::callback("Btn2", "btn2"));
    builder.button("Btn3", InlineKeyboardButton::callback("Btn3", "btn3"));
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard.len(), 1);
    assert_eq!(markup.inline_keyboard[0].len(), 3);
}

#[test]
fn test_inline_keyboard_builder_with_row() {
    let mut builder = InlineKeyboardBuilder::new();
    builder.row();
    builder.button("Btn1", InlineKeyboardButton::callback("Btn1", "btn1"));
    builder.button("Btn2", InlineKeyboardButton::callback("Btn2", "btn2"));
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard.len(), 1); // row() on empty builder
}

#[test]
fn test_inline_keyboard_builder_adjust() {
    let mut builder = InlineKeyboardBuilder::new();
    builder.button("1", InlineKeyboardButton::callback("1", "1"));
    builder.button("2", InlineKeyboardButton::callback("2", "2"));
    builder.button("3", InlineKeyboardButton::callback("3", "3"));
    builder.button("4", InlineKeyboardButton::callback("4", "4"));
    builder.adjust([2, 2]);
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard.len(), 2);
    assert_eq!(markup.inline_keyboard[0].len(), 2);
    assert_eq!(markup.inline_keyboard[1].len(), 2);
}

#[test]
fn test_inline_keyboard_builder_from_markup() {
    let original = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![InlineKeyboardButton::callback("test", "data")]],
    };
    let builder = InlineKeyboardBuilder::from_markup(original);
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard.len(), 1);
}

#[test]
fn test_reply_keyboard_builder_empty() {
    let builder = ReplyKeyboardBuilder::new();
    let markup = builder.build();
    assert!(markup.keyboard.is_empty());
}

#[test]
fn test_reply_keyboard_builder_single_button() {
    let mut builder = ReplyKeyboardBuilder::new();
    builder.button(KeyboardButton::new("Click me"));
    let markup = builder.build();
    assert_eq!(markup.keyboard.len(), 1);
    assert_eq!(markup.keyboard[0].len(), 1);
}

#[test]
fn test_reply_keyboard_builder_multiple_buttons() {
    let mut builder = ReplyKeyboardBuilder::new();
    builder.button(KeyboardButton::new("Btn1"));
    builder.button(KeyboardButton::new("Btn2"));
    builder.button(KeyboardButton::new("Btn3"));
    let markup = builder.build();
    assert_eq!(markup.keyboard.len(), 1);
    assert_eq!(markup.keyboard[0].len(), 3);
}

#[test]
fn test_reply_keyboard_builder_adjust() {
    let mut builder = ReplyKeyboardBuilder::new();
    builder.button(KeyboardButton::new("1"));
    builder.button(KeyboardButton::new("2"));
    builder.button(KeyboardButton::new("3"));
    builder.button(KeyboardButton::new("4"));
    builder.adjust([2, 2]);
    let markup = builder.build();
    assert_eq!(markup.keyboard.len(), 2);
    assert_eq!(markup.keyboard[0].len(), 2);
    assert_eq!(markup.keyboard[1].len(), 2);
}

#[test]
fn test_reply_keyboard_builder_options() {
    let mut builder = ReplyKeyboardBuilder::new();
    builder.button(KeyboardButton::new("Test"));
    let markup = builder
        .resize_keyboard(true)
        .one_time_keyboard(true)
        .selective(true)
        .build();
    assert!(markup.resize_keyboard);
    assert!(markup.one_time_keyboard);
    assert!(markup.selective);
}

#[test]
fn test_reply_keyboard_builder_from_markup() {
    let original = ReplyKeyboardMarkup {
        keyboard: vec![vec![KeyboardButton::new("test")]],
        ..Default::default()
    };
    let builder = ReplyKeyboardBuilder::from_markup(original);
    let markup = builder.build();
    assert_eq!(markup.keyboard.len(), 1);
}

#[test]
fn test_inline_keyboard_url_button() {
    let mut builder = InlineKeyboardBuilder::new();
    builder.button("Visit", InlineKeyboardButton::url("Visit", "https://example.com"));
    let markup = builder.build();
    assert_eq!(markup.inline_keyboard[0][0].url, Some("https://example.com".into()));
}

#[test]
fn test_inline_keyboard_web_app_button() {
    use teloxide_max::types::WebAppInfo;
    let mut builder = InlineKeyboardBuilder::new();
    builder.button(
        "Open App",
        InlineKeyboardButton::web_app("Open App", WebAppInfo { url: "https://myapp.com".into() }),
    );
    let markup = builder.build();
    assert!(markup.inline_keyboard[0][0].web_app.is_some());
}

#[test]
fn test_reply_keyboard_request_contact() {
    let mut builder = ReplyKeyboardBuilder::new();
    let mut btn = KeyboardButton::new("Share Contact");
    btn.request_contact = true;
    builder.button(btn);
    let markup = builder.build();
    assert!(markup.keyboard[0][0].request_contact);
}

#[test]
fn test_reply_keyboard_request_location() {
    let mut builder = ReplyKeyboardBuilder::new();
    let mut btn = KeyboardButton::new("Share Location");
    btn.request_location = true;
    builder.button(btn);
    let markup = builder.build();
    assert!(markup.keyboard[0][0].request_location);
}
