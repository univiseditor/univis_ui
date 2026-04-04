use crate::internal_prelude::*;
use bevy::asset::{AssetEvent, Assets};
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::text::{
    ComputedTextBlock, CosmicFontSystem, FontHinting, LineBreak, LineHeight, TextBounds, TextFont,
    TextLayout, TextPipeline,
};
use std::collections::{HashMap, HashSet};
use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation;
use univis_ui_engine::internal::IntrinsicSize;
use univis_ui_engine::layout::core::layout_cache::LayoutCache;

use super::model::{UTextLabel, UTextLabelLayoutCache, UTextOverflow, UTextTruncateSide};

pub(super) const DEFAULT_ELLIPSIS: &str = "...";
pub(super) const LTR_ISOLATE_START: char = '\u{2066}';
const RTL_ISOLATE_START: char = '\u{2067}';
pub(super) const ISOLATE_END: char = '\u{2069}';

#[derive(Clone, Debug, Default)]
struct MeasuredTextInfo {
    size: Vec2,
    min_content_size: Vec2,
    max_content_size: Vec2,
    line_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TextBaseDirection {
    Ltr,
    Rtl,
}

fn label_text_layout(label: &UTextLabel) -> TextLayout {
    TextLayout {
        justify: label.justify,
        linebreak: label.linebreak,
    }
}

fn label_text_font(label: &UTextLabel) -> TextFont {
    TextFont {
        font: label.font.clone(),
        font_size: label.font_size,
        ..default()
    }
}

fn constrained_node_dimension(spec: &UVal, computed: f32, padding: f32) -> Option<f32> {
    match spec {
        UVal::Px(value) => Some((*value - padding).max(0.0)),
        _ if computed > 0.0 => Some((computed - padding).max(0.0)),
        _ => None,
    }
}

fn node_uses_intrinsic_dimension(spec: &UVal) -> bool {
    spec.uses_intrinsic_measurement()
}

fn parent_label_bounds(
    entity: Entity,
    label: &UTextLabel,
    node: &UNode,
    parents_query: &Query<&ChildOf>,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if label.overflow == UTextOverflow::Visible {
        return TextBounds {
            width: None,
            height: None,
        };
    }

    let Ok(parent) = parents_query.get(entity) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };
    let Ok((parent_node, parent_size)) = parent_query.get(parent.get()) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };

    let width = if node_uses_intrinsic_dimension(&parent_node.width) {
        None
    } else {
        Some(
            (parent_size.width - parent_node.padding.width_sum() - node.margin.width_sum())
                .max(0.0),
        )
    };

    let height = if node_uses_intrinsic_dimension(&parent_node.height) {
        None
    } else {
        Some(
            (parent_size.height - parent_node.padding.height_sum() - node.margin.height_sum())
                .max(0.0),
        )
    };

    TextBounds { width, height }
}

fn parent_label_bounds_from_ids(
    parent_entity: Option<Entity>,
    overflow: UTextOverflow,
    margin: USides,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if overflow == UTextOverflow::Visible {
        return TextBounds {
            width: None,
            height: None,
        };
    }

    let Some(parent_entity) = parent_entity else {
        return TextBounds {
            width: None,
            height: None,
        };
    };
    let Ok((parent_node, parent_size)) = parent_query.get(parent_entity) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };

    let width = if node_uses_intrinsic_dimension(&parent_node.width) {
        None
    } else {
        Some((parent_size.width - parent_node.padding.width_sum() - margin.width_sum()).max(0.0))
    };

    let height = if node_uses_intrinsic_dimension(&parent_node.height) {
        None
    } else {
        Some((parent_size.height - parent_node.padding.height_sum() - margin.height_sum()).max(0.0))
    };

    TextBounds { width, height }
}

fn clamp_outer_size_to_bounds(outer_size: Vec2, bounds: TextBounds) -> Vec2 {
    Vec2::new(
        bounds
            .width
            .map_or(outer_size.x, |width| outer_size.x.min(width)),
        bounds
            .height
            .map_or(outer_size.y, |height| outer_size.y.min(height)),
    )
}

