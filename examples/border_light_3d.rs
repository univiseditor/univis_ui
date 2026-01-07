use bevy::{post_process::bloom::Bloom, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,UnivisUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.),
        Bloom::NATURAL,
    ));
    commands.spawn((
        UWorldRoot {
            is_3d: true,
            ..default()
        },

        ULayout {
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 20.0,
            ..default()
    })).with_children(|root| {
        root.spawn((
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(300.0),
                border_radius: UCornerRadius::all(30.),
                ..default()
            },
            UBorder {
                width: 5.0,
                color: Color::srgb(10.0, 0.0, 5.0),
                ..default()
            },
        ));

        root.spawn((
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(300.0),
                border_radius: UCornerRadius::all(30.),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                width: 5.0,
                color: Color::srgb(10.0, 0.0, 5.0),
                ..default()
            },
        ));
    });
}