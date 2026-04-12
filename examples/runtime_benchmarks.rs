//! Runtime-oriented performance harness.
//!
//! Related docs:
//! - `docs/src/en/performance/benchmarks.md`
//! - `docs/src/ar/performance/benchmarks.md`

use std::collections::BTreeSet;
use std::env;
use std::time::Instant;

use bevy::camera::{
    CameraProjection, ComputedCameraValues, NormalizedRenderTarget, RenderTargetInfo,
};
use bevy::picking::backend::prelude::*;
use bevy::picking::pointer::Location;
use bevy::prelude::*;
use bevy::text::{ComputedTextBlock, Font, TextPlugin};
use univis_ui_engine::layout::UnivisLayoutPlugin;
use univis_ui_engine::layout::geometry::ComputedSize;
use univis_ui_engine::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};
use univis_ui_engine::prelude::*;
use univis_ui_engine::schedule::UiSettlementRuntimeState;
use univis_ui_interaction::interaction::picking::univis_picking_backend;
use univis_ui_interaction::prelude::UInteraction;
use univis_ui_widgets::prelude::UButton;
use univis_ui_widgets::widget::button::UnivisButtonPlugin;
use univis_ui_widgets::widget::text_label::{
    UTextLabel, fit_node_to_text_size, measure_text_label_layout, sync_text_label_intrinsic_size,
};

const DEFAULT_WARMUP: usize = 32;
const DEFAULT_ITERATIONS: usize = 120;
const VIEWPORT_SIZE: UVec2 = UVec2::new(1600, 900);
const SCREEN_CANVAS: Vec2 = Vec2::new(1600.0, 900.0);
const STRESS_ROOT_COUNT: usize = 10_000;
const STRESS_NODE_COUNT: usize = 1_000_000;
const STRESS_NODES_PER_ROOT: usize = STRESS_NODE_COUNT / STRESS_ROOT_COUNT;
const STRESS_PANELS_PER_ROOT: usize = 1;
const STRESS_LEAVES_PER_ROOT: usize = STRESS_NODES_PER_ROOT - STRESS_PANELS_PER_ROOT;
const TEXT_FONT_BYTES: &[u8] =
    include_bytes!("../crates/univis_ui_style/src/style/assets/fonts/Inter-Regular.ttf");

#[derive(Clone, Copy)]
struct BenchmarkSummary {
    average_ms: f64,
    p95_ms: f64,
    max_ms: f64,
}

struct RuntimeWorkload {
    name: &'static str,
    item_count: usize,
    budget_ms: f64,
    stress_only: bool,
    build: fn() -> RuntimeScenario,
}

struct RuntimeScenario {
    app: App,
    before_update: fn(&mut App, usize),
    after_update: fn(&mut App),
}

#[derive(Component)]
struct BenchRootIndex(usize);

#[derive(Component)]
struct BenchLabelIndex(usize);

#[derive(Component)]
struct BenchPanelIndex(usize);

#[derive(Component)]
struct BenchPointerMarker;

#[derive(Resource)]
struct BenchIdleScenario {
    settled_generation: u64,
}

#[derive(Resource)]
struct BenchScopedLeafScenario {
    target_leaves: Vec<Entity>,
    control_leaves: Vec<Entity>,
    last_target_index: usize,
    expected_target_width: f32,
}

#[derive(Resource)]
struct BenchRenderOnlyScenario {
    nodes: Vec<Entity>,
    baseline_sizes: Vec<ComputedSize>,
    last_target_index: usize,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let selected_scenarios = parse_multi_value_flag(&args, "--scenario");
    let selected_lookup: BTreeSet<&str> = selected_scenarios.iter().map(String::as_str).collect();
    let include_stress = args.iter().any(|arg| arg == "--include-stress");
    let enforce_budgets = args.iter().any(|arg| arg == "--check");
    let warmup = parse_env_usize("UNIVIS_PERF_WARMUP", DEFAULT_WARMUP);
    let iterations = parse_env_usize("UNIVIS_PERF_ITERATIONS", DEFAULT_ITERATIONS);

    let all_workloads = vec![
        RuntimeWorkload {
            name: "root_capsules_96",
            item_count: 96,
            budget_ms: 6.000,
            stress_only: false,
            build: build_root_capsules_scenario,
        },
        RuntimeWorkload {
            name: "idle_after_settle_96",
            item_count: 96,
            budget_ms: 2.000,
            stress_only: false,
            build: build_idle_after_settle_scenario,
        },
        RuntimeWorkload {
            name: "single_root_local_change_96",
            item_count: 96,
            budget_ms: 1.200,
            stress_only: false,
            build: build_single_root_local_change_scenario,
        },
        RuntimeWorkload {
            name: "render_only_change_512",
            item_count: 512,
            budget_ms: 1.500,
            stress_only: false,
            build: build_render_only_change_scenario,
        },
        RuntimeWorkload {
            name: "text_measure_180",
            item_count: 180,
            budget_ms: 4.750,
            stress_only: false,
            build: build_text_measure_scenario,
        },
        RuntimeWorkload {
            name: "picking_grid_512",
            item_count: 512,
            budget_ms: 4.000,
            stress_only: false,
            build: build_picking_scenario,
        },
        RuntimeWorkload {
            name: "widget_panels_240",
            item_count: 240,
            budget_ms: 8.000,
            stress_only: false,
            build: build_widget_panels_scenario,
        },
        RuntimeWorkload {
            name: "world3d_panels_48",
            item_count: 48,
            budget_ms: 6.000,
            stress_only: false,
            build: build_world3d_panels_scenario,
        },
        RuntimeWorkload {
            name: "roots_10k_nodes_1m",
            item_count: STRESS_NODE_COUNT,
            budget_ms: f64::INFINITY,
            stress_only: true,
            build: build_roots_10k_nodes_1m_scenario,
        },
    ];

