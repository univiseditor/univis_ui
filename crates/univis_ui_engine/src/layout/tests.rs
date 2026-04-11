use super::UnivisLayoutPlugin;
use crate::internal_prelude::*;
use crate::layout::core::layout_cache::LayoutCache;
use bevy::prelude::*;

#[derive(Component)]
struct DeferredResizeTarget;

#[derive(Resource, Default)]
struct DeferredResizeOnce(bool);

#[derive(Component)]
struct RenderOnlyChild;

fn resize_target_in_render_sync(
    mut resize_once: ResMut<DeferredResizeOnce>,
    mut query: Query<&mut UNode, With<DeferredResizeTarget>>,
) {
    if resize_once.0 {
        return;
    }

    let mut node = query
        .single_mut()
        .expect("test should only spawn one deferred resize target");
    node.width = UVal::Px(120.0);
    resize_once.0 = true;
}

#[test]
fn fresh_ui_tree_settles_on_the_first_frame() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let child = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let cache = app.world().resource::<LayoutCache>();
    let root_layer = cache
        .get_entities_at_depth(0)
        .expect("root depth bucket should be built on the first frame");
    let child_layer = cache
        .get_entities_at_depth(1)
        .expect("child depth bucket should be built on the first frame");

    assert!(root_layer.contains(&root));
    assert!(child_layer.contains(&child));
    assert_eq!(
        app.world()
            .entity(child)
            .get::<LayoutDepth>()
            .copied()
            .expect("child should have a depth assigned on the first frame"),
        LayoutDepth(1)
    );

    let child_size = app
        .world()
        .entity(child)
        .get::<ComputedSize>()
        .copied()
        .expect("child should have a computed size on the first frame");
    assert_eq!(child_size.width, 100.0);
    assert_eq!(child_size.height, 50.0);

    let work_state = app.world().resource::<UiWorkState>();
    let root_settlement = app
        .world()
        .entity(root)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root should expose settlement state after the first frame");
    assert_eq!(work_state.current_generation(), 1);
    assert!(
        work_state.is_settled(),
        "pending={:?}, root_settlement={root_settlement:?}",
        work_state.pending()
    );
    assert_eq!(work_state.last_frame_iterations(), 2);
    assert!(!work_state.budget_exhausted());
    assert_eq!(work_state.completed_generation(UiWorkStage::RootResolve), 1);
    assert_eq!(work_state.completed_generation(UiWorkStage::Hierarchy), 1);
    assert_eq!(work_state.completed_generation(UiWorkStage::Measure), 1);
    assert_eq!(work_state.completed_generation(UiWorkStage::Solve), 1);
    assert_eq!(work_state.completed_generation(UiWorkStage::Render), 1);
    assert_eq!(root_settlement.current_generation, 1);
    assert_eq!(root_settlement.pending_measure, 0);
    assert_eq!(root_settlement.pending_solve, 0);
    assert_eq!(root_settlement.pending_render, 0);
    assert!(root_settlement.is_settled());
}

