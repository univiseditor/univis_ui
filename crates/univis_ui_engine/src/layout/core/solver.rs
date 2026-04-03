use crate::internal_prelude::*;
use bevy::prelude::*;

// =========================================================
// 1. Data Structures
// =========================================================

/// The output result of the solver for a single item.
#[derive(Debug, Clone, Copy, Default)]
pub struct SolverResult {
    pub size: Vec2,
    pub pos: Vec2,
}

/// Combines spec, result, and margin for processing.
pub struct SolverItem<'a> {
    pub spec: SolverSpec,
    pub result: &'a mut SolverResult,
    pub margin: USides,
}

/// Configuration for the solver run (Container properties).
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub layout: ULayout,
    pub gap: f32,
    pub row_gap: Option<f32>,
    pub column_gap: Option<f32>,
    pub padding: USides,
    pub grid_columns: u32,
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
    pub flex_wrap: UFlexWrap,
    pub flex_align_content: Option<UContentAlignExt>,
    pub grid_template_columns: Vec<UTrackSize>,
    pub grid_template_rows: Vec<UTrackSize>,
    pub grid_auto_flow: UGridAutoFlow,
    pub grid_auto_rows: UTrackSize,
    pub grid_auto_columns: UTrackSize,

    // Width/Height modes to determine sizing constraints
    pub width_mode: SolverSizeMode,
    pub height_mode: SolverSizeMode,
}

/// Context passed to Placers to help position items.
pub struct PlacementContext {
    pub container_main_size: f32,
    pub container_cross_size: f32,
    pub padding_main_start: f32,
    pub padding_main_end: f32,
    pub padding_cross_start: f32,

    pub gap: f32,
    pub main_gap: f32,
    pub cross_gap: f32,
    pub justify_content: UJustifyContent,
    pub align_items: UAlignItems,
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
    pub flex_wrap: UFlexWrap,
    pub grid_columns: u32,
    pub grid_template_columns: Vec<UTrackSize>,
    pub grid_template_rows: Vec<UTrackSize>,
    pub grid_auto_flow: UGridAutoFlow,
    pub grid_auto_rows: UTrackSize,
    pub grid_auto_columns: UTrackSize,
}

fn map_align_items_to_ext(align: UAlignItems) -> UAlignItemsExt {
    match align {
        UAlignItems::Center => UAlignItemsExt::Center,
        UAlignItems::End | UAlignItems::FlexEnd => UAlignItemsExt::End,
        UAlignItems::Stretch => UAlignItemsExt::Stretch,
        UAlignItems::Baseline => UAlignItemsExt::Baseline,
        _ => UAlignItemsExt::Start,
    }
}

fn map_align_self_to_ext(align: UAlignSelf) -> UAlignSelfExt {
    match align {
        UAlignSelf::Center => UAlignSelfExt::Center,
        UAlignSelf::End => UAlignSelfExt::End,
        UAlignSelf::Stretch => UAlignSelfExt::Stretch,
        UAlignSelf::Auto => UAlignSelfExt::Auto,
        UAlignSelf::Start => UAlignSelfExt::Start,
    }
}

fn resolve_flex_basis(basis: UVal, default_content: f32, available_main: f32) -> Option<f32> {
    match basis {
        UVal::Px(v) => Some(v.max(0.0)),
        UVal::Percent(p) => Some((p * available_main).max(0.0)),
        UVal::Content | UVal::MinContent | UVal::MaxContent => Some(default_content.max(0.0)),
        UVal::Auto | UVal::Flex(_) => None,
    }
}

fn main_bounds_for_spec(spec: &SolverSpec, axis: &AxisHelper) -> (f32, f32) {
    if axis.is_row() {
        (spec.min_width, spec.max_width)
    } else {
        (spec.min_height, spec.max_height)
    }
}

fn cross_bounds_for_spec(spec: &SolverSpec, axis: &AxisHelper) -> (f32, f32) {
    if axis.is_row() {
        (spec.min_height, spec.max_height)
    } else {
        (spec.min_width, spec.max_width)
    }
}

fn clamp_main_size(spec: &SolverSpec, axis: &AxisHelper, size: f32) -> f32 {
    let (min, max) = main_bounds_for_spec(spec, axis);
    size.clamp(min, max)
}