    if args.iter().any(|arg| arg == "--list") {
        println!("Available runtime workloads:");
        for workload in &all_workloads {
            let tag = if workload.stress_only {
                " [stress]"
            } else {
                ""
            };
            println!("- {}{}", workload.name, tag);
        }
        return;
    }

    let known_workloads: BTreeSet<&str> =
        all_workloads.iter().map(|workload| workload.name).collect();
    for selected in &selected_lookup {
        if !known_workloads.contains(selected) {
            eprintln!("Unknown runtime workload: {selected}");
            std::process::exit(2);
        }
    }

    let workloads: Vec<_> = all_workloads
        .into_iter()
        .filter(|workload| {
            let matches_selection =
                selected_lookup.is_empty() || selected_lookup.contains(workload.name);
            let enabled_by_default_or_flag =
                !workload.stress_only || include_stress || selected_lookup.contains(workload.name);
            matches_selection && enabled_by_default_or_flag
        })
        .collect();

    if workloads.is_empty() {
        eprintln!("No runtime workloads selected.");
        std::process::exit(2);
    }

    println!("Univis runtime benchmark");
    println!(
        "warmup={} iterations={} enforce_budgets={} include_stress={}",
        warmup, iterations, enforce_budgets, include_stress
    );
    println!(
        "{:<24} {:>8} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "scenario", "items", "avg_ms", "p95_ms", "max_ms", "budget", "status"
    );

    let mut failed = Vec::new();

    for workload in &workloads {
        let summary = benchmark_workload(workload, warmup, iterations);
        let within_budget = summary.p95_ms <= workload.budget_ms;
        let status = if within_budget { "ok" } else { "over" };

        println!(
            "{:<24} {:>8} {:>10.3} {:>10.3} {:>10.3} {:>10.3} {:>8}",
            workload.name,
            workload.item_count,
            summary.average_ms,
            summary.p95_ms,
            summary.max_ms,
            workload.budget_ms,
            status
        );

        if enforce_budgets && !within_budget {
            failed.push((workload.name, summary, workload.budget_ms));
        }
    }

    if !failed.is_empty() {
        eprintln!();
        eprintln!("Budget failures:");
        for (name, summary, budget_ms) in failed {
            eprintln!(
                "- {}: p95 {:.3}ms > budget {:.3}ms (avg {:.3}ms, max {:.3}ms)",
                name, summary.p95_ms, budget_ms, summary.average_ms, summary.max_ms
            );
        }
        std::process::exit(1);
    }
}

fn parse_env_usize(key: &str, default_value: usize) -> usize {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn parse_multi_value_flag(args: &[String], flag: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut index = 0;

    while index < args.len() {
        if args[index] == flag {
            let Some(value) = args.get(index + 1) else {
                eprintln!("Expected a value after {flag}");
                std::process::exit(2);
            };
            values.push(value.clone());
            index += 2;
        } else {
            index += 1;
        }
    }

    values
}

fn benchmark_workload(
    workload: &RuntimeWorkload,
    warmup: usize,
    iterations: usize,
) -> BenchmarkSummary {
    let mut scenario = (workload.build)();

    for step in 0..warmup {
        (scenario.before_update)(&mut scenario.app, step);
        scenario.app.update();
        (scenario.after_update)(&mut scenario.app);
    }

    let mut samples = Vec::with_capacity(iterations);
    for step in 0..iterations {
        (scenario.before_update)(&mut scenario.app, step);
        let start = Instant::now();
        scenario.app.update();
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
        (scenario.after_update)(&mut scenario.app);
    }

    samples.sort_by(|left, right| left.total_cmp(right));

    let average_ms = samples.iter().sum::<f64>() / samples.len() as f64;
    let p95_index = ((samples.len() - 1) as f64 * 0.95).round() as usize;
    let p95_ms = samples[p95_index];
    let max_ms = *samples.last().unwrap_or(&0.0);

    BenchmarkSummary {
        average_ms,
        p95_ms,
        max_ms,
    }
}

fn build_root_capsules_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UnivisLayoutPlugin);

    let camera_entity = spawn_orthographic_camera(&mut app, 1200.0);
    populate_root_capsules_scene(&mut app, camera_entity);

    RuntimeScenario {
        app,
        before_update: mutate_root_capsules,
        after_update: no_op_after_update,
    }
}

