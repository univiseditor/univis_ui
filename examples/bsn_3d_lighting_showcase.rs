//! 3D Lighting, Metallic PBR & SDF World-Space UI Showcase.
//!
//! Features demonstrated:
//! - `URootUi::world_3d(Vec2::new(760.0, 500.0))`: World-space 3D spatial UI panel.
//! - Visible neon light orbs (Cyan & Gold) clearly orbiting 360° around and behind the panel.
//! - Panel-mounted Projector SpotLight shining directly from the UI onto an external rotating 3D artifact.
//! - `UPbr`: Metallic, roughness, and emissive glow reacting physically to scene lights.
//! - Interactive controls:
//!   * [Space]: Toggle orbit animation
//!   * [M]: Toggle metallic factor
//!   * [R]: Toggle roughness factor

use std::f32::consts::PI;

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct LightingState {
    animating: bool,
    metallic: f32,
    roughness: f32,
    orbit_time: f32,
}

impl Default for LightingState {
    fn default() -> Self {
        Self {
            animating: true,
            metallic: 0.85,
            roughness: 0.18,
            orbit_time: 0.0,
        }
    }
}

#[derive(Component)]
struct OrbitingLight {
    phase: f32,
    radius_x: f32,
    radius_z: f32,
    height: f32,
}

#[derive(Component)]
struct ExternalArtifact;

#[derive(Component)]
struct PanelProjectorLight;

#[derive(Component)]
struct WorldUiPanel;

#[derive(Component, Clone, Default)]
struct StatusLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - 3D Lighting, Orbiting Orbs & Panel Projector".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<LightingState>()
        .add_systems(Startup, setup_3d_lighting_scene)
        .add_systems(
            Update,
            (
                animate_lights_and_artifact_system,
                handle_keyboard_controls,
                sync_ui_material_state,
            ),
        )
        .run();
}

fn setup_3d_lighting_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>,
) {
    // 1. 3D Perspective Camera angled to clearly see depth and orbit around the panel
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.03, 0.05)),
            ..default()
        },
        Transform::from_xyz(0.25, 0.35, 1.55).looking_at(Vec3::new(0.05, 0.0, 0.0), Vec3::Y),
    ));

    // 2. Ambient Directional Fill
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.0,
            color: Color::srgb(0.4, 0.5, 0.7),
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 3. Orbiting Cyan Light with visible glowing sphere orb
    let cyan_color = Color::srgb(0.15, 0.9, 1.0);
    commands.spawn((
        PointLight {
            intensity: 140_000.0,
            range: 5.0,
            color: cyan_color,
            ..default()
        },
        Mesh3d(meshes.add(Sphere::new(0.045))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::from(cyan_color) * 10.0,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.9, 0.2, 0.0),
        OrbitingLight {
            phase: 0.0,
            radius_x: 0.95,
            radius_z: 0.75, // Traverses from +0.75 in front of panel to -0.75 behind panel
            height: 0.25,
        },
    ));

    // 4. Orbiting Gold Light with visible glowing sphere orb
    let gold_color = Color::srgb(1.0, 0.75, 0.25);
    commands.spawn((
        PointLight {
            intensity: 120_000.0,
            range: 5.0,
            color: gold_color,
            ..default()
        },
        Mesh3d(meshes.add(Sphere::new(0.045))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::from(gold_color) * 10.0,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(-0.9, -0.15, 0.0),
        OrbitingLight {
            phase: PI,
            radius_x: 0.95,
            radius_z: 0.75,
            height: -0.18,
        },
    ));

    // 5. External 3D Rotating Artifact that receives the panel's projector light
    let artifact_pos = Vec3::new(0.56, -0.04, 0.36);
    commands.spawn((
        ExternalArtifact,
        Mesh3d(meshes.add(Cuboid::new(0.20, 0.20, 0.20))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.95, 1.0),
            metallic: 0.95,
            perceptual_roughness: 0.12,
            reflectance: 0.85,
            ..default()
        })),
        Transform::from_translation(artifact_pos).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.5,
            0.4,
            0.2,
        )),
    ));

    // Pedestal supporting the external artifact
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.12, 0.45))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.10, 0.14),
            metallic: 0.3,
            perceptual_roughness: 0.35,
            ..default()
        })),
        Transform::from_xyz(0.56, -0.38, 0.36),
    ));

    // 6. Projector Spotlight emitted from the UI panel targeting the external artifact
    let emitter_origin = Vec3::new(0.18, 0.02, 0.06);
    commands.spawn((
        PanelProjectorLight,
        SpotLight {
            intensity: 400_000.0,
            range: 4.5,
            color: Color::srgb(0.2, 1.0, 0.65), // Emerald beam
            inner_angle: 0.12,
            outer_angle: 0.35,
            ..default()
        },
        Transform::from_translation(emitter_origin).looking_at(artifact_pos, Vec3::Y),
    ));

    // 7. Dark Reflective Backdrop Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.015, 0.02, 0.035),
            perceptual_roughness: 0.18,
            metallic: 0.75,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -0.6).with_rotation(Quat::from_rotation_x(PI * 0.5)),
    ));

    // 8. Interactive 3D World-Space UI Holographic Terminal
    let panel_transform =
        Transform::from_xyz(-0.08, 0.0, 0.0).with_rotation(Quat::from_rotation_y(-0.16));

    let text_font = theme.text.font.inter_regular.clone();
    let title_font = theme.text.font.free_serif.clone();

    commands
        .spawn_scene(lighting_terminal_scene(text_font, title_font))
        .insert((panel_transform, WorldUiPanel));
}

