//! # Sci-Fi Cityscape Showcase
//!
//! A cyberpunk-style 3D scene featuring skyscraper-like cube buildings,
//! each surrounded by multiple holographic UI screens of varying sizes on all
//! four sides. The screens glow and illuminate the buildings with emissive light.
//!
//! **Controls:**
//! - Right-click drag: orbit camera
//! - Scroll: zoom in/out
//! - Middle-click drag: pan camera

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use univis_ui::prelude::*;

// ── Camera ──────────────────────────────────────────────────────────────────
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
    target: Vec3,
}

// ── Building marker ─────────────────────────────────────────────────────────
#[derive(Component)]
#[allow(dead_code)]
struct Building {
    index: usize,
}

// ── Animated UI markers ─────────────────────────────────────────────────────
#[derive(Component)]
struct AnimatedProgress {
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct AnimatedStatusText {
    building_index: usize,
}

#[derive(Component)]
struct DataStreamText {
    speed: f32,
    counter: u32,
}

#[derive(Component)]
struct FloatingPanel {
    base_pos: Vec3,
    amplitude: f32,
    speed: f32,
    phase: f32,
}

// ── Building data ───────────────────────────────────────────────────────────
struct BuildingConfig {
    pos: Vec3,
    size: Vec3,
    color: Color,
    emissive_color: LinearRgba,
    name: &'static str,
    status_lines: &'static [&'static str],
}

const BUILDINGS: &[BuildingConfig] = &[
    BuildingConfig {
        pos: Vec3::new(0.0, 0.0, 0.0),
        size: Vec3::new(2.5, 8.0, 2.5),
        color: Color::srgb(0.06, 0.08, 0.12),
        emissive_color: LinearRgba::rgb(0.0, 0.6, 1.0),
        name: "NEXUS TOWER",
        status_lines: &["SYSTEM: ONLINE", "CORES: 128 ACTIVE", "NET: 94.2 TB/s"],
    },
    BuildingConfig {
        pos: Vec3::new(5.0, 0.0, -2.0),
        size: Vec3::new(2.0, 12.0, 2.0),
        color: Color::srgb(0.07, 0.06, 0.1),
        emissive_color: LinearRgba::rgb(0.8, 0.0, 0.6),
        name: "SYNTH SPIRE",
        status_lines: &["PLASMA: STABLE", "REACTOR: 99.7%", "SHIELD: ACTIVE"],
    },
    BuildingConfig {
        pos: Vec3::new(-5.5, 0.0, 1.0),
        size: Vec3::new(3.0, 6.0, 3.0),
        color: Color::srgb(0.05, 0.09, 0.08),
        emissive_color: LinearRgba::rgb(0.0, 1.0, 0.5),
        name: "BIO COMPLEX",
        status_lines: &["DNA SEQ: RUNNING", "GROWTH: +12.3%", "TEMP: 36.8°C"],
    },
    BuildingConfig {
        pos: Vec3::new(-2.0, 0.0, -6.0),
        size: Vec3::new(2.2, 10.0, 2.2),
        color: Color::srgb(0.08, 0.06, 0.06),
        emissive_color: LinearRgba::rgb(1.0, 0.3, 0.0),
        name: "DATACORE",
        status_lines: &["MINING: BLOCK #9842", "HASH: 847 PH/s", "UPLINK: SECURE"],
    },
    BuildingConfig {
        pos: Vec3::new(3.5, 0.0, 5.0),
        size: Vec3::new(2.8, 7.0, 2.8),
        color: Color::srgb(0.06, 0.06, 0.1),
        emissive_color: LinearRgba::rgb(0.3, 0.3, 1.0),
        name: "QUANTUM LAB",
        status_lines: &["QUBITS: 4096", "COHERENCE: 98.1%", "ENTANGLE: OK"],
    },
];

// ── Screen placement descriptor ─────────────────────────────────────────────
/// Describes a single holographic screen attached to one face of a building.
struct ScreenPlacement {
    /// Vertical position as a fraction of building height (0.0 = bottom, 1.0 = top)
    y_fraction: f32,
    /// Horizontal offset as a fraction of building face width (-0.5..0.5)
    x_offset_fraction: f32,
    /// Outward offset from the building face (meters)
    z_offset: f32,
    /// Width as a fraction of the face width
    width_fraction: f32,
    /// Height as a fraction of the building height
    height_fraction: f32,
    /// Target width in pixels for internal UI rendering
    pixel_width: f32,
    /// What content to show
    content: ScreenContent,
}