fn build_idle_after_settle_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UnivisLayoutPlugin);

    let camera_entity = spawn_orthographic_camera(&mut app, 1200.0);
    populate_root_capsules_scene(&mut app, camera_entity);
    settle_runtime_scenario(&mut app);
    let settled_generation = app
        .world()
        .resource::<UiSettlementRuntimeState>()
        .current_generation();
    app.world_mut()
        .insert_resource(BenchIdleScenario { settled_generation });

    RuntimeScenario {
        app,
        before_update: no_op_before_update,
        after_update: assert_idle_after_update,
    }
}

fn populate_root_capsules_scene(app: &mut App, camera_entity: Entity) {
    for index in 0..96usize {
        let root = match index % 4 {
            0 => URootUi {
                camera: UiCameraRef::Entity(camera_entity),
                ..URootUi::screen()
            },
            1 => URootUi {
                camera: UiCameraRef::Entity(camera_entity),
                meters_per_unit: 0.0015,
                ..URootUi::world_2d(Vec2::new(720.0, 420.0))
            },
            2 => URootUi {
                camera: UiCameraRef::Entity(camera_entity),
                meters_per_unit: 0.0020,
                ..URootUi::world_3d(Vec2::new(640.0, 360.0))
            },
            _ => URootUi {
                camera: UiCameraRef::Entity(camera_entity),
                canvas: UiCanvasSize::FitContent {
                    min: Vec2::new(280.0, 180.0),
                    max: Some(Vec2::new(960.0, 640.0)),
                },
                ..URootUi::world_2d_fit_content()
            },
        };

        let root_entity = app
            .world_mut()
            .spawn((
                root,
                BenchRootIndex(index),
                Transform::from_xyz(
                    (index % 12) as f32 * 0.12,
                    (index / 12) as f32 * 0.08,
                    (index % 9) as f32 * 0.02,
                ),
            ))
            .id();

        for panel_index in 0..6usize {
            let panel = app
                .world_mut()
                .spawn((
                    ChildOf(root_entity),
                    UNode {
                        width: UVal::Px(180.0 + (panel_index % 3) as f32 * 36.0),
                        height: if index % 4 == 3 {
                            UVal::Content
                        } else {
                            UVal::Px(92.0 + (panel_index % 2) as f32 * 22.0)
                        },
                        padding: USides::all(8.0),
                        margin: USides::all(4.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 6.0,
                        ..default()
                    },
                ))
                .id();

            for leaf_index in 0..4usize {
                app.world_mut().spawn((
                    ChildOf(panel),
                    UNode {
                        width: UVal::Px(112.0 + (leaf_index % 2) as f32 * 24.0),
                        height: UVal::Px(20.0 + (leaf_index % 3) as f32 * 6.0),
                        margin: USides::all(2.0),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn build_text_measure_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        TextPlugin,
    ));
    app.add_systems(
        Update,
        (
            measure_text_label_layout,
            sync_text_label_intrinsic_size,
            fit_node_to_text_size,
        )
            .chain(),
    );

    let font = insert_benchmark_font(&mut app);

    let parent_widths = [260.0, 320.0, 380.0, 440.0];
    for column in 0..4usize {
        let parent = app
            .world_mut()
            .spawn((
                UNode {
                    width: UVal::Px(parent_widths[column]),
                    height: UVal::Px(900.0),
                    padding: USides::all(12.0),
                    ..default()
                },
                ComputedSize {
                    width: parent_widths[column],
                    height: 900.0,
                    ..default()
                },
            ))
            .id();

        for row in 0..45usize {
            let index = column * 45 + row;
            app.world_mut().spawn((
                ChildOf(parent),
                BenchLabelIndex(index),
                UNode {
                    width: if index % 5 == 0 {
                        UVal::Auto
                    } else {
                        UVal::Px(180.0 + (index % 4) as f32 * 24.0)
                    },
                    height: UVal::Auto,
                    padding: USides::axes(6.0, 4.0),
                    margin: USides::all(2.0),
                    ..default()
                },
                UTextLabel {
                    text: benchmark_text_variant(index, 0).to_string(),
                    font: font.clone(),
                    font_size: 14.0 + (index % 5) as f32 * 2.0,
                    autosize: index % 5 == 0,
                    max_lines: if index % 4 == 0 { Some(2) } else { None },
                    overflow: match index % 3 {
                        0 => univis_ui_widgets::widget::text_label::UTextOverflow::Ellipsis,
                        1 => univis_ui_widgets::widget::text_label::UTextOverflow::Clip,
                        _ => univis_ui_widgets::widget::text_label::UTextOverflow::Visible,
                    },
                    linebreak: if index % 2 == 0 {
                        LineBreak::WordBoundary
                    } else {
                        LineBreak::AnyCharacter
                    },
                    ..default()
                },
                ComputedSize::default(),
                ComputedTextBlock::default(),
            ));
        }
    }

    RuntimeScenario {
        app,
        before_update: mutate_text_labels,
        after_update: no_op_after_update,
    }
}

fn build_picking_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.add_systems(Update, univis_picking_backend);

    let screen_camera = spawn_cached_camera(
        &mut app,
        false,
        GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1200.0)),
    );
    let perspective_camera = spawn_cached_camera(
        &mut app,
        true,
        GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 5.0)),
    );

    let screen_root = app.world_mut().spawn_empty().id();
    let screen_root_state = sample_resolved_root(screen_root, UiSpace::Screen, screen_camera);
    app.world_mut()
        .entity_mut(screen_root)
        .insert(screen_root_state)
        .insert(GlobalTransform::default());

    let world_root = app.world_mut().spawn_empty().id();
    let world_root_state = sample_resolved_root(world_root, UiSpace::World3d, perspective_camera);
    app.world_mut()
        .entity_mut(world_root)
        .insert(world_root_state)
        .insert(GlobalTransform::default());

    for index in 0..256usize {
        let x = (index % 16) as f32 * 54.0 - 405.0;
        let y = (index / 16) as f32 * 34.0 - 255.0;
        app.world_mut().spawn((
            ChildOf(screen_root),
            UInteraction::default(),
            UNode {
                width: UVal::Px(42.0),
                height: UVal::Px(24.0),
                ..default()
            },
            ComputedSize {
                width: 42.0,
                height: 24.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::from(Transform::from_xyz(x, y, 0.0)),
        ));
    }

    for index in 0..256usize {
        let x = (index % 16) as f32 * 0.11 - 0.82;
        let y = (index / 16) as f32 * 0.07 - 0.56;
        app.world_mut().spawn((
            ChildOf(world_root),
            UInteraction::default(),
            UNode {
                width: UVal::Px(160.0),
                height: UVal::Px(92.0),
                ..default()
            },
            ComputedSize {
                width: 160.0,
                height: 92.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::from(Transform::from_xyz(x, y, 0.0)),
        ));
    }

    app.world_mut().spawn((
        BenchPointerMarker,
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: VIEWPORT_SIZE.x,
                height: VIEWPORT_SIZE.y,
            },
            position: Vec2::new(800.0, 450.0),
        }),
    ));

    RuntimeScenario {
        app,
        before_update: mutate_pointer_location,
        after_update: drain_pointer_hits,
    }
}

