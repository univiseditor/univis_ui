use super::bidi::{
    base_direction_for_text, build_truncate_candidate, resolve_truncate_side, text_char_boundaries,
};
use super::measurement::{TextMeasureError, TextMeasureStage};
use super::*;

pub(super) const DEFAULT_ELLIPSIS: &str = "...";

pub(super) fn text_fits_constraints(
    measured: &measurement::MeasuredTextInfo,
    bounds: TextBounds,
    label: &UTextLabel,
) -> bool {
    let width_ok = bounds
        .width
        .map_or(true, |width| measured.size.x <= width + 0.5);
    let height_ok = bounds
        .height
        .map_or(true, |height| measured.size.y <= height + 0.5);
    let lines_ok = label
        .max_lines
        .map_or(true, |max_lines| measured.line_count <= max_lines);

    width_ok && height_ok && lines_ok
}

pub(super) fn has_overflow_constraints(bounds: TextBounds, label: &UTextLabel) -> bool {
    bounds.width.is_some() || bounds.height.is_some() || label.max_lines.is_some()
}

pub(super) fn build_ellipsized_text(
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
) -> Result<(String, measurement::MeasuredTextInfo), TextMeasureError> {
    let base_direction = base_direction_for_text(&label.text);
    let truncate_side = resolve_truncate_side(label.truncate_side, base_direction);
    let ellipsis_only = measure_layout_for_text(
        entity,
        DEFAULT_ELLIPSIS,
        TextMeasureStage::EllipsisOnly,
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

    if !text_fits_constraints(&ellipsis_only, bounds, label) {
        let empty = measure_layout_for_text(
            entity,
            "",
            TextMeasureStage::EmptyFallback,
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
        return Ok((String::new(), empty));
    }

    let boundaries = text_char_boundaries(&label.text);
    let total_graphemes = boundaries.len().saturating_sub(1);
    if total_graphemes == 0 {
        return Ok((DEFAULT_ELLIPSIS.to_string(), ellipsis_only));
    }

    let mut best_text = DEFAULT_ELLIPSIS.to_string();
    let mut best_measured = ellipsis_only;
    let mut low = 0usize;
    let mut high = total_graphemes;

    while low < high {
        let mid = (low + high + 1) / 2;
        let candidate =
            build_truncate_candidate(&label.text, &boundaries, truncate_side, mid, base_direction);

        let measured = measure_layout_for_text(
            entity,
            &candidate,
            TextMeasureStage::TruncationCandidate,
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

        if text_fits_constraints(&measured, bounds, label) {
            best_text = candidate;
            best_measured = measured;
            low = mid;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    Ok((best_text, best_measured))
}
