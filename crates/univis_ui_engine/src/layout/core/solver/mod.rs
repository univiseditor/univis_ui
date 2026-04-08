#[cfg(test)]
mod tests;

use bevy::prelude::*;

mod absolute;
mod helpers;
mod translate;
mod types;

use self::absolute::solve_absolute_box;
use self::helpers::{
    allows_explicit_stretch, allows_implicit_stretch, clamp_cross_size, clamp_main_size,
    flex_grow_factor, flex_shrink_factor, has_explicit_align_self_override, is_ext_stretch,
    main_bounds_for_spec, map_align_items_to_ext, map_align_self_to_ext, resolve_flex_basis,
    resolved_flex_grow_factor,
};
pub use self::translate::translate_spec;
pub use self::types::{PlacementContext, SolverConfig, SolverItem, SolverResult};
use crate::layout::algorithms::bridge::final_size_with_indices;
use crate::layout::geometry::{AxisHelper, BoxConstraints};
use crate::layout::solver_types::SolverSizeMode;
use crate::layout::univis_node::{UAlignItemsExt, UAlignSelfExt, UFlexDirection, UPositionType};

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

    let positive_free_space = (available_main - used_main).max(0.0);
    if positive_free_space > 0.0 {
        let mut remaining_free_space = positive_free_space;
        let mut growable_indices: Vec<usize> = normal_indices
            .iter()
            .copied()
            .filter(|&idx| {
                let item = &items[idx];
                let grow_factor = resolved_flex_grow_factor(&item.spec, &axis);
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
                    resolved_flex_grow_factor(&item.spec, &axis)
                })
                .sum();

            if total_grow <= 0.0 {
                break;
            }

            let mut distributed = 0.0;
            let mut next_growable = Vec::new();

            for &idx in &growable_indices {
                let item = &mut items[idx];
                let grow_factor = resolved_flex_grow_factor(&item.spec, &axis);
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

    let used_size_from_placer = final_size_with_indices(
        config.layout.clone(),
        items,
        &normal_indices,
        &axis,
        &placement_ctx,
        container_main,
        final_cross,
    );

    let mut final_container_size = axis.to_world(container_main, final_cross);
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
