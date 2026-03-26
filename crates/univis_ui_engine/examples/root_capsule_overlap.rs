use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
#[allow(unused_imports)]
use univis_ui_interaction::prelude::*;
#[allow(unused_imports)]
use univis_ui_style::prelude::*;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            UnivisUiStylePlugin,
            UnivisEnginePlugin,
            UnivisInteractionPlugin,
            UnivisWidgetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 1000.0)));

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
        ))
        .with_children(|hud| {
            hud.spawn(UTextLabel {
                text: "URootUi capsule overlap demo".into(),
                font_size: 28.0,
                color: Color::srgb(0.95, 0.98, 1.0),
                ..default()
            });
            hud.spawn(UTextLabel {
                text: "Both graph nodes below share the same root z. The later-spawned node is above, and the older node's ports stay clipped under it even with higher local order."
                    .into(),
                font_size: 18.0,
                color: Color::srgb(0.72, 0.84, 0.9),
                ..default()
            });
        });

    spawn_graph_node(
        &mut commands,
        "Node A",
        Vec3::new(-90.0, -20.0, 0.0),
        Color::srgb(0.15, 0.19, 0.26),
        Color::srgb(0.48, 0.84, 0.98),
        true,
    );

    spawn_graph_node(
        &mut commands,
        "Node B",
        Vec3::new(30.0, 10.0, 0.0),
        Color::srgb(0.22, 0.15, 0.17),
        Color::srgb(0.99, 0.73, 0.32),
        false,
    );
}

fn spawn_graph_node(
    commands: &mut Commands,
    title: &str,
    position: Vec3,
    background: Color,
    accent: Color,
    ports_on_right: bool,
) {
    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_2d(Vec2::new(280.0, 180.0))
            },
            Transform::from_translation(position),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: background,
                border_radius: UCornerRadius::all(20.0),
                padding: USides::all(18.0),
                ..default()
            },
            UBorder {
                width: 2.0,
                color: accent,
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|node| {
            node.spawn(UTextLabel {
                text: title.into(),
                font_size: 28.0,
                color: accent,
                ..default()
            });

            node.spawn(UTextLabel {
                text: "Ports below intentionally sit outside the card bounds.\nThey keep their local stacking, but they never escape above another root."
                    .into(),
                font_size: 16.0,
                color: Color::srgb(0.92, 0.95, 1.0),
                ..default()
            });

            let left_port = if ports_on_right {
                UVal::Auto
            } else {
                UVal::Px(-22.0)
            };
            let right_port = if ports_on_right {
                UVal::Px(-22.0)
            } else {
                UVal::Auto
            };

            for top in [56.0, 92.0, 128.0] {
                node.spawn((
                    UNode {
                        width: UVal::Px(34.0),
                        height: UVal::Px(34.0),
                        background_color: accent.with_alpha(0.18),
                        border_radius: UCornerRadius::all(17.0),
                        ..default()
                    },
                    UBorder {
                        width: 2.0,
                        color: accent,
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        top: UVal::Px(top),
                        left: left_port,
                        right: right_port,
                        order: 32,
                        ..default()
                    },
                ));
            }
        });
}