#[derive(Clone, Copy)]
enum ScreenContent {
    /// Building name + status lines + animated status
    MainStatus,
    /// Progress bars with labels
    Metrics,
    /// Scrolling hex data stream
    DataStream,
    /// Large title only
    TitleBanner,
    /// Small status indicator
    StatusIndicator,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                camera_control,
                animate_progress_bars,
                animate_status_texts,
                animate_data_streams,
                animate_floating_panels,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Camera with bloom and fog ───────────────────────────────────────
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.02)),
            ..default()
        },
        Bloom {
            intensity: 0.2,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.5,
            high_pass_frequency: 0.8,
            ..default()
        },
        DistanceFog {
            color: Color::srgb(0.01, 0.02, 0.05),
            directional_light_color: Color::srgb(0.1, 0.2, 0.4),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::Exponential { density: 0.015 },
        },
        MainCamera {
            orbit_distance: 25.0,
            pitch: -0.25,
            yaw: 0.6,
            target: Vec3::new(0.0, 4.0, 0.0),
        },
        Transform::from_xyz(10.0, 12.0, 20.0).looking_at(Vec3::new(0.0, 4.0, 0.0), Vec3::Y),
    ));

    // ── Ambient light ───────────────────────────────────────────────────
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.02, 0.03, 0.08), // Darker
        brightness: 10.0,                     // Lower brightness for more contrast
        affects_lightmapped_meshes: true,
    });

    // ── Main directional light (dim moonlight) ──────────────────────────
    commands.spawn((
        DirectionalLight {
            illuminance: 300.0,
            color: Color::srgb(0.2, 0.3, 0.6),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ── Atmospheric point lights ────────────────────────────────────────
    let light_positions = [
        (Vec3::new(0.0, 3.0, 4.0), Color::srgb(0.0, 0.4, 1.0), 3000.0),
        (
            Vec3::new(6.0, 5.0, -3.0),
            Color::srgb(0.8, 0.0, 0.5),
            2000.0,
        ),
        (
            Vec3::new(-6.0, 2.0, 2.0),
            Color::srgb(0.0, 1.0, 0.4),
            2500.0,
        ),
        (
            Vec3::new(-2.0, 7.0, -7.0),
            Color::srgb(1.0, 0.3, 0.0),
            2000.0,
        ),
        (Vec3::new(4.0, 4.0, 6.0), Color::srgb(0.3, 0.3, 1.0), 1800.0),
    ];

    for (pos, color, intensity) in light_positions {
        commands.spawn((
            PointLight {
                color,
                intensity,
                range: 40.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(pos),
        ));
    }

    // ── Background Data Particles (Stars) ───────────────────────────────
    let particle_mesh = meshes.add(Cuboid::new(0.05, 0.05, 0.05));
    let particle_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(0.5, 1.0, 2.0) * 10.0, // Extremely bright cyan/white
        unlit: true,
        ..default()
    });

    for i in 0..300 {
        // Deterministic pseudo-random positions
        let x = (i as f32 * 13.4).sin() * 50.0;
        let y = ((i as f32 * 29.1).cos() * 30.0).abs() + 2.0;
        let z = (i as f32 * 17.7).sin() * 50.0;

        // Keep them outside the main building cluster
        if x.abs() < 15.0 && z.abs() < 15.0 {
            continue;
        }

        commands.spawn((
            Mesh3d(particle_mesh.clone()),
            MeshMaterial3d(particle_mat.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }

    // ── Ground plane ────────────────────────────────────────────────────
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.09, 0.14),
            metallic: 0.15, // Low metallic, mostly dielectric for a distinct floor surface
            perceptual_roughness: 0.5, // Satin/matte finish to catch soft light pools
            reflectance: 0.5,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ── Tron-style glowing grid ──────────────────────────────────────────
    let grid_mat_thick = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(0.0, 0.4, 1.0) * 3.0,
        unlit: true,
        ..default()
    });
    let grid_mat_thin = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(0.0, 0.25, 0.8) * 1.0,
        unlit: true,
        ..default()
    });

    for i in -20..=20 {
        let is_thick = i % 5 == 0;
        let thickness = if is_thick { 0.04 } else { 0.015 };
        let mat = if is_thick {
            grid_mat_thick.clone()
        } else {
            grid_mat_thin.clone()
        };
        let pos = i as f32 * 2.0;

        // X lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(80.0, 0.01, thickness))),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(0.0, 0.005, pos),
        ));
        // Z lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(thickness, 0.01, 80.0))),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(pos, 0.005, 0.0),
        ));
    }

    // ── Spawn buildings and their holographic screens ───────────────────
    for (i, bldg) in BUILDINGS.iter().enumerate() {
        spawn_building(&mut commands, &mut meshes, &mut materials, bldg, i);
    }
}

