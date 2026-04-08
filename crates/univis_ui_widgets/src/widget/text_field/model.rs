use super::*;
use univis_ui_engine::layout::univis_node::{ULayout, UNode};

#[derive(Resource, Default)]
pub(crate) struct TextFieldPluginInstalled;

/// A single-line editable text field widget.
///
/// `UTextField` owns its interaction state, text buffer, cursor state, and the
/// style values used to spawn its visual children.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct UTextField {
    pub text: String,
    pub(crate) previous_text: String,
    pub placeholder: String,
    pub width: f32,
    pub height: f32,
    pub background_color: Color,
    pub background_focused_color: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub border_focused_color: Color,
    pub cursor_color: Color,
    pub font_size: f32,
    pub focused: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub cursor_position: usize,
    pub cursor_visible: bool,
    pub cursor_blink_timer: f32,
    pub cursor_blink_speed: f32,
    pub max_length: Option<usize>,
    pub input_type: TextFieldInputType,
    pub padding: f32,
}

/// Input filtering mode applied while accepting text.
#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum TextFieldInputType {
    /// Accept arbitrary text.
    Text,
    /// Accept numeric input only.
    Number,
    /// Accept email-like text.
    Email,
    /// Display masked characters while keeping the real value internally.
    Password,
}

impl Default for UTextField {
    fn default() -> Self {
        Self {
            text: String::new(),
            previous_text: String::new(),
            placeholder: "Enter text...".to_string(),
            width: 300.0,
            height: 50.0,
            background_color: Color::srgb(0.15, 0.15, 0.2),
            background_focused_color: Color::srgb(0.2, 0.2, 0.3),
            text_color: Color::WHITE,
            placeholder_color: Color::srgb(0.5, 0.5, 0.6),
            border_color: Color::srgb(0.3, 0.3, 0.35),
            border_focused_color: Color::srgb(0.3, 0.7, 1.0),
            cursor_color: Color::srgb(0.3, 0.7, 1.0),
            font_size: 18.0,
            focused: false,
            disabled: false,
            readonly: false,
            cursor_position: 0,
            cursor_visible: true,
            cursor_blink_timer: 0.0,
            cursor_blink_speed: 0.5,
            max_length: None,
            input_type: TextFieldInputType::Text,
            padding: 12.0,
        }
    }
}

impl UTextField {
    /// Creates a text field with default styling and behavior.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current text value.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.cursor_position = text.len();
        self.text = text.clone();
        self.previous_text = text;
        self
    }

    /// Sets the placeholder displayed when the field is empty.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Overrides the visual width and height.
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Limits how many UTF-8 bytes can be stored.
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Sets the input filtering mode.
    pub fn input_type(mut self, input_type: TextFieldInputType) -> Self {
        self.input_type = input_type;
        self
    }
}
