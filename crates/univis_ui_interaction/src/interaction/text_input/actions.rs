//! Pure text manipulation logic, cursor movement, and UTF-8 safe editing routines.

use super::types::{TextInputFilter, TextInputMode, UTextInput};

/// Returns the display text for the text input widget.
///
/// In [`TextInputMode::Password`] mode, returns a string of masked characters
/// matching the character count of `input.value`.
pub fn display_text(input: &UTextInput) -> String {
    match input.mode {
        TextInputMode::Normal => input.value.clone(),
        TextInputMode::Password => {
            let count = input.value.chars().count();
            std::iter::repeat_n(input.mask_char, count).collect()
        }
    }
}

/// Normalizes and returns the active selection range as `(start, end)` where `start <= end`.
///
/// Returns `None` if there is no selection or if start equals end.
pub fn selection_bounds(input: &UTextInput) -> Option<(usize, usize)> {
    let (s, e) = input.selection?;
    let len = input.value.len();
    let start = s.min(e).min(len);
    let end = s.max(e).min(len);
    if start == end {
        None
    } else {
        Some((start, end))
    }
}

/// Clears any active text selection.
pub fn clear_selection(input: &mut UTextInput) {
    input.selection = None;
}

/// Selects the entire text content.
pub fn select_all(input: &mut UTextInput) {
    input.selection = Some((0, input.value.len()));
    input.cursor_position = input.value.len();
}

/// Checks whether a character is permitted under the given [`TextInputFilter`].
pub fn is_char_allowed(
    c: char,
    filter: TextInputFilter,
    current_str: &str,
    byte_pos: usize,
) -> bool {
    match filter {
        TextInputFilter::Any => true,
        TextInputFilter::SingleLine => c != '\n' && c != '\r',
        TextInputFilter::NumericOnly => c.is_ascii_digit(),
        TextInputFilter::Integer => {
            if c == '-' {
                byte_pos == 0 && !current_str.contains('-')
            } else {
                c.is_ascii_digit()
            }
        }
        TextInputFilter::Decimal => {
            if c == '-' {
                byte_pos == 0 && !current_str.contains('-')
            } else if c == '.' {
                !current_str.contains('.')
            } else {
                c.is_ascii_digit()
            }
        }
        TextInputFilter::AlphaNumeric => c.is_alphanumeric(),
    }
}

/// Inserts text at the current cursor position, replacing any active selection.
///
/// Returns `true` if any text was actually inserted.
pub fn insert_text(input: &mut UTextInput, text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    // 1. If selection exists, delete it first
    if let Some((start, end)) = selection_bounds(input) {
        input.value.drain(start..end);
        input.cursor_position = start;
        input.selection = None;
    }

    // 2. Ensure cursor is at a valid UTF-8 boundary
    clamp_cursor_to_boundary(input);

    // 3. Filter characters
    let mut allowed_chars = Vec::new();
    let mut temp_str = input.value.clone();
    let mut current_pos = input.cursor_position;

    for c in text.chars() {
        if is_char_allowed(c, input.filter, &temp_str, current_pos) {
            // Check max length
            if let Some(max_len) = input.max_length
                && temp_str.chars().count() >= max_len
            {
                break;
            }
            allowed_chars.push(c);
            temp_str.insert(current_pos, c);
            current_pos += c.len_utf8();
        }
    }

    if allowed_chars.is_empty() {
        return false;
    }

    let insert_str: String = allowed_chars.into_iter().collect();
    input.value.insert_str(input.cursor_position, &insert_str);
    input.cursor_position += insert_str.len();
    input.selection = None;

    true
}

/// Deletes the character preceding the cursor (Backspace), or the active selection.
///
/// Returns `true` if a deletion occurred.
pub fn delete_backward(input: &mut UTextInput) -> bool {
    if let Some((start, end)) = selection_bounds(input) {
        input.value.drain(start..end);
        input.cursor_position = start;
        input.selection = None;
        return true;
    }

    if input.cursor_position == 0 || input.value.is_empty() {
        return false;
    }

    clamp_cursor_to_boundary(input);

    // Find the preceding UTF-8 character boundary
    let mut prev_idx = input.cursor_position - 1;
    while prev_idx > 0 && !input.value.is_char_boundary(prev_idx) {
        prev_idx -= 1;
    }

    input.value.drain(prev_idx..input.cursor_position);
    input.cursor_position = prev_idx;
    input.selection = None;

    true
}

