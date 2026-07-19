use crate::types::{
    CallbackGame, CopyTextButton, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton,
    KeyboardMarkup, LoginUrl, ReplyMarkup, WebAppInfo,
};

// ============================================================================
// InlineKeyboardBuilder
// ============================================================================

/// A builder for constructing [`InlineKeyboardMarkup`] with a fluent API.
///
/// # Example
///
/// ```rust
/// use teloxide_max::{types::InlineKeyboardButton, utils::keyboard::InlineKeyboardBuilder};
///
/// let keyboard = InlineKeyboardBuilder::new()
///     .callback_button("Button 1", "btn1")
///     .url_button("Button 2", url::Url::parse("https://example.com").unwrap())
///     .row()
///     .callback_button("Button 3", "btn3")
///     .build();
/// ```
pub struct InlineKeyboardBuilder {
    rows: Vec<Vec<InlineKeyboardButton>>,
    current_row: Vec<InlineKeyboardButton>,
}

impl InlineKeyboardBuilder {
    /// Creates a new empty `InlineKeyboardBuilder`.
    pub fn new() -> Self {
        Self { rows: Vec::new(), current_row: Vec::new() }
    }

    /// Creates a new builder from an existing markup.
    pub fn from_markup(mut markup: InlineKeyboardMarkup) -> Self {
        let current_row = markup.inline_keyboard.pop().unwrap_or_default();
        Self { rows: markup.inline_keyboard, current_row }
    }

    /// Adds a button to the current row.
    pub fn button(mut self, button: InlineKeyboardButton) -> Self {
        self.current_row.push(button);
        self
    }

    /// Adds a URL button to the current row.
    pub fn url_button<S>(self, text: S, url: url::Url) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::url(text, url))
    }

    /// Adds a callback data button to the current row.
    pub fn callback_button<S, C>(self, text: S, callback_data: C) -> Self
    where
        S: Into<String>,
        C: Into<String>,
    {
        self.button(InlineKeyboardButton::callback(text, callback_data))
    }

    /// Adds a web app button to the current row.
    pub fn web_app_button<S>(self, text: S, info: WebAppInfo) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::web_app(text, info))
    }

    /// Adds a login URL button to the current row.
    pub fn login_button<S>(self, text: S, url: LoginUrl) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::login(text, url))
    }

    /// Adds a switch inline query button to the current row.
    pub fn switch_inline_query_button<S, Q>(self, text: S, query: Q) -> Self
    where
        S: Into<String>,
        Q: Into<String>,
    {
        self.button(InlineKeyboardButton::switch_inline_query(text, query))
    }

    /// Adds a switch inline query current chat button to the current row.
    pub fn switch_inline_query_current_chat_button<S, Q>(self, text: S, query: Q) -> Self
    where
        S: Into<String>,
        Q: Into<String>,
    {
        self.button(InlineKeyboardButton::switch_inline_query_current_chat(text, query))
    }

    /// Adds a copy text button to the current row.
    pub fn copy_text_button<S>(self, text: S, copy_text: CopyTextButton) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::copy_text_button(text, copy_text))
    }

    /// Adds a callback game button to the current row.
    pub fn callback_game_button<S>(self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::callback_game(text, CallbackGame {}))
    }

    /// Adds a pay button to the current row.
    pub fn pay_button<S>(self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.button(InlineKeyboardButton::pay(text))
    }

    /// Starts a new row.
    pub fn row(mut self) -> Self {
        if !self.current_row.is_empty() {
            self.rows.push(std::mem::take(&mut self.current_row));
        }
        self
    }

    /// Adjusts buttons to have `per_row` buttons per row.
    ///
    /// This reorganizes all buttons (including those in existing rows)
    /// to have exactly `per_row` buttons per row.
    pub fn adjust(mut self, per_row: usize) -> Self {
        // Collect all buttons
        let mut all_buttons: Vec<InlineKeyboardButton> =
            self.rows.iter().flatten().cloned().chain(self.current_row.iter().cloned()).collect();

        // Reorganize into rows of per_row
        self.rows.clear();
        self.current_row.clear();

        while !all_buttons.is_empty() {
            let row: Vec<_> = all_buttons.drain(..per_row.min(all_buttons.len())).collect();
            self.rows.push(row);
        }

        self
    }

    /// Repeats the current button pattern `n` times.
    ///
    /// This takes all current buttons and repeats them `n` times,
    /// maintaining the row structure.
    pub fn repeat(self, n: usize) -> Self {
        let original_rows: Vec<Vec<InlineKeyboardButton>> = self
            .rows
            .into_iter()
            .chain(if self.current_row.is_empty() { None } else { Some(self.current_row) })
            .collect();

        let mut builder = Self::new();
        for _ in 0..n {
            for row in &original_rows {
                for button in row {
                    builder = builder.button(button.clone());
                }
                builder = builder.row();
            }
        }
        builder
    }

    /// Returns the number of buttons in the builder.
    pub fn button_count(&self) -> usize {
        self.rows.iter().map(|r| r.len()).sum::<usize>() + self.current_row.len()
    }

    /// Returns the number of rows.
    pub fn row_count(&self) -> usize {
        self.rows.len() + if self.current_row.is_empty() { 0 } else { 1 }
    }

    /// Builds the final [`InlineKeyboardMarkup`].
    pub fn build(mut self) -> InlineKeyboardMarkup {
        if !self.current_row.is_empty() {
            self.rows.push(self.current_row);
        }
        InlineKeyboardMarkup::new(self.rows)
    }

    /// Builds the final [`ReplyMarkup`].
    pub fn build_markup(self) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(self.build())
    }
}