fn lighting_terminal_scene(text_font: Handle<Font>, title_font: Handle<Font>) -> impl Scene {
    bsn! {
        URootUi::world_3d(Vec2::new(760.0, 500.0))
        UPbr {
            metallic: 0.85,
            roughness: 0.18,
            emissive: LinearRgba::BLACK,
        }
        UNode {
            width: UVal::Px(760.0),
            height: UVal::Px(500.0),
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.92),
            border_radius: UCornerRadius::all(20.0),
            padding: USides::all(28.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.85, 1.0, 0.75),
            width: 2.5,
            radius: UCornerRadius::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            (terminal_header(title_font.clone(), text_font.clone())),
            (terminal_content(title_font, text_font.clone())),
            (terminal_footer(text_font)),
        ]
    }
}

fn terminal_header(title_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(64.0),
            background_color: Color::srgba(0.1, 0.14, 0.22, 0.6),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(16.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.85, 1.0, 0.35),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 2.0,
                }
                Children [
                    (UText {
                        text: { "QUANTUM PBR HUD // WORLD-SPACE 3D".to_string() },
                        font_size: 16.0,
                        font: { title_font },
                        color: Color::srgb(0.35, 0.88, 1.0),
                    }),
                    (UText {
                        text: { "Orbital Light Orbs & Panel Projector Emitter".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.65, 0.75, 0.85),
                    })
                ]
            ),
            (
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    background_color: Color::srgba(0.1, 0.9, 0.5, 0.2),
                    border_radius: UCornerRadius::all(8.0),
                }
                UBorder {
                    color: Color::srgba(0.2, 1.0, 0.65, 0.85),
                    width: 1.2,
                    radius: UCornerRadius::all(8.0),
                    offset: 0.0,
                }
                UPbr {
                    metallic: 0.9,
                    roughness: 0.1,
                    emissive: LinearRgba::new(0.2, 2.2, 1.2, 1.0),
                }
                Children [
                    (UText {
                        text: { "EMITTER: TARGET LOCKED".to_string() },
                        font_size: 11.0,
                        font: { text_font },
                        color: Color::WHITE,
                    })
                ]
            )
        ]
    }
}

