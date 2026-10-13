use crate::layout::univis_node::{ULayout, UNode};
use bevy::prelude::*;
use bevy::text::{ComputedTextBlock, LineBreak};

pub const DEFAULT_TEXT_RENDER_SCALE: f32 = 8.0;

/// Canonical text primitive component for Univis UI.
pub type UText = UTextLabel;

/// Wrapping mode for [`UTextLabel`].
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Default, Debug, Clone, PartialEq)]
pub enum UTextWrapMode {
    /// Keep text on a single line regardless of container width.
    #[default]
    SingleLine,
    /// Wrap text at container boundaries using word breaking.
    WrapToParent,
    /// Clamp to at most N lines of text with wrapping.
    ClampLines(usize),
}

/// Overflow policy used by [`UText`].
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Default, Debug, Clone, PartialEq)]
pub enum UTextOverflow {
    /// Render the full text even if it extends past the available bounds.
    Visible,
    /// Clip the rendered text to the available bounds.
    Clip,
    /// Replace overflowing tail content with an ellipsis when possible.
    #[default]
    Ellipsis,
}

/// Controls which side of the text is shortened when ellipsis is applied.
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Default, Debug, Clone, PartialEq)]
pub enum UTextTruncateSide {
    /// Use the widget's default behavior.
    #[default]
    Auto,
    /// Remove leading content.
    Start,
    /// Remove trailing content.
    End,
    /// Remove inner content and keep both edges.
    Middle,
}

/// A styled segment of text within a [`UTextLabel`].
#[derive(Debug, Reflect, Clone, PartialEq)]
pub struct UTextSpan {
    pub text: String,
    pub color: Option<Color>,
    pub font: Option<Handle<Font>>,
    pub font_size: Option<f32>,
}

impl Default for UTextSpan {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: None,
            font: None,
            font_size: None,
        }
    }
}

impl UTextSpan {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..default()
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_font(mut self, font: Handle<Font>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn styled(text: impl Into<String>, color: Color, font_size: f32) -> Self {
        Self {
            text: text.into(),
            color: Some(color),
            font: None,
            font_size: Some(font_size),
        }
    }
}

impl<T: Into<String>> From<T> for UTextSpan {
    fn from(text: T) -> Self {
        Self::new(text)
    }
}

/// Custom BSN template wrapper for inline text spans.
#[derive(Clone, Default, Debug)]
pub struct UTextSpansTemplate(pub Vec<UTextSpan>);

impl Unpin for UTextSpansTemplate where for<'a> [()]: bevy::ecs::template::SpecializeFromTemplate {}

impl bevy::prelude::Template for UTextSpansTemplate {
    type Output = Vec<UTextSpan>;

    fn build_template(
        &self,
        _context: &mut bevy::ecs::template::TemplateContext,
    ) -> bevy::ecs::error::Result<Self::Output> {
        Ok(self.0.clone())
    }

    fn clone_template(&self) -> Self {
        self.clone()
    }
}

impl From<Vec<UTextSpan>> for UTextSpansTemplate {
    fn from(spans: Vec<UTextSpan>) -> Self {
        Self(spans)
    }
}

/// Core text primitive backed by Bevy text measurement and Univis SDF rendering.
#[derive(Component, Clone, Reflect, FromTemplate)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility, ComputedTextBlock, UTextLabelLayoutCache)]
pub struct UTextLabel {
    pub text: String,
    /// Styled inline segments. When non-empty, these spans define the text layout and formatting.
    #[template(UTextSpansTemplate)]
    pub spans: Vec<UTextSpan>,
    pub font_size: f32,
    pub color: Color,
    pub font: Handle<Font>,
    pub justify: Justify,
    pub linebreak: LineBreak,
    /// Text wrap mode controlling whether lines wrap at word boundaries or clamp.
    pub wrap_mode: UTextWrapMode,
    /// If `true`, the label updates its host [`UNode`] width and height from measured text.
    pub autosize: bool,
    /// SDF raster scale used to increase glyph sharpness while keeping the same final size.
    pub render_scale: f32,
    /// What happens when the available bounds are too small for the full text.
    pub overflow: UTextOverflow,
    /// Which side is shortened when ellipsis is active.
    pub truncate_side: UTextTruncateSide,
    /// Maximum line count before clipping or ellipsis is applied.
    #[template(built_in)]
    pub max_lines: Option<usize>,
    /// Outline stroke width in logical pixels. Rendered via SDF distance fields with zero extra draw calls.
    pub outline_width: f32,
    /// Outline stroke color.
    pub outline_color: Color,
}

/// Marks a text label as selectable and copyable.
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct USelectableText {
    /// Active character/byte selection range: `Some((start, end))` where `start <= end`.
    pub selection: Option<(usize, usize)>,
    /// Whether text selection is enabled on this label.
    pub enabled: bool,
    /// Selection highlight color.
    pub highlight_color: Color,
}

impl Default for USelectableText {
    fn default() -> Self {
        Self {
            selection: None,
            enabled: true,
            highlight_color: Color::srgba(0.2, 0.45, 0.9, 0.35),
        }
    }
}

impl USelectableText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        let (s, e) = if start <= end { (start, end) } else { (end, start) };
        self.selection = Some((s, e));
        self
    }

    pub fn with_highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color;
        self
    }

    pub fn select_all(&mut self, text_len: usize) {
        self.selection = Some((0, text_len));
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn clear(&mut self) {
        self.selection = None;
    }

    pub fn is_selected(&self) -> bool {
        self.selection.is_some()
    }

    pub fn selected_text<'a>(&self, text: &'a str) -> Option<&'a str> {
        let (start, end) = self.selection?;
        if start <= end && end <= text.len() {
            Some(&text[start..end])
        } else {
            None
        }
    }
}