/// Deletes the character following the cursor (Delete key), or the active selection.
///
/// Returns `true` if a deletion occurred.
pub fn delete_forward(input: &mut UTextInput) -> bool {
    if let Some((start, end)) = selection_bounds(input) {
        input.value.drain(start..end);
        input.cursor_position = start;
        input.selection = None;
        return true;
    }

    if input.cursor_position >= input.value.len() {
        return false;
    }

    clamp_cursor_to_boundary(input);

    let next_len = input.value[input.cursor_position..]
        .chars()
        .next()
        .map(|c| c.len_utf8())
        .unwrap_or(0);

    if next_len == 0 {
        return false;
    }

    let end_idx = input.cursor_position + next_len;
    input.value.drain(input.cursor_position..end_idx);
    input.selection = None;

    true
}

/// Deletes backward from the cursor to the start of the previous word (Ctrl+Backspace).
pub fn delete_word_backward(input: &mut UTextInput) -> bool {
    if let Some((start, end)) = selection_bounds(input) {
        input.value.drain(start..end);
        input.cursor_position = start;
        input.selection = None;
        return true;
    }

    if input.cursor_position == 0 {
        return false;
    }

    clamp_cursor_to_boundary(input);

    let prefix = &input.value[..input.cursor_position];
    let mut char_indices: Vec<(usize, char)> = prefix.char_indices().collect();
    if char_indices.is_empty() {
        return false;
    }

    // Skip trailing whitespaces
    while let Some((_, c)) = char_indices.last() {
        if c.is_whitespace() {
            char_indices.pop();
        } else {
            break;
        }
    }

    // Now skip word characters
    while let Some((_, c)) = char_indices.last() {
        if !c.is_whitespace() {
            char_indices.pop();
        } else {
            break;
        }
    }

    let target_idx = char_indices
        .last()
        .map(|(idx, c)| idx + c.len_utf8())
        .unwrap_or(0);

    input.value.drain(target_idx..input.cursor_position);
    input.cursor_position = target_idx;
    input.selection = None;

    true
}

/// Moves cursor left by one UTF-8 character, optionally expanding the selection range.
pub fn move_cursor_left(input: &mut UTextInput, select: bool) {
    if !select && input.selection.is_some() {
        if let Some((start, _)) = selection_bounds(input) {
            input.cursor_position = start;
        }
        input.selection = None;
        return;
    }

    if input.cursor_position == 0 {
        if !select {
            input.selection = None;
        }
        return;
    }

    clamp_cursor_to_boundary(input);

    let anchor = if select {
        input
            .selection
            .map(|(s, _)| s)
            .unwrap_or(input.cursor_position)
    } else {
        0
    };

    let mut prev_idx = input.cursor_position - 1;
    while prev_idx > 0 && !input.value.is_char_boundary(prev_idx) {
        prev_idx -= 1;
    }

    input.cursor_position = prev_idx;

    if select {
        input.selection = Some((anchor, input.cursor_position));
    } else {
        input.selection = None;
    }
}

/// Moves cursor right by one UTF-8 character, optionally expanding the selection range.
pub fn move_cursor_right(input: &mut UTextInput, select: bool) {
    if !select && input.selection.is_some() {
        if let Some((_, end)) = selection_bounds(input) {
            input.cursor_position = end;
        }
        input.selection = None;
        return;
    }

    if input.cursor_position >= input.value.len() {
        if !select {
            input.selection = None;
        }
        return;
    }

    clamp_cursor_to_boundary(input);

    let anchor = if select {
        input
            .selection
            .map(|(s, _)| s)
            .unwrap_or(input.cursor_position)
    } else {
        0
    };

    let next_len = input.value[input.cursor_position..]
        .chars()
        .next()
        .map(|c| c.len_utf8())
        .unwrap_or(0);

    input.cursor_position = (input.cursor_position + next_len).min(input.value.len());

    if select {
        input.selection = Some((anchor, input.cursor_position));
    } else {
        input.selection = None;
    }
}