// ── Building spawner ────────────────────────────────────────────────────────

fn spawn_building(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &BuildingConfig,
    index: usize,
) {
    let half_h = config.size.y / 2.0;

    // ── Building mesh ───────────────────────────────────────────────────
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(config.size.x, config.size.y, config.size.z))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: config.color,
            metallic: 0.7,
            perceptual_roughness: 0.2,
            reflectance: 0.6,
            ..default()
        })),
        Transform::from_xyz(config.pos.x, half_h, config.pos.z),
        Building { index },
    ));

    // ── Emissive accent strips on building ──────────────────────────────
    let strip_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: config.emissive_color * 3.0,
        unlit: true,
        ..default()
    });

    let strip_count = (config.size.y / 1.5).floor() as i32;
    for s in 0..strip_count {
        let y = (s as f32 + 0.5) * 1.5;
        if y > config.size.y - 0.3 {
            break;
        }
        for z_sign in [-1.0f32, 1.0] {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(config.size.x * 0.95, 0.03, 0.01))),
                MeshMaterial3d(strip_mat.clone()),
                Transform::from_xyz(
                    config.pos.x,
                    y,
                    config.pos.z + z_sign * (config.size.z / 2.0 + 0.02),
                ),
            ));
        }
        for x_sign in [-1.0f32, 1.0] {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.01, 0.03, config.size.z * 0.95))),
                MeshMaterial3d(strip_mat.clone()),
                Transform::from_xyz(
                    config.pos.x + x_sign * (config.size.x / 2.0 + 0.02),
                    y,
                    config.pos.z,
                ),
            ));
        }
    }

    // ── Multiple holographic screens on all 4 sides ─────────────────────
    let gap = 0.4;

    // Define screen layouts for each face.
    // Each face gets 2-3 screens of different sizes at different positions.

    // Face rotations: (axis direction to offset, rotation, face_width_axis)
    let faces: [(Vec3, Quat, f32); 4] = [
        // Front (+Z): offset along Z, no rotation, width = X
        (
            Vec3::new(0.0, 0.0, config.size.z / 2.0 + gap),
            Quat::IDENTITY,
            config.size.x,
        ),
        // Back (-Z)
        (
            Vec3::new(0.0, 0.0, -(config.size.z / 2.0 + gap)),
            Quat::from_rotation_y(std::f32::consts::PI),
            config.size.x,
        ),
        // Right (+X)
        (
            Vec3::new(config.size.x / 2.0 + gap, 0.0, 0.0),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            config.size.z,
        ),
        // Left (-X)
        (
            Vec3::new(-(config.size.x / 2.0 + gap), 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            config.size.z,
        ),
    ];

    // Screen placements vary per face to create visual variety
    let face_screen_sets: [&[ScreenPlacement]; 4] = [
        // Front face: Main status and smaller indicators
        &[
            ScreenPlacement {
                y_fraction: 0.6,
                x_offset_fraction: 0.0,
                z_offset: 0.0,
                width_fraction: 0.85,
                height_fraction: 0.5,
                pixel_width: 1200.0,
                content: ScreenContent::MainStatus,
            },
            ScreenPlacement {
                y_fraction: 0.2,
                x_offset_fraction: -0.25,
                z_offset: 0.5,
                width_fraction: 0.4,
                height_fraction: 0.15,
                pixel_width: 600.0,
                content: ScreenContent::TitleBanner,
            },
            ScreenPlacement {
                y_fraction: 0.15,
                x_offset_fraction: 0.3,
                z_offset: 0.3,
                width_fraction: 0.35,
                height_fraction: 0.25,
                pixel_width: 500.0,
                content: ScreenContent::StatusIndicator,
            },
        ],
        // Back face: Metrics and Data
        &[
            ScreenPlacement {
                y_fraction: 0.5,
                x_offset_fraction: 0.0,
                z_offset: 0.2,
                width_fraction: 0.9,
                height_fraction: 0.6,
                pixel_width: 1000.0,
                content: ScreenContent::Metrics,
            },
            ScreenPlacement {
                y_fraction: 0.85,
                x_offset_fraction: 0.0,
                z_offset: 0.8,
                width_fraction: 0.95,
                height_fraction: 0.15,
                pixel_width: 1200.0,
                content: ScreenContent::TitleBanner,
            },
        ],
        // Right face: Data stream
        &[ScreenPlacement {
            y_fraction: 0.5,
            x_offset_fraction: 0.0,
            z_offset: 0.1,
            width_fraction: 0.9,
            height_fraction: 0.8,
            pixel_width: 1000.0,
            content: ScreenContent::DataStream,
        }],
        // Left face: Floating layered screens
        &[
            ScreenPlacement {
                y_fraction: 0.4,
                x_offset_fraction: -0.1,
                z_offset: 0.1,
                width_fraction: 0.7,
                height_fraction: 0.4,
                pixel_width: 800.0,
                content: ScreenContent::MainStatus,
            },
            ScreenPlacement {
                y_fraction: 0.7,
                x_offset_fraction: 0.2,
                z_offset: 0.6,
                width_fraction: 0.6,
                height_fraction: 0.3,
                pixel_width: 600.0,
                content: ScreenContent::StatusIndicator,
            },
            ScreenPlacement {
                y_fraction: 0.2,
                x_offset_fraction: 0.1,
                z_offset: 1.0,
                width_fraction: 0.8,
                height_fraction: 0.2,
                pixel_width: 800.0,
                content: ScreenContent::TitleBanner,
            },
        ],
    ];

    for (face_idx, (face_offset, rotation, face_width)) in faces.iter().enumerate() {
        let screens = face_screen_sets[face_idx];

        for screen in screens {
            let screen_y = screen.y_fraction * config.size.y;
            let screen_pos = config.pos + *face_offset + Vec3::new(0.0, screen_y, 0.0);

            // Apply horizontal offset along the screen's local X, and push outward along local Z
            let local_right = *rotation * Vec3::X;
            let local_forward = *rotation * Vec3::Z;
            let final_pos = screen_pos
                + local_right * (screen.x_offset_fraction * *face_width)
                + local_forward * screen.z_offset;

            spawn_holographic_screen(
                commands,
                final_pos,
                *rotation,
                *face_width,
                config,
                index,
                screen,
            );
        }
    }

    // ── Point light near building top to simulate screen glow ────────────
    commands.spawn((
        PointLight {
            color: Color::srgb(
                config.emissive_color.red.min(1.0),
                config.emissive_color.green.min(1.0),
                config.emissive_color.blue.min(1.0),
            ),
            intensity: 5000.0,
            range: config.size.y * 2.5,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(config.pos.x, config.size.y * 0.8, config.pos.z),
    ));
}

// ── Holographic screen spawner ──────────────────────────────────────────────

fn spawn_holographic_screen(
    commands: &mut Commands,
    position: Vec3,
    rotation: Quat,
    face_width: f32,
    config: &BuildingConfig,
    bldg_index: usize,
    screen: &ScreenPlacement,
) {
    // High glow for main titles (high intensity HDR)
    let high_glow = Color::from(config.emissive_color * 12.0);

    // Medium glow for subtitles and secondary headers
    let medium_glow = Color::from(config.emissive_color * 3.5);

    // Calculate physical dimensions and pixel resolutions
    let physical_w = face_width * screen.width_fraction;
    let physical_h = config.size.y * screen.height_fraction;

    // We want the width in pixels to be large enough for our big fonts
    let pixel_w = screen.pixel_width;
    let pixel_h = pixel_w * (physical_h / physical_w);
    let pixel_size = Vec2::new(pixel_w, pixel_h);

    let meters_per_unit = physical_w / pixel_w;

    // We add a slight random phase to each floating panel based on its position
    let phase = position.x * 2.0 + position.y * 3.0;

    // Root UI in world 3D space
    let root = commands
        .spawn((
            URootUi {
                meters_per_unit,
                ..URootUi::world_3d(pixel_size)
            },
            Transform::from_translation(position).with_rotation(rotation),
            FloatingPanel {
                base_pos: position,
                amplitude: 0.1 + (bldg_index as f32 * 0.02),
                speed: 1.5 + (bldg_index as f32 * 0.1),
                phase,
            },
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

    // Holographic panel — semi-transparent background, thick glowing border
    let panel = commands
        .spawn((
            ChildOf(root),
            UPbr {
                metallic: 0.05,
                roughness: 0.2,
                emissive: config.emissive_color * 5.0, // Stronger Neon
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(60.0),
                background_color: Color::srgba(0.01, 0.02, 0.06, 0.25), // More transparent
                border_radius: UCornerRadius::all(40.0),
                ..default()
            },
            UBorder {
                color: Color::from(config.emissive_color * 2.0),
                width: 12.0, // Thicker border
                radius: UCornerRadius::all(40.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 40.0,
                ..default()
            },
        ))
        .id();

    // Populate content
    match screen.content {
        ScreenContent::MainStatus => {
            spawn_main_status(commands, panel, config, bldg_index, high_glow, medium_glow);
        }
        ScreenContent::Metrics => {
            spawn_metrics(commands, panel, config, bldg_index, high_glow, medium_glow);
        }
        ScreenContent::DataStream => {
            spawn_data_stream(commands, panel, config, bldg_index, high_glow, medium_glow);
        }
        ScreenContent::TitleBanner => {
            spawn_title_banner(commands, panel, config, high_glow, medium_glow);
        }
        ScreenContent::StatusIndicator => {
            spawn_status_indicator(commands, panel, config, bldg_index, high_glow, medium_glow);
        }
    }
}

// ── SCREEN CONTENT: Main Status ─────────────────────────────────────────────

fn spawn_main_status(
    commands: &mut Commands,
    parent: Entity,
    config: &BuildingConfig,
    bldg_index: usize,
    high_glow: Color,
    medium_glow: Color,
) {
    // Header
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: format!("◈ {} ◈", config.name),
            font_size: 110.0,
            color: high_glow, // High glow for main title
            ..default()
        },
    ));

    // Divider
    commands.spawn((
        ChildOf(parent),
        UNode {
            height: UVal::Px(10.0),
            background_color: medium_glow, // Medium glow for divider line
            border_radius: UCornerRadius::all(5.0),
            ..default()
        },
    ));

    // Status lines — large readable text (subtitles/details)
    for line in config.status_lines {
        commands.spawn((
            ChildOf(parent),
            UNode {
                background_color: Color::NONE,
                ..default()
            },
            UTextLabel {
                text: line.to_string(),
                font_size: 70.0,
                color: medium_glow, // Medium glow for subtitles
                ..default()
            },
        ));
    }

    // Animated cycling status text
    commands.spawn((
        ChildOf(parent),
        AnimatedStatusText {
            building_index: bldg_index,
        },
        UNode {
            background_color: Color::srgba(0.0, 0.08, 0.16, 0.5),
            padding: USides::axes(30.0, 20.0),
            border_radius: UCornerRadius::all(15.0),
            ..default()
        },
        UTextLabel {
            text: "▶ INITIALIZING...".to_string(),
            font_size: 65.0,
            color: Color::from(LinearRgba::rgb(1.5, 2.85, 2.1)), // High glow green animated indicator
            ..default()
        },
    ));

    // Progress bar label
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: "POWER OUTPUT".to_string(),
            font_size: 55.0,
            color: medium_glow, // Medium glow
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(parent),
        AnimatedProgress {
            speed: 0.8 + bldg_index as f32 * 0.3,
            phase: bldg_index as f32 * 1.2,
        },
        UNode {
            height: UVal::Px(80.0), // make progress bar thicker
            ..default()
        },
        UProgressBar {
            value: 0.6,
            ..default()
        },
    ));
}