pub(super) fn measured_text_outer_size(node: &UNode, layout_cache: &UTextLabelLayoutCache) -> Vec2 {
    Vec2::new(
        layout_cache.measured_size.x.max(0.0) + node.padding.width_sum(),
        layout_cache.measured_size.y.max(0.0) + node.padding.height_sum(),
    )
}

fn measured_text_outer_bounds(node: &UNode, layout_cache: &UTextLabelLayoutCache) -> (Vec2, Vec2) {
    (
        Vec2::new(
            layout_cache.min_content_size.x.max(0.0) + node.padding.width_sum(),
            layout_cache.min_content_size.y.max(0.0) + node.padding.height_sum(),
        ),
        Vec2::new(
            layout_cache.max_content_size.x.max(0.0) + node.padding.width_sum(),
            layout_cache.max_content_size.y.max(0.0) + node.padding.height_sum(),
        ),
    )
}

pub(super) fn desired_text_label_intrinsic_size(
    node: &UNode,
    layout_cache: &UTextLabelLayoutCache,
    current: IntrinsicSize,
) -> IntrinsicSize {
    let (min_outer_size, max_outer_size) = measured_text_outer_bounds(node, layout_cache);

    IntrinsicSize {
        width: if node_uses_intrinsic_dimension(&node.width) {
            match node.width {
                UVal::MinContent => min_outer_size.x,
                _ => max_outer_size.x,
            }
        } else {
            current.width
        },
        height: if node_uses_intrinsic_dimension(&node.height) {
            match node.height {
                UVal::MinContent => min_outer_size.y,
                _ => max_outer_size.y,
            }
        } else {
            current.height
        },
        min_width: if node_uses_intrinsic_dimension(&node.width) {
            min_outer_size.x
        } else {
            current.min_width
        },
        max_width: if node_uses_intrinsic_dimension(&node.width) {
            max_outer_size.x
        } else {
            current.max_width
        },
        min_height: if node_uses_intrinsic_dimension(&node.height) {
            min_outer_size.y
        } else {
            current.min_height
        },
        max_height: if node_uses_intrinsic_dimension(&node.height) {
            max_outer_size.y
        } else {
            current.max_height
        },
    }
}

pub(super) fn label_measure_bounds(
    label: &UTextLabel,
    node: &UNode,
    computed_size: Option<&ComputedSize>,
    parent_bounds: TextBounds,
) -> TextBounds {
    let computed_width = computed_size.map_or(0.0, |size| size.width);
    let computed_height = computed_size.map_or(0.0, |size| size.height);

    let constrain_width = !label.autosize
        && (label.linebreak != LineBreak::NoWrap
            || label.overflow != UTextOverflow::Visible
            || label.max_lines.is_some());
    let constrain_height = !label.autosize;

    let width = if constrain_width {
        constrained_node_dimension(&node.width, computed_width, node.padding.width_sum())
    } else {
        None
    };

    let height = if constrain_height {
        constrained_node_dimension(&node.height, computed_height, node.padding.height_sum())
    } else {
        None
    };

    TextBounds {
        width: width.or(if label.autosize {
            parent_bounds.width
        } else {
            None
        }),
        height: height.or(if label.autosize {
            parent_bounds.height
        } else {
            None
        }),
    }
}

fn measure_layout_for_text(
    entity: Entity,
    text: &str,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
) -> Result<MeasuredTextInfo, ()> {
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
            )),
            1.0,
            text_layout,
            computed,
            font_system,
            FontHinting::Disabled,
        )
        .map_err(|_| ())?;

    let size = measure.compute_size(bounds, computed, font_system);
    let line_count = computed.buffer().layout_runs().count();

    Ok(MeasuredTextInfo {
        size,
        min_content_size: measure.min,
        max_content_size: measure.max,
        line_count,
    })
}

