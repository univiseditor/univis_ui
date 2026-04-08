use super::*;
use bevy::ecs::system::SystemState;

#[derive(Clone, Copy)]
struct LayoutChangeFlags {
    intrinsic_changed: bool,
    node_changed: bool,
    layout_changed: bool,
    uself_changed: bool,
}

fn should_skip_intrinsic_only_container_change(
    flags: LayoutChangeFlags,
    has_children: bool,
) -> bool {
    flags.intrinsic_changed
        && !flags.node_changed
        && !flags.layout_changed
        && !flags.uself_changed
        && has_children
}

fn rebuild_cache_depths(world: &mut World, cache: &mut LayoutCache, max_depth: usize) {
    let mut system_state: SystemState<Query<(Entity, &LayoutDepth)>> = SystemState::new(world);
    let query = system_state.get(world);
    cache.rebuild_depth_map(&query, max_depth);
}

fn intrinsic_only_flags() -> LayoutChangeFlags {
    LayoutChangeFlags {
        intrinsic_changed: true,
        node_changed: false,
        layout_changed: false,
        uself_changed: false,
    }
}

fn set_generation(work_state: &mut UiWorkState) {
    assert!(work_state.begin_generation(UiPendingStages {
        hierarchy: true,
        solve: true,
        render: true,
        ..default()
    }));
}

fn sample_root(root_entity: Entity) -> (ResolvedRootUi, ResolvedRootStack) {
    let resolved = ResolvedRootUi {
        root_entity,
        space: UiSpace::Screen,
        canvas: UiCanvasSize::Viewport,
        canvas_size: Vec2::new(800.0, 600.0),
        camera_entity: None,
        meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
        resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
    };
    let band_width = 0.04;
    let stack = ResolvedRootStack {
        authored_root_z: 0.0,
        spawn_rank: 0,
        capsule_sort_key: 0.0,
        capsule_band_base: 0.0,
        capsule_band_width: band_width,
        capsule_band_step: band_width / 2048.0,
        applied_root_z: 0.0,
        initialized: true,
    };
    (resolved, stack)
}

#[test]
fn intrinsic_only_change_on_container_is_skipped() {
    assert!(should_skip_intrinsic_only_container_change(
        intrinsic_only_flags(),
        true,
    ));
}

#[test]
fn intrinsic_only_change_on_leaf_is_not_skipped() {
    assert!(!should_skip_intrinsic_only_container_change(
        intrinsic_only_flags(),
        false,
    ));
}

#[test]
fn non_intrinsic_change_is_not_skipped() {
    let mut flags = intrinsic_only_flags();
    flags.node_changed = true;

    assert!(!should_skip_intrinsic_only_container_change(flags, true));
}

#[test]
fn layout_change_is_not_skipped() {
    let mut flags = intrinsic_only_flags();
    flags.layout_changed = true;

    assert!(!should_skip_intrinsic_only_container_change(flags, true));
}

#[test]
fn uself_change_is_not_skipped() {
    let mut flags = intrinsic_only_flags();
    flags.uself_changed = true;

    assert!(!should_skip_intrinsic_only_container_change(flags, true));
}

#[test]
fn stage_versions_mark_and_complete_work() {
    let mut cache = LayoutCache::default();
    let entity = Entity::from_raw_u32(7).expect("test entity should be constructible");

    cache.mark_measure_dirty(entity, 3);
    cache.mark_solve_dirty_generation(entity, 3);
    cache.mark_render_dirty(entity, 4);

    let versions = cache.stage_versions(entity);
    assert_eq!(versions.measure_input_generation, 3);
    assert_eq!(versions.measure_done_generation, 0);
    assert_eq!(versions.solve_input_generation, 3);
    assert_eq!(versions.solve_done_generation, 0);
    assert_eq!(versions.render_input_generation, 4);
    assert_eq!(versions.render_done_generation, 0);

    cache.complete_measure(entity);
    cache.complete_solve(entity);
    cache.complete_render(entity);

    let versions = cache.stage_versions(entity);
    assert_eq!(versions.measure_done_generation, 3);
    assert_eq!(versions.solve_done_generation, 3);
    assert_eq!(versions.render_done_generation, 4);
}

#[test]
fn frontier_queues_dedupe_same_entity_and_keep_latest_generation() {
    let mut world = World::new();
    let entity = world.spawn(LayoutDepth(0)).id();
    let mut cache = LayoutCache::default();
    rebuild_cache_depths(&mut world, &mut cache, 0);

    cache.mark_measure_dirty(entity, 1);
    cache.mark_measure_dirty(entity, 1);
    cache.mark_measure_dirty(entity, 3);
    cache.mark_measure_dirty(entity, 2);

    let frontier = cache.take_measure_frontier();
    assert_eq!(frontier, vec![entity]);
    assert_eq!(cache.stage_versions(entity).measure_input_generation, 3);
}

