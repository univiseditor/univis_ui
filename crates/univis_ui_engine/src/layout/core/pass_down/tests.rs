use super::*;
use crate::layout::components::LayoutTreeDepth;
use crate::layout::core::hierarchy::update_layout_hierarchy;
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

    assert_eq!(constraints.min_width, 240.0);
    assert_eq!(constraints.max_width, 240.0);
    assert_eq!(constraints.min_height, 90.0);
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
            #[allow(deprecated)]
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

#[test]
fn subtree_stacking_keeps_descendants_inside_their_sibling_branch_order() {
    let mut app = App::new();
    app.init_resource::<LayoutTreeDepth>();
    app.init_resource::<LayoutCache>();
    app.add_systems(
        Update,
        (
            update_layout_hierarchy,
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
        .entity_mut(root)
        .get_mut::<ResolvedRootUi>()
        .expect("root should have resolved state")
        .root_entity = root;

    let earlier_branch = app
        .world_mut()
        .spawn((
            UNode {
                width: UVal::Px(140.0),
                height: UVal::Px(90.0),
                ..default()
            },
            ChildOf(root),
        ))
        .id();
    let earlier_child = app
        .world_mut()
        .spawn((
            UNode {
                width: UVal::Px(60.0),
                height: UVal::Px(30.0),
                ..default()
            },
            ChildOf(earlier_branch),
        ))
        .id();
    let later_branch = app
        .world_mut()
        .spawn((
            UNode {
                width: UVal::Px(140.0),
                height: UVal::Px(90.0),
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    app.update();

    let earlier_z = app
        .world()
        .entity(earlier_branch)
        .get::<Transform>()
        .expect("earlier branch should have a transform")
        .translation
        .z;
    let earlier_child_z = app
        .world()
        .entity(earlier_child)
        .get::<Transform>()
        .expect("earlier child should have a transform")
        .translation
        .z;
    let later_z = app
        .world()
        .entity(later_branch)
        .get::<Transform>()
        .expect("later branch should have a transform")
        .translation
        .z;

    assert!(earlier_z < earlier_child_z);
    assert!(earlier_child_z < later_z);
}
