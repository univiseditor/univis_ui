use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::picking::prelude::MeshPickingPlugin;
use bevy::prelude::*;
use univis_ui::prelude::*;

// Components for controlling the 3D scene
#[derive(Component)]
struct RotatingCube;

// Camera control component
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
}

// Marker components for our UI widgets to query them in systems
#[derive(Component)]
struct RotationToggle;

#[derive(Component)]
struct RotationSpeedSeekBar;

#[derive(Component)]
struct EmissiveToggle;

#[derive(Component)]
struct MetallicSeekBar;

#[derive(Component)]
struct RoughnessSeekBar;

#[derive(Component)]
struct StatusText;

// Resource to store scene settings
#[derive(Resource)]
struct AppSettings {
    cube_rotation_enabled: bool,
    cube_rotation_speed: f32,
    panel_metallic: f32,
    panel_roughness: f32,
    panel_emissive: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            cube_rotation_enabled: true,
            cube_rotation_speed: 1.0,
            panel_metallic: 0.2,
            panel_roughness: 0.5,
            panel_emissive: false,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_plugins(MeshPickingPlugin) // Enable picking for 3D meshes (PBR backend)
        .init_resource::<AppSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, sync_ui_to_settings, camera_control))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<AppSettings>,
) {
    // 1. Camera - Positioned to view both the UI and the 3D cube
    // We add the MainCamera component for mouse/scroll control
    commands.spawn((
        Camera3d::default(),
        MainCamera {
            orbit_distance: 4.5,
            pitch: -0.15, // tilt down slightly
            yaw: 0.0,
        },
        Transform::from_xyz(0.0, 1.8, 4.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    // 2. Lights - One main directional light from top/side, one headlight from camera to lit the UI
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 2500.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 3. Spawns an interactive rotating cube in the center
    // We add Pickable and an observer to handle clicks directly on the 3D mesh
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.3, 0.3),
                metallic: 0.1,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(1.2, 1.0, 0.0),
            RotatingCube,
            Pickable::default(),
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             mut settings: ResMut<AppSettings>,
             mut toggle_query: Query<&mut UToggle, With<RotationToggle>>| {
                if trigger.button == PointerButton::Primary {
                    // Toggle rotation in settings
                    settings.cube_rotation_enabled = !settings.cube_rotation_enabled;

                    // Keep the UI toggle widget in sync
                    for mut toggle in &mut toggle_query {
                        toggle.checked = settings.cube_rotation_enabled;
                    }
                }
            },
        );

    // 4. Spawns a world-space 3D UI root
    // Positions: UI at (-1.2, 1.0, 0.0), angled slightly (0.3 rad) to face the camera
    let root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(600.0, 500.0)),
            Transform::from_xyz(-1.2, 1.0, 0.0).with_rotation(Quat::from_rotation_y(0.35)),
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

    // 5. The container Panel
    // We attach `UPbr` here so that the UI panel reacts physically to the 3D scene lighting
    let panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UPbr {
                metallic: settings.panel_metallic,
                roughness: settings.panel_roughness,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(500.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // 6. Header
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Univis 3D UI & Picking".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(panel),
        StatusText,
        UNode::default(),
        UTextLabel {
            text: "Cube Speed: 1.0\nMetallic: 0.2\nRoughness: 0.5".to_string(),
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(panel),
        UNode {
            height: UVal::Px(1.0),
            background_color: Color::srgba(1.0, 1.0, 1.0, 0.2),
            ..default()
        },
    ));

    // 7. Interactive UI Controls

    // A. Cube Rotation Toggle
    let row_spin = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(row_spin),
        UNode::default(),
        UTextLabel {
            text: "Enable Spin:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(row_spin),
        RotationToggle,
        UToggle::ios_style().with_checked(settings.cube_rotation_enabled),
    ));

    // B. Cube Speed Slider
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Spin Speed:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(panel),
        RotationSpeedSeekBar,
        USeekBar::sci_fi_style()
            .with_range(0.0, 5.0)
            .with_value(settings.cube_rotation_speed / 5.0)
            .show_value(),
    ));

    // C. Panel Emissive Toggle (Glow)
    let row_glow = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(row_glow),
        UNode::default(),
        UTextLabel {
            text: "Panel Glow:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(row_glow),
        EmissiveToggle,
        UToggle::material_style().with_checked(settings.panel_emissive),
    ));

    // D. Metallic Slider
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Panel Metallic:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(panel),
        MetallicSeekBar,
        USeekBar::brightness_style()
            .with_range(0.0, 1.0)
            .with_value(settings.panel_metallic)
            .show_value(),
    ));

    // E. Roughness Slider
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Panel Roughness:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(panel),
        RoughnessSeekBar,
        USeekBar::brightness_style()
            .with_range(0.0, 1.0)
            .with_value(settings.panel_roughness)
            .show_value(),
    ));

    // Helper hint text
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Tip: Drag mouse (Hold click) to rotate camera! Scroll to zoom.\nClick the 3D Cube directly to toggle rotation!".to_string(),
            color: Color::srgb(0.5, 0.8, 0.5),
            ..default()
        },
    ));
}