/// Moves cursor to the start of the text (Home key).
pub fn move_cursor_to_start(input: &mut UTextInput, select: bool) {
    let anchor = if select {
        input
            .selection
            .map(|(s, _)| s)
            .unwrap_or(input.cursor_position)
    } else {
        0
    };

    input.cursor_position = 0;

    if select {
        input.selection = Some((anchor, 0));
    } else {
        input.selection = None;
    }
}

/// Moves cursor to the end of the text (End key).
pub fn move_cursor_to_end(input: &mut UTextInput, select: bool) {
    let anchor = if select {
        input
            .selection
            .map(|(s, _)| s)
            .unwrap_or(input.cursor_position)
    } else {
        0
    };

    input.cursor_position = input.value.len();

    if select {
        input.selection = Some((anchor, input.value.len()));
    } else {
        input.selection = None;
    }
}

/// Ensures the cursor index is clamped within bounds and sits on a valid UTF-8 character boundary.
pub fn clamp_cursor_to_boundary(input: &mut UTextInput) {
    let len = input.value.len();
    if input.cursor_position > len {
        input.cursor_position = len;
        return;
    }

    while input.cursor_position > 0 && !input.value.is_char_boundary(input.cursor_position) {
        input.cursor_position -= 1;
    }
}

/// Given an X offset in logical pixels relative to the text content start (excluding left padding)
/// and the total measured text width, computes the closest valid UTF-8 character boundary.
pub fn cursor_index_from_x_offset(
    input: &UTextInput,
    x_in_text: f32,
    measured_width: f32,
) -> usize {
    if input.value.is_empty() || x_in_text <= 0.0 {
        return 0;
    }

    let total_bytes = input.value.len();
    if measured_width <= 0.0 || x_in_text >= measured_width {
        return total_bytes;
    }

    let ratio = (x_in_text / measured_width).clamp(0.0, 1.0);
    let chars: Vec<(usize, char)> = input.value.char_indices().collect();
    if chars.is_empty() {
        return 0;
    }

    let total_chars = chars.len();
    let target_char_idx = (ratio * total_chars as f32).round() as usize;
    if target_char_idx >= total_chars {
        total_bytes
    } else {
        chars[target_char_idx].0
    }
}

/// Finds the (start, end) byte boundaries of the word surrounding the given byte offset.
pub fn find_word_bounds(text: &str, byte_offset: usize) -> (usize, usize) {
    if text.is_empty() {
        return (0, 0);
    }
    let offset = byte_offset.min(text.len());
    let mut word_start = 0;
    let mut word_end = text.len();

    let mut found = false;
    for (idx, c) in text.char_indices() {
        let is_word_char = c.is_alphanumeric() || c == '_';
        if is_word_char {
            if !found {
                word_start = idx;
                found = true;
            }
            word_end = idx + c.len_utf8();
        } else {
            if found {
                if offset <= idx {
                    return (word_start, word_end);
                }
                found = false;
            }
            if idx == offset {
                return (idx, idx + c.len_utf8());
            }
        }
    }

    if found {
        (word_start, word_end)
    } else {
        (offset, offset)
    }
}

