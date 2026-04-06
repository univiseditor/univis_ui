#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use crate::internal_prelude::*;
use bevy::prelude::*;

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

// =========================================================
// النظام الرئيسي - 100% آمن
// =========================================================

#[doc(hidden)]
pub fn downward_solve_pass_safe(
    _tree_depth: Res<LayoutTreeDepth>,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>, // إضافة Profiler اختياري
    mut scratch: Local<LayoutSolveScratch>,

    mut nodes: Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        Option<&CachedUiContext>,
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
                Ok((_, node, layout_opt, _, children_opt, _, cached_context, computed, _)) => {
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
            && let Ok((_, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity)
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

        if let Ok((_, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
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

    // تحديث Profiler
    if let Some(ref mut prof) = profiler {
        prof.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.solved_nodes += solved_count;
    }
}

// =========================================================
// Helper Functions
// =========================================================

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
        &mut ComputedSize,
        &mut Transform,
    )>,
    intrinsic_query: &Query<&IntrinsicSize>,
    scratch: &mut Vec<SolverScratchItem>,
) {
    scratch.reserve(children.len());

    for child_entity in children.iter() {
        let Ok((_, node, _, _, _, uself_opt, _, _, _)) = nodes_query.get(child_entity) else {
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
    let mut constraints = BoxConstraints::tight(container_size);
    let (min_width, max_width) = node_spec.width_bounds();
    let (min_height, max_height) = node_spec.height_bounds();

    if node_spec.width.uses_intrinsic_measurement() {
        constraints.min_width = min_width;
        constraints.max_width = max_width;
    }

    if node_spec.height.uses_intrinsic_measurement() {
        constraints.min_height = min_height;
        constraints.max_height = max_height;
    }

    constraints
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
        &mut ComputedSize,
        &mut Transform,
    )>,
) {
    for solved in solved_children.iter() {
        if let Ok((_, _, _, layout_depth, children, uself, _, mut computed, mut transform)) =
            nodes_query.get_mut(solved.entity)
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

            let order = uself.map(|value| value.order).unwrap_or(0);
            let next_z = root_stack.local_depth_offset(layout_depth.0, order);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::components::LayoutTreeDepth;
    use crate::layout::core::layout_cache::LayoutCache;
    use crate::layout::core::layout_cache::{track_layout_changes, update_depth_cache};
    use crate::layout::core::pass_up::upward_measure_pass_cached;
    use crate::layout::layout_system::sync_fit_content_root_canvas_sizes;

    fn solve_child_under_world_root(meters_per_unit: f32) -> (ComputedSize, Transform) {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit,
                    resolution_scale: 1.0,
                },
                ResolvedRootStack {
                    capsule_sort_key: 0.0,
                    capsule_band_base: 0.0,
                    capsule_band_width: meters_per_unit * 0.04,
                    capsule_band_step: (meters_per_unit * 0.04) / 2048.0,
                    initialized: true,
                    ..default()
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
                LayoutDepth(1),
                ChildOf(root),
            ))
            .id();

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let child_size = *app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .expect("child should have computed size");
        let child_transform = *app
            .world()
            .entity(child)
            .get::<Transform>()
            .expect("child should have transform");

        (child_size, child_transform)
    }

    #[test]
    fn translate_config_reads_container_ext_values() {
        let layout = ULayout {
            gap: 5.0,
            grid_columns: 4,
            container_ext: ULayoutContainerExt {
                box_align: ULayoutBoxAlignContainer {
                    justify_items: Some(UAlignItemsExt::Center),
                    align_content: Some(UContentAlignExt::SpaceAround),
                    row_gap: Some(9.0),
                    column_gap: Some(11.0),
                },
                flex: ULayoutFlexContainer {
                    wrap: UFlexWrap::WrapReverse,
                    align_content: Some(UContentAlignExt::SpaceEvenly),
                },
                grid: ULayoutGridContainer {
                    template_columns: vec![UTrackSize::Fr(1.0), UTrackSize::Px(100.0)],
                    template_rows: vec![UTrackSize::Px(42.0)],
                    auto_flow: UGridAutoFlow::Column,
                    auto_rows: UTrackSize::Px(60.0),
                    auto_columns: UTrackSize::Fr(2.0),
                },
            },
            ..default()
        };
        let node = UNode::default();

        let cfg = translate_config(&layout, &node);

        assert_eq!(cfg.row_gap, Some(9.0));
        assert_eq!(cfg.column_gap, Some(11.0));
        assert_eq!(cfg.justify_items, Some(UAlignItemsExt::Center));
        assert_eq!(cfg.align_content, Some(UContentAlignExt::SpaceAround));
        assert_eq!(cfg.flex_wrap, UFlexWrap::WrapReverse);
        assert_eq!(cfg.flex_align_content, Some(UContentAlignExt::SpaceEvenly));
        assert_eq!(cfg.grid_auto_flow, UGridAutoFlow::Column);
        assert_eq!(cfg.grid_auto_rows, UTrackSize::Px(60.0));
        assert_eq!(cfg.grid_auto_columns, UTrackSize::Fr(2.0));
        assert_eq!(cfg.grid_template_columns.len(), 2);
        assert_eq!(cfg.grid_template_rows.len(), 1);
    }

    #[test]
    fn translate_spec_without_uself_uses_defaults() {
        let node = UNode::default();

        let spec = translate_spec(&node, None);

        assert_eq!(spec.position_type, UPositionType::Relative);
        assert_eq!(spec.order, 0);
        assert_eq!(spec.align_self, None);
        assert_eq!(spec.justify_self_ext, None);
        assert_eq!(spec.align_self_ext, None);
        assert_eq!(spec.flex_grow, None);
        assert_eq!(spec.flex_shrink, None);
        assert_eq!(spec.flex_basis, None);
        assert_eq!(spec.min_width, 0.0);
        assert_eq!(spec.max_width, f32::INFINITY);
        assert_eq!(spec.min_height, 0.0);
        assert_eq!(spec.max_height, f32::INFINITY);
        assert_eq!(spec.grid_column_span, 1);
        assert_eq!(spec.grid_row_span, 1);
    }

    #[test]
    fn build_constraints_respects_min_max_for_content_nodes() {
        let node = UNode {
            width: UVal::Content,
            height: UVal::Auto,
            min_width: 120.0,
            max_width: 240.0,
            min_height: 30.0,
            max_height: 90.0,
            ..default()
        };

        let constraints = build_constraints(Vec2::new(500.0, 200.0), &node);

        assert_eq!(constraints.min_width, 120.0);
        assert_eq!(constraints.max_width, 240.0);
        assert_eq!(constraints.min_height, 30.0);
        assert_eq!(constraints.max_height, 90.0);
    }

    #[test]
    fn downward_pass_uses_resolved_root_canvas_for_percent_sized_children() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Percent(0.5),
                    height: UVal::Percent(0.25),
                    ..default()
                },
                LayoutDepth(1),
                ChildOf(root),
            ))
            .id();

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;

        app.update();

        let root_size = app
            .world()
            .entity(root)
            .get::<ComputedSize>()
            .expect("root should have computed size");
        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .expect("child should have computed size");

        assert_eq!(root_size.width, 400.0);
        assert_eq!(root_size.height, 200.0);
        assert_eq!(child_size.width, 200.0);
        assert_eq!(child_size.height, 50.0);
    }

    #[test]
    fn fixed_child_width_respects_min_width_constraint() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Px(40.0),
                    height: UVal::Px(20.0),
                    min_width: 120.0,
                    ..default()
                },
                LayoutDepth(1),
                ChildOf(root),
            ))
            .id();

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .copied()
            .expect("child should have computed size");

        assert_eq!(child_size.width, 120.0);
        assert_eq!(child_size.height, 20.0);
    }

    #[test]
    fn content_sized_child_respects_max_width_constraint() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Content,
                    height: UVal::Content,
                    max_width: 100.0,
                    ..default()
                },
                ULayout::default(),
                LayoutDepth(1),
                ChildOf(root),
            ))
            .id();

        app.world_mut().spawn((
            UNode {
                width: UVal::Px(200.0),
                height: UVal::Px(20.0),
                ..default()
            },
            LayoutDepth(2),
            ChildOf(child),
        ));

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 2;
        app.update();

        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .copied()
            .expect("child should have computed size");

        assert_eq!(child_size.width, 100.0);
        assert_eq!(child_size.height, 20.0);
    }

    #[test]
    fn world_fit_content_root_adopts_measured_child_size() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                sync_fit_content_root_canvas_sizes,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::FitContent {
                        min: Vec2::ZERO,
                        max: None,
                    },
                    canvas_size: Vec2::ZERO,
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        app.world_mut().spawn((
            UNode {
                width: UVal::Px(120.0),
                height: UVal::Px(48.0),
                ..default()
            },
            LayoutDepth(1),
            ChildOf(root),
        ));

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let root_size = app
            .world()
            .entity(root)
            .get::<ComputedSize>()
            .copied()
            .expect("fit-content root should have computed size");
        let resolved = app
            .world()
            .entity(root)
            .get::<ResolvedRootUi>()
            .copied()
            .expect("fit-content root should have resolved state");

        assert_eq!(root_size.width, 120.0);
        assert_eq!(root_size.height, 48.0);
        assert_eq!(resolved.canvas_size, Vec2::new(120.0, 48.0));
    }

    #[test]
    fn world_fit_content_root_clamps_measured_size_to_bounds() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                sync_fit_content_root_canvas_sizes,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World3d,
                    canvas: UiCanvasSize::FitContent {
                        min: Vec2::new(100.0, 80.0),
                        max: Some(Vec2::new(180.0, 120.0)),
                    },
                    canvas_size: Vec2::ZERO,
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        app.world_mut().spawn((
            UNode {
                width: UVal::Px(260.0),
                height: UVal::Px(30.0),
                ..default()
            },
            LayoutDepth(1),
            ChildOf(root),
        ));

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let root_size = app
            .world()
            .entity(root)
            .get::<ComputedSize>()
            .copied()
            .expect("fit-content root should have computed size");
        let resolved = app
            .world()
            .entity(root)
            .get::<ResolvedRootUi>()
            .copied()
            .expect("fit-content root should have resolved state");

        assert_eq!(root_size.width, 180.0);
        assert_eq!(root_size.height, 80.0);
        assert_eq!(resolved.canvas_size, Vec2::new(180.0, 80.0));
    }

    #[test]
    fn world_roots_scale_physical_transforms_without_changing_logical_layout() {
        let (logical_a, transform_a) = solve_child_under_world_root(1.0);
        let (logical_b, transform_b) = solve_child_under_world_root(0.25);

        assert_eq!(logical_a.width, 100.0);
        assert_eq!(logical_a.height, 50.0);
        assert_eq!(logical_b.width, logical_a.width);
        assert_eq!(logical_b.height, logical_a.height);

        assert!(
            transform_b
                .translation
                .abs_diff_eq(transform_a.translation * 0.25, 1e-5)
        );
    }

    #[test]
    fn child_depth_stays_inside_its_root_capsule_band() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let lower_root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
                ResolvedRootStack {
                    capsule_sort_key: 0.0,
                    capsule_band_base: 0.0,
                    capsule_band_width: 0.04,
                    capsule_band_step: 0.04 / 2048.0,
                    initialized: true,
                    ..default()
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(lower_root)
            .get_mut::<ResolvedRootUi>()
            .expect("lower root should have resolved state")
            .root_entity = lower_root;

        let upper_root_floor = 0.05;
        app.world_mut().spawn((
            UNode::default(),
            ULayout::default(),
            LayoutDepth(0),
            ResolvedRootUi {
                root_entity: Entity::PLACEHOLDER,
                space: UiSpace::World2d,
                canvas: UiCanvasSize::Fixed(Vec2::new(400.0, 200.0)),
                canvas_size: Vec2::new(400.0, 200.0),
                camera_entity: None,
                meters_per_unit: 1.0,
                resolution_scale: 1.0,
            },
            ResolvedRootStack {
                capsule_sort_key: upper_root_floor,
                capsule_band_base: upper_root_floor,
                capsule_band_width: 0.04,
                capsule_band_step: 0.04 / 2048.0,
                initialized: true,
                ..default()
            },
        ));

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
                USelf {
                    order: 32,
                    ..default()
                },
                LayoutDepth(1),
                ChildOf(lower_root),
            ))
            .id();

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let child_transform = *app
            .world()
            .entity(child)
            .get::<Transform>()
            .expect("child should have a transform");

        assert!(child_transform.translation.z < upper_root_floor);
    }

    #[test]
    fn child_depth_updates_for_tiny_root_capsule_steps() {
        let mut app = App::new();
        app.init_resource::<LayoutTreeDepth>();
        app.init_resource::<LayoutCache>();
        app.add_systems(
            Update,
            (
                track_layout_changes,
                update_depth_cache,
                upward_measure_pass_cached,
                downward_solve_pass_safe,
            )
                .chain(),
        );

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ULayout::default(),
                LayoutDepth(0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(400.0, 200.0),
                    camera_entity: None,
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: 1.0,
                },
                ResolvedRootStack {
                    capsule_sort_key: 0.0,
                    capsule_band_base: 0.0,
                    capsule_band_width: 0.004,
                    capsule_band_step: 0.004 / 2048.0,
                    initialized: true,
                    ..default()
                },
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        let child = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
                LayoutDepth(1),
                ChildOf(root),
            ))
            .id();

        app.world_mut().resource_mut::<LayoutTreeDepth>().max_depth = 1;
        app.update();

        let expected_z = app
            .world()
            .entity(root)
            .get::<ResolvedRootStack>()
            .expect("root should keep a stack")
            .local_depth_offset(1, 0);
        let child_transform = *app
            .world()
            .entity(child)
            .get::<Transform>()
            .expect("child should have a transform");

        assert_eq!(
            child_transform.translation.z.to_bits(),
            expected_z.to_bits()
        );
        assert!(expected_z.abs() < LAYOUT_WRITE_EPSILON);
    }
}