fn resolve_final_measured_text(
    entity: Entity,
    label: &UTextLabel,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
    measured: MeasuredTextInfo,
) -> Result<(String, MeasuredTextInfo, bool), ()> {
    let mut final_text = label.text.clone();
    let mut final_measured = measured;

    if !text_fits_constraints(&final_measured, bounds, label)
        && label.overflow == UTextOverflow::Ellipsis
        && has_overflow_constraints(bounds, label)
    {
        let (displayed_text, _) = build_ellipsized_text(
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
        )?;
        final_text = displayed_text;
        final_measured = measure_layout_for_text(
            entity,
            final_text.as_str(),
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
        )?;
    }

    let overflowed =
        final_text != label.text || !text_fits_constraints(&final_measured, bounds, label);

    Ok((final_text, final_measured, overflowed))
}

fn text_fits_constraints(
    measured: &MeasuredTextInfo,
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

fn has_overflow_constraints(bounds: TextBounds, label: &UTextLabel) -> bool {
    bounds.width.is_some() || bounds.height.is_some() || label.max_lines.is_some()
}

pub(super) fn text_char_boundaries(text: &str) -> Vec<usize> {
    let mut boundaries = Vec::with_capacity(text.graphemes(true).count() + 1);
    boundaries.push(0);
    for (idx, grapheme) in text.grapheme_indices(true) {
        boundaries.push(idx + grapheme.len());
    }
    boundaries
}

pub(super) fn base_direction_for_text(text: &str) -> TextBaseDirection {
    let bidi = BidiInfo::new(text, None);
    bidi.paragraphs
        .first()
        .map(|paragraph| {
            if paragraph.level.is_rtl() {
                TextBaseDirection::Rtl
            } else {
                TextBaseDirection::Ltr
            }
        })
        .unwrap_or(TextBaseDirection::Ltr)
}

pub(super) fn resolve_truncate_side(
    truncate_side: UTextTruncateSide,
    _base_direction: TextBaseDirection,
) -> UTextTruncateSide {
    match truncate_side {
        UTextTruncateSide::Auto => UTextTruncateSide::End,
        side => side,
    }
}

fn isolate_for_direction(segment: &str, base_direction: TextBaseDirection) -> String {
    if segment.is_empty() {
        return String::new();
    }

    let isolate_start = match base_direction {
        TextBaseDirection::Ltr => LTR_ISOLATE_START,
        TextBaseDirection::Rtl => RTL_ISOLATE_START,
    };

    let mut isolated = String::with_capacity(segment.len() + 2);
    isolated.push(isolate_start);
    isolated.push_str(segment);
    isolated.push(ISOLATE_END);
    isolated
}

pub(super) fn build_truncate_candidate(
    text: &str,
    boundaries: &[usize],
    truncate_side: UTextTruncateSide,
    kept_graphemes: usize,
    base_direction: TextBaseDirection,
) -> String {
    let total_graphemes = boundaries.len().saturating_sub(1);
    if total_graphemes == 0 {
        return DEFAULT_ELLIPSIS.to_string();
    }

    match truncate_side {
        UTextTruncateSide::End | UTextTruncateSide::Auto => {
            let keep = kept_graphemes.min(total_graphemes);
            let prefix = &text[..boundaries[keep]];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let mut candidate =
                String::with_capacity(isolated_prefix.len() + DEFAULT_ELLIPSIS.len());
            candidate.push_str(&isolated_prefix);
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate
        }
        UTextTruncateSide::Start => {
            let keep = kept_graphemes.min(total_graphemes);
            let suffix = &text[boundaries[total_graphemes - keep]..];
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate =
                String::with_capacity(DEFAULT_ELLIPSIS.len() + isolated_suffix.len());
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
        UTextTruncateSide::Middle => {
            let keep = kept_graphemes.min(total_graphemes);
            let keep_prefix = keep.div_ceil(2);
            let keep_suffix = keep / 2;
            let prefix = &text[..boundaries[keep_prefix]];
            let suffix = &text[boundaries[total_graphemes - keep_suffix]..];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate = String::with_capacity(
                isolated_prefix.len() + DEFAULT_ELLIPSIS.len() + isolated_suffix.len(),
            );
            candidate.push_str(&isolated_prefix);
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
    }
}

fn build_ellipsized_text(
    entity: Entity,
    label: &UTextLabel,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
) -> Result<(String, MeasuredTextInfo), ()> {
    let base_direction = base_direction_for_text(&label.text);
    let truncate_side = resolve_truncate_side(label.truncate_side, base_direction);
    let ellipsis_only = measure_layout_for_text(
        entity,
        DEFAULT_ELLIPSIS,
        text_font,
        text_layout,
        text_color,
        bounds,
        fonts,
        text_pipeline,
        computed,
        font_system,
    )?;

    if !text_fits_constraints(&ellipsis_only, bounds, label) {
        let empty = measure_layout_for_text(
            entity,
            "",
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
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
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
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

pub fn measure_text_label_layout(
    mut font_events: MessageReader<AssetEvent<Font>>,
    fonts: Res<Assets<Font>>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<CosmicFontSystem>,
    parents_query: Query<&ChildOf>,
    parent_query: Query<(&UNode, &ComputedSize)>,
    mut query: Query<(
        Entity,
        Ref<UTextLabel>,
        Ref<UNode>,
        Ref<ComputedSize>,
        &mut ComputedTextBlock,
        &mut UTextLabelLayoutCache,
    )>,
) {
    let mut changed_fonts = HashSet::new();
    for event in font_events.read() {
        match *event {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id }
            | AssetEvent::Removed { id }
            | AssetEvent::Unused { id } => {
                changed_fonts.insert(id);
            }
        }
    }

    for (entity, label, node, computed_size, mut computed, mut cache) in query.iter_mut() {
        let font_changed = changed_fonts.contains(&label.font.id());
        if !(label.is_changed()
            || node.is_changed()
            || computed_size.is_changed()
            || cache.dirty
            || font_changed)
        {
            continue;
        }

        let text_font = label_text_font(&label);
        let text_layout = label_text_layout(&label);
        let text_color = label.color;
        let parent_bounds =
            parent_label_bounds(entity, &label, &node, &parents_query, &parent_query);
        let bounds = label_measure_bounds(&label, &node, Some(&computed_size), parent_bounds);

        match measure_layout_for_text(
            entity,
            label.text.as_str(),
            &text_font,
            &text_layout,
            text_color,
            bounds,
            &fonts,
            &mut text_pipeline,
            &mut computed,
            &mut font_system,
        ) {
            Ok(measured) => {
                let Ok((final_text, final_measured, overflowed)) = resolve_final_measured_text(
                    entity,
                    &label,
                    &text_font,
                    &text_layout,
                    text_color,
                    bounds,
                    &fonts,
                    &mut text_pipeline,
                    &mut computed,
                    &mut font_system,
                    measured,
                ) else {
                    cache.measured_size = Vec2::ZERO;
                    cache.min_content_size = Vec2::ZERO;
                    cache.max_content_size = Vec2::ZERO;
                    cache.displayed_text.clear();
                    cache.line_count = 0;
                    cache.overflowed = false;
                    cache.dirty = false;
                    continue;
                };

                cache.min_content_size = final_measured.min_content_size;
                cache.max_content_size = final_measured.max_content_size;
                cache.measured_size = final_measured.size;
                cache.displayed_text = final_text;
                cache.line_count = final_measured.line_count;
                cache.overflowed = overflowed;
                cache.dirty = false;
            }
            Err(_) => {
                cache.measured_size = Vec2::ZERO;
                cache.min_content_size = Vec2::ZERO;
                cache.max_content_size = Vec2::ZERO;
                cache.displayed_text.clear();
                cache.line_count = 0;
                cache.overflowed = false;
                cache.dirty = false;
            }
        }
    }
}

pub fn fit_node_to_text_size(
    parents_query: Query<&ChildOf>,
    mut params: ParamSet<(
        Query<(&UNode, &ComputedSize)>,
        Query<(Entity, &UTextLabel, &mut UNode, &UTextLabelLayoutCache)>,
    )>,
) {
    let label_snapshot: Vec<_> = {
        let labels = params.p1();
        labels
            .iter()
            .map(|(entity, label, node, _)| {
                (
                    entity,
                    parents_query.get(entity).ok().map(|p| p.get()),
                    label.overflow,
                    node.margin,
                )
            })
            .collect()
    };

    let parent_bounds_by_entity: HashMap<Entity, TextBounds> = {
        let parent_bounds_query = params.p0();
        label_snapshot
            .into_iter()
            .map(|(entity, parent_entity, overflow, margin)| {
                (
                    entity,
                    parent_label_bounds_from_ids(
                        parent_entity,
                        overflow,
                        margin,
                        &parent_bounds_query,
                    ),
                )
            })
            .collect()
    };

    for (entity, label, mut node, layout_cache) in params.p1().iter_mut() {
        if !label.autosize {
            continue;
        }

        let outer_size = measured_text_outer_size(&node, layout_cache);
        let parent_bounds = parent_bounds_by_entity
            .get(&entity)
            .copied()
            .unwrap_or(TextBounds {
                width: None,
                height: None,
            });
        let clamped_outer_size = clamp_outer_size_to_bounds(outer_size, parent_bounds);
        let target_width = clamped_outer_size.x;
        let target_height = clamped_outer_size.y;

        let current_w = match node.width {
            UVal::Px(v) => v,
            _ => -1.0,
        };
        let current_h = match node.height {
            UVal::Px(v) => v,
            _ => -1.0,
        };

        if (current_w - target_width).abs() > 0.1 {
            node.width = UVal::Px(target_width);
        }
        if (current_h - target_height).abs() > 0.1 {
            node.height = UVal::Px(target_height);
        }
    }
}

pub fn sync_text_label_intrinsic_size(
    parents_query: Query<&ChildOf>,
    parent_bounds_query: Query<(&UNode, &ComputedSize)>,
    mut query: Query<
        (
            Entity,
            &UNode,
            &UTextLabel,
            &UTextLabelLayoutCache,
            &mut IntrinsicSize,
        ),
        Or<(
            Added<UTextLabel>,
            Changed<UTextLabel>,
            Changed<UNode>,
            Changed<UTextLabelLayoutCache>,
        )>,
    >,
) {
    for (entity, node, label, layout_cache, mut intrinsic) in query.iter_mut() {
        if layout_cache.dirty {
            continue;
        }

        let mut target = desired_text_label_intrinsic_size(node, layout_cache, *intrinsic);
        if label.autosize {
            let parent_bounds =
                parent_label_bounds(entity, label, node, &parents_query, &parent_bounds_query);
            let clamped_outer_size =
                clamp_outer_size_to_bounds(Vec2::new(target.width, target.height), parent_bounds);
            target.width = clamped_outer_size.x;
            target.height = clamped_outer_size.y;
        }
        if (intrinsic.width - target.width).abs() > 0.1
            || (intrinsic.height - target.height).abs() > 0.1
        {
            *intrinsic = target;
        }
    }
}

pub(super) fn mark_text_label_layout_dirty(
    mut layout_cache: ResMut<LayoutCache>,
    changed_labels: Query<
        Entity,
        (
            With<UTextLabel>,
            Or<(Added<UTextLabel>, Changed<IntrinsicSize>, Changed<UNode>)>,
        ),
    >,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
) {
    for entity in changed_labels.iter() {
        layout_cache.mark_dirty_recursive(entity, &children_query);
        layout_cache.mark_dirty_ancestors(entity, &parents_query);
    }
}
