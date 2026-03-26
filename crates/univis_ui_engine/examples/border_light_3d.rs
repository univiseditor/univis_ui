//! Exercises the lit `World3d` render path with bloom-friendly styling.
//!
//! Related docs:
//! - `docs/src/en/rendering/overview.md`
//! - `docs/src/ar/rendering/overview.md`
//! - `docs/src/en/examples/index.md#border_light_3d`

use bevy::{core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*};
use univis_ui_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UnivisEnginePlugin))
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
        Tonemapping::ReinhardLuminance,
        Bloom::NATURAL,
    ));
    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_3d(Vec2::new(800.0, 600.0))
            },
            ULayout {
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 20.0,
                ..default()
            },
        ))
        .with_children(|root| {
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
