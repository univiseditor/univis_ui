//! # 3D Border & Lighting Showcase
//!
//! An interactive Bevy example showing off the SDF-based border capabilities
//! of Univis UI in world-space 3D, and how they react physically to scene lighting.
//!
//! **Controls:**
//! - Right-click drag: orbit camera
//! - Scroll: zoom in/out
//! - Click and drag sliders to dynamically tune borders and materials!

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use univis_ui::prelude::*;

// ── App Settings ────────────────────────────────────────────────────────────
#[derive(Resource)]
struct AppSettings {
    border_width: f32,
    border_radius: f32,
    border_offset: f32,
    panel_metallic: f32,
    panel_roughness: f32,
    light_orbit_speed: f32,
    orbit_lights_enabled: bool,
    shape_mode: UShapeMode,
    light_intensity: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            border_width: 10.0,
            border_radius: 25.0,
            border_offset: 0.0,
            panel_metallic: 0.15,
            panel_roughness: 0.35,
            light_orbit_speed: 1.2,
            orbit_lights_enabled: true,
            shape_mode: UShapeMode::Round,
            light_intensity: 2200.0,
        }
    }
}

// ── Components ───────────────────────────────────────────────────────────────
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
struct OrbitingLight {
    angle_offset: f32,
    radius: f32,
    height: f32,
}

#[derive(Component)]
struct MainTargetPanel;

#[derive(Component)]
struct StatusText;

// ── Main Entry ──────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .init_resource::<AppSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_control, orbit_lights, sync_ui_to_settings))
        .run();
}

