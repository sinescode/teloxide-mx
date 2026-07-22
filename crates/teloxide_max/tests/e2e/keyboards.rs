//! End-to-end tests for Keyboard Builders.

use teloxide_max::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::keyboard::{InlineKeyboardBuilder, ReplyKeyboardBuilder},
};

#[test]
fn test_inline_keyboard_builder_empty() {
    let markup = InlineKeyboardBuilder::new().build();
    assert!(markup.inline_keyboard.is_empty());
}

#[test]
fn test_inline_keyboard_builder_single_button() {
    let markup = InlineKeyboardBuilder::new().callback_button("Click me", "action:click").build();
    assert_eq!(markup.inline_keyboard.len(), 1);
    assert_eq!(markup.inline_keyboard[0].len(), 1);
}

#[test]
fn test_inline_keyboard_builder_multiple_buttons() {
    let markup = InlineKeyboardBuilder::new()
        .callback_button("Btn1", "btn1")
        .callback_button("Btn2", "btn2")
        .callback_button("Btn3", "btn3")
        .build();
    assert_eq!(markup.inline_keyboard.len(), 1);
    assert_eq!(markup.inline_keyboard[0].len(), 3);
}

#[test]
fn test_inline_keyboard_builder_with_row() {
    let markup = InlineKeyboardBuilder::new()
        .callback_button("Btn1", "btn1")
        .row()
        .callback_button("Btn2", "btn2")
        .build();
    assert_eq!(markup.inline_keyboard.len(), 2);
}

#[test]
fn test_inline_keyboard_builder_adjust() {
    let markup = InlineKeyboardBuilder::new()
        .callback_button("1", "1")
        .callback_button("2", "2")
        .callback_button("3", "3")
        .callback_button("4", "4")
        .adjust(2)
        .build();
    assert_eq!(markup.inline_keyboard.len(), 2);
    assert_eq!(markup.inline_keyboard[0].len(), 2);
    assert_eq!(markup.inline_keyboard[1].len(), 2);
}

#[test]
fn test_inline_keyboard_builder_from_markup() {
    let original = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![InlineKeyboardButton::callback("test", "data")]],
    };
    let markup = InlineKeyboardBuilder::from_markup(original).build();
    assert_eq!(markup.inline_keyboard.len(), 1);
}

#[test]
fn test_reply_keyboard_builder_empty() {
    let markup = ReplyKeyboardBuilder::new().build();
    assert!(markup.keyboard.is_empty());
}

#[test]
fn test_reply_keyboard_builder_single_button() {
    let markup = ReplyKeyboardBuilder::new().button("Click me").build();
    assert_eq!(markup.keyboard.len(), 1);
    assert_eq!(markup.keyboard[0].len(), 1);
}

#[test]
fn test_reply_keyboard_builder_multiple_buttons() {
    let markup = ReplyKeyboardBuilder::new().button("Btn1").button("Btn2").button("Btn3").build();
    assert_eq!(markup.keyboard.len(), 1);
    assert_eq!(markup.keyboard[0].len(), 3);
}

#[test]
fn test_reply_keyboard_builder_adjust() {
    let markup = ReplyKeyboardBuilder::new()
        .button("1")
        .button("2")
        .button("3")
        .button("4")
        .adjust(2)
        .build();
    assert_eq!(markup.keyboard.len(), 2);
}

#[test]
fn test_reply_keyboard_options() {
    let markup = ReplyKeyboardBuilder::new()
        .button("OK")
        .resize_keyboard()
        .one_time_keyboard()
        .persistent()
        .build();
    assert_eq!(markup.keyboard.len(), 1);
    assert!(markup.resize_keyboard);
    assert!(markup.one_time_keyboard);
    assert!(markup.is_persistent);
}

#[test]
fn test_button_counts() {
    let inline = InlineKeyboardBuilder::new()
        .callback_button("a", "a")
        .callback_button("b", "b")
        .row()
        .callback_button("c", "c");
    assert_eq!(inline.button_count(), 3);
    assert_eq!(inline.row_count(), 2);

    let reply = ReplyKeyboardBuilder::new().button("x").button("y");
    assert_eq!(reply.button_count(), 2);
}

#[test]
fn test_build_markup() {
    let _ = InlineKeyboardBuilder::new().callback_button("a", "a").build_markup();
    let _ = ReplyKeyboardBuilder::new().button("a").build_markup();
}

#[test]
fn test_url_button() {
    let url = url::Url::parse("https://example.com").unwrap();
    let markup = InlineKeyboardBuilder::new().url_button("Go", url).build();
    assert_eq!(markup.inline_keyboard[0].len(), 1);
}

#[test]
fn test_repeat() {
    // `repeat(n)` duplicates the current layout `n` times (each copy as its own
    // row(s)).
    let markup = InlineKeyboardBuilder::new().callback_button("x", "x").repeat(3).build();
    assert_eq!(markup.inline_keyboard.len(), 3);
    assert_eq!(markup.inline_keyboard[0].len(), 1);
    assert_eq!(markup.inline_keyboard.iter().map(|r| r.len()).sum::<usize>(), 3);
}