fn build_single_root_local_change_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UnivisLayoutPlugin);

    let mut target_leaves = Vec::with_capacity(96);
    let mut control_leaves = Vec::with_capacity(96);
    for index in 0..96usize {
        let root = app
            .world_mut()
            .spawn((
                BenchRootIndex(index),
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

        let leaf = app
            .world_mut()
            .spawn((
                ChildOf(branch_a),
                UNode {
                    width: scoped_leaf_base_width(index),
                    height: UVal::Px(40.0 + (index % 3) as f32 * 6.0),
                    ..default()
                },
            ))
            .id();
        target_leaves.push(leaf);

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

        let control_leaf = app
            .world_mut()
            .spawn((
                ChildOf(branch_b),
                UNode {
                    width: UVal::Px(56.0 + (index % 4) as f32 * 8.0),
                    height: UVal::Px(24.0 + (index % 2) as f32 * 4.0),
                    ..default()
                },
            ))
            .id();
        control_leaves.push(control_leaf);
    }

    settle_runtime_scenario(&mut app);
    app.world_mut().insert_resource(BenchScopedLeafScenario {
        target_leaves,
        control_leaves,
        last_target_index: 0,
        expected_target_width: scoped_leaf_base_width_px(0),
    });

    RuntimeScenario {
        app,
        before_update: mutate_single_root_local_change,
        after_update: assert_localized_solve_after_update,
    }
}

fn build_render_only_change_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UnivisLayoutPlugin);

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(1600.0, 900.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 16,
                gap: 8.0,
                container_ext: ULayoutContainerExt {
                    grid: ULayoutGridContainer {
                        template_columns: vec![UTrackSize::Fr(1.0); 16],
                        auto_rows: UTrackSize::Px(32.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let mut node_entities = Vec::with_capacity(512);
    for index in 0..512usize {
        let node = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UNode {
                    width: UVal::Auto,
                    height: UVal::Px(28.0 + (index % 3) as f32 * 2.0),
                    margin: USides::all(1.0),
                    background_color: render_only_color(index, 0),
                    border_radius: UCornerRadius::all(6.0),
                    ..default()
                },
            ))
            .id();
        node_entities.push(node);
    }

    settle_runtime_scenario(&mut app);
    let baseline_sizes = node_entities
        .iter()
        .map(|entity| {
            app.world()
                .entity(*entity)
                .get::<ComputedSize>()
                .copied()
                .expect("render-only node should have a computed size after settling")
        })
        .collect();
    app.world_mut().insert_resource(BenchRenderOnlyScenario {
        nodes: node_entities,
        baseline_sizes,
        last_target_index: 0,
    });

    RuntimeScenario {
        app,
        before_update: mutate_render_only_change,
        after_update: assert_idle_after_update,
    }
}

fn build_widget_panels_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        TextPlugin,
    ));
    app.add_plugins(UnivisLayoutPlugin);
    app.add_plugins(UnivisButtonPlugin);
    app.add_systems(
        Update,
        (
            measure_text_label_layout,
            sync_text_label_intrinsic_size,
            fit_node_to_text_size,
        )
            .chain(),
    );

    let font = insert_benchmark_font(&mut app);
    let camera = spawn_orthographic_camera(&mut app, 1400.0);
    let root = app
        .world_mut()
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera),
                ..URootUi::screen()
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 6,
                gap: 12.0,
                container_ext: ULayoutContainerExt {
                    grid: ULayoutGridContainer {
                        template_columns: vec![UTrackSize::Fr(1.0); 6],
                        auto_rows: UTrackSize::Auto,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    for index in 0..60usize {
        let panel = app
            .world_mut()
            .spawn((
                ChildOf(root),
                BenchPanelIndex(index),
                UNode {
                    width: UVal::Percent(1.0),
                    min_width: 180.0,
                    height: UVal::Content,
                    padding: USides::all(12.0),
                    background_color: Color::srgb(0.12, 0.14, 0.18),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.78, 0.84, 0.94, 0.24),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                    ..default()
                },
                ULayout {
                    flex_direction: UFlexDirection::Column,
                    gap: 10.0 + (index % 3) as f32 * 2.0,
                    ..default()
                },
            ))
            .id();

        app.world_mut().spawn((
            ChildOf(panel),
            BenchLabelIndex(index * 3),
            UTextLabel {
                text: format!("Panel {}", index),
                font: font.clone(),
                font_size: 18.0,
                ..default()
            },
        ));

        for button_index in 0..3usize {
            let button = app
                .world_mut()
                .spawn((
                    ChildOf(panel),
                    UButton::secondary(),
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(34.0),
                        ..default()
                    },
                ))
                .id();

            app.world_mut().spawn((
                ChildOf(button),
                BenchLabelIndex(index * 3 + button_index + 1),
                UTextLabel {
                    text: format!("Action {}-{}", index, button_index),
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
            ));
        }
    }

    RuntimeScenario {
        app,
        before_update: mutate_widget_panels,
        after_update: no_op_after_update,
    }
}