/// System to rotate the 3D cube based on settings
fn rotate_cube(
    time: Res<Time>,
    settings: Res<AppSettings>,
    mut query: Query<&mut Transform, With<RotatingCube>>,
) {
    if settings.cube_rotation_enabled {
        let delta = time.delta_secs() * settings.cube_rotation_speed;
        for mut transform in &mut query {
            transform.rotate_y(delta);
            transform.rotate_x(delta * 0.3);
        }
    }
}

/// System to sync values from UI widgets to AppSettings and apply to PBR UI nodes
fn sync_ui_to_settings(
    toggle_rotation: Query<&UToggle, With<RotationToggle>>,
    seekbar_speed: Query<&USeekBar, With<RotationSpeedSeekBar>>,
    toggle_emissive: Query<&UToggle, With<EmissiveToggle>>,
    seekbar_metallic: Query<&USeekBar, With<MetallicSeekBar>>,
    seekbar_roughness: Query<&USeekBar, With<RoughnessSeekBar>>,
    mut settings: ResMut<AppSettings>,
    mut panel_pbr_query: Query<&mut UPbr>,
    mut label_query: Query<&mut UTextLabel, With<StatusText>>,
) {
    if let Some(toggle) = toggle_rotation.iter().next() {
        settings.cube_rotation_enabled = toggle.checked;
    }
    if let Some(seekbar) = seekbar_speed.iter().next() {
        settings.cube_rotation_speed = seekbar.real_value();
    }
    if let Some(toggle) = toggle_emissive.iter().next() {
        settings.panel_emissive = toggle.checked;
    }
    if let Some(seekbar) = seekbar_metallic.iter().next() {
        settings.panel_metallic = seekbar.real_value();
    }
    if let Some(seekbar) = seekbar_roughness.iter().next() {
        settings.panel_roughness = seekbar.real_value();
    }

    // Apply metallic, roughness, and emissive settings to the panel UPbr components
    for mut pbr in &mut panel_pbr_query {
        pbr.metallic = settings.panel_metallic;
        pbr.roughness = settings.panel_roughness;
        pbr.emissive = if settings.panel_emissive {
            LinearRgba::rgb(0.0, 0.4, 0.8) // Emissive Cyan glow
        } else {
            LinearRgba::BLACK
        };
    }

    // Update status label text
    for mut label in &mut label_query {
        label.text = format!(
            "Cube Rotation: {}\nCube Speed: {:.2}\nPanel Metallic: {:.2}\nPanel Roughness: {:.2}",
            if settings.cube_rotation_enabled {
                "ENABLED"
            } else {
                "DISABLED"
            },
            settings.cube_rotation_speed,
            settings.panel_metallic,
            settings.panel_roughness
        );
    }
}

/// System to orbit the camera using mouse dragging and scroll wheel zoom
fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    // 1. Zoom control
    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.15;
        camera.orbit_distance = camera.orbit_distance.clamp(2.0, 10.0);
    }

    // 2. Orbit control (when holding right mouse button)
    if mouse_button.pressed(MouseButton::Right) {
        let delta = motion.delta;
        if delta.length_squared() > 0.0 {
            let sensitivity = 0.005;
            camera.yaw -= delta.x * sensitivity;
            camera.pitch -= delta.y * sensitivity;
            camera.pitch = camera.pitch.clamp(-1.4, 1.4); // Prevent flipping upside down
        }
    }

    // 3. Update the camera transform around target (0.0, 1.0, 0.0)
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let target = Vec3::new(0.0, 1.0, 0.0);
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