#[test]
fn settlement_loop_drains_follow_up_layout_work_in_the_same_frame() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.init_resource::<DeferredResizeOnce>();
    app.add_systems(
        UiSettlementSchedule,
        resize_target_in_render_sync.in_set(UnivisPostUpdateSet::RenderSync),
    );

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let child = app
        .world_mut()
        .spawn((
            ChildOf(root),
            DeferredResizeTarget,
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let child_size = app
        .world()
        .entity(child)
        .get::<ComputedSize>()
        .copied()
        .expect("child should have a computed size after settlement drains");
    assert_eq!(child_size.width, 120.0);
    assert_eq!(child_size.height, 50.0);

    let work_state = app.world().resource::<UiWorkState>();
    assert!(work_state.is_settled());
    assert_eq!(work_state.current_generation(), 2);
    assert_eq!(work_state.last_frame_iterations(), 3);
    assert!(!work_state.budget_exhausted());
}

#[test]
fn settlement_budget_exhaustion_prevents_the_frame_from_reporting_idle() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.insert_resource(UiSettlementConfig { max_iterations: 1 });
    app.init_resource::<DeferredResizeOnce>();
    app.add_systems(
        UiSettlementSchedule,
        resize_target_in_render_sync.in_set(UnivisPostUpdateSet::RenderSync),
    );

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let child = app
        .world_mut()
        .spawn((
            ChildOf(root),
            DeferredResizeTarget,
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let child_size = app
        .world()
        .entity(child)
        .get::<ComputedSize>()
        .copied()
        .expect("child should still have a computed size after the first iteration");
    assert_eq!(child_size.width, 100.0);

    let work_state = app.world().resource::<UiWorkState>();
    assert_eq!(work_state.current_generation(), 1);
    assert_eq!(work_state.last_frame_iterations(), 1);
    assert!(work_state.budget_exhausted());
    assert!(!work_state.is_settled());
}

#[test]
fn render_only_children_do_not_start_new_layout_generations() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let parent = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let initial_generation = app.world().resource::<UiWorkState>().current_generation();
    assert_eq!(initial_generation, 1);

    app.update();
    let idle_work_state = app.world().resource::<UiWorkState>();
    assert_eq!(idle_work_state.current_generation(), initial_generation);
    assert_eq!(idle_work_state.last_frame_iterations(), 1);

    app.world_mut().spawn((ChildOf(parent), RenderOnlyChild));
    app.update();

    let work_state = app.world().resource::<UiWorkState>();
    assert_eq!(work_state.current_generation(), initial_generation);
    assert_eq!(work_state.last_frame_iterations(), 1);
    assert!(work_state.is_settled());
    assert!(!work_state.budget_exhausted());
}

#[test]
fn downward_solve_stays_scoped_to_dirty_subtrees() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.init_resource::<LayoutProfiler>();

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let branch_a = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let leaf_a = app
        .world_mut()
        .spawn((
            ChildOf(branch_a),
            UNode {
                width: UVal::Px(80.0),
                height: UVal::Px(40.0),
                ..default()
            },
        ))
        .id();

    let branch_b = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let leaf_b = app
        .world_mut()
        .spawn((
            ChildOf(branch_b),
            UNode {
                width: UVal::Px(50.0),
                height: UVal::Px(30.0),
                ..default()
            },
        ))
        .id();

    app.update();

    app.world_mut()
        .entity_mut(leaf_a)
        .get_mut::<UNode>()
        .unwrap()
        .width = UVal::Px(120.0);
    app.update();

    let profiler = app.world().resource::<LayoutProfiler>();
    assert_eq!(profiler.solved_nodes, 2);

    let branch_b_size = app
        .world()
        .entity(branch_b)
        .get::<ComputedSize>()
        .copied()
        .expect("branch_b should keep its computed size");
    let leaf_b_size = app
        .world()
        .entity(leaf_b)
        .get::<ComputedSize>()
        .copied()
        .expect("leaf_b should keep its computed size");
    assert_eq!(branch_b_size.width, 50.0);
    assert_eq!(leaf_b_size.width, 50.0);
}

