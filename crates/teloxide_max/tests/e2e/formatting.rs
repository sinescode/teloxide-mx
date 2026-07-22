//! End-to-end tests for Formatting Builder.

use teloxide_max::utils::formatting::{
    Bold, Code, FormatNode, Italic, Pre, Spoiler, Strikethrough, Text, Underline,
};

#[test]
fn test_bold_render() {
    let (text, entities) = Bold::new("Hello").render(0);
    assert_eq!(text, "Hello");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_italic_render() {
    let (text, entities) = Italic::new("World").render(0);
    assert_eq!(text, "World");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_code_render() {
    let (text, entities) = Code::new("let x = 5;").render(0);
    assert_eq!(text, "let x = 5;");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_pre_render() {
    let (text, entities) = Pre::new("fn main() {}").render(0);
    assert_eq!(text, "fn main() {}");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_pre_with_language() {
    let (text, entities) = Pre::new("fn main() {}").language("rust").render(0);
    assert_eq!(text, "fn main() {}");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_underline_render() {
    let (text, entities) = Underline::new("Underlined").render(0);
    assert_eq!(text, "Underlined");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_strikethrough_render() {
    let (text, entities) = Strikethrough::new("Deleted").render(0);
    assert_eq!(text, "Deleted");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_spoiler_render() {
    let (text, entities) = Spoiler::new("Secret").render(0);
    assert_eq!(text, "Secret");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_text_render_empty() {
    let text = Text::new(Vec::new());
    let (content, entities) = text.render();
    assert!(content.is_empty());
    assert!(entities.is_empty());
}

#[test]
fn test_text_render_plain() {
    let text = Text::raw("Hello World");
    let (content, entities) = text.render();
    assert_eq!(content, "Hello World");
    assert!(entities.is_empty());
}

#[test]
fn test_text_render_bold() {
    let text = Text::new(vec![Box::new(Bold::new("Hello")) as Box<dyn FormatNode>]);
    let (content, entities) = text.render();
    assert_eq!(content, "Hello");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_text_composite() {
    let mut text = Text::raw("");
    text.push(Box::new(Bold::new("Hi")));
    text.push(Box::new(Italic::new(" there")));
    let (content, entities) = text.render();
    assert_eq!(content, "Hi there");
    assert_eq!(entities.len(), 2);
}

#[test]
fn test_from_entities() {
    use teloxide_max::types::{MessageEntity, MessageEntityKind};
    let text = Text::from_entities(
        "Hello",
        &[MessageEntity { offset: 0, length: 5, kind: MessageEntityKind::Bold }],
    );
    let (content, entities) = text.render();
    assert_eq!(content, "Hello");
    assert!(!entities.is_empty());
}
