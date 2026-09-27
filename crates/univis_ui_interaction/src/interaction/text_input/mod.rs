//! Interactive text input fields, caret animation, and editing systems for Univis UI.
//!
//! Provides:
//! - [`UTextInput`](crate::interaction::text_input::UTextInput): State component for text input fields.
//! - [`UTextInputCaret`](crate::interaction::text_input::UTextInputCaret): Caret state and cursor animation properties.
//! - [`TextInputMode`](crate::interaction::text_input::TextInputMode): Normal or Password masked display.
//! - [`TextInputFilter`](crate::interaction::text_input::TextInputFilter): Allowed character filtering rules.
//! - [`UTextInputChanged`](crate::interaction::text_input::UTextInputChanged): Event triggered when the input text changes.
//! - [`UTextInputSubmit`](crate::interaction::text_input::UTextInputSubmit): Event triggered when Enter is pressed.
//! - [`UTextInputCancelled`](crate::interaction::text_input::UTextInputCancelled): Event triggered when Escape is pressed.
//! - [`spawn_text_input`](crate::interaction::text_input::spawn_text_input): Helper to assemble a complete interactive input widget.

pub mod actions;
pub mod builder;
mod systems;
mod types;

pub use actions::{
    clear_selection, delete_backward, delete_forward, delete_word_backward, display_text,
    insert_text, is_char_allowed, move_cursor_left, move_cursor_right, move_cursor_to_end,
    move_cursor_to_start, select_all, selection_bounds,
};
pub use builder::spawn_text_input;
pub use systems::{
    on_text_input_focus_lost, on_text_input_pointer_click, text_input_caret_blink_system,
    text_input_keyboard_system, text_input_sync_visual_system,
};
pub use types::{
    TextInputFilter, TextInputMode, UTextInput, UTextInputCancelled, UTextInputCaret,
    UTextInputCaretMarker, UTextInputChanged, UTextInputSubmit, UTextInputTextMarker,
};
