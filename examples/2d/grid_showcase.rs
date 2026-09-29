//! # Comprehensive Native CSS Grid Engine Showcase
//!
//! Demonstrates the native 2D CSS Grid solver in `univis_ui_engine`:
//! 1. **RPG Equipment & Inventory Grid**: Multi-cell spanning (1x2 and 2x2) and explicit slots.
//! 2. **Operations & Analytics Dashboard**: Responsive `Fr` tracks, multi-column and multi-row spans, real-time live telemetry.
//! 3. **Auto-Placement & Dynamic Flow**: Dynamic item insertion, wrapping, toggling Row and Column auto-flow.
//! 4. **Live Engine Telemetry HUD**: Settlement loop generations, iterations per frame, convergence status.

use bevy::prelude::*;
use univis_ui::engine::schedule::UiWorkState;
use univis_ui::prelude::*;

#[path = "grid_showcase/views.rs"]
mod views;

use views::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ShowcaseMode {
    #[default]
    Inventory,
    Dashboard,
    AutoFlow,
}

#[derive(Resource)]
pub struct ShowcaseState {
    mode: ShowcaseMode,
    auto_flow: UGridAutoFlow,
    dynamic_item_count: usize,
    selected_item: String,
    live_metric_timer: f32,
    simulated_kops: f32,
    simulated_latency: f32,
}

impl Default for ShowcaseState {
    fn default() -> Self {
        Self {
            mode: ShowcaseMode::Inventory,
            auto_flow: UGridAutoFlow::Row,
            dynamic_item_count: 8,
            selected_item: "Hover over an item to inspect".into(),
            live_metric_timer: 0.0,
            simulated_kops: 142.5,
            simulated_latency: 0.82,
        }
    }
}

#[derive(Component)]
struct ContentRoot;

#[derive(Component)]
struct ModeButton(ShowcaseMode);

#[derive(Component)]
struct TelemetryHud;

#[derive(Component)]
struct ItemInspectorText;

fn btn_style(is_active: bool) -> (UNode, UBorder) {
    let (bg, border_col) = if is_active {
        (Color::srgb(0.18, 0.38, 0.65), Color::srgb(0.35, 0.65, 0.95))
    } else {
        (
            Color::srgba(0.12, 0.16, 0.24, 0.7),
            Color::srgba(0.22, 0.3, 0.44, 0.5),
        )
    };
    (
        UNode {
            padding: USides::axes(8.0, 14.0),
            background_color: bg,
            border_radius: UCornerRadius::all(6.0),
            ..default()
        },
        border(border_col, 6.0, 1.0),
    )
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Native CSS Grid Engine Showcase".into(),
                    resolution: (1280, 840).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<ShowcaseState>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                handle_input_shortcuts,
                update_hud_telemetry,
                update_live_metrics,
                rebuild_grid_content_on_change,
            ),
        )
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    let screen_root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::axes(20.0, 24.0),
                background_color: Color::srgb(0.05, 0.06, 0.09),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Stretch,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    spawn_header(&mut commands, screen_root);

    let (c_node, c_border) = panel(
        Color::srgba(0.08, 0.10, 0.15, 0.7),
        12.0,
        16.0,
        Color::srgba(0.18, 0.24, 0.35, 0.6),
    );
    let content_container = commands
        .spawn((
            ChildOf(screen_root),
            ContentRoot,
            c_node,
            c_border,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_inventory_view(&mut commands, content_container);
    spawn_footer_hud(&mut commands, screen_root);
}

fn spawn_header(commands: &mut Commands, parent: Entity) {
    let (h_node, h_border) = panel(
        Color::srgba(0.09, 0.12, 0.18, 0.9),
        10.0,
        12.0,
        Color::srgba(0.2, 0.28, 0.42, 0.6),
    );
    let header = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(68.0),
                ..h_node
            },
            h_border,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let title_group = commands
        .spawn((
            ChildOf(header),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        commands,
        title_group,
        "UNIVIS CSS GRID ENGINE",
        Color::srgb(0.95, 0.97, 1.0),
        18.0,
    );
    spawn_label(
        commands,
        title_group,
        "Native Rust 2D/3D Multi-Space Grid Solver & Settlement Parity",
        Color::srgb(0.45, 0.6, 0.75),
        11.0,
    );

    let tabs_group = commands
        .spawn((
            ChildOf(header),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    for (label, mode, active) in [
        ("[1] RPG Inventory", ShowcaseMode::Inventory, true),
        ("[2] Operations Dashboard", ShowcaseMode::Dashboard, false),
        ("[3] Auto-Placement Flow", ShowcaseMode::AutoFlow, false),
    ] {
        spawn_mode_button(commands, tabs_group, label, mode, active);
    }
}

fn spawn_mode_button(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    mode: ShowcaseMode,
    is_active: bool,
) {
    let (node, border) = btn_style(is_active);
    commands
        .spawn((
            ChildOf(parent),
            ModeButton(mode),
            node,
            border,
            ULayout::default(),
        ))
        .observe(
            move |_click: On<Pointer<Click>>, mut state: ResMut<ShowcaseState>| {
                state.mode = mode;
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: label.into(),
                    color: Color::WHITE,
                    font_size: 12.0,
                    ..default()
                },
            ));
        });
}

