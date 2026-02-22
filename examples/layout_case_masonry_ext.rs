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

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.08, 0.1),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel {
                text: "Layout Case: Masonry + Row/Column Gap".to_string(),
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            });

            root
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(420.0),
                        background_color: Color::srgb(0.12, 0.13, 0.16),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Masonry,
                        grid_columns: 4,
                        ..default()
                    },
                    UBoxAlignContainer {
                        row_gap: Some(10.0),
                        column_gap: Some(10.0),
                        ..default()
                    },
                ))
                .with_children(|masonry| {
                    let heights = [68.0, 44.0, 94.0, 56.0, 78.0, 42.0, 66.0, 50.0, 86.0, 40.0, 70.0, 58.0];
                    for (i, h) in heights.iter().enumerate() {
                        masonry.spawn(UNode {
                            width: UVal::Auto,
                            height: UVal::Px(*h),
                            background_color: Color::srgb(0.26 + i as f32 * 0.04, 0.36, 0.64),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        });
                    }
                });
        });
}