fn build_world3d_panels_scenario() -> RuntimeScenario {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        TextPlugin,
    ));
    app.add_plugins(UnivisLayoutPlugin);
    app.add_plugins(UnivisButtonPlugin);
    app.add_systems(
        Update,
        (
            measure_text_label_layout,
            sync_text_label_intrinsic_size,
            fit_node_to_text_size,
        )
            .chain(),
    );

    let font = insert_benchmark_font(&mut app);
    let camera = spawn_perspective_camera(&mut app);

    for index in 0..48usize {
        let root = app
            .world_mut()
            .spawn((
                BenchRootIndex(index),
                URootUi {
                    camera: UiCameraRef::Entity(camera),
                    meters_per_unit: 0.0015 + (index % 3) as f32 * 0.00025,
                    ..URootUi::world_3d(Vec2::new(420.0, 240.0))
                },
                Transform::from_xyz(
                    (index % 8) as f32 * 0.32 - 1.12,
                    (index / 8) as f32 * 0.24 - 0.72,
                    (index % 6) as f32 * 0.03,
                ),
            ))
            .id();

        let panel = app
            .world_mut()
            .spawn((
                ChildOf(root),
                BenchPanelIndex(index),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    padding: USides::all(12.0),
                    background_color: Color::srgba(0.10, 0.12, 0.16, 0.78),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.92, 0.96, 1.0, 0.28),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                    ..default()
                },
                ULayout {
                    flex_direction: UFlexDirection::Column,
                    gap: 10.0,
                    ..default()
                },
            ))
            .id();

        app.world_mut().spawn((
            ChildOf(panel),
            BenchLabelIndex(index * 2),
            UTextLabel {
                text: format!("World panel {}", index),
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
        ));

        let button = app
            .world_mut()
            .spawn((
                ChildOf(panel),
                UButton::primary(),
                UNode {
                    width: UVal::Px(160.0),
                    height: UVal::Px(36.0),
                    ..default()
                },
            ))
            .id();

        app.world_mut().spawn((
            ChildOf(button),
            BenchLabelIndex(index * 2 + 1),
            UTextLabel {
                text: format!("Commit {}", index),
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
        ));
    }

    RuntimeScenario {
        app,
        before_update: mutate_world3d_panels,
        after_update: no_op_after_update,
    }
}