fn clamp_cross_size(spec: &SolverSpec, axis: &AxisHelper, size: f32) -> f32 {
    let (min, max) = cross_bounds_for_spec(spec, axis);
    size.clamp(min, max)
}

fn flex_grow_factor(spec: &SolverSpec, legacy_main_flex_factor: f32) -> f32 {
    let ext_grow = spec.flex_grow.unwrap_or(0.0).max(0.0);
    if ext_grow > 0.0 {
        ext_grow
    } else {
        legacy_main_flex_factor.max(0.0)
    }
}

fn flex_shrink_factor(spec: &SolverSpec) -> f32 {
    spec.flex_shrink.unwrap_or(1.0).max(0.0)
}

fn allows_implicit_stretch(mode: SolverSizeMode) -> bool {
    matches!(mode, SolverSizeMode::Auto)
}

fn allows_explicit_stretch(mode: SolverSizeMode) -> bool {
    !matches!(mode, SolverSizeMode::Fixed | SolverSizeMode::Percent)
}

fn has_explicit_align_self_override(spec: &SolverSpec) -> bool {
    matches!(
        spec.align_self_ext,
        Some(
            UAlignSelfExt::Start
                | UAlignSelfExt::End
                | UAlignSelfExt::Center
                | UAlignSelfExt::Stretch
                | UAlignSelfExt::FlexStart
                | UAlignSelfExt::FlexEnd
                | UAlignSelfExt::SelfStart
                | UAlignSelfExt::SelfEnd
                | UAlignSelfExt::Left
                | UAlignSelfExt::Right
        )
    ) || spec.align_self.is_some()
}

fn is_ext_stretch(value: UAlignSelfExt) -> bool {
    matches!(value, UAlignSelfExt::Stretch)
}

// =========================================================
// 3. The Core Engine
// =========================================================