fn spawn_footer_hud(commands: &mut Commands, parent: Entity) {
    let footer = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(48.0),
                padding: USides::axes(8.0, 16.0),
                background_color: Color::srgba(0.07, 0.09, 0.13, 0.9),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(Color::srgba(0.18, 0.24, 0.35, 0.6), 8.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(footer),
        TelemetryHud,
        UNode::default(),
        UTextLabel {
            text: "Initializing settlement telemetry...".into(),
            color: Color::srgb(0.4, 0.85, 0.5),
            font_size: 12.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(footer),
        ItemInspectorText,
        UNode::default(),
        UTextLabel {
            text: "Hover over slots to inspect".into(),
            color: Color::srgb(0.7, 0.8, 0.95),
            font_size: 12.0,
            ..default()
        },
    ));
}

fn handle_input_shortcuts(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<ShowcaseState>) {
    if keys.just_pressed(KeyCode::Digit1) {
        state.mode = ShowcaseMode::Inventory;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        state.mode = ShowcaseMode::Dashboard;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        state.mode = ShowcaseMode::AutoFlow;
    }
    if keys.just_pressed(KeyCode::KeyF) {
        state.auto_flow = match state.auto_flow {
            UGridAutoFlow::Row => UGridAutoFlow::Column,
            UGridAutoFlow::Column => UGridAutoFlow::Row,
        };
    }
    if keys.just_pressed(KeyCode::Space) {
        if state.dynamic_item_count < 16 {
            state.dynamic_item_count += 1;
        } else {
            state.dynamic_item_count = 4;
        }
    }
}

fn rebuild_grid_content_on_change(
    mut commands: Commands,
    state: Res<ShowcaseState>,
    content_query: Query<Entity, With<ContentRoot>>,
    buttons_query: Query<(Entity, &ModeButton)>,
    children_query: Query<&Children>,
) {
    if !state.is_changed() {
        return;
    }

    let Ok(content_entity) = content_query.single() else {
        return;
    };

    if let Ok(children) = children_query.get(content_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    match state.mode {
        ShowcaseMode::Inventory => spawn_inventory_view(&mut commands, content_entity),
        ShowcaseMode::Dashboard => spawn_dashboard_view(&mut commands, content_entity),
        ShowcaseMode::AutoFlow => spawn_autoflow_view(
            &mut commands,
            content_entity,
            state.auto_flow,
            state.dynamic_item_count,
        ),
    }

    for (btn_entity, mode_btn) in buttons_query.iter() {
        let (node, border) = btn_style(mode_btn.0 == state.mode);
        commands.entity(btn_entity).insert((node, border));
    }
}

fn update_live_metrics(
    time: Res<Time>,
    mut state: ResMut<ShowcaseState>,
    mut query: Query<&mut UTextLabel, With<LiveMetricText>>,
) {
    state.live_metric_timer += time.delta_secs();
    if state.live_metric_timer > 0.15 {
        state.live_metric_timer = 0.0;
        let noise = (time.elapsed_secs() * 3.0).sin() * 4.2;
        state.simulated_kops = 142.0 + noise;
        state.simulated_latency = 0.82 + (noise * 0.02);

        for mut label in query.iter_mut() {
            label.text = format!(
                "Throughput: {:.1} kops/s • P99: {:.2} ms • Convergence Passes: 1 (Clean)",
                state.simulated_kops, state.simulated_latency
            );
        }
    }
}

fn update_hud_telemetry(
    work_state: Option<Res<UiWorkState>>,
    state: Res<ShowcaseState>,
    mut hud_query: Query<&mut UTextLabel, With<TelemetryHud>>,
    mut inspect_query: Query<&mut UTextLabel, (With<ItemInspectorText>, Without<TelemetryHud>)>,
) {
    if let Some(mut label) = hud_query.iter_mut().next() {
        let (gen_val, iters, settled, exhausted) = work_state
            .as_ref()
            .map(|w| {
                (
                    w.current_generation(),
                    w.last_frame_iterations(),
                    w.is_settled(),
                    w.budget_exhausted(),
                )
            })
            .unwrap_or((0, 0, true, false));

        let status_color = if exhausted { "EXHAUSTED" } else { "SETTLED" };
        label.text = format!(
            "Engine: Gen {gen_val} | Passes: {iters} | Status: {status_color} (settled={settled})",
        );
        label.color = if exhausted {
            Color::srgb(0.9, 0.3, 0.3)
        } else {
            Color::srgb(0.3, 0.85, 0.5)
        };
    }

    if let Some(mut label) = inspect_query.iter_mut().next() {
        label.text = format!("Selected: {}", state.selected_item);
    }
}