// ── Setup System ─────────────────────────────────────────────────────────────
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<AppSettings>,
) {
    // 1. Camera with HDR bloom for glowing emissive borders
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.005, 0.005, 0.015)),
            ..default()
        },
        Bloom {
            intensity: 0.25,
            low_frequency_boost: 0.6,
            low_frequency_boost_curvature: 0.5,
            high_pass_frequency: 0.8,
            ..default()
        },
        DistanceFog {
            color: Color::srgb(0.003, 0.003, 0.01),
            directional_light_color: Color::srgb(0.1, 0.2, 0.4),
            directional_light_exponent: 25.0,
            falloff: FogFalloff::Exponential { density: 0.02 },
        },
        MainCamera {
            orbit_distance: 5.5,
            pitch: -0.15,
            yaw: -0.1,
        },
        Transform::from_xyz(0.0, 1.8, 5.5).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
    ));

    // 2. Global Ambient Light (soft blue environment glow)
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.01, 0.02, 0.05),
        brightness: 4.0,
        affects_lightmapped_meshes: true,
    });

    // 3. Directional Light (dim moonlit background highlight)
    commands.spawn((
        DirectionalLight {
            illuminance: 400.0,
            color: Color::srgb(0.2, 0.3, 0.5),
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 4. Orbiting Point Lights (Red, Green, Blue) with glowing visual spheres
    let light_configurations = [
        (
            Color::srgb(1.0, 0.0, 0.05),
            LinearRgba::rgb(8.0, 0.0, 0.4),
            0.0,
            1.3,
        ),
        (
            Color::srgb(0.0, 1.0, 0.2),
            LinearRgba::rgb(0.0, 8.0, 1.6),
            2.094,
            1.8,
        ),
        (
            Color::srgb(0.0, 0.4, 1.0),
            LinearRgba::rgb(0.0, 3.2, 8.0),
            4.188,
            0.6,
        ),
    ];

    let sphere_mesh = meshes.add(Sphere::new(0.05).mesh());

    for (color, emissive_color, angle_offset, height) in light_configurations {
        commands
            .spawn((
                PointLight {
                    color,
                    intensity: settings.light_intensity,
                    range: 10.0,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_xyz(0.0, height, 0.0),
                OrbitingLight {
                    angle_offset,
                    radius: 2.5,
                    height,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d(sphere_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::BLACK,
                        emissive: emissive_color,
                        unlit: true,
                        ..default()
                    })),
                ));
            });
    }

    // 5. Tron-Style Glowing Floor Grid
    let grid_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(0.02, 0.1, 0.25) * 5.0,
        unlit: true,
        ..default()
    });

    for i in -15..=15 {
        let pos = i as f32 * 1.5;
        // X lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(45.0, 0.005, 0.015))),
            MeshMaterial3d(grid_mat.clone()),
            Transform::from_xyz(0.0, 0.0, pos),
        ));
        // Z lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.015, 0.005, 45.0))),
            MeshMaterial3d(grid_mat.clone()),
            Transform::from_xyz(pos, 0.0, 0.0),
        ));
    }

    // 6. Interactive Control UI Panel (World Space 3D UI)
    // Placed on the left, angled to face the camera
    let control_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(700.0, 720.0)),
            Transform::from_xyz(-1.7, 1.25, 0.1).with_rotation(Quat::from_rotation_y(0.35)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let control_panel = commands
        .spawn((
            ChildOf(control_root),
            UPbr {
                metallic: 0.1,
                roughness: 0.4,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(640.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.15, 0.85),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.3),
                width: 3.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
        ))
        .id();

    // Headers
    commands.spawn((
        ChildOf(control_panel),
        UNode::default(),
        UTextLabel {
            text: "◈ BORDER & LIGHTING CONTROL ◈".to_string(),
            color: Color::srgb(0.3, 0.7, 1.0),
            font_size: 24.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(control_panel),
        StatusText,
        UNode {
            margin: USides::bottom(10.0),
            ..default()
        },
        UTextLabel {
            text: "".to_string(),
            color: Color::srgb(0.7, 0.85, 0.95),
            font_size: 15.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(control_panel),
        UNode {
            height: UVal::Px(2.0),
            background_color: Color::srgba(0.2, 0.5, 1.0, 0.25),
            margin: USides::bottom(8.0),
            ..default()
        },
    ));

    // Helper closure to build steppers
    let mut add_stepper = |parent_entity: Entity,
                           label: &'static str,
                           on_dec: fn(&mut AppSettings),
                           on_inc: fn(&mut AppSettings)| {
        let row = commands
            .spawn((
                ChildOf(parent_entity),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(row),
            UNode {
                width: UVal::Px(240.0),
                ..default()
            },
            UTextLabel {
                text: label.to_string(),
                color: Color::WHITE,
                font_size: 14.0,
                ..default()
            },
        ));

        let btn_group = commands
            .spawn((
                ChildOf(row),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    ..default()
                },
            ))
            .id();

        // Minus button
        commands
            .spawn((
                ChildOf(btn_group),
                UNode {
                    padding: USides::axes(14.0, 6.0),
                    background_color: Color::srgb(0.2, 0.25, 0.35),
                    border_radius: UCornerRadius::all(6.0),
                    ..default()
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: Color::srgb(0.2, 0.25, 0.35),
                    hovered: Color::srgb(0.3, 0.35, 0.45),
                    pressed: Color::srgb(0.15, 0.2, 0.28),
                },
                ULayout {
                    display: UDisplay::Flex,
                    ..default()
                },
            ))
            .observe(
                move |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                    on_dec(&mut settings);
                },
            )
            .with_children(|btn| {
                btn.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: " - ".to_string(),
                        color: Color::WHITE,
                        font_size: 14.0,
                        ..default()
                    },
                ));
            });

        // Plus button
        commands
            .spawn((
                ChildOf(btn_group),
                UNode {
                    padding: USides::axes(14.0, 6.0),
                    background_color: Color::srgb(0.2, 0.5, 0.8),
                    border_radius: UCornerRadius::all(6.0),
                    ..default()
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: Color::srgb(0.2, 0.5, 0.8),
                    hovered: Color::srgb(0.3, 0.6, 0.9),
                    pressed: Color::srgb(0.15, 0.4, 0.7),
                },
                ULayout {
                    display: UDisplay::Flex,
                    ..default()
                },
            ))
            .observe(
                move |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                    on_inc(&mut settings);
                },
            )
            .with_children(|btn| {
                btn.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: " + ".to_string(),
                        color: Color::WHITE,
                        font_size: 14.0,
                        ..default()
                    },
                ));
            });
    };

    add_stepper(
        control_panel,
        "Border Width:",
        |s| s.border_width = (s.border_width - 2.0).clamp(0.0, 24.0),
        |s| s.border_width = (s.border_width + 2.0).clamp(0.0, 24.0),
    );

    add_stepper(
        control_panel,
        "Border Radius:",
        |s| s.border_radius = (s.border_radius - 5.0).clamp(0.0, 60.0),
        |s| s.border_radius = (s.border_radius + 5.0).clamp(0.0, 60.0),
    );

    add_stepper(
        control_panel,
        "Border Offset:",
        |s| s.border_offset = (s.border_offset - 2.0).clamp(-15.0, 25.0),
        |s| s.border_offset = (s.border_offset + 2.0).clamp(-15.0, 25.0),
    );

    add_stepper(
        control_panel,
        "Panel Metallic:",
        |s| s.panel_metallic = ((s.panel_metallic - 0.1) * 10.0).round() / 10.0,
        |s| s.panel_metallic = ((s.panel_metallic + 0.1) * 10.0).round() / 10.0,
    );

    add_stepper(
        control_panel,
        "Panel Roughness:",
        |s| s.panel_roughness = ((s.panel_roughness - 0.1) * 10.0).round() / 10.0,
        |s| s.panel_roughness = ((s.panel_roughness + 0.1) * 10.0).round() / 10.0,
    );

    add_stepper(
        control_panel,
        "Light Intensity:",
        |s| s.light_intensity = (s.light_intensity - 500.0).clamp(500.0, 5000.0),
        |s| s.light_intensity = (s.light_intensity + 500.0).clamp(500.0, 5000.0),
    );

    add_stepper(
        control_panel,
        "Light Orbit Speed:",
        |s| s.light_orbit_speed = (s.light_orbit_speed - 0.2).clamp(0.0, 4.0),
        |s| s.light_orbit_speed = (s.light_orbit_speed + 0.2).clamp(0.0, 4.0),
    );

    // Toggles Row
    let toggles_row = commands
        .spawn((
            ChildOf(control_panel),
            UNode {
                margin: USides::top(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceAround,
                ..default()
            },
        ))
        .id();

    // Orbit Toggle Button
    commands
        .spawn((
            ChildOf(toggles_row),
            UNode {
                padding: USides::axes(16.0, 8.0),
                background_color: Color::srgb(0.2, 0.35, 0.5),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.2, 0.35, 0.5),
                hovered: Color::srgb(0.3, 0.45, 0.6),
                pressed: Color::srgb(0.15, 0.25, 0.4),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.orbit_lights_enabled = !settings.orbit_lights_enabled;
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Toggle Orbit".to_string(),
                    color: Color::WHITE,
                    font_size: 13.0,
                    ..default()
                },
            ));
        });

    // Shape Toggle Button
    commands
        .spawn((
            ChildOf(toggles_row),
            UNode {
                padding: USides::axes(16.0, 8.0),
                background_color: Color::srgb(0.2, 0.35, 0.5),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.2, 0.35, 0.5),
                hovered: Color::srgb(0.3, 0.45, 0.6),
                pressed: Color::srgb(0.15, 0.25, 0.4),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.shape_mode = match settings.shape_mode {
                    UShapeMode::Round => UShapeMode::Cut,
                    UShapeMode::Cut => UShapeMode::Round,
                };
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Toggle Shape (Round/Cut)".to_string(),
                    color: Color::WHITE,
                    font_size: 13.0,
                    ..default()
                },
            ));
        });

    // 7. Dynamic Target Panel (Main Showcase)
    // Located in the center, responding to settings
    let target_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(500.0, 500.0)),
            Transform::from_xyz(0.2, 1.25, -0.2).with_rotation(Quat::from_rotation_y(-0.15)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let target_panel = commands
        .spawn((
            ChildOf(target_root),
            MainTargetPanel,
            UPbr {
                metallic: settings.panel_metallic,
                roughness: settings.panel_roughness,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(400.0),
                height: UVal::Px(400.0),
                padding: USides::all(30.0),
                background_color: Color::srgba(0.01, 0.02, 0.05, 0.6),
                border_radius: UCornerRadius::all(settings.border_radius),
                shape_mode: settings.shape_mode,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 20.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(0.0, 1.5, 3.0),
                width: settings.border_width,
                radius: UCornerRadius::all(settings.border_radius),
                offset: settings.border_offset,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(target_panel),
        UNode::default(),
        UTextLabel {
            text: "TARGET PANEL".to_string(),
            color: Color::WHITE,
            font_size: 30.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(target_panel),
        UNode::default(),
        UTextLabel {
            text: "Interactive SDF Border\nReacts physically to lights".to_string(),
            color: Color::srgb(0.7, 0.8, 0.9),
            font_size: 16.0,
            ..default()
        },
    ));

    // 8. Preset Showcase 1: Cut Corners (Amber Emissive)
    let preset1_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(320.0, 240.0)),
            Transform::from_xyz(1.8, 2.05, -0.6).with_rotation(Quat::from_rotation_y(-0.45)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let preset1_panel = commands
        .spawn((
            ChildOf(preset1_root),
            UPbr {
                metallic: 0.1,
                roughness: 0.5,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(280.0),
                height: UVal::Px(200.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.05, 0.03, 0.01, 0.75),
                border_radius: UCornerRadius::all(20.0),
                shape_mode: UShapeMode::Cut, // Cut Corner Mode
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(3.0, 1.2, 0.0), // Glowing Amber
                width: 8.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preset1_panel),
        UNode::default(),
        UTextLabel {
            text: "CHAMFERED BORDER".to_string(),
            color: Color::srgb(1.0, 0.6, 0.1),
            font_size: 16.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preset1_panel),
        UNode::default(),
        UTextLabel {
            text: "UShapeMode::Cut\nAmber Emissive Glow".to_string(),
            color: Color::srgb(0.8, 0.7, 0.6),
            font_size: 12.0,
            ..default()
        },
    ));

    // 9. Preset Showcase 2: Offset Ring (Cyan Border Float)
    let preset2_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(320.0, 240.0)),
            Transform::from_xyz(1.8, 0.55, -0.6).with_rotation(Quat::from_rotation_y(-0.45)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let preset2_panel = commands
        .spawn((
            ChildOf(preset2_root),
            UPbr {
                metallic: 0.2,
                roughness: 0.6,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(280.0),
                height: UVal::Px(200.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.01, 0.04, 0.04, 0.7),
                border_radius: UCornerRadius::all(15.0),
                shape_mode: UShapeMode::Round,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(0.0, 2.5, 2.5), // Neon Cyan
                width: 4.0,
                radius: UCornerRadius::all(20.0), // Rounded border (larger radius to fit offset)
                offset: 14.0,                     // Floating Offset!
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preset2_panel),
        UNode::default(),
        UTextLabel {
            text: "OFFSET FLOATING".to_string(),
            color: Color::srgb(0.2, 0.9, 0.9),
            font_size: 16.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preset2_panel),
        UNode::default(),
        UTextLabel {
            text: "UBorder::offset: 14.0\nFloating Cyan Halo".to_string(),
            color: Color::srgb(0.6, 0.8, 0.8),
            font_size: 12.0,
            ..default()
        },
    ));

    // 10. Preset Showcase 3: Metallic Mirror (Hot Pink Glow)
    let preset3_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(320.0, 480.0)),
            Transform::from_xyz(-1.8, 1.25, -1.2).with_rotation(Quat::from_rotation_y(0.5)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let preset3_panel = commands
        .spawn((
            ChildOf(preset3_root),
            UPbr {
                metallic: 1.0,   // Full mirror metallic
                roughness: 0.05, // Very smooth/reflective
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(280.0),
                height: UVal::Px(420.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.03, 0.01, 0.03, 0.35),
                border_radius: UCornerRadius::all(25.0),
                shape_mode: UShapeMode::Round,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 24.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(3.0, 0.0, 2.0), // Glowing Hot Pink
                width: 7.0,
                radius: UCornerRadius::all(25.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preset3_panel),
        UNode::default(),
        UTextLabel {
            text: "HIGH METALLIC".to_string(),
            color: Color::srgb(1.0, 0.2, 0.8),
            font_size: 18.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preset3_panel),
        UNode::default(),
        UTextLabel {
            text: "UPbr metallic: 1.0\nUPbr roughness: 0.05\n\nActs as a mirror surface\ncatching sharp specular\nhighlights from the\norbiting colored point\nlights dynamically.".to_string(),
            color: Color::srgb(0.9, 0.7, 0.85),
            font_size: 13.0,
            ..default()
        },
    ));

    // 11. Preset Showcase 4: Small Rounded Corners (Green/Emerald Border)
    let preset4_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(280.0, 200.0)),
            Transform::from_xyz(3.0, 1.25, -1.8).with_rotation(Quat::from_rotation_y(-0.6)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let preset4_panel = commands
        .spawn((
            ChildOf(preset4_root),
            UPbr {
                metallic: 0.1,
                roughness: 0.4,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(240.0),
                height: UVal::Px(160.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.01, 0.05, 0.02, 0.8),
                border_radius: UCornerRadius::all(6.0), // Small corner radius!
                shape_mode: UShapeMode::Round,          // Rounded
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(0.0, 3.0, 0.8), // Glowing Green
                width: 4.0,
                radius: UCornerRadius::all(6.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preset4_panel),
        UNode::default(),
        UTextLabel {
            text: "SMALL ROUNDED".to_string(),
            color: Color::srgb(0.2, 1.0, 0.5),
            font_size: 15.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preset4_panel),
        UNode::default(),
        UTextLabel {
            text: "Radius: 6.0px (Round)\nTech terminal look".to_string(),
            color: Color::srgb(0.7, 0.9, 0.8),
            font_size: 12.0,
            ..default()
        },
    ));

    // 12. Preset Showcase 5: Large Cut Chamfer (Magenta Emissive)
    let preset5_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(280.0, 200.0)),
            Transform::from_xyz(-3.0, 1.25, -1.8).with_rotation(Quat::from_rotation_y(0.6)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    let preset5_panel = commands
        .spawn((
            ChildOf(preset5_root),
            UPbr {
                metallic: 0.3,
                roughness: 0.3,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(240.0),
                height: UVal::Px(160.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.05, 0.01, 0.05, 0.75),
                border_radius: UCornerRadius::all(45.0), // Very large corner radius!
                shape_mode: UShapeMode::Cut,             // Cut corners
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(3.0, 0.0, 3.0), // Glowing Magenta
                width: 6.0,
                radius: UCornerRadius::all(45.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preset5_panel),
        UNode::default(),
        UTextLabel {
            text: "LARGE CHAMFERED".to_string(),
            color: Color::srgb(1.0, 0.2, 1.0),
            font_size: 15.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preset5_panel),
        UNode::default(),
        UTextLabel {
            text: "Radius: 45.0px (Cut)\nHUD shield layout".to_string(),
            color: Color::srgb(0.9, 0.7, 0.9),
            font_size: 12.0,
            ..default()
        },
    ));

    // 13. Hint Text at the bottom
    let hint_root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(800.0, 100.0)),
            Transform::from_xyz(0.0, 0.25, 1.2),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
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

    commands.spawn((
        ChildOf(hint_root),
        UNode::default(),
        UTextLabel {
            text: "Drag Right Mouse Button to Rotate Scene Camera | Scroll to Zoom".to_string(),
            color: Color::srgb(0.5, 0.8, 0.6),
            font_size: 15.0,
            ..default()
        },
    ));
}

// ── Orbit Lights System ──────────────────────────────────────────────────────
fn orbit_lights(
    time: Res<Time>,
    settings: Res<AppSettings>,
    mut query: Query<(&mut Transform, &OrbitingLight)>,
) {
    if !settings.orbit_lights_enabled {
        return;
    }
    let t = time.elapsed_secs() * settings.light_orbit_speed;
    for (mut transform, light) in &mut query {
        let angle = t + light.angle_offset;
        let x = angle.cos() * light.radius;
        let z = angle.sin() * light.radius;
        transform.translation = Vec3::new(x, light.height, z);
    }
}

// ── Sync UI To Settings System ───────────────────────────────────────────────
fn sync_ui_to_settings(
    settings: Res<AppSettings>,
    mut target_panel_query: Query<(&mut UBorder, &mut UPbr, &mut UNode), With<MainTargetPanel>>,
    mut point_light_query: Query<&mut PointLight, With<OrbitingLight>>,
    mut status_text_query: Query<&mut UTextLabel, With<StatusText>>,
) {
    // Apply metallic, roughness, shape mode, and border changes to the main target panel
    for (mut border, mut pbr, mut node) in &mut target_panel_query {
        border.width = settings.border_width;
        border.radius = UCornerRadius::all(settings.border_radius);
        border.offset = settings.border_offset;

        pbr.metallic = settings.panel_metallic;
        pbr.roughness = settings.panel_roughness;

        node.border_radius = UCornerRadius::all(settings.border_radius);
        node.shape_mode = settings.shape_mode;
    }

    // Update point light intensities
    for mut light in &mut point_light_query {
        light.intensity = settings.light_intensity;
    }

    // Update status text on the control panel
    for mut label in &mut status_text_query {
        label.text = format!(
            "Width: {:.1}px | Radius: {:.1}px | Offset: {:.1}px\nMetallic: {:.2} | Roughness: {:.2}\nLight Intensity: {:.0} lm | Corner Shape: {:?}",
            settings.border_width,
            settings.border_radius,
            settings.border_offset,
            settings.panel_metallic,
            settings.panel_roughness,
            settings.light_intensity,
            settings.shape_mode
        );
    }
}

// ── Camera Control System ────────────────────────────────────────────────────
fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    // Zoom control
    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.2;
        camera.orbit_distance = camera.orbit_distance.clamp(2.5, 12.0);
    }

    // Orbit control (Right mouse button drag)
    if mouse_button.pressed(MouseButton::Right) {
        let delta = motion.delta;
        if delta.length_squared() > 0.0 {
            let sensitivity = 0.005;
            camera.yaw -= delta.x * sensitivity;
            camera.pitch -= delta.y * sensitivity;
            camera.pitch = camera.pitch.clamp(-1.4, 1.4);
        }
    }

    // Recalculate camera transform around target
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let target = Vec3::new(0.0, 1.2, 0.0);
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