/// The main layout logic. Calculates sizes and positions for a list of items.
///
/// It handles:
/// 1. Separation of absolute/relative items.
/// 2. Main axis sizing (including Flex Grow).
/// 3. Cross axis sizing (including Stretch).
/// 4. Delegating placement to the specific algorithm (Bridge).
/// 5. Handling absolute positioning.
pub fn solve_flex_layout(
    config: &SolverConfig,
    constraints: BoxConstraints,
    items: &mut [SolverItem],
) -> Vec2 {
    let axis = AxisHelper::new(config.layout.flex_direction);

    // 1. Separate Normal / Absolute (Check position_type)
    let mut normal_indices = Vec::new();
    let mut absolute_indices = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if item.spec.position_type == UPositionType::Absolute {
            absolute_indices.push(i);
        } else {
            normal_indices.push(i);
        }
    }
    normal_indices.sort_by_key(|&i| items[i].spec.order);

    // 2. Prepare Constraints
    let (min_main, max_main, min_cross, max_cross) = axis.extract_constraints(constraints);
    let padding = axis.extract_padding(config.padding);
    let available_main = (max_main - padding.main).max(0.0);
    let main_gap = if axis.is_row() {
        config.column_gap.unwrap_or(config.gap)
    } else {
        config.row_gap.unwrap_or(config.gap)
    };
    let cross_gap = if axis.is_row() {
        config.row_gap.unwrap_or(config.gap)
    } else {
        config.column_gap.unwrap_or(config.gap)
    };

    // 3. Calculate Sizes (Flexbox Sizing Loop)
    let mut used_main = 0.0;

    for &idx in &normal_indices {
        let item = &mut items[idx];
        let (m_start, m_end, _, _) = axis.extract_margin_sides(item.margin);
        let margin_span = m_start + m_end;
        let (main_mode, main_val, main_flex_factor) = axis.get_main_spec(&item.spec);
        let grow_factor = flex_grow_factor(&item.spec, main_flex_factor);

        let mut base_size = match main_mode {
            SolverSizeMode::Fixed => main_val,
            SolverSizeMode::Percent => main_val * available_main,
            SolverSizeMode::Flex => 0.0,
            SolverSizeMode::MinContent => main_val,
            SolverSizeMode::Content => main_val,
            SolverSizeMode::Auto => main_val,
        };
        if let Some(basis) = item.spec.flex_basis
            && let Some(resolved_basis) = resolve_flex_basis(basis, base_size, available_main)
        {
            base_size = resolved_basis;
        }
        base_size = clamp_main_size(&item.spec, &axis, base_size);

        let _ = grow_factor;
        used_main += base_size + margin_span;
        item.result.size = axis.to_world(base_size, 0.0);
    }

    if normal_indices.len() > 1 {
        used_main += (normal_indices.len() as f32 - 1.0) * main_gap;
    }

    // 4. Apply Flex Grow
    let positive_free_space = (available_main - used_main).max(0.0);
    if positive_free_space > 0.0 {
        let mut remaining_free_space = positive_free_space;
        let mut growable_indices: Vec<usize> = normal_indices
            .iter()
            .copied()
            .filter(|&idx| {
                let item = &items[idx];
                let (_, _, legacy_main_flex_factor) = axis.get_main_spec(&item.spec);
                let grow_factor = flex_grow_factor(&item.spec, legacy_main_flex_factor);
                if grow_factor <= 0.0 {
                    return false;
                }

                let current_main = axis.from_world(item.result.size).0;
                let (_, max_main) = main_bounds_for_spec(&item.spec, &axis);
                current_main + 0.0001 < max_main
            })
            .collect();

        while remaining_free_space > 0.0001 && !growable_indices.is_empty() {
            let total_grow: f32 = growable_indices
                .iter()
                .map(|&idx| {
                    let item = &items[idx];
                    let (_, _, legacy_main_flex_factor) = axis.get_main_spec(&item.spec);
                    flex_grow_factor(&item.spec, legacy_main_flex_factor)
                })
                .sum();

            if total_grow <= 0.0 {
                break;
            }

            let mut distributed = 0.0;
            let mut next_growable = Vec::new();

            for &idx in &growable_indices {
                let item = &mut items[idx];
                let (_, _, legacy_main_flex_factor) = axis.get_main_spec(&item.spec);
                let grow_factor = flex_grow_factor(&item.spec, legacy_main_flex_factor);
                let (current_main, current_cross) = axis.from_world(item.result.size);
                let (_, max_main) = main_bounds_for_spec(&item.spec, &axis);
                let grow_share = remaining_free_space * (grow_factor / total_grow);
                let next_main = (current_main + grow_share).min(max_main);
                let added = next_main - current_main;

                if added > 0.0 {
                    item.result.size = axis.to_world(next_main, current_cross);
                    used_main += added;
                    distributed += added;
                }

                if next_main + 0.0001 < max_main {
                    next_growable.push(idx);
                }
            }

            if distributed <= 0.0001 {
                break;
            }

            remaining_free_space = (remaining_free_space - distributed).max(0.0);
            growable_indices = next_growable;
        }
    }

    // 4b. Apply Flex Shrink if content overflows the main axis.
    let overflow = (used_main - available_main).max(0.0);
    if overflow > 0.0 {
        let mut remaining_overflow = overflow;
        let mut shrinkable_indices: Vec<usize> = normal_indices
            .iter()
            .copied()
            .filter(|&idx| {
                let item = &items[idx];
                let shrink_factor = flex_shrink_factor(&item.spec);
                if shrink_factor <= 0.0 {
                    return false;
                }

                let current_main = axis.from_world(item.result.size).0;
                let (min_main, _) = main_bounds_for_spec(&item.spec, &axis);
                current_main > min_main + 0.0001
            })
            .collect();

        while remaining_overflow > 0.0001 && !shrinkable_indices.is_empty() {
            let total_shrink_weight: f32 = shrinkable_indices
                .iter()
                .map(|&idx| {
                    let item = &items[idx];
                    let current_main = axis.from_world(item.result.size).0;
                    current_main.max(1.0) * flex_shrink_factor(&item.spec)
                })
                .sum();

            if total_shrink_weight <= 0.0 {
                break;
            }

            let mut absorbed = 0.0;
            let mut next_shrinkable = Vec::new();

            for &idx in &shrinkable_indices {
                let item = &mut items[idx];
                let (current_main, current_cross) = axis.from_world(item.result.size);
                let (min_main, _) = main_bounds_for_spec(&item.spec, &axis);
                let shrink_weight = current_main.max(1.0) * flex_shrink_factor(&item.spec);
                let shrink_share = remaining_overflow * (shrink_weight / total_shrink_weight);
                let next_main = (current_main - shrink_share).max(min_main);
                let reduced = current_main - next_main;

                if reduced > 0.0 {
                    item.result.size = axis.to_world(next_main, current_cross);
                    used_main -= reduced;
                    absorbed += reduced;
                }

                if next_main > min_main + 0.0001 {
                    next_shrinkable.push(idx);
                }
            }

            if absorbed <= 0.0001 {
                break;
            }

            remaining_overflow = (remaining_overflow - absorbed).max(0.0);
            shrinkable_indices = next_shrinkable;
        }
    }

    // 5. Cross Axis Sizing
    let available_cross = (max_cross - padding.cross).max(0.0);
    let mut max_child_cross: f32 = 0.0;
    for &idx in &normal_indices {
        let item = &mut items[idx];
        let (cross_mode, cross_val, _) = axis.get_cross_spec(&item.spec);
        let (_, _, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

        let mut child_cross = match cross_mode {
            SolverSizeMode::Fixed => cross_val,
            SolverSizeMode::Percent => cross_val * available_cross,
            SolverSizeMode::Flex => available_cross,
            SolverSizeMode::MinContent => cross_val,
            SolverSizeMode::Content => cross_val,
            SolverSizeMode::Auto => cross_val,
        };

        let ext_self = item
            .spec
            .align_self_ext
            .or_else(|| item.spec.align_self.map(map_align_self_to_ext));
        let container_ext_align = map_align_items_to_ext(config.layout.align_items);
        let has_explicit_align_override = has_explicit_align_self_override(&item.spec);
        let should_stretch = match ext_self {
            Some(UAlignSelfExt::Auto | UAlignSelfExt::Normal) | None => {
                container_ext_align == UAlignItemsExt::Stretch
            }
            Some(value) => is_ext_stretch(value),
        };
        let stretch_allowed = if has_explicit_align_override {
            allows_explicit_stretch(cross_mode)
        } else {
            allows_implicit_stretch(cross_mode)
        };

        if should_stretch && stretch_allowed {
            child_cross = (available_cross - m_cross_start - m_cross_end).max(0.0);
        }
        child_cross = clamp_cross_size(&item.spec, &axis, child_cross);

        max_child_cross = max_child_cross.max(child_cross + m_cross_start + m_cross_end);
        let current_main = axis.from_world(item.result.size).0;
        item.result.size = axis.to_world(current_main, child_cross);
    }

    // 6. Initial Container Size
    let container_main = (used_main + padding.main).clamp(min_main, max_main);
    let fixed_cross = (constraints.min_width == constraints.max_width
        && config.layout.flex_direction == UFlexDirection::Column)
        || (constraints.min_height == constraints.max_height
            && config.layout.flex_direction == UFlexDirection::Row);
    let container_cross = if fixed_cross {
        max_cross
    } else {
        max_child_cross + padding.cross
    };
    let final_cross = container_cross.clamp(min_cross, max_cross);

    // 7. Placement (Using the Placer Bridge)
    let (p_main_start, p_main_end, p_cross_start, _) = axis.extract_margin_sides(config.padding);
    let placement_ctx = PlacementContext {
        container_main_size: container_main,
        container_cross_size: final_cross,
        padding_main_start: p_main_start,
        padding_main_end: p_main_end,
        padding_cross_start: p_cross_start,
        gap: config.gap,
        main_gap,
        cross_gap,
        justify_content: config.layout.justify_content,
        align_items: config.layout.align_items,
        justify_items: config.justify_items,
        align_content: config.flex_align_content.or(config.align_content),
        flex_wrap: config.flex_wrap,
        grid_columns: config.grid_columns,
        grid_template_columns: config.grid_template_columns.clone(),
        grid_template_rows: config.grid_template_rows.clone(),
        grid_auto_flow: config.grid_auto_flow,
        grid_auto_rows: config.grid_auto_rows,
        grid_auto_columns: config.grid_auto_columns,
    };

    // Receive actual size from the placer
    let used_size_from_placer = final_size_with_indices(
        config.layout.clone(),
        items,
        &normal_indices,
        &axis,
        &placement_ctx,
        container_main,
        final_cross,
    );

    // 8. Update Final Container Size (Override logic)
    let mut final_container_size = axis.to_world(container_main, final_cross); // Default
    let placer_size_world = axis.to_world(used_size_from_placer.x, used_size_from_placer.y);

    if matches!(
        config.width_mode,
        SolverSizeMode::Content | SolverSizeMode::MinContent | SolverSizeMode::Auto
    ) {
        final_container_size.x = placer_size_world
            .x
            .clamp(constraints.min_width, constraints.max_width);
    }
    if matches!(
        config.height_mode,
        SolverSizeMode::Content | SolverSizeMode::MinContent | SolverSizeMode::Auto
    ) {
        final_container_size.y = placer_size_world
            .y
            .clamp(constraints.min_height, constraints.max_height);
    }

    // 9. Relative Offsets
    for &idx in &normal_indices {
        let item = &mut items[idx];
        if item.spec.position_type == UPositionType::Relative {
            let offset_x = item.spec.left.resolve_or_zero(final_container_size.x)
                - item.spec.right.resolve_or_zero(final_container_size.x);
            let offset_y = item.spec.top.resolve_or_zero(final_container_size.y)
                - item.spec.bottom.resolve_or_zero(final_container_size.y);
            item.result.pos.x += offset_x;
            item.result.pos.y += offset_y;
        }
    }

    // 10. Handle Absolute Items
    for &idx in &absolute_indices {
        let item = &mut items[idx];
        let intrinsic = Vec2::new(
            if matches!(
                item.spec.width_mode,
                SolverSizeMode::Content | SolverSizeMode::MinContent
            ) {
                item.spec.width_val
            } else {
                0.0
            },
            if matches!(
                item.spec.height_mode,
                SolverSizeMode::Content | SolverSizeMode::MinContent
            ) {
                item.spec.height_val
            } else {
                0.0
            },
        );
        let (new_size, new_pos) =
            solve_absolute_box(final_container_size, &item.spec, item.margin, intrinsic);
        item.result.size = new_size;
        item.result.pos = new_pos;
    }

    final_container_size
}

