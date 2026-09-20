//! Responsive CSS Grid auto-fit demonstration using `UTrackSize::repeat_fit` and `UTrackRepeat::minmax`.
//!
//! Demonstrates how `repeat(auto-fit, minmax(200px, 1fr))` dynamically adapts the number of
//! columns based on container width without media queries, and expands the cards to fill available space.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Responsive Grid auto-fit & minmax()".into(),
                resolution: (1200.0, 780.0).into(),
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
    Desktop,
    Tablet,
    Compact,
    Mobile,
}

impl ContainerPreset {
    fn width(self) -> f32 {
        match self {
            Self::Desktop => 960.0,
            Self::Tablet => 700.0,
            Self::Compact => 460.0,
            Self::Mobile => 300.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop (960px)",
            Self::Tablet => "Tablet (700px)",
            Self::Compact => "Compact (460px)",
            Self::Mobile => "Mobile (300px)",
        }
    }

    fn expected_cols(self) -> &'static str {
        match self {
            Self::Desktop => "4 columns",
            Self::Tablet => "3 columns",
            Self::Compact => "2 columns",
            Self::Mobile => "1 column",
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
            preset: ContainerPreset::Desktop,
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
                background_color: Color::srgb(0.04, 0.06, 0.09),
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
                background_color: Color::srgba(0.07, 0.09, 0.13, 0.98),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.7, 0.8, 0.92, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
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
            "CSS Grid: `repeat(auto-fit, minmax(200px, 1fr))`",
            26.0,
            Color::srgb(0.95, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Click preset buttons to change container width. The layout engine recalculates track count automatically.",
            14.0,
            Color::srgb(0.72, 0.78, 0.86),
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
        ContainerPreset::Desktop,
        ContainerPreset::Tablet,
        ContainerPreset::Compact,
        ContainerPreset::Mobile,
    ] {
        let is_active = preset == state.preset;
        let bg_color = if is_active {
            Color::srgb(0.2, 0.45, 0.9)
        } else {
            Color::srgba(0.14, 0.18, 0.26, 0.9)
        };

        let btn = commands
            .spawn((
                ChildOf(btn_bar),
                PresetButton(preset),
                UNode {
                    padding: USides::axes(14.0, 8.0),
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                UBorder {
                    color: if is_active {
                        Color::srgb(0.4, 0.65, 1.0)
                    } else {
                        Color::srgba(0.5, 0.6, 0.75, 0.2)
                    },
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
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
            text(preset.label(), 13.0, Color::WHITE),
        ));
    }

    // Status pill
    let status_pill = commands
        .spawn((
            ChildOf(shell),
            UNode {
                padding: USides::axes(14.0, 8.0),
                background_color: Color::srgba(0.1, 0.22, 0.35, 0.6),
                border_radius: UCornerRadius::all(8.0),
                ..default()
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
                "Width: {:.0}px | Result: {} | Track: minmax(200px, 1fr) with 14px gap",
                state.preset.width(),
                state.preset.expected_cols()
            ),
            13.0,
            Color::srgb(0.45, 0.8, 1.0),
        ),
    ));

    // Centered wrapper for resizable container
    let center_wrapper = commands
        .spawn((
            ChildOf(shell),
            UNode {
                height: UVal::Px(460.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.04, 0.06, 0.09, 0.7),
                border_radius: UCornerRadius::all(18.0),
                ..default()
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
                background_color: Color::srgba(0.09, 0.12, 0.18, 0.95),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.3, 0.5, 0.8, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(16.0),
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

    // Spawn 6 cards with metrics and colors
    let cards = [
        ("Revenue", "$48,290", "+14.2%", Color::srgb(0.2, 0.45, 0.85)),
        ("Active Users", "12,840", "+8.7%", Color::srgb(0.18, 0.65, 0.48)),
        ("Conversion", "3.42%", "+1.1%", Color::srgb(0.85, 0.55, 0.15)),
        ("Bounce Rate", "24.6%", "-2.4%", Color::srgb(0.75, 0.25, 0.45)),
        ("Avg Duration", "4m 32s", "+18s", Color::srgb(0.55, 0.3, 0.8)),
        ("Server Uptime", "99.98%", "Stable", Color::srgb(0.15, 0.65, 0.75)),
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
                height: UVal::Px(110.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.12, 0.16, 0.23, 0.95),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
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
                height: UVal::Px(24.0),
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
        text(title, 13.0, Color::srgb(0.7, 0.76, 0.85)),
    ));

    let badge_node = commands
        .spawn((
            ChildOf(top_row),
            UNode {
                padding: USides::axes(8.0, 3.0),
                background_color: accent.with_alpha(0.2),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(badge_node),
        label_node(),
        text(badge, 11.0, accent),
    ));

    // Value
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(val, 24.0, Color::WHITE),
    ));
}

fn handle_preset_clicks(
    mut state: ResMut<GridDemoState>,
    mut container_q: Query<&mut UNode, With<ResizableGridContainer>>,
    mut status_q: Query<&mut Text, With<StatusText>>,
    mut buttons_q: Query<(&PresetButton, &mut UNode, &mut UBorder), Without<ResizableGridContainer>>,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let window = match windows.single() {
        w => w,
    };
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = match window.cursor_position() {
        Some(c) => c,
        None => return,
    };

    // Cycle through presets on click anywhere for easy demonstration
    let next_preset = match state.preset {
        ContainerPreset::Desktop => ContainerPreset::Tablet,
        ContainerPreset::Tablet => ContainerPreset::Compact,
        ContainerPreset::Compact => ContainerPreset::Mobile,
        ContainerPreset::Mobile => ContainerPreset::Desktop,
    };

    // Only switch if clicked near top bar area or simple click cycle
    if cursor.y < 250.0 {
        state.preset = next_preset;

        if let Ok(mut container_node) = container_q.get_single_mut() {
            container_node.width = UVal::Px(state.preset.width());
        }

        if let Ok(mut txt) = status_q.get_single_mut() {
            txt.0 = format!(
                "Width: {:.0}px | Result: {} | Track: minmax(200px, 1fr) with 14px gap",
                state.preset.width(),
                state.preset.expected_cols()
            );
        }

        for (btn, mut node, mut border) in &mut buttons_q {
            let is_active = btn.0 == state.preset;
            node.background_color = if is_active {
                Color::srgb(0.2, 0.45, 0.9)
            } else {
                Color::srgba(0.14, 0.18, 0.26, 0.9)
            };
            border.color = if is_active {
                Color::srgb(0.4, 0.65, 1.0)
            } else {
                Color::srgba(0.5, 0.6, 0.75, 0.2)
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

fn text(content: impl Into<String>, font_size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(content),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )
}
