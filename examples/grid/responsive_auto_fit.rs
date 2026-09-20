//! Responsive Tactical HUD Subsystem Monitor using `UTrackSize::repeat_fit` and `UTrackRepeat::minmax`.
//!
//! Demonstrates how `repeat(auto-fit, minmax(200px, 1fr))` dynamically adapts the number of
//! HUD subsystem slots in real time based on viewport width (via live mouse drag or preset modes).

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Tactical Sci-Fi HUD (CSS Grid auto-fit)".into(),
                resolution: (1200, 780).into(),
                resizable: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 380.0,
                    min_height: 520.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<GridDemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, sync_window_and_layout))
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ContainerPreset {
    CockpitFull,
    CombatFocus,
    AuxDisplay,
    HelmetMini,
}

impl ContainerPreset {
    fn window_width(self) -> f32 {
        match self {
            Self::CockpitFull => 1200.0,
            Self::CombatFocus => 920.0,
            Self::AuxDisplay => 640.0,
            Self::HelmetMini => 420.0,
        }
    }

    fn container_width(self) -> f32 {
        match self {
            Self::CockpitFull => 980.0,
            Self::CombatFocus => 720.0,
            Self::AuxDisplay => 480.0,
            Self::HelmetMini => 320.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::CockpitFull => "Full Cockpit (1200px)",
            Self::CombatFocus => "Combat Focus (920px)",
            Self::AuxDisplay => "Aux Display (640px)",
            Self::HelmetMini => "Helmet Mini (420px)",
        }
    }

