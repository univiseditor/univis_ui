use crate::internal_prelude::*;
use bevy::prelude::*;

// =========================================================
// نسخة Owned آمنة من SolverItem
// =========================================================

/// نسخة آمنة من SolverItem - تملك بياناتها
pub struct SolverItemOwned {
    pub spec: SolverSpec,
    pub result: Box<SolverResult>,
    pub margin: USides,
}

impl SolverItemOwned {
    /// تحويل إلى SolverItem مؤقت (للاستخدام مع الـ Solver)
    pub fn as_solver_item(&mut self) -> SolverItem<'_> {
        SolverItem {
            spec: self.spec,
            result: &mut *self.result,
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

pub fn downward_solve_pass_safe(
    tree_depth: Res<LayoutTreeDepth>,
    cache: Res<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>, // إضافة Profiler اختياري

    mut nodes: Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        &mut ComputedSize,
        &mut Transform,
    )>,

    intrinsic_query: Query<&IntrinsicSize>,
    root_query: Query<&ResolvedRootUi>,
    root_stack_query: Query<&ResolvedRootStack>,
    parents_query: Query<&ChildOf>,
) {
    let start = std::time::Instant::now();

    for depth in 0..=tree_depth.max_depth {
        // استخدام Cache
        let Some(layer_entities) = cache.get_entities_at_depth(depth) else {
            continue;
        };

        for &entity in layer_entities {
            // 1. استخراج البيانات
            let Some(node_data) = extract_node_data(entity, &nodes) else {
                continue;
            };

            // 2. حساب حجم الحاوية
            let container_size = if depth == 0 {
                calculate_root_size(entity, &root_query)
            } else {
                Vec2::new(
                    node_data.computed_size.width,
                    node_data.computed_size.height,
                )
            };

            // تحديث الجذر
            if depth == 0 {
                if let Ok((_, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
                    computed.width = container_size.x;
                    computed.height = container_size.y;
                }
            }

            // 3. جمع بيانات الأطفال
            let children_layout_data =
                collect_children_layout_data(&node_data.children, &nodes, &intrinsic_query);

            if children_layout_data.is_empty() {
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
            let final_size = if depth == 0 {
                container_size
            } else {
                solved_size
            };
            let world_scale = resolved_world_scale_for_entity(entity, &parents_query, &root_query);
            let root_stack =
                resolved_root_stack_for_entity(entity, &parents_query, &root_stack_query)
                    .unwrap_or_default();

            // 8. تحديث حجم الحاوية
            if let Ok((_, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
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
                &mut nodes,
            );
        }
    }

    // تحديث Profiler
    if let Some(ref mut prof) = profiler {
        prof.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
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
        &mut ComputedSize,
        &mut Transform,
    )>,
) -> Option<NodeData> {
    let (_, node, layout_opt, _, children_opt, _, computed, _) = query.get(entity).ok()?;

    Some(NodeData {
        spec: node.clone(),
        layout: layout_opt.cloned().unwrap_or_default(),
        children: children_opt.map(|c| c.iter().collect()).unwrap_or_default(),
        computed_size: *computed,
    })
}

fn calculate_root_size(entity: Entity, root_query: &Query<&ResolvedRootUi>) -> Vec2 {
    root_query
        .get(entity)
        .map(|root| root.canvas_size)
        .unwrap_or(Vec2::new(800.0, 600.0))
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

fn collect_children_layout_data(
    children: &[Entity],
    nodes_query: &Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        &mut ComputedSize,
        &mut Transform,
    )>,
    intrinsic_query: &Query<&IntrinsicSize>,
) -> Vec<ChildLayoutData> {
    children
        .iter()
        .filter_map(|&child_entity| {
            let (_, node, _, _, _, uself_opt, _, _) = nodes_query.get(child_entity).ok()?;
            let intrinsic = intrinsic_query.get(child_entity).ok()?;

            let mut spec = translate_spec(node, uself_opt.as_deref());

            if spec.width_mode == SolverSizeMode::Content {
                spec.width_val = intrinsic.width;
            }
            if spec.height_mode == SolverSizeMode::Content {
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

    if matches!(node_spec.width, UVal::Auto | UVal::Content) {
        constraints.min_width = 0.0;
        constraints.max_width = f32::INFINITY;
    }

    if matches!(node_spec.height, UVal::Auto | UVal::Content) {
        constraints.min_height = 0.0;
        constraints.max_height = f32::INFINITY;
    }

    constraints
}

fn apply_results_to_children(
    solved_children: &[SolvedChild],
    parent_size: Vec2,
    world_scale: f32,
    root_stack: ResolvedRootStack,
    nodes_query: &mut Query<(
        Entity,
        &UNode,
        Option<&ULayout>,
        &LayoutDepth,
        Option<&Children>,
        Option<&USelf>,
        &mut ComputedSize,
        &mut Transform,
    )>,
) {
    for solved in solved_children.iter() {
        if let Ok((_, _, _, layout_depth, _, uself, mut computed, mut transform)) =
            nodes_query.get_mut(solved.entity)
        {
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
        UVal::Content | UVal::Auto => SolverSizeMode::Content,
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
    let map_dim = |dim: UVal| -> (SolverSizeMode, f32, f32) {
        match dim {
            UVal::Px(v) => (SolverSizeMode::Fixed, v, 0.0),
            UVal::Percent(p) => (SolverSizeMode::Percent, p, 0.0),
            UVal::Flex(f) => (SolverSizeMode::Flex, 0.0, f),
            UVal::Content | UVal::Auto => (SolverSizeMode::Content, 0.0, 0.0),
        }
    };

    let (w_mode, w_val, w_flex) = map_dim(node.width);
    let (h_mode, h_val, h_flex) = map_dim(node.height);

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
        height_mode: h_mode,
        height_val: h_val,
        height_flex: h_flex,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::components::LayoutTreeDepth;
    use crate::layout::core::layout_cache::LayoutCache;
    use crate::layout::core::layout_cache::{track_layout_changes, update_depth_cache};
    use crate::layout::core::pass_up::upward_measure_pass_cached;

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
        assert_eq!(spec.grid_column_span, 1);
        assert_eq!(spec.grid_row_span, 1);
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
