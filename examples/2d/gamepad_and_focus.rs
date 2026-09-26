//! # Gamepad & Keyboard Focus Navigation Showcase
//!
//! Demonstrates:
//! 1. **2D Spatial Navigation**: Seamless directional navigation across a matrix of cards
//!    using keyboard Arrow keys or Gamepad D-pad / Left analog stick.
//! 2. **Sequential Tab Cycling**: Natural Tab / Shift+Tab cycling respecting authored `tab_index`.
//! 3. **Input Modalities**: Real-time tracking of active modality (`Keyboard`, `Gamepad`, `Pointer`).
//! 4. **Action Activation**: Dual-mode activation (Enter, Space, Gamepad South button) triggering
//!    [`UFocusActivate`] and click feedback.
//! 5. **Spring Focus Rings**: Declarative [`UFocusVisual`] + [`UTransition`] for dynamic scale zoom
//!    and glowing borders.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct DemoState {
    activated_card: Option<String>,
    activation_count: usize,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            activated_card: None,
            activation_count: 0,
        }
    }
}

#[derive(Component)]
enum TelemetryField {
    FocusedTarget,
    ActiveModality,
    ActivationCount,
    GamepadStatus,
    WrapAroundStatus,
}

#[derive(Component)]
struct MissionCard {
    id: usize,
    title: String,
}

fn border(color: Color, r: f32, width: f32) -> UBorder {
    UBorder {
        color,
        width,
        radius: UCornerRadius::all(r),
        offset: 0.0,
    }
}

