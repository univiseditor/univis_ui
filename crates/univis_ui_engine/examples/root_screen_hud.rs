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

#[derive(Component)]
struct DemoCamera;

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
        .add_systems(Update, animate_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, DemoCamera, Transform::from_xyz(0.0, 0.0, 1000.0)));

    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_2d(Vec2::new(820.0, 460.0))
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.09, 0.12, 0.17),
                border_radius: UCornerRadius::all(24.0),
                padding: USides::all(24.0),
                ..default()
            },
            UBorder {
                width: 2.0,
                color: Color::srgba(0.4, 0.72, 0.92, 0.85),
                ..default()
            },
            ULayout {
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .with_children(|world| {
            world.spawn(UTextLabel {
                text: "World Canvas".into(),
                font_size: 34.0,
                color: Color::srgb(0.86, 0.96, 1.0),
                ..default()
            });

            world
                .spawn((
                    UNode {
                        width: UVal::Px(420.0),
                        height: UVal::Px(170.0),
                        background_color: Color::srgb(0.12, 0.17, 0.24),
                        border_radius: UCornerRadius::all(18.0),
                        padding: USides::all(18.0),
                        ..default()
                    },
                    UBorder {
                        width: 2.0,
                        color: Color::srgba(0.95, 0.78, 0.32, 0.85),
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|card| {
                    card.spawn(UTextLabel {
                        text: "This panel lives in world space.\nCamera motion, zoom, and rotation affect it."
                            .into(),
                        font_size: 24.0,
                        justify: Justify::Center,
                        color: Color::srgb(0.95, 0.97, 1.0),
                        ..default()
                    });
                });
        });

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .with_children(|hud| {
            hud.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(116.0),
                    background_color: Color::srgba(0.03, 0.05, 0.08, 0.86),
                    border_radius: UCornerRadius::all(20.0),
                    padding: USides::all(18.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.36, 0.88, 0.78, 0.82),
                    ..default()
                },
                ULayout {
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|top| {
                top.spawn(UTextLabel {
                    text: "URootUi::screen() stays fixed on the viewport".into(),
                    font_size: 30.0,
                    color: Color::srgb(0.92, 0.98, 0.98),
                    ..default()
                });

                top.spawn(UTextLabel {
                    text: "Watch the world canvas drift, rotate, and rescale while this HUD remains anchored."
                        .into(),
                    font_size: 20.0,
                    color: Color::srgb(0.74, 0.88, 0.92),
                    ..default()
                });
            });

            hud.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    ..default()
                },
                ULayout {
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|center| {
                center
                    .spawn((
                        UNode {
                            width: UVal::Px(92.0),
                            height: UVal::Px(92.0),
                            background_color: Color::srgba(0.02, 0.04, 0.06, 0.68),
                            border_radius: UCornerRadius::all(46.0),
                            ..default()
                        },
                        UBorder {
                            width: 2.0,
                            color: Color::srgba(0.98, 0.72, 0.22, 0.9),
                            ..default()
                        },
                        ULayout {
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|marker| {
                        marker.spawn(UTextLabel {
                            text: "HUD".into(),
                            font_size: 26.0,
                            color: Color::srgb(0.98, 0.94, 0.82),
                            ..default()
                        });
                    });
            });
        });
}

fn animate_camera(
    mut cameras: Query<(&mut Transform, &mut Projection), With<DemoCamera>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };
    let t = time.elapsed_secs();

    transform.translation = Vec3::new(t.sin() * 220.0, (t * 0.7).cos() * 140.0, 1000.0);
    transform.rotation = Quat::from_rotation_z((t * 0.45).sin() * 0.35);

    if let Projection::Orthographic(ortho) = &mut *projection {
        ortho.scale = 0.8 + 0.28 * (t * 0.8).sin().abs();
    }
}
