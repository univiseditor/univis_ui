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

    let panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UNode {
                width: UVal::Px(350.0),
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    // Standard Select
    commands.spawn((ChildOf(panel), UTextLabel::new("Graphics Quality:")));
    let select1 = USelect::new().with_options(vec![
        USelectOption::new("Low", "low"),
        USelectOption::new("Medium", "med"),
        USelectOption::new("High", "high"),
        USelectOption::new("Ultra", "ultra"),
    ]);
    commands.spawn((ChildOf(panel), select1));

    // Select with disabled options
    commands.spawn((ChildOf(panel), UTextLabel::new("Programming Language:")));
    let select2 = USelect::new().with_options(vec![
        USelectOption::new("Rust", "rust"),
        USelectOption::new("C++", "cpp").disabled(), // Unavailable
        USelectOption::new("Python", "py"),
        USelectOption::new("JavaScript", "js"),
    ]);
    commands.spawn((ChildOf(panel), select2));

    // Spacer to allow popup space at the bottom
    commands.spawn((
        ChildOf(panel),
        UNode {
            height: UVal::Px(150.0),
            ..default()
        },
    ));
}
