use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.1),
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
                width: UVal::Px(1040.0),
                height: UVal::Px(680.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.98),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.72, 0.8, 0.9, 0.18),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(shell),
        label_node(),
        text("Stack layout", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "The left panel shows layered surfaces. The right panel builds a compact HUD stack from the same layout mode.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(992.0),
                height: UVal::Px(560.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    spawn_layers_panel(&mut commands, content);
    spawn_hud_panel(&mut commands, content);
}

fn spawn_layers_panel(commands: &mut Commands, parent: Entity) {
    let panel = panel(commands, parent, 487.0, 560.0);

    commands.spawn((
        ChildOf(panel),
        label_node(),
        text("Layered surfaces", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "Each child uses the same center space, so the stack creates an overlapped deck.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let stack = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(455.0),
                height: UVal::Px(450.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Stack,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let layers = [
        ("Base", 300.0, 220.0, Color::srgba(0.19, 0.34, 0.78, 0.92)),
        ("Focus", 250.0, 176.0, Color::srgba(0.9, 0.63, 0.18, 0.9)),
        ("Info", 196.0, 138.0, Color::srgba(0.18, 0.68, 0.48, 0.9)),
        ("Badge", 132.0, 92.0, Color::srgba(0.77, 0.27, 0.31, 0.9)),
    ];

    for (title, width, height, color) in layers {
        spawn_stack_card(commands, stack, title, width, height, color);
    }
}

fn spawn_hud_panel(commands: &mut Commands, parent: Entity) {
    let panel = panel(commands, parent, 487.0, 560.0);

    commands.spawn((
        ChildOf(panel),
        label_node(),
        text("HUD composition", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "This one uses the same stack mode for concentric rings, a center chip, and an alert badge.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let stack = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(455.0),
                height: UVal::Px(450.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Stack,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_stack_card(
        commands,
        stack,
        "",
        264.0,
        264.0,
        Color::srgba(0.2, 0.37, 0.82, 0.28),
    );
    spawn_stack_card(
        commands,
        stack,
        "",
        214.0,
        214.0,
        Color::srgba(0.16, 0.57, 0.73, 0.38),
    );
    spawn_stack_card(
        commands,
        stack,
        "LOCK",
        138.0,
        86.0,
        Color::srgba(0.18, 0.68, 0.48, 0.92),
    );
    spawn_stack_card(
        commands,
        stack,
        "ALERT",
        124.0,
        64.0,
        Color::srgba(0.77, 0.27, 0.31, 0.94),
    );
}

fn panel(commands: &mut Commands, parent: Entity, width: f32, height: f32) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.1, 0.13, 0.18, 0.96),
                border_radius: UCornerRadius::all(22.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.7, 0.8, 0.92, 0.12),
                width: 1.0,
                radius: UCornerRadius::all(22.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id()
}

fn spawn_stack_card(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    width: f32,
    height: f32,
    background_color: Color,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
                padding: USides::all(12.0),
                background_color,
                border_radius: UCornerRadius::all(18.0),
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

    if !title.is_empty() {
        commands.spawn((ChildOf(card), label_node(), text(title, 22.0, Color::WHITE)));
    }
}

fn label_node() -> UNode {
    UNode {
        background_color: Color::NONE,
        ..default()
    }
}

fn text(value: &str, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: value.to_string(),
        font_size,
        color,
        ..default()
    }
}