fn build_roots_10k_nodes_1m_scenario() -> RuntimeScenario {
    assert_eq!(
        STRESS_ROOT_COUNT * STRESS_NODES_PER_ROOT,
        STRESS_NODE_COUNT,
        "stress node count should divide evenly across roots"
    );
    assert_eq!(
        STRESS_PANELS_PER_ROOT + STRESS_LEAVES_PER_ROOT,
        STRESS_NODES_PER_ROOT,
        "stress per-root layout should account for every node"
    );

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UnivisLayoutPlugin);

    let camera_entity = spawn_orthographic_camera(&mut app, 1200.0);

    for root_index in 0..STRESS_ROOT_COUNT {
        let root = app
            .world_mut()
            .spawn((
                BenchRootIndex(root_index),
                URootUi {
                    camera: UiCameraRef::Entity(camera_entity),
                    ..URootUi::screen()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        let panel = app
            .world_mut()
            .spawn((
                ChildOf(root),
                BenchPanelIndex(root_index),
                UNode {
                    width: UVal::Px(340.0 + (root_index % 3) as f32 * 8.0),
                    height: UVal::Px(212.0 + (root_index % 4) as f32 * 6.0),
                    padding: USides::all(6.0),
                    margin: USides::all(2.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Grid,
                    grid_columns: 9,
                    gap: 4.0 + (root_index % 2) as f32,
                    container_ext: ULayoutContainerExt {
                        grid: ULayoutGridContainer {
                            template_columns: vec![UTrackSize::Fr(1.0); 9],
                            auto_rows: UTrackSize::Auto,
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        for leaf_index in 0..STRESS_LEAVES_PER_ROOT {
            app.world_mut().spawn((
                ChildOf(panel),
                UNode {
                    width: if leaf_index % 5 == 0 {
                        UVal::Percent(1.0)
                    } else {
                        UVal::Px(18.0 + (leaf_index % 4) as f32 * 4.0)
                    },
                    height: UVal::Px(12.0 + (leaf_index % 3) as f32 * 3.0),
                    margin: USides::all(1.0),
                    ..default()
                },
            ));
        }
    }

    RuntimeScenario {
        app,
        before_update: mutate_roots_10k_nodes_1m,
        after_update: no_op_after_update,
    }
}

fn mutate_root_capsules(app: &mut App, iteration: usize) {
    let phase = (iteration % 24) as f32 * 0.01;
    let mut query = app.world_mut().query::<(&BenchRootIndex, &mut Transform)>();
    for (index, mut transform) in query.iter_mut(app.world_mut()) {
        if index.0 % 8 < 2 {
            transform.translation.z = (index.0 % 9) as f32 * 0.02 + phase;
        }
    }
}

fn mutate_single_root_local_change(app: &mut App, iteration: usize) {
    let target_index = iteration % 96;
    let widen = (iteration / 96).is_multiple_of(2);
    let expected_width = if widen {
        124.0 + (target_index % 3) as f32 * 12.0
    } else {
        scoped_leaf_base_width_px(target_index)
    };
    let entity = app
        .world()
        .resource::<BenchScopedLeafScenario>()
        .target_leaves[target_index];
    let mut entity_mut = app.world_mut().entity_mut(entity);
    let mut node = entity_mut
        .get_mut::<UNode>()
        .expect("scoped leaf should keep its UNode");
    node.width = UVal::Px(expected_width);
    let mut scenario = app.world_mut().resource_mut::<BenchScopedLeafScenario>();
    scenario.last_target_index = target_index;
    scenario.expected_target_width = expected_width;
}

fn mutate_text_labels(app: &mut App, iteration: usize) {
    let mut query = app
        .world_mut()
        .query::<(&BenchLabelIndex, &mut UTextLabel, &mut UNode)>();
    for (index, mut label, mut node) in query.iter_mut(app.world_mut()) {
        if index.0 % 7 == iteration % 7 {
            label.text = benchmark_text_variant(index.0, iteration).to_string();
            if index.0 % 5 == 0 {
                node.width = if iteration.is_multiple_of(2) {
                    UVal::Auto
                } else {
                    UVal::Px(190.0 + (index.0 % 4) as f32 * 20.0)
                };
            }
        }
    }
}

fn mutate_render_only_change(app: &mut App, iteration: usize) {
    let target_index = iteration % 512;
    let entity = app.world().resource::<BenchRenderOnlyScenario>().nodes[target_index];
    let mut entity_mut = app.world_mut().entity_mut(entity);
    let mut node = entity_mut
        .get_mut::<UNode>()
        .expect("render-only node should keep its UNode");
    node.background_color = render_only_color(target_index, iteration + 1);
    app.world_mut()
        .resource_mut::<BenchRenderOnlyScenario>()
        .last_target_index = target_index;
}

fn mutate_pointer_location(app: &mut App, iteration: usize) {
    let step = iteration % 18;
    let position = Vec2::new(760.0 + step as f32 * 3.0, 420.0 + (step % 6) as f32 * 4.0);

    let mut query = app
        .world_mut()
        .query_filtered::<&mut PointerLocation, With<BenchPointerMarker>>();
    for mut pointer in query.iter_mut(app.world_mut()) {
        *pointer = PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: VIEWPORT_SIZE.x,
                height: VIEWPORT_SIZE.y,
            },
            position,
        });
    }
}

fn mutate_widget_panels(app: &mut App, iteration: usize) {
    let mut panels = app.world_mut().query::<(&BenchPanelIndex, &mut ULayout)>();
    for (index, mut layout) in panels.iter_mut(app.world_mut()) {
        if index.0 % 6 == iteration % 6 {
            layout.gap = 8.0 + ((iteration + index.0) % 5) as f32 * 2.0;
        }
    }

    let mut labels = app
        .world_mut()
        .query::<(&BenchLabelIndex, &mut UTextLabel)>();
    for (index, mut label) in labels.iter_mut(app.world_mut()) {
        if index.0 % 9 == iteration % 9 {
            label.text = format!("Panel metric {}", (iteration + index.0) % 128);
        }
    }
}

fn mutate_world3d_panels(app: &mut App, iteration: usize) {
    let phase = (iteration % 20) as f32 * 0.015;

    let mut roots = app.world_mut().query::<(&BenchRootIndex, &mut Transform)>();
    for (index, mut transform) in roots.iter_mut(app.world_mut()) {
        if index.0 % 5 == iteration % 5 {
            transform.translation.z = (index.0 % 6) as f32 * 0.03 + phase;
        }
    }

    let mut labels = app
        .world_mut()
        .query::<(&BenchLabelIndex, &mut UTextLabel)>();
    for (index, mut label) in labels.iter_mut(app.world_mut()) {
        if index.0 % 6 == iteration % 6 {
            label.text = format!("World {}", (iteration + index.0) % 96);
        }
    }
}

fn mutate_roots_10k_nodes_1m(app: &mut App, iteration: usize) {
    let phase = (iteration % 32) as f32 * 0.0005;

    let mut roots = app.world_mut().query::<(&BenchRootIndex, &mut Transform)>();
    for (index, mut transform) in roots.iter_mut(app.world_mut()) {
        if index.0 % 128 == iteration % 128 {
            transform.translation.z = phase + (index.0 % 16) as f32 * 0.00025;
        }
    }

    let mut panels = app.world_mut().query::<(&BenchPanelIndex, &mut ULayout)>();
    for (index, mut layout) in panels.iter_mut(app.world_mut()) {
        if index.0 % 128 == iteration % 128 {
            layout.gap = 3.0 + ((iteration + index.0) % 4) as f32;
        }
    }
}

fn no_op_before_update(_app: &mut App, _iteration: usize) {}

fn no_op_after_update(_app: &mut App) {}

fn assert_idle_after_update(app: &mut App) {
    assert_ui_settled(app);
    let settlement_runtime = app.world().resource::<UiSettlementRuntimeState>();

    if let Some(idle) = app.world().get_resource::<BenchIdleScenario>() {
        assert_eq!(
            settlement_runtime.current_generation(),
            idle.settled_generation,
            "idle scenario should not start a new generation"
        );
    }

    if let Some(render_only) = app.world().get_resource::<BenchRenderOnlyScenario>() {
        let index = render_only.last_target_index;
        let entity = render_only.nodes[index];
        let baseline = render_only.baseline_sizes[index];
        let computed = app
            .world()
            .entity(entity)
            .get::<ComputedSize>()
            .copied()
            .expect("render-only node should keep its computed size");
        assert!(
            approx_eq(computed.width, baseline.width)
                && approx_eq(computed.height, baseline.height),
            "render-only mutation should keep geometry stable, baseline=({}, {}), actual=({}, {})",
            baseline.width,
            baseline.height,
            computed.width,
            computed.height
        );
    }
}

fn assert_localized_solve_after_update(app: &mut App) {
    assert_ui_settled(app);
    let scenario = app.world().resource::<BenchScopedLeafScenario>();
    let target_index = scenario.last_target_index;
    let target_entity = scenario.target_leaves[target_index];
    let control_entity = scenario.control_leaves[target_index];
    let target_size = app
        .world()
        .entity(target_entity)
        .get::<ComputedSize>()
        .copied()
        .expect("target leaf should keep its computed size");
    let control_size = app
        .world()
        .entity(control_entity)
        .get::<ComputedSize>()
        .copied()
        .expect("control leaf should keep its computed size");
    let expected_control_width = 56.0 + (target_index % 4) as f32 * 8.0;

    assert!(
        approx_eq(target_size.width, scenario.expected_target_width),
        "single-root local change should update the targeted leaf width, expected={}, actual={}",
        scenario.expected_target_width,
        target_size.width
    );
    assert!(
        approx_eq(control_size.width, expected_control_width),
        "single-root local change should leave sibling geometry unchanged, expected={}, actual={}",
        expected_control_width,
        control_size.width
    );
}

fn assert_ui_settled(app: &mut App) {
    let settlement_runtime = app.world().resource::<UiSettlementRuntimeState>();
    assert!(
        settlement_runtime.is_settled(),
        "ui pipeline should be settled after benchmark frame, runtime_state={settlement_runtime:?}"
    );
}

fn settle_runtime_scenario(app: &mut App) {
    for _ in 0..8 {
        app.update();
        if app
            .world()
            .resource::<UiSettlementRuntimeState>()
            .is_settled()
        {
            return;
        }
    }

    panic!(
        "benchmark scenario failed to settle before measurement, runtime_state={:?}",
        app.world().resource::<UiSettlementRuntimeState>()
    );
}

fn scoped_leaf_base_width(index: usize) -> UVal {
    UVal::Px(scoped_leaf_base_width_px(index))
}

fn scoped_leaf_base_width_px(index: usize) -> f32 {
    80.0 + (index % 3) as f32 * 8.0
}

fn render_only_color(index: usize, iteration: usize) -> Color {
    let phase = ((index + iteration) % 11) as f32;
    Color::srgb(
        0.12 + phase * 0.018,
        0.18 + phase * 0.012,
        0.24 + phase * 0.010,
    )
}

fn approx_eq(left: f32, right: f32) -> bool {
    (left - right).abs() <= 0.01
}

fn drain_pointer_hits(app: &mut App) {
    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    for _ in hits.drain() {}
}

fn insert_benchmark_font(app: &mut App) -> Handle<Font> {
    let font = Font::try_from_bytes(TEXT_FONT_BYTES.to_vec()).expect("benchmark font should load");
    app.world_mut().resource_mut::<Assets<Font>>().add(font)
}

fn benchmark_text_variant(index: usize, iteration: usize) -> &'static str {
    match (index + iteration) % 4 {
        0 => "Realtime panels keep layout stable while text wraps across compact surfaces.",
        1 => "Arabic and Latin labels should stay measurable without clipping regressions.",
        2 => "Autosize cards adapt to changing labels, counters, and operator notes.",
        _ => "Cache churn matters when dashboards rewrite short status lines every frame.",
    }
}

fn spawn_orthographic_camera(app: &mut App, z: f32) -> Entity {
    let projection = OrthographicProjection {
        area: Rect::new(
            -SCREEN_CANVAS.x * 0.5,
            -SCREEN_CANVAS.y * 0.5,
            SCREEN_CANVAS.x * 0.5,
            SCREEN_CANVAS.y * 0.5,
        ),
        ..OrthographicProjection::default_2d()
    };
    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: VIEWPORT_SIZE,
            scale_factor: 1.0,
        }),
        clip_from_view: projection.get_clip_from_view(),
        ..default()
    };

    app.world_mut()
        .spawn((
            camera,
            Projection::Orthographic(projection),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, z)),
        ))
        .id()
}

