use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use univis_ui::prelude::*;

// 1. Components for scene interactions
#[derive(Component)]
struct QuantumCore;

#[derive(Component)]
struct CoreRotationToggle;

#[derive(Component)]
struct CoreSpeedSlider;

#[derive(Component)]
struct DiagnosticProgress;

// Camera control component
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
}

// 2. Resource to sync UI actions with the 3D world state
#[derive(Resource)]
struct TerminalState {
    core_active: bool,
    core_speed: f32,
    sync_progress: f32,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            core_active: true,
            core_speed: 1.5,
            sync_progress: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Futuristic 2D/3D Showcase".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<TerminalState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rotate_core,
                sync_ui_to_state,
                update_diagnostic_sim,
                camera_control,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- 1. LIGHTS & CAMERAS ---
    // Standard 3D Camera with MainCamera orbit component
    let main_camera_entity = commands
        .spawn((
            Camera3d::default(),
            MainCamera {
                orbit_distance: 4.8,
                pitch: -0.15,
                yaw: -0.25,
            },
            Transform::from_xyz(0.0, 1.8, 4.5).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
        ))
        .id();

    // Secondary overlay Camera for 2D Screen-space HUD
    let hud_camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
        ))
        .id();

    // Directional light for shadows & physical highlights
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Cyan glowing point light near the quantum core
    commands.spawn((
        PointLight {
            color: Color::srgb(0.0, 0.9, 1.0),
            intensity: 800.0,
            range: 10.0,
            ..default()
        },
        Transform::from_xyz(1.5, 0.8, -0.5),
    ));

    // --- 2. 3D WORLD OBJECTS (THE QUANTUM CORE) ---
    let core_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.7, 0.95),
        emissive: LinearRgba::rgb(0.0, 0.4, 0.6),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });

    // Spawn a central orb representing the reactor core
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(core_mat.clone()),
        Transform::from_xyz(1.5, 0.8, -1.0),
        QuantumCore,
    ));

    // Spawn outer rings orbiting the core
    let ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.95),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.6, 0.04))),
        MeshMaterial3d(ring_mat),
        Transform::from_xyz(1.5, 0.8, -1.0).with_rotation(Quat::from_rotation_x(1.0)),
        QuantumCore,
    ));

    // --- 3. PHYSICAL 3D HOLOGRAPHIC TERMINAL (WORLD SPACE UI) ---
    let terminal_root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(main_camera_entity),
                ..URootUi::world_3d(Vec2::new(760.0, 560.0))
            },
            // Place terminal on the left, angled towards the camera
            Transform::from_xyz(-0.8, 0.8, 0.0).with_rotation(Quat::from_rotation_y(0.4)),
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

    // Futuristic glass container panel (SDF rendering under 3D angles)
    let glass_panel = commands
        .spawn((
            ChildOf(terminal_root),
            UPanel::glass(),
            UNode {
                width: UVal::Px(720.0),
                height: UVal::Px(520.0),
                padding: USides::all(20.0),
                border_radius: UCornerRadius::all(24.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(24.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // LEFT SECTION: Radial Orbital Selector
    let left_col = commands
        .spawn((
            ChildOf(glass_panel),
            UNode {
                width: UVal::Px(320.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(left_col),
        UTextLabel {
            text: "ORBITAL SYSTEMS MENU".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.0, 0.85, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    // The Radial Layout container
    let radial_selector = commands
        .spawn((
            ChildOf(left_col),
            UNode {
                width: UVal::Px(240.0),
                height: UVal::Px(240.0),
                background_color: Color::srgba(0.0, 0.1, 0.2, 0.3),
                border_radius: UCornerRadius::all(120.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(120.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Radial,
                ..default()
            },
        ))
        .id();

    let sectors = ["NAV", "SCAN", "LINK", "DRON", "PING", "DATA"];
    let sector_colors = [
        Color::srgb(0.0, 0.8, 1.0),
        Color::srgb(0.1, 0.9, 0.6),
        Color::srgb(0.8, 0.3, 1.0),
        Color::srgb(1.0, 0.7, 0.1),
        Color::srgb(0.95, 0.2, 0.3),
        Color::srgb(0.7, 0.8, 0.9),
    ];

    for (name, color) in sectors.into_iter().zip(sector_colors) {
        let btn = commands
            .spawn((
                ChildOf(radial_selector),
                UButton::secondary(),
                UNode {
                    width: UVal::Px(58.0),
                    height: UVal::Px(58.0),
                    border_radius: UCornerRadius::all(29.0),
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
            ChildOf(btn),
            UTextLabel {
                text: name.to_string(),
                font_size: 11.0,
                color,
                ..default()
            },
            UNode::default(),
        ));
    }

    commands.spawn((
        ChildOf(left_col),
        UTextLabel {
            text: "Select subsystem node".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.6, 0.7, 0.8),
            ..default()
        },
        UNode::default(),
    ));

    // Divider between columns
    commands.spawn((ChildOf(glass_panel), UDivider::horizontal()));

    // RIGHT SECTION: Reactor Core Telemetry & Controls
    let right_col = commands
        .spawn((
            ChildOf(glass_panel),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Start,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    let title_block = commands
        .spawn((
            ChildOf(right_col),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(title_block),
        UTextLabel {
            text: "QUANTUM CORE TELEMETRY".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));
    commands.spawn((
        ChildOf(title_block),
        UTextLabel {
            text: "Diagnostics node #8731-B. System status operational.".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.5, 0.6, 0.7),
            ..default()
        },
        UNode::default(),
    ));

    // Controls Container
    let ctrl_area = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    // Core Active Toggle
    let toggle_row = commands
        .spawn((
            ChildOf(ctrl_area),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(toggle_row),
        UTextLabel {
            text: "REACTOR MATRIX STATE:".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.8, 0.85, 0.9),
            ..default()
        },
        UNode::default(),
    ));
    commands.spawn((
        ChildOf(toggle_row),
        CoreRotationToggle,
        UToggle::ios_style().with_checked(true),
    ));

    // Core Rotation Speed SeekBar
    commands.spawn((
        ChildOf(ctrl_area),
        UTextLabel {
            text: "ROTATIONAL VELOCITY SPEED".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
        UNode::default(),
    ));
    commands.spawn((
        ChildOf(ctrl_area),
        CoreSpeedSlider,
        USeekBar::sci_fi_style()
            .with_range(0.0, 5.0)
            .with_value(1.5)
            .show_value(),
    ));

    // Diagnostic Progress
    commands.spawn((
        ChildOf(ctrl_area),
        UTextLabel {
            text: "DIAGNOSTIC CORE DECAY RATIO".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
        UNode::default(),
    ));
    commands.spawn((
        ChildOf(ctrl_area),
        DiagnosticProgress,
        UProgressBar {
            value: 0.0,
            ..default()
        },
    ));

    // Action button
    let actions_row = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::End,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    // Emergency purge button
    let purge_btn = commands
        .spawn((
            ChildOf(actions_row),
            UButton::danger(),
            UNode {
                padding: USides::axes(18.0, 8.0),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(purge_btn),
        UTextLabel {
            text: "EMERGENCY PURGE".to_string(),
            font_size: 13.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    // --- 4. SCREEN-SPACE HUD ---
    let hud_root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(hud_camera_entity),
                ..URootUi::screen()
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    // Header bar
    let hud_header = commands
        .spawn((
            ChildOf(hud_root),
            UPanel::glass(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(24.0, 10.0),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(hud_header),
        UTextLabel {
            text: "UNIVIS OS v0.3.0".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.0, 0.85, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(hud_header),
        UTextLabel {
            text: "SYSTEM ONLINE".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.1, 0.9, 0.4),
            ..default()
        },
        UNode::default(),
    ));

    // Footer info
    let hud_footer = commands
        .spawn((
            ChildOf(hud_root),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(hud_footer),
        UTextLabel {
            text: "[RMB + Drag] Rotate Camera  |  [Scroll] Zoom".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.6, 0.7, 0.8),
            ..default()
        },
        UNode::default(),
    ));
}

// System to rotate the 3D meshes based on speed state
fn rotate_core(
    time: Res<Time>,
    state: Res<TerminalState>,
    mut query: Query<&mut Transform, With<QuantumCore>>,
) {
    if state.core_active {
        let speed = state.core_speed;
        let delta = time.delta_secs() * speed;
        for (i, mut transform) in query.iter_mut().enumerate() {
            if i == 0 {
                transform.rotate_y(delta * 0.4);
            } else {
                transform.rotate_y(delta);
                transform.rotate_x(delta * 0.5);
            }
        }
    }
}

// System to simulate core diagnostic bar progression
fn update_diagnostic_sim(time: Res<Time>, mut state: ResMut<TerminalState>) {
    state.sync_progress = (time.elapsed_secs() * 0.2).sin() * 0.5 + 0.5;
}

// System to sync changes from UI widgets back to the game state
fn sync_ui_to_state(
    toggle_q: Query<&UToggle, With<CoreRotationToggle>>,
    slider_q: Query<&USeekBar, With<CoreSpeedSlider>>,
    mut progress_q: Query<&mut UProgressBar, With<DiagnosticProgress>>,
    mut state: ResMut<TerminalState>,
) {
    if let Some(toggle) = toggle_q.iter().next() {
        state.core_active = toggle.checked;
    }

    if let Some(slider) = slider_q.iter().next() {
        state.core_speed = slider.real_value();
    }

    for mut progress in &mut progress_q {
        progress.value = state.sync_progress;
    }
}

// Orbiting camera control system using Bevy mouse events
fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    // Zoom handling
    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.2;
        camera.orbit_distance = camera.orbit_distance.clamp(2.0, 10.0);
    }

    // Drag to rotate camera around target (0.0, 0.6, 0.0)
    if mouse_button.pressed(MouseButton::Right) {
        let delta = motion.delta;
        if delta.length_squared() > 0.0 {
            let sensitivity = 0.005;
            camera.yaw -= delta.x * sensitivity;
            camera.pitch -= delta.y * sensitivity;
            camera.pitch = camera.pitch.clamp(-1.4, 1.4);
        }
    }

    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let target = Vec3::new(0.0, 0.6, 0.0); // Center of focus
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
