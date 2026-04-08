use bevy::prelude::*;
use bevy::text::{ComputedTextBlock, LineBreak};
use univis_ui_engine::layout::univis_node::{ULayout, UNode};

const DEFAULT_TEXT_RENDER_SCALE: f32 = 8.0;

/// Overflow policy used by [`UTextLabel`].
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

/// A text-rendering widget backed by Bevy text measurement and Univis SDF rendering.
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility, ComputedTextBlock, UTextLabelLayoutCache)]
pub struct UTextLabel {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub font: Handle<Font>,
    pub justify: Justify,
    pub linebreak: LineBreak,
    /// If `true`, the label updates its host [`UNode`] width and height from measured text.
    pub autosize: bool,
    /// SDF raster scale used to increase glyph sharpness while keeping the same final size.
    pub render_scale: f32,
    /// What happens when the available bounds are too small for the full text.
    pub overflow: UTextOverflow,
    /// Which side is shortened when ellipsis is active.
    pub truncate_side: UTextTruncateSide,
    /// Maximum line count before clipping or ellipsis is applied.
    pub max_lines: Option<usize>,
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
            font_size: 16.0,
            color: Color::WHITE,
            font: Handle::default(),
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
            autosize: true,
            render_scale: DEFAULT_TEXT_RENDER_SCALE,
            overflow: UTextOverflow::Ellipsis,
            truncate_side: UTextTruncateSide::Auto,
            max_lines: None,
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

    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = Some(max_lines);
        self
    }
}