// ── SCREEN CONTENT: Metrics ─────────────────────────────────────────────────

fn spawn_metrics(
    commands: &mut Commands,
    parent: Entity,
    _config: &BuildingConfig,
    bldg_index: usize,
    high_glow: Color,
    medium_glow: Color,
) {
    // Header
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: "◆ SYSTEM METRICS ◆".to_string(),
            font_size: 90.0,
            color: high_glow, // High glow
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(parent),
        UNode {
            height: UVal::Px(10.0),
            background_color: medium_glow, // Medium glow
            border_radius: UCornerRadius::all(5.0),
            ..default()
        },
    ));

    // Metric progress bars with large labels
    let metrics = [
        ("CPU LOAD", 0.4, 0.7),
        ("MEM USAGE", 0.6, 0.5),
        ("NET I/O", 0.3, 0.9),
        ("DISK WRITE", 0.8, 1.1),
    ];

    for (label, phase_offset, speed) in metrics {
        commands.spawn((
            ChildOf(parent),
            UNode {
                background_color: Color::NONE,
                ..default()
            },
            UTextLabel {
                text: label.to_string(),
                font_size: 60.0,
                color: medium_glow, // Medium glow for subtitles
                ..default()
            },
        ));
        commands.spawn((
            ChildOf(parent),
            AnimatedProgress {
                speed: speed + bldg_index as f32 * 0.1,
                phase: phase_offset + bldg_index as f32 * 0.5,
            },
            UNode {
                height: UVal::Px(60.0), // make progress bar thicker
                ..default()
            },
            UProgressBar {
                value: 0.5,
                ..default()
            },
        ));
    }
}

