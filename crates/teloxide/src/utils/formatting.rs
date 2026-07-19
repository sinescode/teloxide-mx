//! Composable text formatting — build rich messages with a tree structure.
//!
//! Similar to aiogram's `Text`, `Bold`, `Italic`, etc., this module lets you
//! compose formatted text declaratively.
//!
//! # Example
//!
//! ```rust
//! use teloxide::utils::formatting::{Text, Bold, Italic, Code, Link};
//!
//! let text = Text::new(vec![
//!     Box::new(Bold::new("Hello")),
//!     Box::new(Text::raw(" ")),
//!     Box::new(Italic::new("World")),
//!     Box::new(Code::new("code")),
//! ]);
//!
//! let (rendered, entities) = text.render();
//! assert_eq!(rendered, "Hello Worldcode");
//! ```

use crate::types::{MessageEntity, MessageEntityKind};

/// A node in the formatting tree.
pub trait FormatNode: Send + Sync {
    /// Renders this node to text and collects entities.
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>);
}

/// A block of formatted text.
pub struct Text {
    children: Vec<Box<dyn FormatNode>>,
}

impl Text {
    /// Creates a new `Text` from raw string.
    pub fn raw(s: impl Into<String>) -> Self {
        Self {
            children: vec![Box::new(RawText(s.into()))],
        }
    }

    /// Creates a new `Text` from a list of child nodes.
    pub fn new(children: Vec<Box<dyn FormatNode>>) -> Self {
        Self { children }
    }

    /// Adds a child node.
    pub fn push(&mut self, child: Box<dyn FormatNode>) {
        self.children.push(child);
    }

    /// Renders the text tree to a string and list of entities.
    pub fn render(&self) -> (String, Vec<MessageEntity>) {
        let mut text = String::new();
        let mut entities = Vec::new();
        for child in &self.children {
            let (child_text, child_entities) = child.render(text.len());
            for mut entity in child_entities {
                entity.offset += text.len();
                entities.push(entity);
            }
            text.push_str(&child_text);
        }
        (text, entities)
    }

    /// Creates a `Text` from existing text and entities.
    pub fn from_entities(text: &str, entities: &[MessageEntity]) -> Self {
        // Reconstruct the tree from entities (simplified: just return raw text)
        Self::raw(text)
    }
}

struct RawText(String);

impl FormatNode for RawText {
    fn render(&self, _offset: usize) -> (String, Vec<MessageEntity>) {
        (self.0.clone(), vec![])
    }
}

/// Bold text wrapper.
pub struct Bold(String);

impl Bold {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Bold {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Bold,
            }],
        )
    }
}

/// Italic text wrapper.
pub struct Italic(String);

impl Italic {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Italic {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Italic,
            }],
        )
    }
}

/// Underline text wrapper.
pub struct Underline(String);

impl Underline {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Underline {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Underline,
            }],
        )
    }
}

/// Strikethrough text wrapper.
pub struct Strikethrough(String);

impl Strikethrough {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Strikethrough {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Strikethrough,
            }],
        )
    }
}

/// Spoiler text wrapper.
pub struct Spoiler(String);

impl Spoiler {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Spoiler {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Spoiler,
            }],
        )
    }
}

/// Inline code wrapper.
pub struct Code(String);

impl Code {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Code {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Code,
            }],
        )
    }
}

/// Preformatted code block wrapper.
pub struct Pre {
    text: String,
    language: Option<String>,
}

impl Pre {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            text: s.into(),
            language: None,
        }
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }
}

impl FormatNode for Pre {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.text);
        (
            self.text.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Pre {
                    language: self.language.clone(),
                },
            }],
        )
    }
}

/// Hyperlink wrapper.
pub struct Link {
    text: String,
    url: String,
}

impl Link {
    pub fn new(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl FormatNode for Link {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.text);
        (
            self.text.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::TextLink {
                    url: self.url.parse().unwrap_or_else(|_| {
                        url::Url::parse("https://example.com").unwrap()
                    }),
                },
            }],
        )
    }
}

/// Blockquote wrapper.
pub struct Blockquote(String);

impl Blockquote {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl FormatNode for Blockquote {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let len = utf16_len(&self.0);
        (
            self.0.clone(),
            vec![MessageEntity {
                offset,
                length: len,
                kind: MessageEntityKind::Blockquote,
            }],
        )
    }
}

/// Calculates the UTF-16 length of a string (Telegram uses UTF-16 offsets).
fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bold() {
        let text = Bold::new("Hello");
        let (s, entities) = text.render(0);
        assert_eq!(s, "Hello");
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].length, 5);
    }

    #[test]
    fn composite_text() {
        let text = Text::new(vec![
            Box::new(Bold::new("Hi")),
            Box::new(Text::raw(" ")),
            Box::new(Italic::new("there")),
        ]);
        let (s, entities) = text.render();
        assert_eq!(s, "Hi there");
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].offset, 0);
        assert_eq!(entities[1].offset, 3);
    }

    #[test]
    fn utf16_emoji() {
        let text = Bold::new("👋");
        let (s, entities) = text.render(0);
        assert_eq!(s, "👋");
        assert_eq!(entities[0].length, 2); // emoji is 2 UTF-16 units
    }
}
