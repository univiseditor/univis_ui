#![allow(clippy::type_complexity, clippy::too_many_arguments)]

#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::layout::components::*;
use crate::layout::core::layout_cache::LayoutCache;
use crate::layout::core::solver::*;
use crate::layout::geometry::*;
use crate::layout::layout_system::*;
use crate::layout::profiling::LayoutProfiler;
use crate::layout::solver_types::{SolverSizeMode, SolverSpec};
use crate::layout::univis_node::*;
use crate::schedule::UiRolloutConfig;

const LAYOUT_WRITE_EPSILON: f32 = 0.001;

fn z_translation_changed(current: f32, next: f32) -> bool {
    current.to_bits() != next.to_bits()
}

struct SolverScratchItem {
    entity: Entity,
    spec: SolverSpec,
    result: SolverResult,
    margin: USides,
}

impl SolverScratchItem {
    fn as_solver_item(&mut self) -> SolverItem {
        SolverItem::new(self.spec, &mut self.result, self.margin)
    }
}

#[derive(Default)]
#[doc(hidden)]
pub struct LayoutSolveScratch {
    solver_items: Vec<SolverScratchItem>,
    solver_refs: Vec<SolverItem>,
}

// Main downward solve pass.

#[doc(hidden)]
pub fn downward_solve_pass_safe(
    _tree_depth: Res<LayoutTreeDepth>,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut scratch: Local<LayoutSolveScratch>,

    mut nodes: Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        Option<&CachedUiContext>,
        Option<&UiLocalStacking>,
        &mut ComputedSize,
        &mut Transform,
    )>,

    intrinsic_query: Query<&IntrinsicSize>,
    root_query: Query<&ResolvedRootUi>,
    root_stack_query: Query<&ResolvedRootStack>,
    parents_query: Query<&ChildOf>,
) {
    let start = std::time::Instant::now();
    let mut solved_count = 0;
    let use_incremental_solve = rollout
        .as_ref()
        .map_or(true, |config| config.use_incremental_solve);
    let use_cached_ui_context = rollout
        .as_ref()
        .map_or(true, |config| config.use_cached_ui_context);
    let solve_frontier = if use_incremental_solve {
        cache.take_solve_frontier()
    } else {
        cache.all_entities_top_down()
    };

    for entity in solve_frontier {
        let depth = cache.depth_for_entity(entity).unwrap_or_default();

        let Some((container_size, constraints, solver_config, cached_context)) = (|| {
            let (node, layout_opt, children_opt, cached_context, computed) = match nodes.get(entity)
            {
                Ok((_, node, layout_opt, _, children_opt, _, cached_context, _, computed, _)) => {
                    (node, layout_opt, children_opt, cached_context, computed)
                }
                Err(_) => return None,
            };

            let container_size = if depth == 0 {
                calculate_root_size(entity, &root_query, &intrinsic_query)
            } else {
                Vec2::new(computed.width, computed.height)
            };

            let solver_items_capacity_before = scratch.solver_items.capacity();
            scratch.solver_items.clear();
            if let Some(children) = children_opt
                && !children.is_empty()
            {
                collect_solver_items_into(
                    children,
                    &nodes,
                    &intrinsic_query,
                    &mut scratch.solver_items,
                );
            }
            if let Some(prof) = profiler.as_mut() {
                if scratch.solver_items.capacity() > solver_items_capacity_before {
                    prof.solve_scratch_alloc_grows += 1;
                }
                prof.solve_scratch_peak = prof.solve_scratch_peak.max(scratch.solver_items.len());
            }

            let default_layout;
            let layout = if let Some(layout) = layout_opt {
                layout
            } else {
                default_layout = ULayout::default();
                &default_layout
            };

            Some((
                container_size,
                if depth == 0 {
                    BoxConstraints::tight(container_size)
                } else {
                    build_constraints(container_size, node)
                },
                translate_config(layout, node),
                cached_context.copied(),
            ))
        })() else {
            cache.clear_solve_dirty(entity);
            continue;
        };

        if depth == 0
            && let Ok((_, _, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity)
        {
            if (computed.width - container_size.x).abs() > LAYOUT_WRITE_EPSILON {
                computed.width = container_size.x;
            }
            if (computed.height - container_size.y).abs() > LAYOUT_WRITE_EPSILON {
                computed.height = container_size.y;
            }
        }

        if scratch.solver_items.is_empty() {
            cache.complete_solve(entity);
            cache.clear_solve_dirty(entity);
            continue;
        }

        let solved_size = {
            let (solver_refs_capacity_before, solver_refs_len) = {
                let LayoutSolveScratch {
                    solver_items,
                    solver_refs,
                } = &mut *scratch;
                let solver_refs_capacity_before = solver_refs.capacity();
                solver_refs.clear();
                solver_refs.reserve(solver_items.len());
                for item in solver_items.iter_mut() {
                    solver_refs.push(item.as_solver_item());
                }
                (solver_refs_capacity_before, solver_refs.len())
            };
            if let Some(prof) = profiler.as_mut() {
                if scratch.solver_refs.capacity() > solver_refs_capacity_before {
                    prof.solve_ref_alloc_grows += 1;
                }
                prof.solve_ref_peak = prof.solve_ref_peak.max(solver_refs_len);
            }
            solve_flex_layout(&solver_config, constraints, &mut scratch.solver_refs)
        };
        solved_count += 1;
        let final_size = if depth == 0 {
            container_size
        } else {
            solved_size
        };
        let solve_generation = cache.stage_versions(entity).solve_input_generation;
        let (world_scale, root_stack) = resolve_solver_context(
            entity,
            use_cached_ui_context.then_some(cached_context).flatten(),
            &parents_query,
            &root_query,
            &root_stack_query,
        );

        if let Ok((_, _, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
            if (computed.width - final_size.x).abs() > LAYOUT_WRITE_EPSILON {
                computed.width = final_size.x;
            }
            if (computed.height - final_size.y).abs() > LAYOUT_WRITE_EPSILON {
                computed.height = final_size.y;
            }
        }

        apply_results_to_children(
            &scratch.solver_items,
            final_size,
            solve_generation,
            world_scale,
            root_stack,
            use_incremental_solve,
            &mut cache,
            &mut nodes,
        );
        cache.complete_solve(entity);

        if use_incremental_solve {
            cache.clear_solve_dirty(entity);
        }
    }

    // Update profiling counters after the pass completes.
    if let Some(ref mut prof) = profiler {
        prof.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.solved_nodes += solved_count;
    }
}

// Helper functions.

fn calculate_root_size(
    entity: Entity,
    root_query: &Query<&ResolvedRootUi>,
    intrinsic_query: &Query<&IntrinsicSize>,
) -> Vec2 {
    let Ok(root) = root_query.get(entity) else {
        return Vec2::new(800.0, 600.0);
    };

    match root.canvas {
        UiCanvasSize::FitContent { min, max } if root.space != UiSpace::Screen => {
            let measured = intrinsic_query
                .get(entity)
                .map(|intrinsic| Vec2::new(intrinsic.width, intrinsic.height))
                .unwrap_or(root.canvas_size);
            clamp_root_canvas_size(measured.max(Vec2::ZERO), min, max)
        }
        UiCanvasSize::Viewport | UiCanvasSize::Fixed(_) | UiCanvasSize::FitContent { .. } => {
            root.canvas_size
        }
    }
}

fn clamp_root_canvas_size(size: Vec2, min: Vec2, max: Option<Vec2>) -> Vec2 {
    let mut clamped = Vec2::new(size.x.max(min.x), size.y.max(min.y));

    if let Some(max) = max {
        clamped.x = clamped.x.min(max.x);
        clamped.y = clamped.y.min(max.y);
    }

    clamped
}

fn resolved_world_scale_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
) -> f32 {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return root.ui_units_to_world_scale();
        }

        let Ok(parent) = parents_query.get(current) else {
            return 1.0;
        };
        current = parent.parent();
    }
}