#[test]
fn frontier_queues_keep_measure_bottom_up_and_solve_top_down_order() {
    let mut world = World::new();
    let root = world.spawn(LayoutDepth(0)).id();
    let child = world.spawn(LayoutDepth(1)).id();
    let leaf = world.spawn(LayoutDepth(2)).id();
    let mut cache = LayoutCache::default();
    rebuild_cache_depths(&mut world, &mut cache, 2);

    cache.mark_measure_dirty(root, 1);
    cache.mark_measure_dirty(child, 1);
    cache.mark_measure_dirty(leaf, 1);
    assert_eq!(cache.take_measure_frontier(), vec![leaf, child, root]);

    cache.mark_solve_dirty_generation(root, 1);
    cache.mark_solve_dirty_generation(child, 1);
    cache.mark_solve_dirty_generation(leaf, 1);
    assert_eq!(cache.take_solve_frontier(), vec![root, child, leaf]);
}

#[test]
fn root_stack_position_noise_does_not_dirty_any_subtree() {
    let mut app = App::new();
    app.init_resource::<LayoutCache>();

    let mut work_state = UiWorkState::default();
    set_generation(&mut work_state);
    app.insert_resource(work_state);
    app.add_systems(Update, track_root_layout_changes);

    let root = app.world_mut().spawn((UNode::default(),)).id();
    let child = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(root)))
        .id();
    let (resolved, stack) = sample_root(root);
    app.world_mut().entity_mut(root).insert((resolved, stack));

    {
        let mut cache = app.world_mut().resource_mut::<LayoutCache>();
        cache.update_root_snapshot(root, resolved);
        cache.update_root_stack_snapshot(root, Some(stack.descendant_context_snapshot()));
    }

    {
        let world = app.world_mut();
        let mut entity = world.entity_mut(root);
        let mut stack = entity
            .get_mut::<ResolvedRootStack>()
            .expect("root should have stack");
        stack.authored_root_z = 24.0;
        stack.capsule_sort_key = 24.0;
        stack.capsule_band_base = 24.0;
        stack.applied_root_z = 96.0;
    }

    app.update();

    let cache = app.world().resource::<LayoutCache>();
    assert!(!cache.is_solve_dirty(root));
    assert!(!cache.is_solve_dirty(child));
}

#[test]
fn root_stack_band_change_dirties_only_the_affected_subtree() {
    let mut app = App::new();
    app.init_resource::<LayoutCache>();

    let mut work_state = UiWorkState::default();
    set_generation(&mut work_state);
    app.insert_resource(work_state);
    app.add_systems(Update, track_root_layout_changes);

    let root_a = app.world_mut().spawn((UNode::default(),)).id();
    let child_a = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(root_a)))
        .id();
    let root_b = app.world_mut().spawn((UNode::default(),)).id();
    let child_b = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(root_b)))
        .id();

    let (mut resolved_a, stack_a) = sample_root(root_a);
    let (mut resolved_b, stack_b) = sample_root(root_b);
    resolved_a.space = UiSpace::World2d;
    resolved_a.canvas = UiCanvasSize::Fixed(Vec2::new(800.0, 600.0));
    resolved_b.space = UiSpace::World2d;
    resolved_b.canvas = UiCanvasSize::Fixed(Vec2::new(800.0, 600.0));
    app.world_mut()
        .entity_mut(root_a)
        .insert((resolved_a, stack_a));
    app.world_mut()
        .entity_mut(root_b)
        .insert((resolved_b, stack_b));

    {
        let mut cache = app.world_mut().resource_mut::<LayoutCache>();
        cache.update_root_snapshot(root_a, resolved_a);
        cache.update_root_stack_snapshot(root_a, Some(stack_a.descendant_context_snapshot()));
        cache.update_root_snapshot(root_b, resolved_b);
        cache.update_root_stack_snapshot(root_b, Some(stack_b.descendant_context_snapshot()));
    }

    {
        let world = app.world_mut();
        let mut entity = world.entity_mut(root_a);
        let mut stack = entity
            .get_mut::<ResolvedRootStack>()
            .expect("root_a should have stack");
        stack.capsule_band_width *= 2.0;
        stack.capsule_band_step *= 2.0;
    }

    app.update();

    let cache = app.world().resource::<LayoutCache>();
    assert!(cache.is_solve_dirty(root_a));
    assert!(cache.is_solve_dirty(child_a));
    assert!(!cache.is_solve_dirty(root_b));
    assert!(!cache.is_solve_dirty(child_b));
}
