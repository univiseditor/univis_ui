use crate::layout::UnivisLayoutPlugin;
use crate::layout::components::LayoutDepth;
use crate::layout::core::layout_cache::LayoutCache;
use crate::layout::geometry::UVal;
use crate::layout::layout_system::{URootUi, UiCanvasSize, UiRootSettlementState};
use crate::layout::query::ComputedSize;
use crate::layout::univis_node::{ULayout, UNode};
use crate::schedule::{
    UiSettlementConfig, UiSettlementSchedule, UiWorkStage, UiWorkState, UnivisPostUpdateSet,
};
use bevy::prelude::*;

#[derive(Component)]
struct DeferredResizeTarget;

#[derive(Resource, Default)]
struct DeferredResizeOnce(bool);

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
fn deeply_nested_layout_hierarchy_settles_without_exhausting_budget() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 400.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let mut current_parent = root;
    for _ in 0..7 {
        let child = app
            .world_mut()
            .spawn((
                ChildOf(current_parent),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    ..default()
                },
                ULayout::default(),
            ))
            .id();
        current_parent = child;
    }

    let leaf = app
        .world_mut()
        .spawn((
            ChildOf(current_parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
        ))
        .id();

    app.update();

    let work_state = app.world().resource::<UiWorkState>();
    assert!(work_state.is_settled(), "Frame 1 must be settled");
    assert!(
        !work_state.budget_exhausted(),
        "Frame 1 must not exhaust budget"
    );

    // Frame 2: Resize root.
    app.world_mut()
        .entity_mut(root)
        .get_mut::<URootUi>()
        .expect("root must exist")
        .canvas = UiCanvasSize::Fixed(Vec2::new(800.0, 800.0));

    app.update();

    let work_state = app.world().resource::<UiWorkState>();
    assert!(
        work_state.is_settled(),
        "Frame 2 must be settled (iterations={}, budget_exhausted={})",
        work_state.last_frame_iterations(),
        work_state.budget_exhausted()
    );
    assert!(
        !work_state.budget_exhausted(),
        "Frame 2 must not exhaust budget"
    );
    assert_eq!(work_state.last_frame_iterations(), 2);

    let leaf_size = app
        .world()
        .entity(leaf)
        .get::<ComputedSize>()
        .copied()
        .expect("leaf must have computed size");
    assert_eq!(leaf_size.width, 800.0);
    assert_eq!(leaf_size.height, 800.0);
}
