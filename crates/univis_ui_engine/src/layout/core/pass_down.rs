#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use crate::internal_prelude::*;
use bevy::prelude::*;

// =========================================================
// نسخة Owned آمنة من SolverItem
// =========================================================

#[doc(hidden)]
pub struct SolverItemOwned {
    pub spec: SolverSpec,
    pub result: Box<SolverResult>,
    pub margin: USides,
}

impl SolverItemOwned {
    #[doc(hidden)]
    pub fn as_solver_item(&mut self) -> SolverItem<'_> {
        SolverItem {
            spec: self.spec,
            result: &mut self.result,
            margin: self.margin,
        }
    }
}

// =========================================================
// Data Structures
// =========================================================

#[derive(Clone)]
struct NodeData {
    spec: UNode,
    layout: ULayout,
    children: Vec<Entity>,
    computed_size: ComputedSize,
    cached_context: Option<CachedUiContext>,
}

struct ChildLayoutData {
    entity: Entity,
    spec: SolverSpec,
    margin: USides,
}

struct SolvedChild {
    entity: Entity,
    result: SolverResult,
}

// =========================================================
// النظام الرئيسي - 100% آمن
// =========================================================

#[doc(hidden)]
pub fn downward_solve_pass_safe(
    tree_depth: Res<LayoutTreeDepth>,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>, // إضافة Profiler اختياري

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

    for depth in 0..=tree_depth.max_depth {
        // استخدام Cache
        let Some(layer_entities) = cache.get_entities_at_depth(depth).cloned() else {
            continue;
        };

        for entity in layer_entities {
            if use_incremental_solve && !cache.is_solve_dirty(entity) {
                continue;
            }

            // 1. استخراج البيانات
            let Some(node_data) = extract_node_data(entity, &nodes) else {
                cache.clear_solve_dirty(entity);
                continue;
            };

            // 2. حساب حجم الحاوية
            let container_size = if depth == 0 {
                calculate_root_size(entity, &root_query, &intrinsic_query)
            } else {
                Vec2::new(
                    node_data.computed_size.width,
                    node_data.computed_size.height,
                )
            };

            // تحديث الجذر
            if depth == 0
                && let Ok((_, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity)
            {
                computed.width = container_size.x;
                computed.height = container_size.y;
            }

            // 3. جمع بيانات الأطفال
            let children_layout_data =
                collect_children_layout_data(&node_data.children, &nodes, &intrinsic_query);

            if children_layout_data.is_empty() {
                cache.clear_solve_dirty(entity);
                continue;
            }

            // 4. تحويل إلى Solver (الطريقة الآمنة)
            let (mut solver_items_owned, entities_map) =
                prepare_solver_data_safe(children_layout_data);

            // 5. تحويل مؤقت إلى SolverItem
            let mut solver_items_refs: Vec<SolverItem> = solver_items_owned
                .iter_mut()
                .map(|item| item.as_solver_item())
                .collect();

            // 6. إعداد القيود
            let constraints = if depth == 0 {
                BoxConstraints::tight(container_size)
            } else {
                build_constraints(container_size, &node_data.spec)
            };

            // 7. تشغيل Solver
            let solver_config = translate_config(&node_data.layout, &node_data.spec);
            let solved_size =
                solve_flex_layout(&solver_config, constraints, &mut solver_items_refs);
            solved_count += 1;
            let final_size = if depth == 0 {
                container_size
            } else {
                solved_size
            };
            let (world_scale, root_stack) = resolve_solver_context(
                entity,
                use_cached_ui_context
                    .then_some(node_data.cached_context)
                    .flatten(),
                &parents_query,
                &root_query,
                &root_stack_query,
            );

            // 8. تحديث حجم الحاوية
            if let Ok((_, _, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
                computed.width = final_size.x;
                computed.height = final_size.y;
            }

            // 9. ترجمة النتائج
            let solved_children: Vec<SolvedChild> = solver_items_owned
                .iter()
                .zip(entities_map.iter())
                .map(|(item, &entity)| SolvedChild {
                    entity,
                    result: *item.result,
                })
                .collect();

            // 10. تطبيق النتائج
            apply_results_to_children(
                &solved_children,
                final_size,
                world_scale,
                root_stack,
                use_incremental_solve,
                &mut cache,
                &mut nodes,
            );

            if use_incremental_solve {
                cache.clear_solve_dirty(entity);
            }
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

fn extract_node_data(
    entity: Entity,
    query: &Query<(
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
) -> Option<NodeData> {
    let (_, node, layout_opt, _, children_opt, _, cached_context, computed, _) =
        query.get(entity).ok()?;

    Some(NodeData {
        spec: node.clone(),
        layout: layout_opt.cloned().unwrap_or_default(),
        children: children_opt.map(|c| c.iter().collect()).unwrap_or_default(),
        computed_size: *computed,
        cached_context: cached_context.copied(),
    })
}

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

fn collect_children_layout_data(
    children: &[Entity],
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
) -> Vec<ChildLayoutData> {
    children
        .iter()
        .filter_map(|&child_entity| {
            let (_, node, _, _, _, uself_opt, _, _, _) = nodes_query.get(child_entity).ok()?;
            let intrinsic = intrinsic_query.get(child_entity).ok()?;

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

            Some(ChildLayoutData {
                entity: child_entity,
                spec,
                margin: node.margin,
            })
        })
        .collect()
}

/// ✅ الطريقة الآمنة لتحضير بيانات Solver
fn prepare_solver_data_safe(
    children_data: Vec<ChildLayoutData>,
) -> (Vec<SolverItemOwned>, Vec<Entity>) {
    let mut entities_map = Vec::new();
    let mut solver_items = Vec::new();

    for child_data in children_data {
        entities_map.push(child_data.entity);

        solver_items.push(SolverItemOwned {
            spec: child_data.spec,
            result: Box::new(SolverResult::default()),
            margin: child_data.margin,
        });
    }

    (solver_items, entities_map)
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
    solved_children: &[SolvedChild],
    parent_size: Vec2,
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
            let size_changed = (computed.width - solved.result.size.x).abs() > 0.001
                || (computed.height - solved.result.size.y).abs() > 0.001;

            computed.width = solved.result.size.x;
            computed.height = solved.result.size.y;

            let child_w = solved.result.size.x;
            let child_h = solved.result.size.y;

            transform.translation.x =
                ((-parent_size.x / 2.0) + solved.result.pos.x + (child_w / 2.0)) * world_scale;

            transform.translation.y =
                ((parent_size.y / 2.0) - solved.result.pos.y - (child_h / 2.0)) * world_scale;

            let order = uself.map(|value| value.order).unwrap_or(0);
            transform.translation.z = root_stack.local_depth_offset(layout_depth.0, order);

            if use_incremental_solve
                && size_changed
                && children.is_some_and(|value| !value.is_empty())
            {
                cache.mark_solve_dirty(solved.entity);
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
}