impl Default for InlineKeyboardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ReplyKeyboardBuilder
// ============================================================================

/// A builder for constructing [`KeyboardMarkup`] with a fluent API.
///
/// # Example
///
/// ```rust
/// use teloxide_max::utils::keyboard::ReplyKeyboardBuilder;
///
/// let keyboard = ReplyKeyboardBuilder::new()
///     .button("Button 1")
///     .button("Button 2")
///     .row()
///     .button("Button 3")
///     .resize_keyboard()
///     .one_time_keyboard()
///     .build();
/// ```
pub struct ReplyKeyboardBuilder {
    rows: Vec<Vec<KeyboardButton>>,
    current_row: Vec<KeyboardButton>,
    is_persistent: bool,
    resize_keyboard: bool,
    one_time_keyboard: bool,
    input_field_placeholder: Option<String>,
    selective: bool,
}

impl ReplyKeyboardBuilder {
    /// Creates a new empty `ReplyKeyboardBuilder`.
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            current_row: Vec::new(),
            is_persistent: false,
            resize_keyboard: false,
            one_time_keyboard: false,
            input_field_placeholder: None,
            selective: false,
        }
    }

    /// Creates a new builder from an existing markup.
    pub fn from_markup(markup: KeyboardMarkup) -> Self {
        Self {
            rows: markup.keyboard,
            current_row: Vec::new(),
            is_persistent: markup.is_persistent,
            resize_keyboard: markup.resize_keyboard,
            one_time_keyboard: markup.one_time_keyboard,
            input_field_placeholder: if markup.input_field_placeholder.is_empty() {
                None
            } else {
                Some(markup.input_field_placeholder)
            },
            selective: markup.selective,
        }
    }

    /// Adds a text button to the current row.
    pub fn button<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.current_row.push(KeyboardButton::new(text));
        self
    }

    /// Adds a [`KeyboardButton`] to the current row.
    pub fn add_button(mut self, button: KeyboardButton) -> Self {
        self.current_row.push(button);
        self
    }

    /// Starts a new row.
    pub fn row(mut self) -> Self {
        if !self.current_row.is_empty() {
            self.rows.push(std::mem::take(&mut self.current_row));
        }
        self
    }

    /// Adjusts buttons to have `per_row` buttons per row.
    pub fn adjust(mut self, per_row: usize) -> Self {
        let mut all_buttons: Vec<KeyboardButton> =
            self.rows.iter().flatten().cloned().chain(self.current_row.iter().cloned()).collect();

        self.rows.clear();
        self.current_row.clear();

        while !all_buttons.is_empty() {
            let row: Vec<_> = all_buttons.drain(..per_row.min(all_buttons.len())).collect();
            self.rows.push(row);
        }

        self
    }

    /// Repeats the current button pattern `n` times.
    pub fn repeat(self, n: usize) -> Self {
        let original_rows: Vec<Vec<KeyboardButton>> = self
            .rows
            .into_iter()
            .chain(if self.current_row.is_empty() { None } else { Some(self.current_row) })
            .collect();

        let mut builder = Self::new();
        for _ in 0..n {
            for row in &original_rows {
                for button in row {
                    builder = builder.button(&button.text);
                }
                builder = builder.row();
            }
        }
        builder
    }

    /// Sets `is_persistent` to `true`.
    pub fn persistent(mut self) -> Self {
        self.is_persistent = true;
        self
    }

    /// Sets `resize_keyboard` to `true`.
    pub fn resize_keyboard(mut self) -> Self {
        self.resize_keyboard = true;
        self
    }

    /// Sets `one_time_keyboard` to `true`.
    pub fn one_time_keyboard(mut self) -> Self {
        self.one_time_keyboard = true;
        self
    }

    /// Sets the input field placeholder.
    pub fn input_field_placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.input_field_placeholder = Some(placeholder.into());
        self
    }

    /// Sets `selective` to `true`.
    pub fn selective(mut self) -> Self {
        self.selective = true;
        self
    }

    /// Returns the number of buttons in the builder.
    pub fn button_count(&self) -> usize {
        self.rows.iter().map(|r| r.len()).sum::<usize>() + self.current_row.len()
    }

    /// Returns the number of rows.
    pub fn row_count(&self) -> usize {
        self.rows.len() + if self.current_row.is_empty() { 0 } else { 1 }
    }

    /// Builds the final [`KeyboardMarkup`].
    pub fn build(mut self) -> KeyboardMarkup {
        if !self.current_row.is_empty() {
            self.rows.push(self.current_row);
        }
        KeyboardMarkup {
            keyboard: self.rows,
            is_persistent: self.is_persistent,
            resize_keyboard: self.resize_keyboard,
            one_time_keyboard: self.one_time_keyboard,
            input_field_placeholder: self.input_field_placeholder.unwrap_or_default(),
            selective: self.selective,
        }
    }

    /// Builds the final [`ReplyMarkup`].
    pub fn build_markup(self) -> ReplyMarkup {
        ReplyMarkup::Keyboard(self.build())
    }
}

