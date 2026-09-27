//! Type definitions and component primitives for text input fields.

use bevy::prelude::*;
use core::time::Duration;

/// Display mode for the text input field.
#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextInputMode {
    /// Normal plaintext display where typed characters are directly visible.
    #[default]
    Normal,
    /// Masked display (e.g. passwords) where typed characters are hidden behind a mask character.
    Password,
}

/// Character and format filtering rules for text inputs.
#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextInputFilter {
    /// Any unicode character is allowed (default).
    #[default]
    Any,
    /// Rejects line breaks (`\n` and `\r`), enforcing single-line input.
    SingleLine,
    /// Only allows numeric digits (`0` through `9`).
    NumericOnly,
    /// Allows optional leading minus sign and numeric digits.
    Integer,
    /// Allows optional leading minus sign, numeric digits, and at most one decimal point (`.`).
    Decimal,
    /// Allows alphanumeric characters (`a-z`, `A-Z`, `0-9`).
    AlphaNumeric,
}

/// Core component marking an entity as an interactive text input field.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct UTextInput {
    /// The current textual value entered by the user.
    pub value: String,
    /// Placeholder text rendered when `value` is empty.
    pub placeholder: String,
    /// Color of the placeholder text.
    pub placeholder_color: Color,
    /// Color of the entered text.
    pub text_color: Color,
    /// Font size for the text and placeholder.
    pub font_size: f32,
    /// Caret position in byte offset within `value`.
    pub cursor_position: usize,
    /// Optional selection range `(start_byte_idx, end_byte_idx)`.
    pub selection: Option<(usize, usize)>,
    /// Whether the text input is active and can receive input events.
    pub enabled: bool,
    /// Maximum number of characters allowed (None = unlimited).
    pub max_length: Option<usize>,
    /// Visual display mode (Normal or Password).
    pub mode: TextInputMode,
    /// Character used to mask password inputs (default `'*'`).
    pub mask_char: char,
    /// Automatically clear the value when Enter/Submit is pressed.
    pub clear_on_submit: bool,
    /// Filter rule applied to typed or pasted characters.
    pub filter: TextInputFilter,
}

impl Default for UTextInput {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            placeholder_color: Color::srgba(0.6, 0.6, 0.65, 0.6),
            text_color: Color::WHITE,
            font_size: 16.0,
            cursor_position: 0,
            selection: None,
            enabled: true,
            max_length: None,
            mode: TextInputMode::Normal,
            mask_char: '*',
            clear_on_submit: false,
            filter: TextInputFilter::SingleLine,
        }
    }
}

impl UTextInput {
    /// Creates a new text input with the given placeholder text.
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    /// Sets the initial text value and moves the cursor to the end.
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        let v = value.into();
        self.cursor_position = v.len();
        self.value = v;
        self
    }

    /// Sets the placeholder text.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the visual display mode (e.g. Password).
    pub fn with_mode(mut self, mode: TextInputMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the character filter (e.g. NumericOnly).
    pub fn with_filter(mut self, filter: TextInputFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Sets the maximum allowed characters.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Sets whether the input clears automatically on submission.
    pub fn with_clear_on_submit(mut self, clear: bool) -> Self {
        self.clear_on_submit = clear;
        self
    }

    /// Sets font size.
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets text and placeholder colors.
    pub fn with_colors(mut self, text_color: Color, placeholder_color: Color) -> Self {
        self.text_color = text_color;
        self.placeholder_color = placeholder_color;
        self
    }

    /// Sets whether the input is enabled.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Caret state tracking blinking and visual cursor properties.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct UTextInputCaret {
    /// Color of the caret line.
    pub color: Color,
    /// Width of the caret in pixels.
    pub width: f32,
    /// Height of the caret in pixels (defaults to font_size * 1.2 if None).
    pub height: Option<f32>,
    /// Blinking timer.
    pub timer: Timer,
    /// Current visibility state in the blink cycle.
    pub visible: bool,
    /// Child entity displaying the visual caret cursor line.
    pub caret_entity: Option<Entity>,
    /// Child entity displaying the text label.
    pub text_entity: Option<Entity>,
}

impl Default for UTextInputCaret {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.0, 0.9, 1.0),
            width: 2.0,
            height: None,
            timer: Timer::new(Duration::from_millis(530), TimerMode::Repeating),
            visible: true,
            caret_entity: None,
            text_entity: None,
        }
    }
}

impl UTextInputCaret {
    /// Creates a caret with a custom color.
    pub fn new(color: Color) -> Self {
        Self { color, ..default() }
    }

    /// Resets the blink timer so the caret stays solidly visible upon typing or navigation.
    pub fn reset_blink(&mut self) {
        self.timer.reset();
        self.visible = true;
    }
}

/// Marker component attached to the internal caret child entity.
#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct UTextInputCaretMarker;

/// Marker component attached to the internal text label child entity.
#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct UTextInputTextMarker;

/// Event fired when the value of a [`UTextInput`] changes.
#[derive(Event, Reflect, Clone, Debug)]
pub struct UTextInputChanged {
    /// Entity of the text input that changed.
    pub entity: Entity,
    /// New string value of the input.
    pub value: String,
}

/// Event fired when the user submits a [`UTextInput`] (by pressing Enter or NumpadEnter).
#[derive(Event, Reflect, Clone, Debug)]
pub struct UTextInputSubmit {
    /// Entity of the text input that was submitted.
    pub entity: Entity,
    /// Submitted string value.
    pub value: String,
}

/// Event fired when text input is cancelled (e.g. by pressing Escape).
#[derive(Event, Reflect, Clone, Debug)]
pub struct UTextInputCancelled {
    /// Entity of the text input that was cancelled.
    pub entity: Entity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_builder() {
        let input = UTextInput::new("Enter name...")
            .with_value("Alice")
            .with_mode(TextInputMode::Password)
            .with_filter(TextInputFilter::AlphaNumeric)
            .with_max_length(20)
            .with_clear_on_submit(true)
            .with_font_size(18.0)
            .with_enabled(true);

        assert_eq!(input.placeholder, "Enter name...");
        assert_eq!(input.value, "Alice");
        assert_eq!(input.cursor_position, 5);
        assert_eq!(input.mode, TextInputMode::Password);
        assert_eq!(input.filter, TextInputFilter::AlphaNumeric);
        assert_eq!(input.max_length, Some(20));
        assert!(input.clear_on_submit);
        assert_eq!(input.font_size, 18.0);
        assert!(input.enabled);
    }

    #[test]
    fn test_caret_reset() {
        let mut caret = UTextInputCaret::default();
        caret.visible = false;
        caret.reset_blink();
        assert!(caret.visible);
    }
}
