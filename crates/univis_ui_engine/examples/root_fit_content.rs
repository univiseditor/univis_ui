use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
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
struct FitContentWorld3dCard;

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
        .add_systems(Update, animate_world3d_card)
        .run();
}

fn setup(mut commands: Commands) {
    let camera_3d = commands
        .spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.07, 0.1)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 920.0).looking_at(Vec3::ZERO, Vec3::Y),
            Tonemapping::ReinhardLuminance,
        ))
        .id();

    commands.spawn((
        PointLight {
            intensity: 380_000.0,
            range: 4000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-260.0, 260.0, 420.0),
    ));

    let camera_2d = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1000.0),
        ))
        .id();

    commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera_2d),
                ..URootUi::screen()
            },
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|hud| {
            hud.spawn(UTextLabel {
                text: "URootUi fit-content roots".into(),
                font_size: 30.0,
                color: Color::srgb(0.95, 0.98, 1.0),
                ..default()
            });
            hud.spawn(UTextLabel {
                text: "Left: World3d root using UiCanvasSize::FitContent { min, max }. Right: World2d root using URootUi::world_2d_fit_content(). In both cases the root grows from measured children instead of a fixed canvas."
                    .into(),
                font_size: 18.0,
                color: Color::srgb(0.72, 0.84, 0.92),
                ..default()
            });
        });

    spawn_world3d_fit_content_card(&mut commands, camera_3d);
    spawn_world2d_fit_content_card(&mut commands, camera_2d);
}

fn spawn_world2d_fit_content_card(commands: &mut Commands, camera: Entity) {
    let accent = Color::srgb(0.55, 0.88, 0.98);

    commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera),
                meters_per_unit: 1.0,
                ..URootUi::world_2d_fit_content()
            },
            Transform::from_xyz(250.0, -40.0, 0.0),
            UNode {
                background_color: Color::srgb(0.14, 0.19, 0.26),
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
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn(UTextLabel {
                text: "World2d fit-content".into(),
                font_size: 28.0,
                color: accent,
                ..default()
            });

            panel
                .spawn((
                    UNode {
                        width: UVal::Px(280.0),
                        background_color: accent.with_alpha(0.12),
                        border_radius: UCornerRadius::all(14.0),
                        padding: USides::all(12.0),
                        ..default()
                    },
                    UBorder {
                        width: 1.0,
                        color: accent.with_alpha(0.55),
                        ..default()
                    },
                ))
                .with_children(|blurb| {
                    blurb.spawn(UTextLabel {
                        text: "This root uses the convenience constructor and wraps its content automatically. No fixed canvas size is declared on the root."
                            .into(),
                        font_size: 16.0,
                        color: Color::srgb(0.92, 0.95, 1.0),
                        ..default()
                    });
                });

            panel
                .spawn((
                    UNode {
                        width: UVal::Px(190.0),
                        height: UVal::Px(48.0),
                        background_color: accent.with_alpha(0.16),
                        border_radius: UCornerRadius::all(14.0),
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|badge| {
                    badge.spawn(UTextLabel {
                        text: "Convenience path".into(),
                        font_size: 16.0,
                        justify: Justify::Center,
                        color: Color::srgb(0.97, 0.99, 1.0),
                        ..default()
                    });
                });
        });
}

fn spawn_world3d_fit_content_card(commands: &mut Commands, camera: Entity) {
    let accent = Color::srgb(0.99, 0.77, 0.36);
    let mut transform = Transform::from_xyz(-240.0, -40.0, 0.0);
    transform.rotation = Quat::from_rotation_y(-0.28);

    commands
        .spawn((
            URootUi {
                space: UiSpace::World3d,
                canvas: UiCanvasSize::FitContent {
                    min: Vec2::new(260.0, 150.0),
                    max: Some(Vec2::new(430.0, 280.0)),
                },
                camera: UiCameraRef::Entity(camera),
                meters_per_unit: 1.0,
                resolution_scale: 1.0,
            },
            transform,
            FitContentWorld3dCard,
            UNode {
                background_color: Color::srgb(0.24, 0.16, 0.13),
                border_radius: UCornerRadius::all(22.0),
                padding: USides::all(18.0),
                ..default()
            },
            UBorder {
                width: 2.0,
                color: accent,
                ..default()
            },
            UPbr {
                metallic: 0.08,
                roughness: 0.32,
                emissive: LinearRgba::rgb(0.06, 0.03, 0.0),
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn(UTextLabel {
                text: "World3d fit-content".into(),
                font_size: 28.0,
                color: accent,
                ..default()
            });

            panel
                .spawn((
                    UNode {
                        width: UVal::Px(300.0),
                        background_color: accent.with_alpha(0.1),
                        border_radius: UCornerRadius::all(14.0),
                        padding: USides::all(12.0),
                        ..default()
                    },
                    UBorder {
                        width: 1.0,
                        color: accent.with_alpha(0.5),
                        ..default()
                    },
                ))
                .with_children(|blurb| {
                    blurb.spawn(UTextLabel {
                        text: "This root uses the full FitContent form, so the logical canvas is measured from content and clamped by min and max bounds."
                            .into(),
                        font_size: 16.0,
                        color: Color::srgb(0.98, 0.96, 0.92),
                        ..default()
                    });
                });

            panel
                .spawn((
                    UNode {
                        width: UVal::Px(210.0),
                        height: UVal::Px(54.0),
                        background_color: accent.with_alpha(0.18),
                        border_radius: UCornerRadius::all(14.0),
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|badge| {
                    badge.spawn(UTextLabel {
                        text: "min/max clamp ready".into(),
                        font_size: 17.0,
                        justify: Justify::Center,
                        color: Color::srgb(1.0, 0.99, 0.95),
                        ..default()
                    });
                });
        });
}

fn animate_world3d_card(
    time: Res<Time>,
    mut cards: Query<&mut Transform, With<FitContentWorld3dCard>>,
) {
    for mut transform in cards.iter_mut() {
        transform.rotation = Quat::from_rotation_y(-0.28 + time.elapsed_secs().sin() * 0.08);
    }
}
