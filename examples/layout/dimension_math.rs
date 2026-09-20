//! CSS dimension math demonstration using `calc()`, `min()`, `max()`, and `clamp()` in `UVal`.
//!
//! Demonstrates zero-allocation `<length-percentage>` arithmetic, operator overloading (`UVal::Percent - UVal::Px`),
//! and fluid mathematical bounds adapting in real-time to container resizing.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Dimension Math (calc, min, max, clamp)".into(),
                resolution: (1200, 780).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<MathDemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_math_preset_clicks)
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MathContainerWidth {
    Full,
    Medium,
    Narrow,
}

impl MathContainerWidth {
    fn width(self) -> f32 {
        match self {
            Self::Full => 1000.0,
            Self::Medium => 680.0,
            Self::Narrow => 380.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Full => "Full Width (1000px)",
            Self::Medium => "Medium Width (680px)",
            Self::Narrow => "Narrow Width (380px)",
        }
    }
}

#[derive(Resource)]
struct MathDemoState {
    preset: MathContainerWidth,
}

impl Default for MathDemoState {
    fn default() -> Self {
        Self {
            preset: MathDemoState::preset_default(),
        }
    }
}

impl MathDemoState {
    const fn preset_default() -> MathContainerWidth {
        MathContainerWidth::Full
    }
}

#[derive(Component)]
struct ResizableMathShell;

#[derive(Component)]
struct MathPresetButton(MathContainerWidth);

#[derive(Component)]
struct MathStatusText;

fn setup(mut commands: Commands, state: Res<MathDemoState>) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.04, 0.05, 0.08),
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

    let outer = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(1120.0),
                height: UVal::Px(720.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.07, 0.09, 0.14, 0.98),
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

    // Title
    commands.spawn((
        ChildOf(outer),
        label_node(),
        text(
            "Dimension Math in `UVal`: `calc()`, `clamp()`, `min()`, `max()`",
            25.0,
            Color::srgb(0.95, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(outer),
        label_node(),
        text(
            "All operations run with zero heap allocations (Copy enum). Resize the container below to test fluid bounds.",
            14.0,
            Color::srgb(0.72, 0.78, 0.86),
        ),
    ));

    // Preset buttons
    let btn_bar = commands
        .spawn((
            ChildOf(outer),
            UNode {
                height: UVal::Px(42.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    for preset in [
        MathContainerWidth::Full,
        MathContainerWidth::Medium,
        MathContainerWidth::Narrow,
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
                MathPresetButton(preset),
                UNode {
                    padding: USides::axes(16.0, 8.0),
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

    // Status
    let status_pill = commands
        .spawn((
            ChildOf(outer),
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
        MathStatusText,
        label_node(),
        text(
            format!(
                "Container Width: {:.0}px | Click any preset button to trigger solver re-evaluation",
                state.preset.width()
            ),
            13.0,
            Color::srgb(0.45, 0.8, 1.0),
        ),
    ));

    // Centered wrapper
    let wrapper = commands
        .spawn((
            ChildOf(outer),
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

    // Resizable shell
    let math_shell = commands
        .spawn((
            ChildOf(wrapper),
            ResizableMathShell,
            UNode {
                width: UVal::Px(state.preset.width()),
                height: UVal::Px(430.0),
                padding: USides::all(18.0),
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
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    // Demo 1: Operator Overloading: UVal::Percent(1.0) - UVal::Px(48.0)
    let calc_expr = UVal::Percent(1.0) - UVal::Px(48.0);
    spawn_math_row(
        &mut commands,
        math_shell,
        "calc(100% - 48px) via `UVal::Percent(1.0) - UVal::Px(48.0)`",
        "Takes full container width minus 48px exact margin offset",
        calc_expr,
        Color::srgb(0.2, 0.5, 0.9),
    );

    // Demo 2: clamp(200px, 50%, 650px)
    let clamp_expr = UVal::clamp(
        UValLength::px(200.0),
        UValLength::percent(0.5),
        UValLength::px(650.0),
    );
    spawn_math_row(
        &mut commands,
        math_shell,
        "clamp(200px, 50%, 650px)",
        "Halves container width, but never drops below 200px and never exceeds 650px",
        clamp_expr,
        Color::srgb(0.18, 0.7, 0.5),
    );

    // Demo 3: min(100%, 450px)
    let min_expr = UVal::min(UValLength::percent(1.0), UValLength::px(450.0));
    spawn_math_row(
        &mut commands,
        math_shell,
        "min(100%, 450px)",
        "Caps size to 450px on wide screens, but shrinks fluidly to 100% on narrow screens",
        min_expr,
        Color::srgb(0.9, 0.55, 0.15),
    );

    // Demo 4: max(30%, 250px)
    let max_expr = UVal::max(UValLength::percent(0.3), UValLength::px(250.0));
    spawn_math_row(
        &mut commands,
        math_shell,
        "max(30%, 250px)",
        "Guarantees a 250px floor, but expands with 30% of wide containers",
        max_expr,
        Color::srgb(0.7, 0.35, 0.85),
    );
}

fn spawn_math_row(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    desc: &str,
    dimension: UVal,
    color: Color,
) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                height: UVal::Px(78.0),
                padding: USides::all(10.0),
                background_color: Color::srgba(0.12, 0.15, 0.22, 0.9),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.07),
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

    let text_row = commands
        .spawn((
            ChildOf(row),
            UNode {
                height: UVal::Px(20.0),
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
        ChildOf(text_row),
        label_node(),
        text(label, 13.0, Color::WHITE),
    ));

    commands.spawn((
        ChildOf(text_row),
        label_node(),
        text(desc, 11.0, Color::srgb(0.65, 0.72, 0.82)),
    ));

    // The visual bar sized by the math expression!
    commands.spawn((
        ChildOf(row),
        UNode {
            width: dimension,
            height: UVal::Px(28.0),
            background_color: color,
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        UBorder {
            color: Color::srgba(1.0, 1.0, 1.0, 0.2),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
            offset: 0.0,
        },
    ));
}

fn handle_math_preset_clicks(
    mut state: ResMut<MathDemoState>,
    mut container_q: Query<&mut UNode, With<ResizableMathShell>>,
    mut status_q: Query<&mut UTextLabel, With<MathStatusText>>,
    mut buttons_q: Query<
        (&MathPresetButton, &mut UNode, &mut UBorder),
        Without<ResizableMathShell>,
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

    let next_preset = match state.preset {
        MathContainerWidth::Full => MathContainerWidth::Medium,
        MathContainerWidth::Medium => MathContainerWidth::Narrow,
        MathContainerWidth::Narrow => MathContainerWidth::Full,
    };

    if cursor.y < 250.0 {
        state.preset = next_preset;

        if let Ok(mut node) = container_q.single_mut() {
            node.width = UVal::Px(state.preset.width());
        }

        if let Ok(mut txt) = status_q.single_mut() {
            txt.text = format!(
                "Container Width: {:.0}px | Click any preset button to trigger solver re-evaluation",
                state.preset.width()
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

fn text(content: impl Into<String>, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: content.into(),
        font_size,
        color,
        ..default()
    }
}