#[test]
fn root_canvas_resize_requeues_nested_containers() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let branch = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let nested = app
        .world_mut()
        .spawn((
            ChildOf(branch),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                ..default()
            },
        ))
        .id();

    let leaf = app
        .world_mut()
        .spawn((
            ChildOf(nested),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Px(40.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let initial_nested_size = app
        .world()
        .entity(nested)
        .get::<ComputedSize>()
        .copied()
        .expect("nested container should be solved on the first frame");
    let initial_leaf_size = app
        .world()
        .entity(leaf)
        .get::<ComputedSize>()
        .copied()
        .expect("leaf should be solved on the first frame");

    assert_eq!(initial_nested_size.width, 400.0);
    assert_eq!(initial_leaf_size.width, 400.0);

    app.world_mut()
        .entity_mut(root)
        .get_mut::<URootUi>()
        .expect("root should keep its root component")
        .canvas = UiCanvasSize::Fixed(Vec2::new(520.0, 200.0));

    app.update();

    let resized_nested_size = app
        .world()
        .entity(nested)
        .get::<ComputedSize>()
        .copied()
        .expect("nested container should be re-solved after root resize");
    let resized_leaf_size = app
        .world()
        .entity(leaf)
        .get::<ComputedSize>()
        .copied()
        .expect("leaf should be re-solved after root resize");

    assert_eq!(resized_nested_size.width, 520.0);
    assert_eq!(resized_leaf_size.width, 520.0);
}

#[test]
fn root_settlement_generation_stays_scoped_to_the_changed_root() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root_a = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let leaf_a = app
        .world_mut()
        .spawn((
            ChildOf(root_a),
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                ..default()
            },
        ))
        .id();

    let root_b = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(320.0, 180.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    app.world_mut().spawn((
        ChildOf(root_b),
        UNode {
            width: UVal::Px(80.0),
            height: UVal::Px(40.0),
            ..default()
        },
    ));

    app.update();

    let root_a_state = app
        .world()
        .entity(root_a)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_a should expose settlement state");
    let root_b_state = app
        .world()
        .entity(root_b)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_b should expose settlement state");
    assert_eq!(root_a_state.current_generation, 1);
    assert_eq!(root_b_state.current_generation, 1);
    assert!(root_a_state.is_settled());
    assert!(root_b_state.is_settled());

    app.world_mut()
        .entity_mut(leaf_a)
        .get_mut::<UNode>()
        .expect("leaf_a should keep its UNode")
        .width = UVal::Px(140.0);

    app.update();

    let root_a_state = app
        .world()
        .entity(root_a)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_a should keep settlement state");
    let root_b_state = app
        .world()
        .entity(root_b)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_b should keep settlement state");

    assert_eq!(
        root_a_state.current_generation,
        2,
        "work_state={:?}, root_a={root_a_state:?}, root_b={root_b_state:?}",
        app.world().resource::<UiWorkState>()
    );
    assert_eq!(root_a_state.pending_measure, 0);
    assert_eq!(root_a_state.pending_solve, 0);
    assert_eq!(root_a_state.pending_render, 0);
    assert!(root_a_state.is_settled());

    assert_eq!(root_b_state.current_generation, 1);
    assert_eq!(root_b_state.pending_measure, 0);
    assert_eq!(root_b_state.pending_solve, 0);
    assert_eq!(root_b_state.pending_render, 0);
    assert!(root_b_state.is_settled());
}

#[test]
fn render_only_node_change_skips_solve_work() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.init_resource::<LayoutProfiler>();

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let child = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(50.0),
                background_color: Color::srgb(0.1, 0.1, 0.1),
                ..default()
            },
        ))
        .id();

    app.update();

    app.world_mut()
        .entity_mut(child)
        .get_mut::<UNode>()
        .expect("child should keep its UNode")
        .background_color = Color::srgb(0.8, 0.2, 0.1);

    app.update();

    let profiler = app.world().resource::<LayoutProfiler>();
    assert_eq!(profiler.solved_nodes, 0);

    let child_size = app
        .world()
        .entity(child)
        .get::<ComputedSize>()
        .copied()
        .expect("child should keep its computed size");
    assert_eq!(child_size.width, 100.0);
    assert_eq!(child_size.height, 50.0);
}

