use super::*;

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

pub(crate) fn mark_text_label_layout_dirty(
    work_state: Option<Res<UiWorkState>>,
    mut layout_cache: ResMut<LayoutCache>,
    changed_labels: Query<
        Entity,
        (
            With<UTextLabel>,
            Or<(Added<UTextLabel>, Changed<IntrinsicSize>, Changed<UNode>)>,
        ),
    >,
    parents_query: Query<&ChildOf>,
) {
    let current_generation = work_state
        .as_ref()
        .map_or(0, |state| state.current_generation());
    for entity in changed_labels.iter() {
        layout_cache.mark_dirty(entity);
        layout_cache.mark_dirty_ancestors(entity, &parents_query);
        layout_cache.mark_measure_dirty(entity, current_generation);
        layout_cache.mark_measure_dirty_ancestors(entity, current_generation, &parents_query);
        layout_cache.mark_solve_dirty_generation(entity, current_generation);
        layout_cache.mark_solve_dirty_ancestors_generation(
            entity,
            current_generation,
            &parents_query,
        );
    }
}
