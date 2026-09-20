//! Responsive Tactical HUD Subsystem Monitor using `UTrackSize::repeat_fit` and `UTrackRepeat::minmax`.
//!
//! Demonstrates how `repeat(auto-fit, minmax(200px, 1fr))` dynamically adapts the number of
//! HUD subsystem slots based on viewport / cockpit mode width without media queries.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Tactical Sci-Fi HUD (CSS Grid auto-fit)".into(),
                resolution: (1200, 780).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<GridDemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_preset_clicks)
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
    fn width(self) -> f32 {
        match self {
            Self::CockpitFull => 960.0,
            Self::CombatFocus => 700.0,
            Self::AuxDisplay => 460.0,
            Self::HelmetMini => 300.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::CockpitFull => "Full Cockpit (960px)",
            Self::CombatFocus => "Combat Focus (700px)",
            Self::AuxDisplay => "Aux Display (460px)",
            Self::HelmetMini => "Helmet Mini (300px)",
        }
    }

    fn expected_cols(self) -> &'static str {
        match self {
            Self::CockpitFull => "4 subsystem slots",
            Self::CombatFocus => "3 subsystem slots",
            Self::AuxDisplay => "2 subsystem slots",
            Self::HelmetMini => "1 subsystem slot",
        }
    }
}

#[derive(Resource)]
struct GridDemoState {
    preset: ContainerPreset,
}

impl Default for GridDemoState {
    fn default() -> Self {
        Self {
            preset: ContainerPreset::CockpitFull,
        }
    }
}

#[derive(Component)]
struct ResizableGridContainer;

#[derive(Component)]
struct PresetButton(ContainerPreset);

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
                width: UVal::Px(1120.0),
                height: UVal::Px(720.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.95),
                border_radius: UCornerRadius::all(20.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
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
            "Adaptive CSS Grid: `repeat(auto-fit, minmax(200px, 1fr))` across tactical display modes.",
            13.0,
            Color::srgb(0.65, 0.78, 0.9),
        ),
    ));

    // Preset buttons bar
    let btn_bar = commands
        .spawn((
            ChildOf(shell),
            UNode {
                height: UVal::Px(42.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    for preset in [
        ContainerPreset::CockpitFull,
        ContainerPreset::CombatFocus,
        ContainerPreset::AuxDisplay,
        ContainerPreset::HelmetMini,
    ] {
        let is_active = preset == state.preset;
        let bg_color = if is_active {
            Color::srgba(0.0, 0.45, 0.7, 0.9)
        } else {
            Color::srgba(0.05, 0.09, 0.16, 0.9)
        };

        let btn = commands
            .spawn((
                ChildOf(btn_bar),
                PresetButton(preset),
                UNode {
                    padding: USides::axes(14.0, 8.0),
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(8.0),
                    shape_mode: UShapeMode::Cut,
                    ..default()
                },
                UBorder {
                    color: if is_active {
                        Color::srgb(0.0, 0.9, 1.0)
                    } else {
                        Color::srgba(0.0, 0.8, 1.0, 0.2)
                    },
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
            .id();

        commands.spawn((
            ChildOf(btn),
            label_node(),
            text(preset.label(), 12.0, Color::WHITE),
        ));
    }

    // Status pill
    let status_pill = commands
        .spawn((
            ChildOf(shell),
            UNode {
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
                "COCKPIT WIDTH: {:.0}px | {} | TRACK: minmax(200px, 1fr) @ 14px GAP",
                state.preset.width(),
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
            UNode {
                height: UVal::Px(460.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.02, 0.035, 0.06, 0.8),
                border_radius: UCornerRadius::all(16.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(16.0),
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
                width: UVal::Px(state.preset.width()),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.04, 0.07, 0.12, 0.95),
                border_radius: UCornerRadius::all(14.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.85, 1.0, 0.35),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
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
                height: UVal::Px(112.0),
                padding: USides::all(12.0),
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
        text("TELEMETRY: SYNCED", 9.0, Color::srgba(0.5, 0.6, 0.75, 0.6)),
    ));
}

fn handle_preset_clicks(
    mut state: ResMut<GridDemoState>,
    mut container_q: Query<&mut UNode, With<ResizableGridContainer>>,
    mut status_q: Query<&mut UTextLabel, With<StatusText>>,
    mut buttons_q: Query<
        (&PresetButton, &mut UNode, &mut UBorder),
        Without<ResizableGridContainer>,
    >,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    // Cycle through presets on click anywhere for easy demonstration
    let next_preset = match state.preset {
        ContainerPreset::CockpitFull => ContainerPreset::CombatFocus,
        ContainerPreset::CombatFocus => ContainerPreset::AuxDisplay,
        ContainerPreset::AuxDisplay => ContainerPreset::HelmetMini,
        ContainerPreset::HelmetMini => ContainerPreset::CockpitFull,
    };

    // Only switch if clicked near top bar area or simple click cycle
    if cursor.y < 250.0 {
        state.preset = next_preset;

        if let Ok(mut container_node) = container_q.single_mut() {
            container_node.width = UVal::Px(state.preset.width());
        }

        if let Ok(mut txt) = status_q.single_mut() {
            txt.text = format!(
                "COCKPIT WIDTH: {:.0}px | {} | TRACK: minmax(200px, 1fr) @ 14px GAP",
                state.preset.width(),
                state.preset.expected_cols()
            );
        }

        for (btn, mut node, mut border) in &mut buttons_q {
            let is_active = btn.0 == state.preset;
            node.background_color = if is_active {
                Color::srgba(0.0, 0.45, 0.7, 0.9)
            } else {
                Color::srgba(0.05, 0.09, 0.16, 0.9)
            };
            border.color = if is_active {
                Color::srgb(0.0, 0.9, 1.0)
            } else {
                Color::srgba(0.0, 0.8, 1.0, 0.2)
            };
        }
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