// 2. Spec Translation Helper
pub fn translate_spec(node: &UNode, uself: Option<&USelf>) -> SolverSpec {
    let map_dim = |dim: UVal| -> (SolverSizeMode, f32, f32) {
        match dim {
            UVal::Px(v) => (SolverSizeMode::Fixed, v, 0.0),
            UVal::Percent(p) => (SolverSizeMode::Percent, p, 0.0),
            UVal::Flex(f) => (SolverSizeMode::Flex, 0.0, f),
            UVal::MinContent => (SolverSizeMode::MinContent, 0.0, 0.0),
            UVal::Content | UVal::MaxContent => (SolverSizeMode::Content, 0.0, 0.0),
            UVal::Auto => (SolverSizeMode::Auto, 0.0, 0.0),
        }
    };

    let (w_mode, w_val, w_flex) = map_dim(node.width);
    let (h_mode, h_val, h_flex) = map_dim(node.height);
    let (min_width, max_width) = node.width_bounds();
    let (min_height, max_height) = node.height_bounds();

    let (pos_type, l, r, t, b, align, order) = if let Some(u) = uself {
        (
            u.position_type,
            u.left,
            u.right,
            u.top,
            u.bottom,
            if u.align_self == UAlignSelf::Auto {
                None
            } else {
                Some(u.align_self)
            },
            u.order,
        )
    } else {
        (
            UPositionType::Relative,
            UVal::Auto,
            UVal::Auto,
            UVal::Auto,
            UVal::Auto,
            None,
            0,
        )
    };
    let (align_self_ext, justify_self_ext, justify_overflow, align_overflow) =
        if let Some(u) = uself {
            (
                u.item_ext.box_align.align_self,
                u.item_ext.box_align.justify_self,
                u.item_ext.box_align.justify_overflow,
                u.item_ext.box_align.align_overflow,
            )
        } else {
            (
                None,
                None,
                UOverflowPosition::Unsafe,
                UOverflowPosition::Unsafe,
            )
        };
    let (flex_grow, flex_shrink, flex_basis) = if let Some(u) = uself {
        (
            u.item_ext.flex.flex_grow.map(|v| v.max(0.0)),
            u.item_ext.flex.flex_shrink.map(|v| v.max(0.0)),
            u.item_ext.flex.flex_basis,
        )
    } else {
        (None, None, None)
    };
    let (grid_column_start, grid_column_span, grid_row_start, grid_row_span) =
        if let Some(u) = uself {
            (
                u.item_ext.grid.column_start,
                u.item_ext.grid.column_span.max(1),
                u.item_ext.grid.row_start,
                u.item_ext.grid.row_span.max(1),
            )
        } else {
            (None, 1, None, 1)
        };

    SolverSpec {
        width_mode: w_mode,
        width_val: w_val,
        width_flex: w_flex,
        min_width,
        max_width,
        height_mode: h_mode,
        height_val: h_val,
        height_flex: h_flex,
        min_height,
        max_height,

        position_type: pos_type,
        left: l,
        right: r,
        top: t,
        bottom: b,
        align_self: align,
        align_self_ext,
        justify_self_ext,
        justify_overflow,
        align_overflow,
        flex_grow,
        flex_shrink,
        flex_basis,
        grid_column_start,
        grid_column_span,
        grid_row_start,
        grid_row_span,
        order,
    }
}

