pub(super) mod bidi;
pub(super) mod bounds;
pub(super) mod invalidation;
mod measurement;
#[cfg(test)]
mod tests;
mod truncation;

use bevy::asset::{AssetEvent, Assets};
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::text::{
    ComputedTextBlock, FontCx, LayoutCx, LineBreak, LineHeight, LetterSpacing, TextBounds, TextFont,
    TextLayout, TextPipeline,
};
use std::collections::HashSet;
use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation;
use univis_ui_engine::layout::geometry::{USides, UVal};
#[cfg(test)]
use univis_ui_engine::layout::layout_system::URootUi;
use univis_ui_engine::layout::query::{ComputedSize, IntrinsicSize};
#[cfg(test)]
use univis_ui_engine::layout::univis_node::ULayout;
use univis_ui_engine::layout::univis_node::UNode;
#[cfg(test)]
use univis_ui_engine::schedule::{UiSettlementSchedule, UnivisPostUpdateSet};

use self::bounds::{
    clamp_outer_size_to_bounds, desired_text_label_intrinsic_size, label_measure_bounds,
    measured_text_outer_size, parent_label_bounds, parent_label_bounds_from_ids,
};
#[cfg(test)]
use self::invalidation::mark_text_label_layout_dirty;
use self::invalidation::{
    next_text_label_layout_cache, parent_bounds_changed, reset_text_label_layout_cache,
};
use self::measurement::{
    TextMeasureStage, label_text_font, label_text_layout, log_text_measure_error,
    measure_layout_for_text, resolve_final_measured_text,
};
use super::model::{UTextLabel, UTextLabelLayoutCache, UTextOverflow, UTextTruncateSide};

pub fn measure_text_label_layout(
    mut font_events: MessageReader<AssetEvent<Font>>,
    fonts: Res<Assets<Font>>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_cx: ResMut<FontCx>,
    mut layout_cx: ResMut<LayoutCx>,
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
        let parent_bounds =
            parent_label_bounds(entity, &label, &node, &parents_query, &parent_query);
        let bounds_changed = parent_bounds_changed(&cache, parent_bounds);

        if !(label.is_changed()
            || node.is_changed()
            || computed_size.is_changed()
            || cache.dirty
            || bounds_changed
            || font_changed)
        {
            continue;
        }

        let text_font = label_text_font(&label);
        let text_layout = label_text_layout(&label);
        let text_color = label.color;
        let bounds = label_measure_bounds(&label, &node, Some(&computed_size), parent_bounds);

        match measure_layout_for_text(
            entity,
            label.text.as_str(),
            TextMeasureStage::FullText,
            &text_font,
            &text_layout,
            text_color,
            bounds,
            &fonts,
            &mut text_pipeline,
            &mut computed,
            &mut font_cx,
            &mut layout_cx,
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
                    &mut font_cx,
                    &mut layout_cx,
                    measured,
                ) else {
                    reset_text_label_layout_cache(&mut cache);
                    continue;
                };

                let next_cache = next_text_label_layout_cache(
                    final_text,
                    final_measured,
                    parent_bounds,
                    overflowed,
                );
                if *cache != next_cache {
                    *cache = next_cache;
                }
            }
            Err(error) => {
                log_text_measure_error(error);
                reset_text_label_layout_cache(&mut cache);
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
    let autosize_snapshot: Vec<_> = {
        let labels = params.p1();
        labels
            .iter()
            .filter_map(|(entity, label, node, _)| {
                label.autosize.then_some((
                    entity,
                    parents_query.get(entity).ok().map(|parent| parent.get()),
                    label.overflow,
                    node.margin,
                ))
            })
            .collect()
    };

    if autosize_snapshot.is_empty() {
        return;
    }

    for (entity, parent_entity, overflow, margin) in autosize_snapshot {
        let parent_bounds = {
            let parent_bounds_query = params.p0();
            parent_label_bounds_from_ids(parent_entity, overflow, margin, &parent_bounds_query)
        };

        let mut labels = params.p1();
        let Ok((_, _, mut node, layout_cache)) = labels.get_mut(entity) else {
            continue;
        };

        let outer_size = measured_text_outer_size(&node, layout_cache);
        let clamped_outer_size = clamp_outer_size_to_bounds(outer_size, parent_bounds);
        let target_width = clamped_outer_size.x;
        let target_height = clamped_outer_size.y;

        let current_w = match node.width {
            UVal::Px(value) => value,
            _ => -1.0,
        };
        let current_h = match node.height {
            UVal::Px(value) => value,
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