fn terminal_content(title_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::all(16.0),
            background_color: Color::srgba(0.05, 0.07, 0.11, 0.75),
            border_radius: UCornerRadius::all(14.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.35, 0.5, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
        }
        Children [
            (UText {
                text: { "Interactive Photometric Material Demonstration".to_string() },
                font_size: 20.0,
                font: { title_font },
                color: Color::WHITE,
            }),
            (UText {
                text: { "1. The Cyan & Gold glowing orbs orbit 360° completely in front of and behind the UI panel.\n2. The panel's right emitter projects an emerald spotlight directly striking the external rotating cube.\n3. UPbr on the panel captures true microfacet specular highlights as lights pass across its SDF borders.".to_string() },
                font_size: 13.0,
                font: { text_font.clone() },
                color: Color::srgb(0.75, 0.82, 0.92),
            }),
            (
                StatusLabel
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(50.0),
                    background_color: Color::srgba(0.12, 0.16, 0.24, 0.8),
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::axes(14.0, 10.0),
                }
                UBorder {
                    color: Color::srgba(1.0, 0.75, 0.3, 0.5),
                    width: 1.5,
                    radius: UCornerRadius::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "METALLIC: 0.85 | ROUGHNESS: 0.18 | STATUS: ORBITING".to_string() },
                        font_size: 13.0,
                        font: { text_font },
                        color: Color::srgb(1.0, 0.85, 0.4),
                    })
                ]
            )
        ]
    }
}

fn terminal_footer(text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(8.0, 4.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            (UText {
                text: { "[Space] Pause   [M] Metallic Mode   [R] Roughness Mode".to_string() },
                font_size: 12.0,
                font: { text_font.clone() },
                color: Color::srgb(0.45, 0.6, 0.75),
            }),
            (UText {
                text: { "Univis UI Spatial Engine".to_string() },
                font_size: 12.0,
                font: { text_font },
                color: Color::srgb(0.3, 0.45, 0.6),
            })
        ]
    }
}

fn animate_lights_and_artifact_system(
    time: Res<Time>,
    mut state: ResMut<LightingState>,
    mut light_query: Query<(&OrbitingLight, &mut Transform), Without<ExternalArtifact>>,
    mut artifact_query: Query<&mut Transform, (With<ExternalArtifact>, Without<OrbitingLight>)>,
) {
    let dt = time.delta_secs();

    if state.animating {
        state.orbit_time += dt * 1.3;
    }

    // 1. Move orbiting light orbs in full 3D circles (front -> side -> back -> other side)
    let t = state.orbit_time;
    for (light, mut transform) in &mut light_query {
        let angle = t + light.phase;
        let x = angle.cos() * light.radius_x - 0.08;
        let z = angle.sin() * light.radius_z;
        let y = light.height + (angle * 2.0).sin() * 0.12;

        transform.translation = Vec3::new(x, y, z);
    }

    // 2. Rotate external artifact so the spotlight from the panel reflects off its metallic facets
    for mut transform in &mut artifact_query {
        transform.rotate_y(0.75 * dt);
        transform.rotate_x(0.35 * dt);
    }
}

fn handle_keyboard_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<LightingState>,
    status_query: Query<&Children, With<StatusLabel>>,
    mut text_query: Query<&mut UText>,
) {
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::Space) {
        state.animating = !state.animating;
        changed = true;
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        state.metallic = if state.metallic > 0.5 { 0.12 } else { 0.88 };
        changed = true;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.roughness = if state.roughness > 0.3 { 0.14 } else { 0.75 };
        changed = true;
    }

    if changed {
        for children in &status_query {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    let anim_str = if state.animating { "ORBITING" } else { "PAUSED" };
                    text.text = format!(
                        "METALLIC: {:.2} | ROUGHNESS: {:.2} | STATUS: {}",
                        state.metallic, state.roughness, anim_str
                    );
                }
            }
        }
    }
}

fn sync_ui_material_state(
    state: Res<LightingState>,
    mut pbr_query: Query<&mut UPbr, With<WorldUiPanel>>,
) {
    if state.is_changed() {
        for mut pbr in &mut pbr_query {
            pbr.metallic = state.metallic;
            pbr.roughness = state.roughness;
        }
    }
}