fn resolved_root_stack_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_stack_query: &Query<&ResolvedRootStack>,
) -> Option<ResolvedRootStack> {
    let mut current = entity;

    loop {
        if let Ok(stack) = root_stack_query.get(current) {
            return Some(*stack);
        }

        let Ok(parent) = parents_query.get(current) else {
            return None;
        };
        current = parent.parent();
    }
}

fn resolve_solver_context(
    entity: Entity,
    cached_context: Option<CachedUiContext>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
    root_stack_query: &Query<&ResolvedRootStack>,
) -> (f32, ResolvedRootStack) {
    if let Some(context) = cached_context
        && context.root_entity.is_some()
    {
        return (context.ui_to_world_scale, context.root_stack);
    }

    (
        resolved_world_scale_for_entity(entity, parents_query, root_query),
        resolved_root_stack_for_entity(entity, parents_query, root_stack_query).unwrap_or_default(),
    )
}

fn collect_solver_items_into(
    children: &Children,
    nodes_query: &Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        Option<&CachedUiContext>,
        Option<&UiLocalStacking>,
        &mut ComputedSize,
        &mut Transform,
    )>,
    intrinsic_query: &Query<&IntrinsicSize>,
    scratch: &mut Vec<SolverScratchItem>,
) {
    scratch.reserve(children.len());

    for child_entity in children.iter() {
        let Ok((_, node, _, _, _, uself_opt, _, _, _, _)) = nodes_query.get(child_entity) else {
            continue;
        };
        let Ok(intrinsic) = intrinsic_query.get(child_entity) else {
            continue;
        };

        let mut spec = translate_spec(node, uself_opt);

        if spec.width_mode == SolverSizeMode::MinContent {
            spec.width_val = intrinsic.min_width;
        } else if spec.width_mode == SolverSizeMode::Content {
            spec.width_val = intrinsic.max_width;
        } else if spec.width_mode == SolverSizeMode::Auto {
            spec.width_val = intrinsic.width;
        }
        if spec.height_mode == SolverSizeMode::MinContent {
            spec.height_val = intrinsic.min_height;
        } else if spec.height_mode == SolverSizeMode::Content {
            spec.height_val = intrinsic.max_height;
        } else if spec.height_mode == SolverSizeMode::Auto {
            spec.height_val = intrinsic.height;
        }

        scratch.push(SolverScratchItem {
            entity: child_entity,
            spec,
            result: SolverResult::default(),
            margin: node.margin,
        });
    }
}

