//! End-to-end tests for Formatting Builder.
//!
//! Tests the composable text formatting system.

use teloxide_max::utils::formatting::{
    as_list, as_marked_list, as_numbered_list, Bold, Code, Italic, Pre, Spoiler, Strikethrough,
    Text, Underline,
};

#[test]
fn test_bold_creation() {
    let bold = Bold::new("Hello");
    assert_eq!(bold.text(), "Hello");
}

#[test]
fn test_italic_creation() {
    let italic = Italic::new("World");
    assert_eq!(italic.text(), "World");
}

#[test]
fn test_code_creation() {
    let code = Code::new("let x = 5;");
    assert_eq!(code.text(), "let x = 5;");
}

#[test]
fn test_pre_creation() {
    let pre = Pre::new("fn main() {}");
    assert_eq!(pre.text(), "fn main() {}");
}

#[test]
fn test_pre_with_language() {
    let pre = Pre::language("fn main() {}", "rust");
    assert_eq!(pre.text(), "fn main() {}");
    assert_eq!(pre.language(), Some("rust"));
}

#[test]
fn test_underline_creation() {
    let underline = Underline::new("Underlined");
    assert_eq!(underline.text(), "Underlined");
}

#[test]
fn test_strikethrough_creation() {
    let strikethrough = Strikethrough::new("Deleted");
    assert_eq!(strikethrough.text(), "Deleted");
}

#[test]
fn test_spoiler_creation() {
    let spoiler = Spoiler::new("Secret");
    assert_eq!(spoiler.text(), "Secret");
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
    let text = Text::new(vec!["Hello World".into()]);
    let (content, entities) = text.render();
    assert_eq!(content, "Hello World");
    assert!(entities.is_empty());
}

#[test]
fn test_text_render_bold() {
    let text = Text::new(vec![Bold::new("Hello").into()]);
    let (content, entities) = text.render();
    assert_eq!(content, "Hello");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_text_render_mixed() {
    let text = Text::new(vec![
        Bold::new("Name: ").into(),
        "Alice".into(),
        "\n".into(),
        Code::new("Age: ").into(),
        "25".into(),
    ]);
    let (content, _entities) = text.render();
    assert!(content.contains("Name:"));
    assert!(content.contains("Alice"));
    assert!(content.contains("Age:"));
}

#[test]
fn test_text_as_html() {
    let text = Text::new(vec![Bold::new("Hello").into(), " ".into(), Italic::new("World").into()]);
    let html = text.as_html();
    assert!(html.contains("<b>Hello</b>"));
    assert!(html.contains("<i>World</i>"));
}

#[test]
fn test_text_as_markdown() {
    let text = Text::new(vec![Bold::new("Hello").into(), " ".into(), Italic::new("World").into()]);
    let md = text.as_markdown();
    assert!(md.contains("**Hello**"));
    assert!(md.contains("_World_"));
}

#[test]
fn test_as_list() {
    let text = as_list(vec![
        Bold::new("Item 1").into(),
        Italic::new("Item 2").into(),
        Code::new("Item 3").into(),
    ]);
    let (content, _) = text.render();
    assert!(content.contains("Item 1"));
    assert!(content.contains("Item 2"));
    assert!(content.contains("Item 3"));
}

#[test]
fn test_as_marked_list() {
    let text = as_marked_list(vec!["A", "B", "C"]);
    let (content, _) = text.render();
    assert!(content.contains("- A"));
    assert!(content.contains("- B"));
    assert!(content.contains("- C"));
}

#[test]
fn test_as_numbered_list() {
    let text = as_numbered_list(vec!["First", "Second", "Third"]);
    let (content, _) = text.render();
    assert!(content.contains("1. First"));
    assert!(content.contains("2. Second"));
    assert!(content.contains("3. Third"));
}

#[test]
fn test_text_from_entities() {
    let text = Text::from_entities("Hello World", &[]);
    let (content, _) = text.render();
    assert_eq!(content, "Hello World");
}

#[test]
fn test_nested_formatting() {
    let text = Text::new(vec![
        Bold::new(vec![Italic::new("nested").into()]).into(),
    ]);
    let (content, _) = text.render();
    assert!(content.contains("nested"));
}

#[test]
fn test_formatting_with_link() {
    use teloxide_max::utils::formatting::Link;
    let link = Link::new("Click here", "https://example.com");
    let text = Text::new(vec![link.into()]);
    let (content, entities) = text.render();
    assert_eq!(content, "Click here");
    assert_eq!(entities.len(), 1);
}

#[test]
fn test_formatting_display() {
    let text = Text::new(vec![Bold::new("Test").into()]);
    let display = format!("{text}");
    assert_eq!(display, "Test");
}