impl Default for ReplyKeyboardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_keyboard_builder_basic() {
        let keyboard = InlineKeyboardBuilder::new()
            .callback_button("Btn 1", "data1")
            .callback_button("Btn 2", "data2")
            .row()
            .callback_button("Btn 3", "data3")
            .build();

        assert_eq!(keyboard.inline_keyboard.len(), 2);
        assert_eq!(keyboard.inline_keyboard[0].len(), 2);
        assert_eq!(keyboard.inline_keyboard[1].len(), 1);
    }

    #[test]
    fn inline_keyboard_builder_adjust() {
        let keyboard = InlineKeyboardBuilder::new()
            .callback_button("1", "1")
            .callback_button("2", "2")
            .callback_button("3", "3")
            .callback_button("4", "4")
            .callback_button("5", "5")
            .adjust(2)
            .build();

        assert_eq!(keyboard.inline_keyboard.len(), 3);
        assert_eq!(keyboard.inline_keyboard[0].len(), 2);
        assert_eq!(keyboard.inline_keyboard[1].len(), 2);
        assert_eq!(keyboard.inline_keyboard[2].len(), 1);
    }

    #[test]
    fn inline_keyboard_builder_repeat() {
        let keyboard = InlineKeyboardBuilder::new()
            .callback_button("A", "a")
            .row()
            .callback_button("B", "b")
            .repeat(2)
            .build();

        // Should have 2 repetitions × 2 rows = 4 rows
        assert_eq!(keyboard.inline_keyboard.len(), 4);
    }

    #[test]
    fn inline_keyboard_builder_from_markup() {
        let markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "existing", "data",
        )]]);

        let keyboard =
            InlineKeyboardBuilder::from_markup(markup).callback_button("new", "new_data").build();

        assert_eq!(keyboard.inline_keyboard.len(), 1);
        assert_eq!(keyboard.inline_keyboard[0].len(), 2);
    }

    #[test]
    fn reply_keyboard_builder_basic() {
        let keyboard = ReplyKeyboardBuilder::new()
            .button("Btn 1")
            .button("Btn 2")
            .row()
            .button("Btn 3")
            .resize_keyboard()
            .one_time_keyboard()
            .build();

        assert_eq!(keyboard.keyboard.len(), 2);
        assert_eq!(keyboard.keyboard[0].len(), 2);
        assert_eq!(keyboard.keyboard[1].len(), 1);
        assert!(keyboard.resize_keyboard);
        assert!(keyboard.one_time_keyboard);
    }

    #[test]
    fn reply_keyboard_builder_adjust() {
        let keyboard = ReplyKeyboardBuilder::new()
            .button("1")
            .button("2")
            .button("3")
            .button("4")
            .adjust(2)
            .build();

        assert_eq!(keyboard.keyboard.len(), 2);
        assert_eq!(keyboard.keyboard[0].len(), 2);
        assert_eq!(keyboard.keyboard[1].len(), 2);
    }

    #[test]
    fn reply_keyboard_builder_options() {
        let keyboard = ReplyKeyboardBuilder::new()
            .button("Btn")
            .persistent()
            .selective()
            .input_field_placeholder("Type here...")
            .build();

        assert!(keyboard.is_persistent);
        assert!(keyboard.selective);
        assert_eq!(keyboard.input_field_placeholder, "Type here...");
    }

    #[test]
    fn button_count() {
        let builder = InlineKeyboardBuilder::new()
            .callback_button("1", "1")
            .callback_button("2", "2")
            .row()
            .callback_button("3", "3");

        assert_eq!(builder.button_count(), 3);
        assert_eq!(builder.row_count(), 2);
    }

    #[test]
    fn build_markup_inline() {
        let markup = InlineKeyboardBuilder::new().callback_button("Btn", "data").build_markup();

        assert!(matches!(markup, ReplyMarkup::InlineKeyboard(_)));
    }

    #[test]
    fn build_markup_reply() {
        let markup = ReplyKeyboardBuilder::new().button("Btn").build_markup();

        assert!(matches!(markup, ReplyMarkup::Keyboard(_)));
    }
}