/// Selects the word surrounding the current cursor position.
pub fn select_word_at_cursor(input: &mut UTextInput) {
    if input.value.is_empty() {
        input.selection = None;
        return;
    }
    let pos = input.cursor_position.min(input.value.len());
    let (start, end) = find_word_bounds(&input.value, pos);
    if start != end {
        input.selection = Some((start, end));
        input.cursor_position = end;
    } else {
        input.selection = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_delete_ascii() {
        let mut input = UTextInput::default();
        insert_text(&mut input, "Hello");
        assert_eq!(input.value, "Hello");
        assert_eq!(input.cursor_position, 5);

        insert_text(&mut input, " World");
        assert_eq!(input.value, "Hello World");
        assert_eq!(input.cursor_position, 11);

        delete_backward(&mut input);
        assert_eq!(input.value, "Hello Worl");
        assert_eq!(input.cursor_position, 10);

        move_cursor_to_start(&mut input, false);
        assert_eq!(input.cursor_position, 0);

        delete_forward(&mut input);
        assert_eq!(input.value, "ello Worl");
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_utf8_multi_byte_arabic_and_emoji() {
        let mut input = UTextInput::default();
        insert_text(&mut input, "مرحبا");
        assert_eq!(input.value, "مرحبا");
        assert_eq!(input.cursor_position, "مرحبا".len());

        insert_text(&mut input, " 🌟");
        assert_eq!(input.value, "مرحبا 🌟");

        // Backspace should delete the entire emoji
        delete_backward(&mut input);
        assert_eq!(input.value, "مرحبا ");

        // Backspace should delete the space
        delete_backward(&mut input);
        assert_eq!(input.value, "مرحبا");

        // Backspace should delete the last Arabic char 'ا'
        delete_backward(&mut input);
        assert_eq!(input.value, "مرحب");
    }

    #[test]
    fn test_selection_replacement() {
        let mut input = UTextInput::default();
        insert_text(&mut input, "The quick brown fox");
        input.selection = Some((4, 9)); // "quick"
        input.cursor_position = 9;

        insert_text(&mut input, "slow");
        assert_eq!(input.value, "The slow brown fox");
        assert_eq!(input.selection, None);
    }

    #[test]
    fn test_filters() {
        let mut input = UTextInput::default().with_filter(TextInputFilter::NumericOnly);
        insert_text(&mut input, "abc123xyz45");
        assert_eq!(input.value, "12345");

        let mut dec_input = UTextInput::default().with_filter(TextInputFilter::Decimal);
        insert_text(&mut dec_input, "-12.34.56");
        assert_eq!(dec_input.value, "-12.3456");

        let mut alpha_input = UTextInput::default().with_filter(TextInputFilter::AlphaNumeric);
        insert_text(&mut alpha_input, "User_123!");
        assert_eq!(alpha_input.value, "User123");
    }

    #[test]
    fn test_max_length_limit() {
        let mut input = UTextInput::default().with_max_length(5);
        insert_text(&mut input, "HelloWorld");
        assert_eq!(input.value, "Hello");
        assert_eq!(input.cursor_position, 5);

        insert_text(&mut input, "!");
        assert_eq!(input.value, "Hello");
    }

    #[test]
    fn test_password_display_mask() {
        let input = UTextInput::default()
            .with_value("Secret123")
            .with_mode(TextInputMode::Password);

        assert_eq!(display_text(&input), "*********");
    }

    #[test]
    fn test_delete_word_backward() {
        let mut input = UTextInput::default();
        insert_text(&mut input, "first second third");
        delete_word_backward(&mut input);
        assert_eq!(input.value, "first second ");

        delete_word_backward(&mut input);
        assert_eq!(input.value, "first ");
    }

    #[test]
    fn test_cursor_index_from_x_offset() {
        let input = UTextInput::default().with_value("HelloWorld");
        let width = 100.0;

        assert_eq!(cursor_index_from_x_offset(&input, -10.0, width), 0);
        assert_eq!(cursor_index_from_x_offset(&input, 0.0, width), 0);
        assert_eq!(cursor_index_from_x_offset(&input, 10.0, width), 1);
        assert_eq!(cursor_index_from_x_offset(&input, 50.0, width), 5);
        assert_eq!(cursor_index_from_x_offset(&input, 100.0, width), 10);
        assert_eq!(cursor_index_from_x_offset(&input, 150.0, width), 10);

        // Multi-byte UTF-8 safely landing on valid char boundary
        let arabic_input = UTextInput::default().with_value("مرحبا"); // 5 chars, 10 bytes
        let arabic_width = 80.0;
        let mid_idx = cursor_index_from_x_offset(&arabic_input, 40.0, arabic_width);
        assert!(arabic_input.value.is_char_boundary(mid_idx));
    }

    #[test]
    fn test_find_word_bounds_and_select_word() {
        let text = "alpha beta_gamma delta";
        assert_eq!(find_word_bounds(text, 2), (0, 5)); // "alpha"
        assert_eq!(find_word_bounds(text, 5), (0, 5)); // boundary at end of "alpha"
        assert_eq!(find_word_bounds(text, 10), (6, 16)); // "beta_gamma"
        assert_eq!(find_word_bounds(text, 18), (17, 22)); // "delta"

        let mut input = UTextInput::default().with_value("station_access granted");
        input.cursor_position = 4;
        select_word_at_cursor(&mut input);
        assert_eq!(input.selection, Some((0, 14))); // "station_access"
        assert_eq!(input.cursor_position, 14);
    }
}
