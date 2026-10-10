//! 3D Lighting, Metallic PBR & SDF World-Space UI Showcase.
//!
//! Features demonstrated:
//! - `URootUi::world_3d(Vec2::new(760.0, 500.0))`: World-space 3D spatial UI panel.
//! - `UPbr`: Metallic, roughness, and emissive glow reacting physically to scene lights.
//! - Dynamic orbiting PointLights (Cyan & Gold) casting specular highlights across SDF borders.
//! - Keyboard controls:
//!   * [Space]: Toggle light orbit animation
//!   * [M]: Toggle high metallic vs dielectric mode
//!   * [R]: Toggle glossy vs rough surface

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
            metallic: 0.82,
            roughness: 0.22,
            orbit_time: 0.0,
        }
    }
}

#[derive(Component)]
struct OrbitingLight {
    phase: f32,
    radius: f32,
    height: f32,
}

#[derive(Component)]
struct WorldUiPanel;

#[derive(Component, Clone, Default)]
struct StatusLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - 3D Lighting & PBR Showcase (Space: Pause | M: Metal | R: Rough)".into(),
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
                animate_lights_system,
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
    // 1. 3D Perspective Camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.03, 0.04, 0.06)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.2, 1.35).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    // 2. Ambient Light (Subtle dark mood)
    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            color: Color::srgb(0.5, 0.6, 0.8),
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 3. Orbiting Cyan PointLight
    commands.spawn((
        PointLight {
            intensity: 120_000.0,
            range: 4.5,
            color: Color::srgb(0.15, 0.85, 1.0),
            ..default()
        },
        Transform::from_xyz(0.6, 0.3, 0.5),
        OrbitingLight {
            phase: 0.0,
            radius: 0.75,
            height: 0.25,
        },
    ));

    // 4. Orbiting Gold PointLight
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            range: 4.5,
            color: Color::srgb(1.0, 0.72, 0.25),
            ..default()
        },
        Transform::from_xyz(-0.6, -0.2, 0.5),
        OrbitingLight {
            phase: PI,
            radius: 0.75,
            height: -0.15,
        },
    ));

    // 5. Dark Reflective Backdrop Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.025, 0.04),
            perceptual_roughness: 0.15,
            metallic: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -0.4).with_rotation(Quat::from_rotation_x(PI * 0.5)),
    ));

    // 6. Interactive 3D World-Space UI Holographic Terminal
    let panel_transform =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_y(-0.12));

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
            metallic: 0.82,
            roughness: 0.22,
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
                        text: { "Dynamic Light Reaction & SDF Specular Highlights".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.65, 0.75, 0.85),
                    })
                ]
            ),
            (
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    background_color: Color::srgba(0.0, 0.8, 0.9, 0.2),
                    border_radius: UCornerRadius::all(8.0),
                }
                UBorder {
                    color: Color::srgba(0.0, 0.9, 1.0, 0.8),
                    width: 1.2,
                    radius: UCornerRadius::all(8.0),
                    offset: 0.0,
                }
                UPbr {
                    metallic: 0.9,
                    roughness: 0.1,
                    emissive: LinearRgba::new(0.0, 1.8, 2.2, 1.0),
                }
                Children [
                    (UText {
                        text: { "ACTIVE LIGHTS: 2".to_string() },
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
                text: { "Physical Material & Shading Demonstration".to_string() },
                font_size: 20.0,
                font: { title_font },
                color: Color::WHITE,
            }),
            (UText {
                text: { "Observe the cyan and gold highlights gliding along the rounded SDF borders.\nAs point lights rotate in 3D world space, UPbr calculates microfacet specular reflections\nmatching true physically-based rendering.".to_string() },
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
                        text: { "METALLIC: 0.82 | ROUGHNESS: 0.22 | STATUS: ORBITING".to_string() },
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
                text: { "[Space] Pause Orbit   [M] Toggle Metal   [R] Toggle Roughness".to_string() },
                font_size: 12.0,
                font: { text_font.clone() },
                color: Color::srgb(0.45, 0.6, 0.75),
            }),
            (UText {
                text: { "Univis UI 3D Engine".to_string() },
                font_size: 12.0,
                font: { text_font },
                color: Color::srgb(0.3, 0.45, 0.6),
            })
        ]
    }
}

fn animate_lights_system(
    time: Res<Time>,
    mut state: ResMut<LightingState>,
    mut query: Query<(&OrbitingLight, &mut Transform)>,
) {
    if state.animating {
        state.orbit_time += time.delta_secs() * 1.4;
    }

    let t = state.orbit_time;
    for (light, mut transform) in &mut query {
        let angle = t + light.phase;
        let x = angle.cos() * light.radius;
        let z = angle.sin() * (light.radius * 0.8) + 0.35;
        let y = light.height + (angle * 2.0).sin() * 0.08;

        transform.translation = Vec3::new(x, y, z);
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
        state.metallic = if state.metallic > 0.5 { 0.1 } else { 0.88 };
        changed = true;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.roughness = if state.roughness > 0.4 { 0.18 } else { 0.75 };
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