// ── SCREEN CONTENT: Data Stream ─────────────────────────────────────────────

fn spawn_data_stream(
    commands: &mut Commands,
    parent: Entity,
    _config: &BuildingConfig,
    bldg_index: usize,
    high_glow: Color,
    medium_glow: Color,
) {
    // Header
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: "⟐ DATA STREAM ⟐".to_string(),
            font_size: 90.0,
            color: high_glow, // High glow
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(parent),
        UNode {
            height: UVal::Px(10.0),
            background_color: medium_glow,
            border_radius: UCornerRadius::all(5.0),
            ..default()
        },
    ));

    // Data stream text lines — large monospace-like readout
    for j in 0..12 {
        commands.spawn((
            ChildOf(parent),
            DataStreamText {
                speed: 1.5 + j as f32 * 0.4,
                counter: (bldg_index * 1000 + j * 100) as u32,
            },
            UNode {
                background_color: Color::srgba(0.0, 0.04, 0.08, 0.45),
                padding: USides::axes(30.0, 15.0),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            UTextLabel {
                text: format!(
                    "[0x{:04X}] PKT {} :: SYNC OK",
                    bldg_index * 1000 + j * 100,
                    j
                ),
                font_size: 55.0,
                color: Color::from(LinearRgba::rgb(0.9, 2.55, 1.65)), // Glowing matrix green
                ..default()
            },
        ));
    }
}