fn spawn_label(
    commands: &mut Commands,
    parent: Entity,
    text: &str,
    color: Color,
    font_size: f32,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            UTextLabel {
                text: text.into(),
                color,
                font_size,
                ..default()
            },
        ))
        .id()
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Gamepad & Keyboard Focus Navigation".into(),
                    resolution: (1280, 800).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<DemoState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_telemetry, handle_toggle_keys))
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    let missions = [
        (
            "NEXUS CORE",
            "Sector 01",
            "CRITICAL",
            Color::srgb(0.9, 0.3, 0.3),
        ),
        (
            "ORBITAL ARRAY",
            "Sector 02",
            "ACTIVE",
            Color::srgb(0.2, 0.8, 0.5),
        ),
        (
            "QUANTUM RELAY",
            "Sector 03",
            "STANDBY",
            Color::srgb(0.4, 0.7, 1.0),
        ),
        (
            "PLASMA CONDUIT",
            "Sector 04",
            "NOMINAL",
            Color::srgb(0.9, 0.7, 0.2),
        ),
        (
            "HYPER-LIFT 07",
            "Sector 05",
            "ACTIVE",
            Color::srgb(0.2, 0.8, 0.5),
        ),
        (
            "WARP STABILIZER",
            "Sector 06",
            "CRITICAL",
            Color::srgb(0.9, 0.3, 0.3),
        ),
        (
            "SHIELD MATRIX",
            "Sector 07",
            "STANDBY",
            Color::srgb(0.4, 0.7, 1.0),
        ),
        (
            "DATA ARCHIVE",
            "Sector 08",
            "NOMINAL",
            Color::srgb(0.9, 0.7, 0.2),
        ),
        (
            "COMMAND DECK",
            "Sector 09",
            "SECURE",
            Color::srgb(0.7, 0.4, 0.9),
        ),
    ];

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.11),
                padding: USides::all(28.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // --- Header Section ---
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(64.0),
                padding: USides::axes(12.0, 20.0),
                background_color: Color::srgb(0.08, 0.11, 0.17),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            border(Color::srgb(0.18, 0.24, 0.36), 12.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Left Title Block
    let title_block = commands
        .spawn((
            ChildOf(header),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        title_block,
        "SYSTEM NAVIGATION PROTOCOL",
        Color::srgb(0.3, 0.8, 1.0),
        18.0,
    );
    spawn_label(
        &mut commands,
        title_block,
        "Seamless Spatial & Tab Navigation for Controllers and Keyboards",
        Color::srgb(0.5, 0.6, 0.7),
        11.5,
    );

    // Right Telemetry Indicators
    let right_chips = commands
        .spawn((
            ChildOf(header),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 14.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Modality chip
    let mod_chip = commands
        .spawn((
            ChildOf(right_chips),
            UNode {
                padding: USides::axes(6.0, 12.0),
                background_color: Color::srgb(0.12, 0.17, 0.25),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(Color::srgb(0.25, 0.35, 0.5), 8.0, 1.0),
        ))
        .id();
    commands.spawn((
        ChildOf(mod_chip),
        UNode::default(),
        TelemetryField::ActiveModality,
        UTextLabel {
            text: "MODALITY: KEYBOARD".into(),
            color: Color::srgb(0.3, 0.8, 1.0),
            font_size: 11.0,
            ..default()
        },
    ));

    // Gamepad status chip
    let gp_chip = commands
        .spawn((
            ChildOf(right_chips),
            UNode {
                padding: USides::axes(6.0, 12.0),
                background_color: Color::srgb(0.12, 0.17, 0.25),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(Color::srgb(0.25, 0.35, 0.5), 8.0, 1.0),
        ))
        .id();
    commands.spawn((
        ChildOf(gp_chip),
        UNode::default(),
        TelemetryField::GamepadStatus,
        UTextLabel {
            text: "GAMEPAD: DISCONNECTED".into(),
            color: Color::srgb(0.6, 0.7, 0.8),
            font_size: 11.0,
            ..default()
        },
    ));

    // Wrap-around chip
    let wrap_chip = commands
        .spawn((
            ChildOf(right_chips),
            UNode {
                padding: USides::axes(6.0, 12.0),
                background_color: Color::srgb(0.12, 0.17, 0.25),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(Color::srgb(0.25, 0.35, 0.5), 8.0, 1.0),
        ))
        .id();
    commands.spawn((
        ChildOf(wrap_chip),
        UNode::default(),
        TelemetryField::WrapAroundStatus,
        UTextLabel {
            text: "WRAP: OFF [W]".into(),
            color: Color::srgb(0.8, 0.6, 0.3),
            font_size: 11.0,
            ..default()
        },
    ));

    // --- Center 3x3 Card Grid ---
    let grid_container = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(960.0),
                height: UVal::Px(460.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 3,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    for (idx, (title, sector, status, status_color)) in missions.iter().enumerate() {
        let card_entity = commands
            .spawn((
                ChildOf(grid_container),
                MissionCard {
                    id: idx + 1,
                    title: title.to_string(),
                },
                UNode {
                    width: UVal::Auto,
                    height: UVal::Px(130.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgb(0.09, 0.12, 0.18),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                border(Color::srgb(0.16, 0.22, 0.32), 14.0, 1.5),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                },
                // 1. Focusable component with sequential tab ordering
                UFocusable::new().with_tab_index(idx as i32 + 1),
                // 2. High-performance spring focus visual
                UFocusVisual::glow(Color::srgb(0.2, 0.85, 1.0)).with_scale(1.05),
                // 3. Spring transition zoom physics
                UTransition::bouncy(280.0, 0.68),
                // 4. Interactive pointer hover/click colors
                UInteractionColors {
                    normal: Color::srgb(0.09, 0.12, 0.18),
                    hovered: Color::srgb(0.13, 0.18, 0.28),
                    pressed: Color::srgb(0.06, 0.09, 0.14),
                },
            ))
            .observe(
                |trigger: On<Pointer<Click>>,
                 query: Query<&MissionCard>,
                 mut state: ResMut<DemoState>| {
                    let entity = trigger.entity.entity();
                    if let Ok(card) = query.get(entity) {
                        state.activated_card = Some(format!("#{} {}", card.id, card.title));
                        state.activation_count += 1;
                    }
                },
            )
            .observe(
                |trigger: On<UFocusActivate>,
                 query: Query<&MissionCard>,
                 mut state: ResMut<DemoState>| {
                    let entity = trigger.entity;
                    if let Ok(card) = query.get(entity) {
                        state.activated_card = Some(format!("#{} {}", card.id, card.title));
                        state.activation_count += 1;
                    }
                },
            )
            .id();

        // Top row
        let top_row = commands
            .spawn((
                ChildOf(card_entity),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .id();

        spawn_label(
            &mut commands,
            top_row,
            *sector,
            Color::srgb(0.5, 0.6, 0.7),
            11.0,
        );

        let pill = commands
            .spawn((
                ChildOf(top_row),
                UNode {
                    padding: USides::axes(3.0, 8.0),
                    background_color: status_color.with_alpha(0.18),
                    border_radius: UCornerRadius::all(6.0),
                    ..default()
                },
                border(*status_color, 6.0, 1.0),
            ))
            .id();

        spawn_label(&mut commands, pill, *status, *status_color, 10.0);

        // Card Title
        spawn_label(&mut commands, card_entity, *title, Color::WHITE, 15.0);

        // Footer hint
        spawn_label(
            &mut commands,
            card_entity,
            &format!("ENTER / (A) TO ENGAGE [TAB: {}]", idx + 1),
            Color::srgb(0.4, 0.5, 0.65),
            9.5,
        );
    }

    // --- Bottom Controls & Telemetry Footer ---
    let footer = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(56.0),
                padding: USides::axes(12.0, 20.0),
                background_color: Color::srgb(0.07, 0.09, 0.14),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            border(Color::srgb(0.16, 0.22, 0.32), 12.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Live Status Output
    let left_footer = commands
        .spawn((
            ChildOf(footer),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 24.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(left_footer),
        UNode::default(),
        TelemetryField::FocusedTarget,
        UTextLabel {
            text: "ACTIVE FOCUS: NONE".into(),
            color: Color::srgb(0.4, 0.85, 1.0),
            font_size: 13.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(left_footer),
        UNode::default(),
        TelemetryField::ActivationCount,
        UTextLabel {
            text: "ACTIVATIONS: 0".into(),
            color: Color::srgb(0.2, 0.9, 0.5),
            font_size: 13.0,
            ..default()
        },
    ));

    // Navigation Legend
    let right_legend = commands
        .spawn((
            ChildOf(footer),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        right_legend,
        "[Arrows / D-Pad / Stick: Nav]  [Tab: Cycle]  [Enter / Space / (A): Activate]  [W: Wrap]",
        Color::srgb(0.5, 0.6, 0.7),
        11.0,
    );
}

fn update_telemetry(
    focus_state: Res<UFocusState>,
    demo_state: Res<DemoState>,
    settings: Res<UFocusNavigationSettings>,
    gamepads: Query<&Gamepad>,
    cards: Query<&MissionCard>,
    mut text_query: Query<(&mut UTextLabel, &TelemetryField)>,
) {
    let has_gamepad = !gamepads.is_empty();

    for (mut label, field) in text_query.iter_mut() {
        match field {
            TelemetryField::FocusedTarget => {
                if let Some(focused) = focus_state.focused() {
                    if let Ok(card) = cards.get(focused) {
                        label.text = format!("ACTIVE FOCUS: #{} {}", card.id, card.title);
                    } else {
                        label.text = format!("ACTIVE FOCUS: Entity {:?}", focused);
                    }
                } else {
                    label.text = "ACTIVE FOCUS: NONE".into();
                }
            }
            TelemetryField::ActiveModality => {
                let mod_str = match focus_state.modality() {
                    FocusModality::Keyboard => "MODALITY: KEYBOARD",
                    FocusModality::Gamepad => "MODALITY: GAMEPAD",
                    FocusModality::Pointer => "MODALITY: POINTER",
                };
                label.text = mod_str.into();
            }
            TelemetryField::ActivationCount => {
                if let Some(ref last) = demo_state.activated_card {
                    label.text = format!(
                        "ACTIVATIONS: {} (LAST: {})",
                        demo_state.activation_count, last
                    );
                } else {
                    label.text = format!("ACTIVATIONS: {}", demo_state.activation_count);
                }
            }
            TelemetryField::GamepadStatus => {
                if has_gamepad {
                    label.text = "GAMEPAD: CONNECTED".into();
                } else {
                    label.text = "GAMEPAD: DISCONNECTED".into();
                }
            }
            TelemetryField::WrapAroundStatus => {
                if settings.wrap_around {
                    label.text = "WRAP: ON [W]".into();
                } else {
                    label.text = "WRAP: OFF [W]".into();
                }
            }
        }
    }
}

fn handle_toggle_keys(
    keyboard_opt: Option<Res<ButtonInput<KeyCode>>>,
    mut settings: ResMut<UFocusNavigationSettings>,
) {
    let Some(keyboard) = keyboard_opt else {
        return;
    };

    if keyboard.just_pressed(KeyCode::KeyW) {
        settings.wrap_around = !settings.wrap_around;
    }
}