    fn expected_cols(self) -> &'static str {
        match self {
            Self::CockpitFull => "4-5 slots",
            Self::CombatFocus => "3 slots",
            Self::AuxDisplay => "2 slots",
            Self::HelmetMini => "1 slot",
        }
    }

    fn from_window_width(w: f32) -> Self {
        if w >= 1060.0 {
            Self::CockpitFull
        } else if w >= 800.0 {
            Self::CombatFocus
        } else if w >= 540.0 {
            Self::AuxDisplay
        } else {
            Self::HelmetMini
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PresetSelection {
    Preset(ContainerPreset),
    AutoFluid,
}

#[derive(Resource)]
struct GridDemoState {
    preset: ContainerPreset,
    fluid_sync: bool,
    last_window_size: Vec2,
    preset_request: Option<PresetSelection>,
}

impl Default for GridDemoState {
    fn default() -> Self {
        Self {
            preset: ContainerPreset::CockpitFull,
            fluid_sync: true,
            last_window_size: Vec2::new(1200.0, 780.0),
            preset_request: None,
        }
    }
}

#[derive(Component)]
struct ResizableGridContainer;

#[derive(Component)]
struct PresetButton(PresetSelection);

#[derive(Component)]
struct StatusText;

fn setup(mut commands: Commands, state: Res<GridDemoState>) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.015, 0.022, 0.035),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let shell = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(0.96),
                height: UVal::Percent(0.95),
                max_width: 1400.0,
                max_height: 920.0,
                padding: USides::axes(20.0, 16.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.95),
                border_radius: UCornerRadius::all(18.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(18.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    // Header
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "TACTICAL HUD: SUBSYSTEM MONITOR",
            24.0,
            Color::srgb(0.0, 0.9, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Drag/resize window edges with mouse for live auto-fit reflow, or click presets / press [1-5, Space] to resize.",
            13.0,
            Color::srgb(0.65, 0.78, 0.9),
        ),
    ));

    // Preset buttons bar
    let btn_bar = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
                align_items: UAlignItems::Center,
                container_ext: ULayoutContainerExt {
                    flex: ULayoutFlexContainer {
                        wrap: UFlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let buttons = [
        (
            PresetSelection::Preset(ContainerPreset::CockpitFull),
            "[1] Full Cockpit (1200px)",
        ),
        (
            PresetSelection::Preset(ContainerPreset::CombatFocus),
            "[2] Combat Focus (920px)",
        ),
        (
            PresetSelection::Preset(ContainerPreset::AuxDisplay),
            "[3] Aux Display (640px)",
        ),
        (
            PresetSelection::Preset(ContainerPreset::HelmetMini),
            "[4] Helmet Mini (420px)",
        ),
        (PresetSelection::AutoFluid, "[5] Auto Fluid (Drag Window)"),
    ];

    for (selection, label) in buttons {
        let is_active = match selection {
            PresetSelection::Preset(p) => p == state.preset,
            PresetSelection::AutoFluid => state.fluid_sync,
        };

        let bg_color = if is_active {
            if selection == PresetSelection::AutoFluid {
                Color::srgba(0.0, 0.45, 0.3, 0.95)
            } else {
                Color::srgba(0.0, 0.45, 0.7, 0.9)
            }
        } else {
            Color::srgba(0.05, 0.09, 0.16, 0.9)
        };

        let border_color = if is_active {
            if selection == PresetSelection::AutoFluid {
                Color::srgb(0.1, 0.9, 0.5)
            } else {
                Color::srgb(0.0, 0.9, 1.0)
            }
        } else {
            Color::srgba(0.0, 0.8, 1.0, 0.2)
        };

        let btn = commands
            .spawn((
                ChildOf(btn_bar),
                PresetButton(selection),
                UNode {
                    padding: USides::axes(12.0, 7.0),
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(8.0),
                    shape_mode: UShapeMode::Cut,
                    ..default()
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: bg_color,
                    hovered: Color::srgba(0.0, 0.6, 0.9, 0.95),
                    pressed: Color::srgba(0.0, 0.35, 0.6, 0.95),
                },
                UBorder {
                    color: border_color,
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                    offset: 0.0,
                },
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
            ))
            .observe(
                move |_click: On<Pointer<Click>>, mut state: ResMut<GridDemoState>| {
                    state.preset_request = Some(selection);
                },
            )
            .id();

        commands.spawn((ChildOf(btn), label_node(), text(label, 12.0, Color::WHITE)));
    }

    // Status pill
    let status_pill = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(14.0, 8.0),
                background_color: Color::srgba(0.0, 0.2, 0.35, 0.5),
                border_radius: UCornerRadius::all(8.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.85, 1.0, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(status_pill),
        StatusText,
        label_node(),
        text(
            format!(
                "VIEWPORT: 1200x780px | AUTO-FLUID SYNC: {} ({}) | TRACK: minmax(200px, 1fr)",
                state.preset.label(),
                state.preset.expected_cols()
            ),
            12.0,
            Color::srgb(0.0, 0.9, 1.0),
        ),
    ));

    // Centered wrapper for resizable container
    let center_wrapper = commands
        .spawn((
            ChildOf(shell),
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(1.0),
                        flex_shrink: Some(1.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgba(0.02, 0.035, 0.06, 0.8),
                border_radius: UCornerRadius::all(14.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Start,
                ..default()
            },
        ))
        .id();

    // The resizable grid container
    let grid_container = commands
        .spawn((
            ChildOf(center_wrapper),
            ResizableGridContainer,
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.04, 0.07, 0.12, 0.95),
                border_radius: UCornerRadius::all(12.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.85, 1.0, 0.35),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 1, // Fallback
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(14.0),
                        column_gap: Some(14.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: vec![UTrackSize::repeat_fit(UTrackRepeat::minmax(
                            UTrackBound::px(200.0),
                            UTrackBound::fr(1.0),
                        ))],
                        template_rows: vec![UTrackSize::Auto],
                        auto_rows: UTrackSize::Auto,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // Spawn 6 tactical subsystem cards with metrics and colors
    let cards = [
        (
            "HULL INTEGRITY",
            "98.4%",
            "NOMINAL",
            Color::srgb(0.0, 0.85, 1.0),
        ),
        (
            "SHIELD MATRIX",
            "2,450 MW",
            "ONLINE",
            Color::srgb(0.1, 0.85, 0.45),
        ),
        (
            "WARP CORE FLUX",
            "1.21 GW",
            "STABLE",
            Color::srgb(1.0, 0.72, 0.1),
        ),
        (
            "WEAPONS ARRAY",
            "READY",
            "ARMED",
            Color::srgb(1.0, 0.25, 0.4),
        ),
        (
            "THRUST VECTOR",
            "84.6 kN",
            "+12.4%",
            Color::srgb(0.7, 0.35, 1.0),
        ),
        (
            "RADAR SCANNER",
            "4 TARGETS",
            "ALERT",
            Color::srgb(0.2, 0.6, 1.0),
        ),
    ];

    for (title, val, badge, accent) in cards {
        spawn_card(&mut commands, grid_container, title, val, badge, accent);
    }
}

fn spawn_card(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    val: &str,
    badge: &str,
    accent: Color,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                height: UVal::Px(104.0),
                padding: USides::all(11.0),
                background_color: Color::srgba(0.05, 0.09, 0.15, 0.95),
                border_radius: UCornerRadius::all(10.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: accent.with_alpha(0.35),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    // Top row: title and badge
    let top_row = commands
        .spawn((
            ChildOf(card),
            UNode {
                height: UVal::Px(22.0),
                ..default()
            },
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
        ChildOf(top_row),
        label_node(),
        text(title, 11.0, Color::srgb(0.65, 0.75, 0.85)),
    ));

    let badge_node = commands
        .spawn((
            ChildOf(top_row),
            UNode {
                padding: USides::axes(7.0, 2.5),
                background_color: accent.with_alpha(0.2),
                border_radius: UCornerRadius::all(4.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(badge_node), label_node(), text(badge, 10.0, accent)));

    // Value
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(val, 22.0, Color::srgb(0.95, 0.98, 1.0)),
    ));

    // Bottom telemetry line
    commands.spawn((
        ChildOf(card),
        label_node(),
        text("TELEMETRY: NOMINAL", 9.0, Color::srgba(0.5, 0.6, 0.75, 0.6)),
    ));
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<GridDemoState>) {
    if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Numpad1) {
        state.preset_request = Some(PresetSelection::Preset(ContainerPreset::CockpitFull));
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Numpad2) {
        state.preset_request = Some(PresetSelection::Preset(ContainerPreset::CombatFocus));
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Numpad3) {
        state.preset_request = Some(PresetSelection::Preset(ContainerPreset::AuxDisplay));
    } else if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Numpad4) {
        state.preset_request = Some(PresetSelection::Preset(ContainerPreset::HelmetMini));
    } else if keyboard.just_pressed(KeyCode::Digit5)
        || keyboard.just_pressed(KeyCode::Numpad5)
        || keyboard.just_pressed(KeyCode::KeyF)
    {
        state.preset_request = Some(PresetSelection::AutoFluid);
    } else if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Tab) {
        let next = match state.preset {
            ContainerPreset::CockpitFull => ContainerPreset::CombatFocus,
            ContainerPreset::CombatFocus => ContainerPreset::AuxDisplay,
            ContainerPreset::AuxDisplay => ContainerPreset::HelmetMini,
            ContainerPreset::HelmetMini => ContainerPreset::CockpitFull,
        };
        state.preset_request = Some(PresetSelection::Preset(next));
    }
}

fn sync_window_and_layout(
    mut state: ResMut<GridDemoState>,
    mut windows: Query<&mut Window>,
    mut container_q: Query<&mut UNode, With<ResizableGridContainer>>,
    mut status_q: Query<&mut UTextLabel, With<StatusText>>,
    mut buttons_q: Query<
        (&PresetButton, &mut UNode, &mut UBorder),
        Without<ResizableGridContainer>,
    >,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };
    let cur_size = Vec2::new(window.width(), window.height());

    // 1. Check if the user manually dragged/resized the OS window with mouse
    let window_resized = (cur_size.x - state.last_window_size.x).abs() > 1.5
        || (cur_size.y - state.last_window_size.y).abs() > 1.5;

    if window_resized && state.preset_request.is_none() {
        state.last_window_size = cur_size;
        state.fluid_sync = true;
        state.preset = ContainerPreset::from_window_width(cur_size.x);
        if let Ok(mut container_node) = container_q.single_mut() {
            container_node.width = UVal::Percent(1.0);
        }
    }

    // 2. Handle requested preset or fluid mode
    if let Some(req) = state.preset_request.take() {
        match req {
            PresetSelection::Preset(preset) => {
                state.preset = preset;
                state.fluid_sync = false;
                window.resolution.set(preset.window_width(), 780.0);
                state.last_window_size = Vec2::new(preset.window_width(), 780.0);
                if let Ok(mut container_node) = container_q.single_mut() {
                    container_node.width = UVal::Px(preset.container_width());
                }
            }
            PresetSelection::AutoFluid => {
                state.fluid_sync = true;
                state.preset = ContainerPreset::from_window_width(cur_size.x);
                state.last_window_size = cur_size;
                if let Ok(mut container_node) = container_q.single_mut() {
                    container_node.width = UVal::Percent(1.0);
                }
            }
        }
    }

    // 3. Update status telemetry text
    if let Ok(mut txt) = status_q.single_mut() {
        let mode_desc = if cur_size.x < 650.0 {
            if state.fluid_sync {
                format!(
                    "{:.0}px | {} ({})",
                    cur_size.x,
                    state.preset.label(),
                    state.preset.expected_cols()
                )
            } else {
                format!(
                    "LOCKED: {} ({})",
                    state.preset.label(),
                    state.preset.expected_cols()
                )
            }
        } else if state.fluid_sync {
            format!(
                "AUTO-FLUID WINDOW SYNC | DETECTED: {} ({})",
                state.preset.label(),
                state.preset.expected_cols()
            )
        } else {
            format!(
                "PRESET LOCKED: {} ({})",
                state.preset.label(),
                state.preset.expected_cols()
            )
        };
        txt.text = if cur_size.x < 650.0 {
            format!(
                "VIEWPORT: {:.0}x{:.0}px | {}",
                cur_size.x, cur_size.y, mode_desc
            )
        } else {
            format!(
                "VIEWPORT: {:.0}x{:.0}px | {} | TRACKS: minmax(200px, 1fr) @ 14px GAP",
                cur_size.x, cur_size.y, mode_desc
            )
        };
    }

    // 4. Update button styles
    for (btn, mut node, mut border) in &mut buttons_q {
        let is_active = match btn.0 {
            PresetSelection::Preset(p) => p == state.preset,
            PresetSelection::AutoFluid => state.fluid_sync,
        };

        let bg_color = if is_active {
            if btn.0 == PresetSelection::AutoFluid {
                Color::srgba(0.0, 0.45, 0.3, 0.95)
            } else {
                Color::srgba(0.0, 0.45, 0.7, 0.9)
            }
        } else {
            Color::srgba(0.05, 0.09, 0.16, 0.9)
        };

        let border_color = if is_active {
            if btn.0 == PresetSelection::AutoFluid {
                Color::srgb(0.1, 0.9, 0.5)
            } else {
                Color::srgb(0.0, 0.9, 1.0)
            }
        } else {
            Color::srgba(0.0, 0.8, 1.0, 0.2)
        };

        node.background_color = bg_color;
        border.color = border_color;
    }
}

fn label_node() -> UNode {
    UNode {
        background_color: Color::NONE,
        ..default()
    }
}

fn text(content: impl Into<String>, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: content.into(),
        font_size,
        color,
        ..default()
    }
}
