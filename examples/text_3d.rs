use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UnivisUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // A ground plane to see shadows and context
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 3D UI Node with Text
    commands
        .spawn((
            URootUi::world_3d(Vec2::new(800.0, 600.0)),
            Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::splat(1.0)),
            UPanel::card(),
            UNode {
                width: UVal::Px(800.0),
                height: UVal::Px(600.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                UTextLabel {
                    text: "Hello 3D World!".into(),
                    font_size: 64.0,
                    color: Color::srgb(1.0, 0.8, 0.2),
                    ..default()
                },
                UNode {
                    background_color: Color::NONE,
                    ..default()
                },
            ));

            parent.spawn((
                UTextLabel {
                    text: "This text is rendered using a PBR shader.".into(),
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
                UNode {
                    background_color: Color::NONE,
                    ..default()
                },
            ));

            parent
                .spawn((
                    UButton::primary(),
                    UNode {
                        padding: USides::all(16.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        UTextLabel {
                            text: "Click Me".into(),
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                        UNode {
                            background_color: Color::NONE,
                            ..default()
                        },
                    ));
                });
        });
}
