use super::*;

fn constrained_node_dimension(spec: &UVal, computed: f32, padding: f32) -> Option<f32> {
    match spec {
        UVal::Px(value) => Some((*value - padding).max(0.0)),
        _ if !spec.uses_intrinsic_measurement() && computed > 0.0 => {
            Some((computed - padding).max(0.0))
        }
        _ => None,
    }
}

fn node_uses_intrinsic_dimension(spec: &UVal) -> bool {
    spec.uses_intrinsic_measurement()
}

fn resolve_parent_dimension_bound(
    spec: &UVal,
    computed: f32,
    parent_padding: f32,
    node_margin: f32,
) -> Option<f32> {
    let total_spacing = parent_padding + node_margin;
    match *spec {
        UVal::Px(value) => Some((value - total_spacing).max(0.0)),
        _ if spec.uses_intrinsic_measurement() => None,
        _ if computed > 0.0 => Some((computed - total_spacing).max(0.0)),
        _ => None,
    }
}

pub(super) fn parent_label_bounds(
    entity: Entity,
    label: &UTextLabel,
    node: &UNode,
    parents_query: &Query<&ChildOf>,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if label.overflow == UTextOverflow::Visible && !label.is_wrapping() {
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

    let width = resolve_parent_dimension_bound(
        &parent_node.width,
        parent_size.width,
        parent_node.padding.width_sum(),
        node.margin.width_sum(),
    );

    let height = if label.overflow == UTextOverflow::Visible
        || (label.is_wrapping() && label.max_lines.is_none() && parent_node.height.uses_intrinsic_measurement())
    {
        None
    } else {
        resolve_parent_dimension_bound(
            &parent_node.height,
            parent_size.height,
            parent_node.padding.height_sum(),
            node.margin.height_sum(),
        )
    };

    TextBounds { width, height }
}

pub(super) fn parent_label_bounds_from_ids(
    parent_entity: Option<Entity>,
    overflow: UTextOverflow,
    is_wrapping: bool,
    margin: USides,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if overflow == UTextOverflow::Visible && !is_wrapping {
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

    let width = resolve_parent_dimension_bound(
        &parent_node.width,
        parent_size.width,
        parent_node.padding.width_sum(),
        margin.width_sum(),
    );

    let height = if overflow == UTextOverflow::Visible
        || (is_wrapping && parent_node.height.uses_intrinsic_measurement())
    {
        None
    } else {
        resolve_parent_dimension_bound(
            &parent_node.height,
            parent_size.height,
            parent_node.padding.height_sum(),
            margin.height_sum(),
        )
    };

    TextBounds { width, height }
}

pub(super) fn clamp_outer_size_to_bounds(outer_size: Vec2, bounds: TextBounds) -> Vec2 {
    Vec2::new(
        bounds
            .width
            .map_or(outer_size.x, |width| outer_size.x.min(width)),
        bounds
            .height
            .map_or(outer_size.y, |height| outer_size.y.min(height)),
    )
}

pub(crate) fn measured_text_outer_size(node: &UNode, layout_cache: &UTextLabelLayoutCache) -> Vec2 {
    Vec2::new(
        layout_cache.measured_size.x.max(0.0) + node.padding.width_sum(),
        layout_cache.measured_size.y.max(0.0) + node.padding.height_sum(),
    )
}

fn measured_text_outer_bounds(
    label: &UTextLabel,
    node: &UNode,
    layout_cache: &UTextLabelLayoutCache,
) -> (Vec2, Vec2) {
    if label.is_wrapping() {
        (
            Vec2::new(
                layout_cache.min_content_size.x.max(0.0) + node.padding.width_sum(),
                layout_cache.measured_size.y.max(0.0) + node.padding.height_sum(),
            ),
            Vec2::new(
                layout_cache.measured_size.x.max(0.0) + node.padding.width_sum(),
                layout_cache.measured_size.y.max(0.0) + node.padding.height_sum(),
            ),
        )
    } else {
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
}

pub(crate) fn desired_text_label_intrinsic_size(
    label: &UTextLabel,
    node: &UNode,
    layout_cache: &UTextLabelLayoutCache,
    current: IntrinsicSize,
) -> IntrinsicSize {
    let (min_outer_size, max_outer_size) = measured_text_outer_bounds(label, node, layout_cache);

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

pub(crate) fn label_measure_bounds(
    label: &UTextLabel,
    node: &UNode,
    computed_size: Option<&ComputedSize>,
    parent_bounds: TextBounds,
) -> TextBounds {
    let computed_width = computed_size.map_or(0.0, |size| size.width);
    let computed_height = computed_size.map_or(0.0, |size| size.height);

    let node_width =
        constrained_node_dimension(&node.width, computed_width, node.padding.width_sum());
    let node_height =
        constrained_node_dimension(&node.height, computed_height, node.padding.height_sum());

    let constrain_width = label.is_wrapping()
        || label.overflow != UTextOverflow::Visible
        || label.effective_max_lines().is_some()
        || node_width.is_some();

    let width = if constrain_width {
        match (node_width, parent_bounds.width) {
            (Some(nw), Some(pw)) => Some(nw.min(pw)),
            (Some(nw), None) => Some(nw),
            (None, Some(pw)) => Some(pw),
            (None, None) => None,
        }
    } else {
        None
    };

    let constrain_height = (!label.autosize && label.overflow != UTextOverflow::Visible)
        || node_height.is_some()
        || (label.overflow != UTextOverflow::Visible && parent_bounds.height.is_some());

    let height = if constrain_height {
        match (node_height, parent_bounds.height) {
            (Some(nh), Some(ph)) => Some(nh.min(ph)),
            (Some(nh), None) => Some(nh),
            (None, Some(ph)) if label.overflow != UTextOverflow::Visible => Some(ph),
            _ => None,
        }
    } else {
        None
    };

    TextBounds { width, height }
}