// 4. Isolated Box Solver (for Absolute Positioning)
fn solve_absolute_box(
    container_size: Vec2,
    spec: &SolverSpec,
    margin: USides,
    intrinsic_size: Vec2,
) -> (Vec2, Vec2) {
    // a. Calculate width (with Stretch support)
    let is_h_stretch = !matches!(spec.left, UVal::Auto) && !matches!(spec.right, UVal::Auto);
    let width = if is_h_stretch {
        let l = spec.left.resolve_or_zero(container_size.x);
        let r = spec.right.resolve_or_zero(container_size.x);
        (container_size.x - l - r - margin.left - margin.right).max(0.0)
    } else {
        match spec.width_mode {
            SolverSizeMode::Fixed => spec.width_val,
            SolverSizeMode::Percent => spec.width_val * container_size.x,
            _ => intrinsic_size.x,
        }
    }
    .clamp(spec.min_width, spec.max_width);

    // b. Calculate height (with Stretch support)
    let is_v_stretch = !matches!(spec.top, UVal::Auto) && !matches!(spec.bottom, UVal::Auto);
    let height = if is_v_stretch {
        let t = spec.top.resolve_or_zero(container_size.y);
        let b = spec.bottom.resolve_or_zero(container_size.y);
        (container_size.y - t - b - margin.top - margin.bottom).max(0.0)
    } else {
        match spec.height_mode {
            SolverSizeMode::Fixed => spec.height_val,
            SolverSizeMode::Percent => spec.height_val * container_size.y,
            _ => intrinsic_size.y,
        }
    }
    .clamp(spec.min_height, spec.max_height);

    // c. Calculate X Position
    let x = if let Some(l) = spec.left.resolve(container_size.x) {
        l + margin.left
    } else if let Some(r) = spec.right.resolve(container_size.x) {
        container_size.x - r - width - margin.right
    } else {
        margin.left // Default
    };

    // d. Calculate Y Position
    let y = if let Some(t) = spec.top.resolve(container_size.y) {
        t + margin.top
    } else if let Some(b) = spec.bottom.resolve(container_size.y) {
        container_size.y - b - height - margin.bottom
    } else {
        margin.top
    };

    (Vec2::new(width, height), Vec2::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_solver_config() -> SolverConfig {
        SolverConfig {
            layout: ULayout::default(),
            gap: 0.0,
            row_gap: None,
            column_gap: None,
            padding: USides::default(),
            grid_columns: 1,
            justify_items: None,
            align_content: None,
            flex_wrap: UFlexWrap::NoWrap,
            flex_align_content: None,
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_auto_flow: UGridAutoFlow::Row,
            grid_auto_rows: UTrackSize::Auto,
            grid_auto_columns: UTrackSize::Auto,
            width_mode: SolverSizeMode::Fixed,
            height_mode: SolverSizeMode::Fixed,
        }
    }

    #[test]
    fn resolve_flex_basis_handles_percent() {
        let value = resolve_flex_basis(UVal::Percent(0.5), 10.0, 300.0);
        assert_eq!(value, Some(150.0));
    }

    #[test]
    fn translate_spec_copies_ext_fields() {
        let node = UNode {
            width: UVal::Px(40.0),
            height: UVal::Px(20.0),
            min_width: 12.0,
            max_width: 96.0,
            min_height: 8.0,
            max_height: 72.0,
            ..default()
        };
        let uself = USelf {
            align_self: UAlignSelf::Center,
            item_ext: ULayoutItemExt {
                box_align: ULayoutBoxAlignSelf {
                    justify_self: Some(UAlignSelfExt::End),
                    align_self: Some(UAlignSelfExt::Start),
                    justify_overflow: UOverflowPosition::Safe,
                    align_overflow: UOverflowPosition::Unsafe,
                },
                flex: ULayoutFlexItem {
                    flex_grow: Some(2.0),
                    flex_shrink: Some(0.5),
                    flex_basis: Some(UVal::Px(30.0)),
                },
                grid: ULayoutGridItem {
                    column_start: Some(2),
                    column_span: 3,
                    row_start: Some(1),
                    row_span: 2,
                },
            },
            ..default()
        };

        let spec = translate_spec(&node, Some(&uself));

        assert_eq!(spec.align_self, Some(UAlignSelf::Center));
        assert_eq!(spec.align_self_ext, Some(UAlignSelfExt::Start));
        assert_eq!(spec.justify_self_ext, Some(UAlignSelfExt::End));
        assert_eq!(spec.justify_overflow, UOverflowPosition::Safe);
        assert_eq!(spec.flex_grow, Some(2.0));
        assert_eq!(spec.flex_basis, Some(UVal::Px(30.0)));
        assert_eq!(spec.min_width, 12.0);
        assert_eq!(spec.max_width, 96.0);
        assert_eq!(spec.min_height, 8.0);
        assert_eq!(spec.max_height, 72.0);
        assert_eq!(spec.grid_column_start, Some(2));
        assert_eq!(spec.grid_column_span, 3);
        assert_eq!(spec.grid_row_span, 2);
    }

    #[test]
    fn translate_spec_maps_intrinsic_modes() {
        let node = UNode {
            width: UVal::MinContent,
            height: UVal::Auto,
            ..default()
        };

        let spec = translate_spec(&node, None);

        assert_eq!(spec.width_mode, SolverSizeMode::MinContent);
        assert_eq!(spec.height_mode, SolverSizeMode::Auto);
    }

    #[test]
    fn explicit_max_content_stays_on_content_mode() {
        let node = UNode {
            width: UVal::MaxContent,
            height: UVal::Content,
            ..default()
        };

        let spec = translate_spec(&node, None);

        assert_eq!(spec.width_mode, SolverSizeMode::Content);
        assert_eq!(spec.height_mode, SolverSizeMode::Content);
    }

    #[test]
    fn auto_cross_size_stretches_but_content_cross_size_keeps_intrinsic_value() {
        let mut config = base_solver_config();
        config.layout.align_items = UAlignItems::Stretch;
        let constraints = BoxConstraints::tight(Vec2::new(160.0, 100.0));

        let mut auto_result = SolverResult::default();
        let mut content_result = SolverResult::default();
        let mut items = vec![
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 40.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Auto,
                    height_val: 18.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut auto_result,
                margin: USides::default(),
            },
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 40.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Content,
                    height_val: 18.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut content_result,
                margin: USides::default(),
            },
        ];

        solve_flex_layout(&config, constraints, &mut items);

        assert_eq!(auto_result.size.y, 100.0);
        assert_eq!(content_result.size.y, 18.0);
    }

    #[test]
    fn flex_shrink_respects_min_width_and_redistributes_remaining_overflow() {
        let config = base_solver_config();
        let constraints = BoxConstraints::tight(Vec2::new(100.0, 40.0));
        let mut first = SolverResult::default();
        let mut second = SolverResult::default();
        let mut items = vec![
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 80.0,
                    width_flex: 0.0,
                    min_width: 60.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 20.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut first,
                margin: USides::default(),
            },
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 80.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 20.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut second,
                margin: USides::default(),
            },
        ];

        let solved = solve_flex_layout(&config, constraints, &mut items);

        assert_eq!(solved, Vec2::new(100.0, 40.0));
        assert_eq!(first.size.x, 60.0);
        assert_eq!(second.size.x, 40.0);
    }

    #[test]
    fn flex_grow_respects_max_width_and_redistributes_remaining_space() {
        let config = base_solver_config();
        let constraints = BoxConstraints::tight(Vec2::new(200.0, 40.0));
        let mut first = SolverResult::default();
        let mut second = SolverResult::default();
        let mut items = vec![
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 50.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: 70.0,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 20.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: Some(1.0),
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut first,
                margin: USides::default(),
            },
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 50.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 20.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: Some(1.0),
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut second,
                margin: USides::default(),
            },
        ];

        let solved = solve_flex_layout(&config, constraints, &mut items);

        assert_eq!(solved, Vec2::new(200.0, 40.0));
        assert_eq!(first.size.x, 70.0);
        assert_eq!(second.size.x, 130.0);
    }

    #[test]
    fn wrapped_row_moves_item_to_next_line_when_min_width_blocks_further_shrink() {
        let mut config = base_solver_config();
        config.flex_wrap = UFlexWrap::Wrap;
        let constraints = BoxConstraints::tight(Vec2::new(150.0, 120.0));
        let mut first = SolverResult::default();
        let mut second = SolverResult::default();
        let mut items = vec![
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 100.0,
                    width_flex: 0.0,
                    min_width: 90.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 24.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut first,
                margin: USides::default(),
            },
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 100.0,
                    width_flex: 0.0,
                    min_width: 90.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 24.0,
                    height_flex: 0.0,
                    min_height: 0.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut second,
                margin: USides::default(),
            },
        ];

        solve_flex_layout(&config, constraints, &mut items);

        assert_eq!(first.size.x, 90.0);
        assert_eq!(second.size.x, 90.0);
        assert!(second.pos.y >= first.size.y - 0.1);
    }

    #[test]
    fn wrapped_column_moves_item_to_next_column_when_min_height_blocks_further_shrink() {
        let mut config = base_solver_config();
        config.layout.flex_direction = UFlexDirection::Column;
        config.flex_wrap = UFlexWrap::Wrap;
        let constraints = BoxConstraints::tight(Vec2::new(120.0, 150.0));
        let mut first = SolverResult::default();
        let mut second = SolverResult::default();
        let mut items = vec![
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 24.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 100.0,
                    height_flex: 0.0,
                    min_height: 90.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut first,
                margin: USides::default(),
            },
            SolverItem {
                spec: SolverSpec {
                    width_mode: SolverSizeMode::Fixed,
                    width_val: 24.0,
                    width_flex: 0.0,
                    min_width: 0.0,
                    max_width: f32::INFINITY,
                    height_mode: SolverSizeMode::Fixed,
                    height_val: 100.0,
                    height_flex: 0.0,
                    min_height: 90.0,
                    max_height: f32::INFINITY,
                    position_type: UPositionType::Relative,
                    left: UVal::Auto,
                    right: UVal::Auto,
                    top: UVal::Auto,
                    bottom: UVal::Auto,
                    align_self: None,
                    align_self_ext: None,
                    justify_self_ext: None,
                    justify_overflow: UOverflowPosition::Unsafe,
                    align_overflow: UOverflowPosition::Unsafe,
                    flex_grow: None,
                    flex_shrink: Some(1.0),
                    flex_basis: None,
                    grid_column_start: None,
                    grid_column_span: 1,
                    grid_row_start: None,
                    grid_row_span: 1,
                    order: 0,
                },
                result: &mut second,
                margin: USides::default(),
            },
        ];

        solve_flex_layout(&config, constraints, &mut items);

        assert_eq!(first.size.y, 90.0);
        assert_eq!(second.size.y, 90.0);
        assert!(second.pos.x >= first.size.x - 0.1);
    }
}
