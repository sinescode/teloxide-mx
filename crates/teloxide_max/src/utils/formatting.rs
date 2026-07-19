//! Composable text formatting — build rich messages with a tree structure.
//!
//! Similar to aiogram's `Text`, `Bold`, `Italic`, etc., this module lets you
//! compose formatted text declaratively.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::utils::formatting::{Bold, Code, Italic, Link, Text};
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
        Self { children: vec![Box::new(RawText(s.into()))] }
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
            entities.extend(child_entities);
            text.push_str(&child_text);
        }
        (text, entities)
    }

    /// Creates a `Text` from existing text and entities.
    ///
    /// Overlapping entities are nested properly in the resulting tree.
    pub fn from_entities(text: &str, entities: &[MessageEntity]) -> Self {
        if entities.is_empty() {
            return Self::raw(text);
        }

        let text_len = text.len();

        // Filter out zero-length or out-of-bounds entities
        let valid: Vec<&MessageEntity> =
            entities.iter().filter(|e| e.length > 0 && e.offset < text_len).collect();

        if valid.is_empty() {
            return Self::raw(text);
        }

        // Sort by offset, then by descending length (outermost first)
        let mut sorted = valid;
        sorted.sort_by(|a, b| a.offset.cmp(&b.offset).then_with(|| b.length.cmp(&a.length)));

        // Build events: (position, is_start, index_into_sorted)
        let mut events: Vec<(usize, bool, usize)> = Vec::new();
        for (i, e) in sorted.iter().enumerate() {
            let end = (e.offset + e.length).min(text_len);
            events.push((e.offset, true, i));
            events.push((end, false, i));
        }
        // At same position: ends before starts
        events.sort_by(|a, b| {
            a.0.cmp(&b.0).then_with(|| match (a.1, b.1) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            })
        });

        let mut children: Vec<Box<dyn FormatNode>> = Vec::new();
        let mut active: Vec<usize> = Vec::new();
        let mut last_pos = 0;
        let mut i = 0;

        while i < events.len() {
            let pos = events[i].0;

            // Emit text segment from last_pos to pos
            if pos > last_pos && last_pos < text_len {
                let end = pos.min(text_len);
                let segment = &text[last_pos..end];
                children.push(build_nested_node(segment, &active, &sorted));
            }

            // Process all events at this position
            while i < events.len() && events[i].0 == pos {
                let (_, is_start, idx) = events[i];
                if is_start {
                    active.push(idx);
                } else {
                    active.retain(|&j| j != idx);
                }
                i += 1;
            }

            last_pos = pos;
        }

        // Emit remaining text
        if last_pos < text_len {
            children.push(Box::new(RawText(text[last_pos..].to_string())));
        }

        Self::new(children)
    }
}

impl FormatNode for Text {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let mut text = String::new();
        let mut entities = Vec::new();
        for child in &self.children {
            let (child_text, child_entities) = child.render(offset + text.len());
            entities.extend(child_entities);
            text.push_str(&child_text);
        }
        (text, entities)
    }
}

struct RawText(String);

impl FormatNode for RawText {
    fn render(&self, _offset: usize) -> (String, Vec<MessageEntity>) {
        (self.0.clone(), vec![])
    }
}

/// A node that wraps text with a single entity kind, containing an optional
/// inner node. Used for nesting overlapping entities from `from_entities`.
struct NestedFormat {
    text: String,
    kind: MessageEntityKind,
    inner: Box<dyn FormatNode>,
}

impl FormatNode for NestedFormat {
    fn render(&self, offset: usize) -> (String, Vec<MessageEntity>) {
        let (_inner_text, mut inner_entities) = self.inner.render(offset);
        let len = utf16_len(&self.text);
        inner_entities.push(MessageEntity { offset, length: len, kind: self.kind.clone() });
        (self.text.clone(), inner_entities)
    }
}

/// Builds a nested node tree from a text segment and the active entity indices.
fn build_nested_node(
    text: &str,
    active: &[usize],
    sorted: &[&MessageEntity],
) -> Box<dyn FormatNode> {
    if active.is_empty() {
        return Box::new(RawText(text.to_string()));
    }

    // active[0] is outermost (started first), active[last] is innermost.
    // Wrap from innermost to outermost.
    let mut node: Box<dyn FormatNode> = Box::new(RawText(text.to_string()));
    for &idx in active.iter().rev() {
        node = Box::new(NestedFormat {
            text: text.to_string(),
            kind: sorted[idx].kind.clone(),
            inner: node,
        });
    }
    node
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
        (self.0.clone(), vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Bold }])
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
            vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Italic }],
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
            vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Underline }],
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
            vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Strikethrough }],
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
            vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Spoiler }],
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
        (self.0.clone(), vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Code }])
    }
}

/// Preformatted code block wrapper.
pub struct Pre {
    text: String,
    language: Option<String>,
}

impl Pre {
    pub fn new(s: impl Into<String>) -> Self {
        Self { text: s.into(), language: None }
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
                kind: MessageEntityKind::Pre { language: self.language.clone() },
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
        Self { text: text.into(), url: url.into() }
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
                    url: self
                        .url
                        .parse()
                        .unwrap_or_else(|_| url::Url::parse("https://example.com").unwrap()),
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
            vec![MessageEntity { offset, length: len, kind: MessageEntityKind::Blockquote }],
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

    #[test]
    fn from_entities_single_bold() {
        let text = "Hello World";
        let entities = vec![MessageEntity::bold(0, 5)];
        let t = Text::from_entities(text, &entities);
        let (rendered, ents) = t.render();
        assert_eq!(rendered, "Hello World");
        assert_eq!(ents.len(), 1);
        assert_eq!(ents[0].kind, MessageEntityKind::Bold);
        assert_eq!(ents[0].offset, 0);
        assert_eq!(ents[0].length, 5);
    }

    #[test]
    fn from_entities_non_overlapping() {
        let text = "Hello World";
        let entities = vec![MessageEntity::bold(0, 5), MessageEntity::italic(6, 5)];
        let t = Text::from_entities(text, &entities);
        let (rendered, ents) = t.render();
        assert_eq!(rendered, "Hello World");
        assert_eq!(ents.len(), 2);
        assert_eq!(ents[0].kind, MessageEntityKind::Bold);
        assert_eq!(ents[1].kind, MessageEntityKind::Italic);
    }

    #[test]
    fn from_entities_overlapping() {
        let text = "Hello World";
        // Bold covers [0,10), Italic covers [3,8)
        let entities = vec![MessageEntity::bold(0, 10), MessageEntity::italic(3, 5)];
        let t = Text::from_entities(text, &entities);
        let (rendered, ents) = t.render();
        assert_eq!(rendered, "Hello World");
        // Should have entities for: Bold [0,3), Bold+Italic [3,8), Bold [8,10)
        assert!(ents.iter().any(|e| e.kind == MessageEntityKind::Italic));
        assert!(ents.iter().filter(|e| e.kind == MessageEntityKind::Bold).count() >= 2);
    }

    #[test]
    fn from_entities_same_range() {
        let text = "Hello";
        let entities = vec![MessageEntity::bold(0, 5), MessageEntity::italic(0, 5)];
        let t = Text::from_entities(text, &entities);
        let (rendered, ents) = t.render();
        assert_eq!(rendered, "Hello");
        assert_eq!(ents.len(), 2);
    }

    #[test]
    fn from_entities_empty() {
        let t = Text::from_entities("Hello", &[]);
        let (rendered, ents) = t.render();
        assert_eq!(rendered, "Hello");
        assert!(ents.is_empty());
    }
}