#[test]
fn settled_root_means_all_node_stage_versions_are_clean() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let parent = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    app.world_mut().spawn((
        ChildOf(parent),
        UNode {
            width: UVal::Px(100.0),
            height: UVal::Px(50.0),
            ..default()
        },
    ));

    app.update();

    let root_state = app
        .world()
        .entity(root)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root should expose settlement state");
    assert!(root_state.is_settled());

    let node_roots = {
        let world = app.world_mut();
        let mut query = world.query::<(Entity, Option<&CachedUiContext>)>();
        query
            .iter(world)
            .map(|(entity, cached)| {
                let root_entity = cached
                    .and_then(|context| context.root_entity)
                    .unwrap_or(entity);
                (entity, root_entity)
            })
            .collect::<Vec<_>>()
    };
    let cache = app.world().resource::<LayoutCache>();
    for (entity, node_root) in node_roots {
        if node_root != root {
            continue;
        }

        let versions = cache.stage_versions(entity);
        assert_eq!(
            versions.measure_done_generation,
            versions.measure_input_generation
        );
        assert_eq!(
            versions.solve_done_generation,
            versions.solve_input_generation
        );
        assert_eq!(
            versions.render_done_generation,
            versions.render_input_generation
        );
    }
}

#[test]
fn parent_layout_updates_immediately_after_child_size_change() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let parent = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let leaf = app
        .world_mut()
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(100.0),
                height: UVal::Px(24.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let initial_parent = app
        .world()
        .entity(parent)
        .get::<ComputedSize>()
        .copied()
        .expect("parent should have computed size");
    assert_eq!(initial_parent.width, 100.0);
    assert_eq!(initial_parent.height, 24.0);

    app.world_mut()
        .entity_mut(leaf)
        .get_mut::<UNode>()
        .expect("leaf should keep its UNode")
        .width = UVal::Px(160.0);

    app.update();

    let updated_parent = app
        .world()
        .entity(parent)
        .get::<ComputedSize>()
        .copied()
        .expect("parent should keep its computed size");
    let updated_leaf = app
        .world()
        .entity(leaf)
        .get::<ComputedSize>()
        .copied()
        .expect("leaf should keep its computed size");
    let work_state = app.world().resource::<UiWorkState>();

    assert_eq!(updated_leaf.width, 160.0);
    assert_eq!(updated_parent.width, 160.0);
    assert_eq!(updated_parent.height, 24.0);
    assert!(work_state.is_settled());
}

#[test]
fn rollout_flags_can_fall_back_to_full_scan_layout_paths() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.insert_resource(UiRolloutConfig {
        use_incremental_measure: false,
        use_incremental_solve: false,
        use_incremental_render: false,
        ..default()
    });

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let child = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(120.0),
                height: UVal::Px(36.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let child_size = app
        .world()
        .entity(child)
        .get::<ComputedSize>()
        .copied()
        .expect("child should have computed size");
    let work_state = app.world().resource::<UiWorkState>();

    assert_eq!(child_size.width, 120.0);
    assert_eq!(child_size.height, 36.0);
    assert!(work_state.is_settled());
}

#[test]
fn root_resolution_change_stays_scoped_to_the_changed_root() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root_a = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    app.world_mut().spawn((
        ChildOf(root_a),
        UNode {
            width: UVal::Px(100.0),
            height: UVal::Px(50.0),
            ..default()
        },
    ));

    let root_b = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(320.0, 180.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    app.world_mut().spawn((
        ChildOf(root_b),
        UNode {
            width: UVal::Px(80.0),
            height: UVal::Px(40.0),
            ..default()
        },
    ));

    app.update();

    app.world_mut()
        .entity_mut(root_a)
        .get_mut::<URootUi>()
        .expect("root_a should keep its root component")
        .canvas = UiCanvasSize::Fixed(Vec2::new(520.0, 200.0));

    app.update();

    let root_a_state = app
        .world()
        .entity(root_a)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_a should expose settlement state");
    let root_b_state = app
        .world()
        .entity(root_b)
        .get::<UiRootSettlementState>()
        .copied()
        .expect("root_b should expose settlement state");

    assert_eq!(root_a_state.current_generation, 2);
    assert!(root_a_state.is_settled());
    assert_eq!(root_b_state.current_generation, 1);
    assert!(root_b_state.is_settled());
}
