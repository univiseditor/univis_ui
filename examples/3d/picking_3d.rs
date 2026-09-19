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
            |trigger: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                if trigger.button == PointerButton::Primary {
                    // Toggle rotation in settings
                    settings.cube_rotation_enabled = !settings.cube_rotation_enabled;
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
            UPbr {
                metallic: settings.panel_metallic,
                roughness: settings.panel_roughness,
                emissive: LinearRgba::BLACK,
            },
            UNode {
                width: UVal::Px(500.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.15, 0.85),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.3, 0.5, 0.8, 0.35),
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
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
            font_size: 20.0,
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
            text: "Cube Spin:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands
        .spawn((
            ChildOf(row_spin),
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
                pressed: Color::srgb(0.1, 0.4, 0.7),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.cube_rotation_enabled = !settings.cube_rotation_enabled;
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Toggle Spin".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });

    // B. Cube Speed Row
    let row_speed = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(row_speed),
        UNode::default(),
        UTextLabel {
            text: "Spin Speed:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands
        .spawn((
            ChildOf(row_speed),
            UNode {
                padding: USides::axes(12.0, 6.0),
                background_color: Color::srgb(0.3, 0.3, 0.35),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.3, 0.3, 0.35),
                hovered: Color::srgb(0.4, 0.4, 0.45),
                pressed: Color::srgb(0.2, 0.2, 0.25),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.cube_rotation_speed = (settings.cube_rotation_speed - 0.5).max(0.0);
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "- Slower".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            ChildOf(row_speed),
            UNode {
                padding: USides::axes(12.0, 6.0),
                background_color: Color::srgb(0.3, 0.3, 0.35),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.3, 0.3, 0.35),
                hovered: Color::srgb(0.4, 0.4, 0.45),
                pressed: Color::srgb(0.2, 0.2, 0.25),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.cube_rotation_speed = (settings.cube_rotation_speed + 0.5).min(5.0);
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "+ Faster".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });

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
    commands
        .spawn((
            ChildOf(row_glow),
            UNode {
                padding: USides::axes(14.0, 6.0),
                background_color: Color::srgb(0.1, 0.5, 0.6),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.1, 0.5, 0.6),
                hovered: Color::srgb(0.2, 0.6, 0.7),
                pressed: Color::srgb(0.05, 0.4, 0.5),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.panel_emissive = !settings.panel_emissive;
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Toggle Glow".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });

    // D. Material Controls Row
    let row_mat = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(row_mat),
        UNode::default(),
        UTextLabel {
            text: "Material:".to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands
        .spawn((
            ChildOf(row_mat),
            UNode {
                padding: USides::axes(10.0, 6.0),
                background_color: Color::srgb(0.3, 0.3, 0.35),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.3, 0.3, 0.35),
                hovered: Color::srgb(0.4, 0.4, 0.45),
                pressed: Color::srgb(0.2, 0.2, 0.25),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.panel_metallic = (settings.panel_metallic + 0.25).min(1.0);
                if settings.panel_metallic > 0.99 {
                    settings.panel_metallic = 0.0;
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Cycle Metal".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            ChildOf(row_mat),
            UNode {
                padding: USides::axes(10.0, 6.0),
                background_color: Color::srgb(0.3, 0.3, 0.35),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.3, 0.3, 0.35),
                hovered: Color::srgb(0.4, 0.4, 0.45),
                pressed: Color::srgb(0.2, 0.2, 0.25),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut settings: ResMut<AppSettings>| {
                settings.panel_roughness = (settings.panel_roughness + 0.25).min(1.0);
                if settings.panel_roughness > 0.99 {
                    settings.panel_roughness = 0.1;
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Cycle Rough".to_string(),
                    color: Color::WHITE,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });

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

/// System to sync values from AppSettings and apply to PBR UI nodes
fn sync_ui_to_settings(
    settings: Res<AppSettings>,
    mut panel_pbr_query: Query<&mut UPbr>,
    mut label_query: Query<&mut UTextLabel, With<StatusText>>,
) {
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
