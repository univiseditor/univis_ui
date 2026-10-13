use super::*;

use super::truncation::{has_overflow_constraints, text_fits_constraints};

#[derive(Clone, Debug, Default)]
pub(super) struct MeasuredTextInfo {
    pub(super) size: Vec2,
    pub(super) min_content_size: Vec2,
    pub(super) max_content_size: Vec2,
    pub(super) line_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TextMeasureStage {
    FullText,
    EllipsisOnly,
    EmptyFallback,
    TruncationCandidate,
    FinalDisplayText,
}

impl TextMeasureStage {
    fn as_str(self) -> &'static str {
        match self {
            Self::FullText => "full_text",
            Self::EllipsisOnly => "ellipsis_only",
            Self::EmptyFallback => "empty_fallback",
            Self::TruncationCandidate => "truncation_candidate",
            Self::FinalDisplayText => "final_display_text",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TextMeasureErrorKind {
    PipelineCreateFailed,
}

impl TextMeasureErrorKind {
    fn reason(self) -> &'static str {
        match self {
            Self::PipelineCreateFailed => {
                "bevy text pipeline could not create a measurement request"
            }
        }
    }

    fn action(self) -> &'static str {
        match self {
            Self::PipelineCreateFailed => {
                "verify the font handle is loaded and the text pipeline resources are available"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TextMeasureError {
    pub(super) entity: Entity,
    pub(super) stage: TextMeasureStage,
    pub(super) kind: TextMeasureErrorKind,
    pub(super) text_len: usize,
}

impl TextMeasureError {
    fn new(entity: Entity, stage: TextMeasureStage, text_len: usize) -> Self {
        Self {
            entity,
            stage,
            kind: TextMeasureErrorKind::PipelineCreateFailed,
            text_len,
        }
    }
}

pub(super) fn log_text_measure_error(error: TextMeasureError) {
    bevy::log::debug!(
        "[widget/text_label_measure] entity={:?} stage={} text_len={} reason={} action={}",
        error.entity,
        error.stage.as_str(),
        error.text_len,
        error.kind.reason(),
        error.kind.action(),
    );
}

pub(super) fn label_text_layout(label: &UTextLabel) -> TextLayout {
    let linebreak = match label.wrap_mode {
        UTextWrapMode::SingleLine => label.linebreak,
        UTextWrapMode::WrapToParent | UTextWrapMode::ClampLines(_) => {
            if label.linebreak == LineBreak::NoWrap {
                LineBreak::WordBoundary
            } else {
                label.linebreak
            }
        }
    };
    TextLayout {
        justify: label.justify,
        linebreak,
    }
}

pub(super) fn label_text_font(label: &UTextLabel) -> TextFont {
    TextFont {
        font: FontSource::Handle(label.font.clone()),
        font_size: FontSize::Px(label.font_size),
        ..default()
    }
}

pub(super) fn measure_layout_for_text(
    entity: Entity,
    text: &str,
    stage: TextMeasureStage,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut FontCx,
    layout_cx: &mut LayoutCx,
) -> Result<MeasuredTextInfo, TextMeasureError> {
    let mut measure = text_pipeline
        .create_text_measure(
            entity,
            fonts,
            std::iter::once((
                entity,
                0,
                text,
                text_font,
                text_color,
                LineHeight::default(),
                LetterSpacing::default(),
            )),
            1.0,
            text_layout,
            computed,
            font_system,
            layout_cx,
            Vec2::ZERO,
            20.0,
        )
        .map_err(|_| TextMeasureError::new(entity, stage, text.chars().count()))?;

    let computed_size = measure.compute_size(bounds, computed, font_system);
    let line_count = computed.buffer().lines().count();

    let size = if text_layout.linebreak == LineBreak::NoWrap {
        measure.max
    } else {
        computed_size
    };

    let (min_content_size, max_content_size) = if text_layout.linebreak == LineBreak::NoWrap {
        (measure.max, measure.max)
    } else {
        let max_w = bounds
            .width
            .map_or(measure.max.x, |bw| computed_size.x.min(bw));
        (
            Vec2::new(measure.min.x, computed_size.y),
            Vec2::new(max_w, computed_size.y),
        )
    };

    Ok(MeasuredTextInfo {
        size,
        min_content_size,
        max_content_size,
        line_count,
    })
}

pub(super) fn measure_layout_for_label(
    entity: Entity,
    label: &UTextLabel,
    stage: TextMeasureStage,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut FontCx,
    layout_cx: &mut LayoutCx,
) -> Result<MeasuredTextInfo, TextMeasureError> {
    if label.spans.is_empty() {
        return measure_layout_for_text(
            entity,
            label.text.as_str(),
            stage,
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
            layout_cx,
        );
    }

    let span_fonts: Vec<TextFont> = label
        .spans
        .iter()
        .map(|span| TextFont {
            font: FontSource::Handle(span.font.as_ref().unwrap_or(&label.font).clone()),
            font_size: FontSize::Px(span.font_size.unwrap_or(label.font_size)),
            ..default()
        })
        .collect();

    let spans_iter = label.spans.iter().enumerate().map(|(idx, span)| {
        (
            entity,
            idx,
            span.text.as_str(),
            &span_fonts[idx],
            span.color.unwrap_or(label.color),
            LineHeight::default(),
            LetterSpacing::default(),
        )
    });

    let mut measure = text_pipeline
        .create_text_measure(
            entity,
            fonts,
            spans_iter,
            1.0,
            text_layout,
            computed,
            font_system,
            layout_cx,
            Vec2::ZERO,
            20.0,
        )
        .map_err(|_| TextMeasureError::new(entity, stage, label.text.chars().count()))?;

    let computed_size = measure.compute_size(bounds, computed, font_system);
    let line_count = computed.buffer().lines().count();

    let size = if text_layout.linebreak == LineBreak::NoWrap {
        measure.max
    } else {
        computed_size
    };

    let (min_content_size, max_content_size) = if text_layout.linebreak == LineBreak::NoWrap {
        (measure.max, measure.max)
    } else {
        let max_w = bounds
            .width
            .map_or(measure.max.x, |bw| computed_size.x.min(bw));
        (
            Vec2::new(measure.min.x, computed_size.y),
            Vec2::new(max_w, computed_size.y),
        )
    };

    Ok(MeasuredTextInfo {
        size,
        min_content_size,
        max_content_size,
        line_count,
    })
}

pub(super) fn resolve_final_measured_text(
    entity: Entity,
    label: &UTextLabel,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut FontCx,
    layout_cx: &mut LayoutCx,
    measured: MeasuredTextInfo,
) -> Result<(String, MeasuredTextInfo, bool), TextMeasureError> {
    let mut final_text = if label.text.is_empty() && !label.spans.is_empty() {
        label.spans.iter().map(|s| s.text.as_str()).collect()
    } else {
        label.text.clone()
    };
    let mut final_measured = measured;

    if label.spans.is_empty()
        && !text_fits_constraints(&final_measured, bounds, label)
        && (label.overflow == UTextOverflow::Ellipsis || label.effective_max_lines().is_some())
        && has_overflow_constraints(bounds, label)
    {
        let (displayed_text, _) = super::truncation::build_ellipsized_text(
            entity,
            label,
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
            layout_cx,
        )?;
        final_text = displayed_text;
        final_measured = measure_layout_for_text(
            entity,
            final_text.as_str(),
            TextMeasureStage::FinalDisplayText,
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
            layout_cx,
        )?;
    }

    let overflowed =
        final_text != label.text || !text_fits_constraints(&final_measured, bounds, label);

    Ok((final_text, final_measured, overflowed))
}
