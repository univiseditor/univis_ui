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
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(panel),
        UCheckbox::new("Default Unchecked").checked(false),
    ));
    commands.spawn((
        ChildOf(panel),
        UCheckbox::new("Default Checked").checked(true),
    ));

    // Custom Color Checkboxes
    commands.spawn((
        ChildOf(panel),
        UCheckbox::new("Custom Color (Warning)")
            .checked(true)
            .with_color(Color::srgb(0.9, 0.6, 0.1)),
    ));
    commands.spawn((
        ChildOf(panel),
        UCheckbox::new("Custom Color (Danger)")
            .checked(true)
            .with_color(Color::srgb(0.9, 0.2, 0.2)),
    ));
}