// ── SCREEN CONTENT: Title Banner ────────────────────────────────────────────

fn spawn_title_banner(
    commands: &mut Commands,
    parent: Entity,
    config: &BuildingConfig,
    high_glow: Color,
    medium_glow: Color,
) {
    // Large centered building name
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: format!("◈ {} ◈", config.name),
            font_size: 42.0,
            color: high_glow, // High glow for main title
            ..default()
        },
    ));

    // Subtitle
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: "SECTOR ONLINE — ALL SYSTEMS NOMINAL".to_string(),
            font_size: 20.0,
            color: medium_glow, // Medium glow for subtitle
            ..default()
        },
    ));
}

// ── SCREEN CONTENT: Status Indicator ────────────────────────────────────────

fn spawn_status_indicator(
    commands: &mut Commands,
    parent: Entity,
    config: &BuildingConfig,
    bldg_index: usize,
    high_glow: Color,
    medium_glow: Color,
) {
    // Animated status line
    commands.spawn((
        ChildOf(parent),
        AnimatedStatusText {
            building_index: bldg_index + 10, // offset to desync from main screens
        },
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: "▶ STANDBY...".to_string(),
            font_size: 22.0,
            color: high_glow, // High glow for main status/action text
            ..default()
        },
    ));

    // Progress bar
    commands.spawn((
        ChildOf(parent),
        AnimatedProgress {
            speed: 1.0 + bldg_index as f32 * 0.2,
            phase: bldg_index as f32 * 2.0,
        },
        UProgressBar {
            value: 0.5,
            ..default()
        },
    ));

    // Status label at bottom
    commands.spawn((
        ChildOf(parent),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: format!("{} — NODE ACTIVE", config.name),
            font_size: 16.0,
            color: medium_glow, // Medium glow for subtitle
            ..default()
        },
    ));
}

