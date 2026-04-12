use super::*;
use crate::layout::invalidation::UiLayoutInvalidation as LayoutInvalidation;

pub(super) fn apply_layout_invalidation(
    cache: &mut LayoutCache,
    entity: Entity,
    current_generation: u64,
    invalidation: LayoutInvalidation,
    parents_query: &Query<&ChildOf>,
) {
    if invalidation.dirty_self {
        cache.mark_dirty(entity);
    }

    if invalidation.dirty_ancestors {
        cache.mark_dirty_ancestors(entity, parents_query);
    }

    if invalidation.measure_self {
        cache.mark_measure_dirty(entity, current_generation);
    }

    if invalidation.measure_ancestors {
        cache.mark_measure_dirty_ancestors(entity, current_generation, parents_query);
    }

    if invalidation.solve_self {
        cache.mark_solve_dirty_generation(entity, current_generation);
    }

    if invalidation.solve_ancestors {
        cache.mark_solve_dirty_ancestors_generation(entity, current_generation, parents_query);
    }

    if invalidation.render_self {
        cache.mark_render_dirty(entity, current_generation);
    }
}

pub(super) fn classify_node_mutation(
    previous: Option<&UNode>,
    current: &UNode,
    has_children: bool,
    has_parent: bool,
) -> LayoutInvalidation {
    let Some(previous) = previous else {
        return LayoutInvalidation {
            dirty_self: true,
            dirty_ancestors: has_parent,
            measure_self: true,
            measure_ancestors: has_parent,
            solve_self: has_children || !has_parent,
            solve_ancestors: has_parent,
            render_self: true,
        };
    };

    let render_changed = previous.background_color != current.background_color
        || previous.border_radius != current.border_radius
        || previous.shape_mode != current.shape_mode;
    let intrinsic_inputs_changed = previous.width != current.width
        || previous.height != current.height
        || previous.min_width != current.min_width
        || previous.max_width != current.max_width
        || previous.min_height != current.min_height
        || previous.max_height != current.max_height
        || previous.padding != current.padding;
    let margin_changed = previous.margin != current.margin;

    LayoutInvalidation {
        dirty_self: intrinsic_inputs_changed,
        dirty_ancestors: intrinsic_inputs_changed || margin_changed,
        measure_self: intrinsic_inputs_changed,
        measure_ancestors: intrinsic_inputs_changed || margin_changed,
        solve_self: intrinsic_inputs_changed && (has_children || !has_parent),
        solve_ancestors: intrinsic_inputs_changed || margin_changed,
        render_self: render_changed,
    }
}

pub(super) fn classify_layout_mutation(
    previous: Option<&ULayout>,
    current: &ULayout,
    has_children: bool,
    has_parent: bool,
) -> LayoutInvalidation {
    let changed = previous.is_none_or(|previous| previous != current);
    if !changed || !has_children {
        return LayoutInvalidation::default();
    }

    LayoutInvalidation {
        dirty_self: true,
        dirty_ancestors: has_parent,
        measure_self: true,
        measure_ancestors: has_parent,
        solve_self: true,
        solve_ancestors: has_parent,
        render_self: false,
    }
}

pub(super) fn classify_uself_mutation(
    previous: Option<&USelf>,
    current: &USelf,
    has_children: bool,
    has_parent: bool,
) -> LayoutInvalidation {
    let changed = previous.is_none_or(|previous| previous != current);
    if !changed {
        return LayoutInvalidation::default();
    }

    let position_type_changed =
        previous.is_none_or(|previous| previous.position_type != current.position_type);

    LayoutInvalidation {
        dirty_self: false,
        dirty_ancestors: position_type_changed && has_parent,
        measure_self: false,
        measure_ancestors: position_type_changed && has_parent,
        solve_self: !has_parent || (position_type_changed && has_children),
        solve_ancestors: has_parent,
        render_self: false,
    }
}

pub(super) fn classify_children_mutation(has_parent: bool) -> LayoutInvalidation {
    LayoutInvalidation {
        dirty_self: true,
        dirty_ancestors: has_parent,
        measure_self: true,
        measure_ancestors: has_parent,
        solve_self: true,
        solve_ancestors: has_parent,
        render_self: false,
    }
}

pub(super) fn root_canvas_changed(previous: ResolvedRootUi, current: ResolvedRootUi) -> bool {
    previous.canvas_size != current.canvas_size
}

pub(super) fn root_projection_context_changed(
    previous: ResolvedRootUi,
    current: ResolvedRootUi,
) -> bool {
    previous.space != current.space
        || previous.ui_units_to_world_scale() != current.ui_units_to_world_scale()
}

pub(super) fn root_render_context_changed(
    previous: ResolvedRootUi,
    current: ResolvedRootUi,
) -> bool {
    previous.resolution_scale != current.resolution_scale
}

pub(super) fn mark_render_dirty_recursive(
    cache: &mut LayoutCache,
    entity: Entity,
    generation: u64,
    children_query: &Query<&Children>,
) {
    cache.mark_render_dirty(entity, generation);

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            mark_render_dirty_recursive(cache, child, generation, children_query);
        }
    }
}