/// Event triggered when text from a [`USelectableText`] label should be copied to the clipboard.
#[derive(Event, Clone, Debug)]
pub struct UCopyText {
    pub text: String,
}

/// Cached measurement output for [`UTextLabel`].
#[derive(Component, Reflect, Default, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct UTextLabelLayoutCache {
    pub measured_size: Vec2,
    pub min_content_size: Vec2,
    pub max_content_size: Vec2,
    pub parent_bound_width: Option<f32>,
    pub parent_bound_height: Option<f32>,
    pub displayed_text: String,
    pub line_count: usize,
    pub overflowed: bool,
    pub dirty: bool,
}

/// Internal marker for generated text-render child entities.
#[derive(Component)]
#[doc(hidden)]
pub struct TextChildMarker;

impl Default for UTextLabel {
    fn default() -> Self {
        Self {
            text: "Label".to_string(),
            spans: Vec::new(),
            font_size: 16.0,
            color: Color::WHITE,
            font: Handle::default(),
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
            wrap_mode: UTextWrapMode::SingleLine,
            autosize: true,
            render_scale: DEFAULT_TEXT_RENDER_SCALE,
            overflow: UTextOverflow::Ellipsis,
            truncate_side: UTextTruncateSide::Auto,
            max_lines: None,
            outline_width: 0.0,
            outline_color: Color::NONE,
        }
    }
}

impl UTextLabel {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..default()
        }
    }

    pub fn multiline(text: &str) -> Self {
        Self {
            text: text.to_string(),
            wrap_mode: UTextWrapMode::WrapToParent,
            overflow: UTextOverflow::Visible,
            ..default()
        }
    }

    pub fn clamped(text: &str, max_lines: usize) -> Self {
        Self {
            text: text.to_string(),
            wrap_mode: UTextWrapMode::ClampLines(max_lines),
            max_lines: Some(max_lines),
            ..default()
        }
    }

    pub fn with_render_scale(mut self, render_scale: f32) -> Self {
        self.render_scale = render_scale;
        self
    }

    pub fn with_overflow(mut self, overflow: UTextOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_truncate_side(mut self, truncate_side: UTextTruncateSide) -> Self {
        self.truncate_side = truncate_side;
        self
    }

    pub fn with_wrap_mode(mut self, wrap_mode: UTextWrapMode) -> Self {
        self.wrap_mode = wrap_mode;
        self
    }

    pub fn is_wrapping(&self) -> bool {
        matches!(
            self.wrap_mode,
            UTextWrapMode::WrapToParent | UTextWrapMode::ClampLines(_)
        )
    }

    pub fn effective_max_lines(&self) -> Option<usize> {
        match self.wrap_mode {
            UTextWrapMode::ClampLines(limit) => Some(self.max_lines.map_or(limit, |m| m.min(limit))),
            _ => self.max_lines,
        }
    }

    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = Some(max_lines);
        self
    }

    /// Creates a rich-text label composed of multiple styled inline spans.
    pub fn rich(spans: impl IntoIterator<Item = impl Into<UTextSpan>>) -> Self {
        let spans_vec: Vec<UTextSpan> = spans.into_iter().map(Into::into).collect();
        let text: String = spans_vec.iter().map(|s| s.text.as_str()).collect();
        Self {
            text,
            spans: spans_vec,
            ..default()
        }
    }

    /// Appends a styled inline span to this text label.
    pub fn with_span(mut self, span: impl Into<UTextSpan>) -> Self {
        if self.spans.is_empty() && !self.text.is_empty() {
            self.spans.push(UTextSpan::new(self.text.clone()));
        }
        self.spans.push(span.into());
        self.sync_text_from_spans();
        self
    }

    /// Sets the outline stroke width and color for SDF text rendering.
    pub fn with_outline(mut self, width: f32, color: Color) -> Self {
        self.outline_width = width;
        self.outline_color = color;
        self
    }

    pub fn with_outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
        self
    }

    pub fn with_outline_color(mut self, color: Color) -> Self {
        self.outline_color = color;
        self
    }

    /// Synchronizes `self.text` from `self.spans` when spans are non-empty.
    pub fn sync_text_from_spans(&mut self) {
        if !self.spans.is_empty() {
            self.text = self.spans.iter().map(|s| s.text.as_str()).collect();
        }
    }

    /// Returns `true` if this label contains rich text spans.
    pub fn has_spans(&self) -> bool {
        !self.spans.is_empty()
    }
}