fn build_constraints(container_size: Vec2, node_spec: &UNode) -> BoxConstraints {
    let (min_width, max_width) = node_spec.width_bounds();
    let (min_height, max_height) = node_spec.height_bounds();

    let clamped_width = container_size.x.clamp(min_width, max_width);
    let clamped_height = container_size.y.clamp(min_height, max_height);

    BoxConstraints::tight(Vec2::new(clamped_width, clamped_height))
}

fn apply_results_to_children(
    solved_children: &[SolverScratchItem],
    parent_size: Vec2,
    solve_generation: u64,
    world_scale: f32,
    root_stack: ResolvedRootStack,
    use_incremental_solve: bool,
    cache: &mut LayoutCache,
    nodes_query: &mut Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        Option<&CachedUiContext>,
        Option<&UiLocalStacking>,
        &mut ComputedSize,
        &mut Transform,
    )>,
) {
    for solved in solved_children.iter() {
        if let Ok((
            _,
            _,
            _,
            layout_depth,
            children,
            uself,
            _,
            local_stacking,
            mut computed,
            mut transform,
        )) = nodes_query.get_mut(solved.entity)
        {
            let size_changed = (computed.width - solved.result.size.x).abs() > LAYOUT_WRITE_EPSILON
                || (computed.height - solved.result.size.y).abs() > LAYOUT_WRITE_EPSILON;

            if size_changed {
                computed.width = solved.result.size.x;
                computed.height = solved.result.size.y;
            }

            let child_w = solved.result.size.x;
            let child_h = solved.result.size.y;

            let next_x =
                ((-parent_size.x / 2.0) + solved.result.pos.x + (child_w / 2.0)) * world_scale;
            let next_y =
                ((parent_size.y / 2.0) - solved.result.pos.y - (child_h / 2.0)) * world_scale;

            #[allow(deprecated)]
            let order = uself.map(|value| value.order).unwrap_or(0);
            let next_z = local_stacking.map_or_else(
                || root_stack.local_depth_offset(layout_depth.0, order),
                |stack| root_stack.local_depth_offset_for_fraction(stack.normalized),
            );

            if (transform.translation.x - next_x).abs() > LAYOUT_WRITE_EPSILON {
                transform.translation.x = next_x;
            }
            if (transform.translation.y - next_y).abs() > LAYOUT_WRITE_EPSILON {
                transform.translation.y = next_y;
            }
            if z_translation_changed(transform.translation.z, next_z) {
                transform.translation.z = next_z;
            }

            if use_incremental_solve
                && size_changed
                && children.is_some_and(|value| !value.is_empty())
            {
                cache.mark_solve_dirty_generation(solved.entity, solve_generation);
            }

            if children.is_none_or(|value| value.is_empty()) {
                cache.complete_solve(solved.entity);
            }
        }
    }
}

// =========================================================
// Translation Functions
// =========================================================

fn map_uval_to_mode(val: UVal) -> SolverSizeMode {
    match val {
        UVal::Px(_) => SolverSizeMode::Fixed,
        UVal::Percent(_) => SolverSizeMode::Percent,
        UVal::Flex(_) => SolverSizeMode::Flex,
        UVal::MinContent => SolverSizeMode::MinContent,
        UVal::Content | UVal::MaxContent => SolverSizeMode::Content,
        UVal::Auto => SolverSizeMode::Auto,
    }
}

fn translate_config(layout: &ULayout, node: &UNode) -> SolverConfig {
    SolverConfig {
        layout: layout.clone(),
        gap: layout.gap,
        row_gap: layout.container_ext.box_align.row_gap,
        column_gap: layout.container_ext.box_align.column_gap,
        padding: node.padding,
        grid_columns: layout.grid_columns,
        justify_items: layout.container_ext.box_align.justify_items,
        align_content: layout.container_ext.box_align.align_content,
        flex_wrap: layout.container_ext.flex.wrap,
        flex_align_content: layout.container_ext.flex.align_content,
        grid_template_columns: layout.container_ext.grid.template_columns.clone(),
        grid_template_rows: layout.container_ext.grid.template_rows.clone(),
        grid_auto_flow: layout.container_ext.grid.auto_flow,
        grid_auto_rows: layout.container_ext.grid.auto_rows,
        grid_auto_columns: layout.container_ext.grid.auto_columns,
        width_mode: map_uval_to_mode(node.width),
        height_mode: map_uval_to_mode(node.height),
    }
}

fn translate_spec(node: &UNode, uself: Option<&USelf>) -> SolverSpec {
    super::solver::translate_spec(node, uself)
}
