use super::*;
use std::collections::HashMap;
use univis_ui_engine::layout::invalidation::{UiInvalidateRequestQueue, UiLayoutInvalidation};

use super::measurement::MeasuredTextInfo;

pub(super) fn next_text_label_layout_cache(
    displayed_text: String,
    measured: MeasuredTextInfo,
    parent_bounds: TextBounds,
    overflowed: bool,
) -> UTextLabelLayoutCache {
    UTextLabelLayoutCache {
        measured_size: measured.size,
        min_content_size: measured.min_content_size,
        max_content_size: measured.max_content_size,
        parent_bound_width: parent_bounds.width,
        parent_bound_height: parent_bounds.height,
        displayed_text,
        line_count: measured.line_count,
        overflowed,
        dirty: false,
    }
}

pub(super) fn reset_text_label_layout_cache(cache: &mut UTextLabelLayoutCache) {
    let next = UTextLabelLayoutCache::default();
    if *cache != next {
        *cache = next;
    }
}

fn optional_bound_changed(previous: Option<f32>, current: Option<f32>) -> bool {
    match (previous, current) {
        (Some(previous), Some(current)) => (previous - current).abs() > 0.1,
        (None, None) => false,
        _ => true,
    }
}

pub(super) fn parent_bounds_changed(
    cache: &UTextLabelLayoutCache,
    parent_bounds: TextBounds,
) -> bool {
    optional_bound_changed(cache.parent_bound_width, parent_bounds.width)
        || optional_bound_changed(cache.parent_bound_height, parent_bounds.height)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TextLabelIntrinsicSignature {
    width: f32,
    height: f32,
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
}

impl From<IntrinsicSize> for TextLabelIntrinsicSignature {
    fn from(intrinsic: IntrinsicSize) -> Self {
        Self {
            width: intrinsic.width,
            height: intrinsic.height,
            min_width: intrinsic.min_width,
            max_width: intrinsic.max_width,
            min_height: intrinsic.min_height,
            max_height: intrinsic.max_height,
        }
    }
}

fn intrinsic_signature_changed(
    previous: Option<TextLabelIntrinsicSignature>,
    current: TextLabelIntrinsicSignature,
) -> bool {
    let Some(previous) = previous else {
        return true;
    };

    (previous.width - current.width).abs() > 0.001
        || (previous.height - current.height).abs() > 0.001
        || (previous.min_width - current.min_width).abs() > 0.001
        || (previous.max_width - current.max_width).abs() > 0.001
        || (previous.min_height - current.min_height).abs() > 0.001
        || (previous.max_height - current.max_height).abs() > 0.001
}

pub(crate) fn mark_text_label_layout_dirty(
    mut invalidation_requests: ResMut<UiInvalidateRequestQueue>,
    mut last_invalidated_intrinsics: Local<HashMap<Entity, TextLabelIntrinsicSignature>>,
    changed_labels: Query<
        (Entity, &UTextLabel, &IntrinsicSize),
        (
            With<UTextLabel>,
            Or<(Added<UTextLabel>, Changed<IntrinsicSize>)>,
        ),
    >,
    mut removed_labels: RemovedComponents<UTextLabel>,
) {
    for entity in removed_labels.read() {
        last_invalidated_intrinsics.remove(&entity);
    }

    for (entity, label, intrinsic) in changed_labels.iter() {
        if label.autosize {
            continue;
        }

        let signature = TextLabelIntrinsicSignature::from(*intrinsic);
        if !intrinsic_signature_changed(
            last_invalidated_intrinsics.get(&entity).copied(),
            signature,
        ) {
            continue;
        }

        invalidation_requests.request(entity, UiLayoutInvalidation::intrinsic_change());
        last_invalidated_intrinsics.insert(entity, signature);
    }
}