fn spawn_perspective_camera(app: &mut App) -> Entity {
    let projection = PerspectiveProjection {
        fov: core::f32::consts::FRAC_PI_3,
        aspect_ratio: SCREEN_CANVAS.x / SCREEN_CANVAS.y,
        near: 0.1,
        ..default()
    };
    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: VIEWPORT_SIZE,
            scale_factor: 1.0,
        }),
        clip_from_view: projection.get_clip_from_view(),
        ..default()
    };

    app.world_mut()
        .spawn((
            camera,
            Projection::Perspective(projection),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 5.0)),
        ))
        .id()
}

fn spawn_cached_camera(app: &mut App, perspective: bool, transform: GlobalTransform) -> Entity {
    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: VIEWPORT_SIZE,
            scale_factor: 1.0,
        }),
        clip_from_view: if perspective {
            PerspectiveProjection {
                fov: core::f32::consts::FRAC_PI_2,
                aspect_ratio: SCREEN_CANVAS.x / SCREEN_CANVAS.y,
                near: 0.1,
                ..default()
            }
            .get_clip_from_view()
        } else {
            OrthographicProjection {
                area: Rect::new(
                    -SCREEN_CANVAS.x * 0.5,
                    -SCREEN_CANVAS.y * 0.5,
                    SCREEN_CANVAS.x * 0.5,
                    SCREEN_CANVAS.y * 0.5,
                ),
                ..OrthographicProjection::default_2d()
            }
            .get_clip_from_view()
        },
        ..default()
    };

    app.world_mut().spawn((camera, transform)).id()
}

fn sample_resolved_root(
    root_entity: Entity,
    space: UiSpace,
    camera_entity: Entity,
) -> (ResolvedRootUi, ResolvedRootStack) {
    let root = ResolvedRootUi {
        root_entity,
        space,
        canvas: match space {
            UiSpace::Screen => UiCanvasSize::Viewport,
            UiSpace::World2d | UiSpace::World3d => UiCanvasSize::Fixed(Vec2::new(800.0, 600.0)),
        },
        canvas_size: Vec2::new(800.0, 600.0),
        camera_entity: Some(camera_entity),
        meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
        resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
    };

    let band_width = if matches!(space, UiSpace::Screen) {
        0.004
    } else {
        root.ui_units_to_world_scale() * 0.04
    };

    (root, ResolvedRootStack::with_capsule(0.0, band_width))
}