// ── ANIMATION SYSTEMS ───────────────────────────────────────────────────────

fn animate_progress_bars(
    time: Res<Time>,
    mut query: Query<(&AnimatedProgress, &mut UProgressBar)>,
) {
    let t = time.elapsed_secs();
    for (anim, mut bar) in &mut query {
        let wave = ((t * anim.speed + anim.phase).sin() * 0.5 + 0.5) * 0.6 + 0.2;
        bar.value = wave;
    }
}

fn animate_status_texts(time: Res<Time>, mut query: Query<(&AnimatedStatusText, &mut UTextLabel)>) {
    let t = time.elapsed_secs();
    for (anim, mut label) in &mut query {
        let cycle = ((t * 0.5 + anim.building_index as f32 * 1.3) as u32) % 5;
        let messages = [
            "▶ ALL SYSTEMS NOMINAL",
            "▶ SCANNING PERIMETER...",
            "▶ UPLINK SYNCHRONIZED",
            "▶ DIAGNOSTICS PASS",
            "▶ QUANTUM LOCK STABLE",
        ];
        label.text = messages[cycle as usize].to_string();
    }
}

fn animate_data_streams(time: Res<Time>, mut query: Query<(&mut DataStreamText, &mut UTextLabel)>) {
    let t = time.elapsed_secs();
    for (mut stream, mut label) in &mut query {
        let tick = (t * stream.speed) as u32;
        if tick != stream.counter {
            stream.counter = tick;
            let addr = 0xA000 + (tick * 17) % 0xFFFF;
            let size = 64 + (tick * 7) % 512;
            label.text = format!("[0x{:04X}] {} BYTES :: CRC OK", addr, size);
        }
    }
}

fn animate_floating_panels(time: Res<Time>, mut query: Query<(&FloatingPanel, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (panel, mut transform) in &mut query {
        // Calculate a gentle sine wave hovering effect along the local Y axis
        // We use the base_pos to reset to local origin before applying sine wave
        let offset_y = (t * panel.speed + panel.phase).sin() * panel.amplitude;
        let offset_z = (t * panel.speed * 0.7 + panel.phase).cos() * (panel.amplitude * 0.5);

        let local_up = transform.rotation * Vec3::Y;
        let local_forward = transform.rotation * Vec3::Z;

        transform.translation = panel.base_pos + local_up * offset_y + local_forward * offset_z;
    }
}

// ── CAMERA CONTROL ──────────────────────────────────────────────────────────

fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    // Zoom
    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.5;
        camera.orbit_distance = camera.orbit_distance.clamp(5.0, 50.0);
    }

    // Orbit (right mouse button)
    if mouse_button.pressed(MouseButton::Right) {
        let delta = motion.delta;
        if delta.length_squared() > 0.0 {
            let sensitivity = 0.004;
            camera.yaw -= delta.x * sensitivity;
            camera.pitch -= delta.y * sensitivity;
            camera.pitch = camera.pitch.clamp(-1.3, 1.0);
        }
    }

    // Pan (middle mouse button)
    if mouse_button.pressed(MouseButton::Middle) {
        let delta = motion.delta;
        if delta.length_squared() > 0.0 {
            let sensitivity = 0.02;
            let right = transform.rotation * Vec3::X;
            let up = Vec3::Y;
            camera.target += right * (-delta.x * sensitivity) + up * (delta.y * sensitivity);
        }
    }

    // Update transform
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    transform.translation =
        camera.target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(camera.target, Vec3::Y);
}
