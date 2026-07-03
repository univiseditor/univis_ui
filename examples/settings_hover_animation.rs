use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use univis_ui::prelude::*;

// 1. Hover animation configuration component
#[derive(Component)]
struct AnimHover {
    base_scale: Vec3,
    target_scale: Vec3,
    current_scale: Vec3,

    base_border_width: f32,
    target_border_width: f32,
    current_border_width: f32,

    base_border_color: Color,
    target_border_color: Color,
    current_border_color: Color,
}

impl Default for AnimHover {
    fn default() -> Self {
        Self {
            base_scale: Vec3::ONE,
            target_scale: Vec3::splat(1.05), // Scale up by 5%
            current_scale: Vec3::ONE,

            base_border_width: 1.0,
            target_border_width: 3.0, // Increase border thickness
            current_border_width: 1.0,

            base_border_color: Color::srgba(0.2, 0.3, 0.4, 0.3),
            target_border_color: Color::srgb(0.0, 0.9, 1.0), // Glow neon cyan
            current_border_color: Color::srgba(0.2, 0.3, 0.4, 0.3),
        }
    }
}

impl AnimHover {
    fn with_target_color(color: Color) -> Self {
        Self {
            target_border_color: color,
            ..default()
        }
    }
}

// 2. Camera controller component
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Settings Hover Animation".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_hover_elements, camera_control))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- 1. LIGHTS & CAMERAS ---
    let camera_3d = commands
        .spawn((
            Camera3d::default(),
            MainCamera {
                orbit_distance: 4.6,
                pitch: -0.15,
                yaw: -0.3,
            },
            Transform::from_xyz(0.0, 1.8, 4.5).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
        ))
        .id();

    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.02, 0.03, 0.08),
        brightness: 40.0,
        ..default()
    });

    // Golden sunlight highlight
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: 1500.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // --- 2. 3D BACKGROUND (FUTURISTIC RINGS) ---
    let ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.15, 0.25),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(1.8, 0.05))),
        MeshMaterial3d(ring_mat.clone()),
        Transform::from_xyz(0.0, 0.8, -1.5).with_rotation(Quat::from_rotation_x(1.1)),
    ));

    // --- 3. SETTINGS TERMINAL (3D WORLD SPACE UI) ---
    let root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera_3d),
                ..URootUi::world_3d(Vec2::new(800.0, 600.0))
            },
            Transform::from_xyz(-0.5, 0.8, 0.0).with_rotation(Quat::from_rotation_y(0.35)),
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

    // Main Settings board
    let settings_board = commands
        .spawn((
            ChildOf(root),
            UPanel::glass(),
            UNode {
                width: UVal::Px(750.0),
                height: UVal::Px(550.0),
                padding: USides::all(24.0),
                border_radius: UCornerRadius::all(24.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.4, 0.8, 0.2),
                width: 1.5,
                radius: UCornerRadius::all(24.0),
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

    // Header Title
    commands.spawn((
        ChildOf(settings_board),
        UTextLabel {
            text: "◈ SYSTEM SETTINGS ◈".to_string(),
            font_size: 24.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((ChildOf(settings_board), UDivider::horizontal()));

    // Two-column content layout
    let content = commands
        .spawn((
            ChildOf(settings_board),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Flex(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // --- LEFT COLUMN: Settings Categories ---
    let left_col = commands
        .spawn((
            ChildOf(content),
            UNode {
                width: UVal::Px(240.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    let categories = [
        ("GRAPHICS", Color::srgb(0.0, 0.9, 1.0)),
        ("AUDIO", Color::srgb(0.9, 0.1, 0.6)),
        ("CONTROLS", Color::srgb(1.0, 0.7, 0.0)),
        ("NETWORK", Color::srgb(0.1, 0.9, 0.4)),
    ];

    for (name, glow_color) in categories {
        commands
            .spawn((
                ChildOf(left_col),
                UInteraction::default(),
                Pickable::default(),
                AnimHover::with_target_color(glow_color),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(56.0),
                    padding: USides::axes(16.0, 10.0),
                    background_color: Color::srgba(0.05, 0.07, 0.12, 0.5),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.2, 0.3, 0.4, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                    offset: 0.0,
                },
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|card| {
                card.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: name.to_string(),
                        font_size: 14.0,
                        color: Color::srgba(1.0, 1.0, 1.0, 0.85),
                        ..default()
                    },
                ));
            });
    }

    // --- RIGHT COLUMN: Active Option Settings ---
    let right_col = commands
        .spawn((
            ChildOf(content),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.04, 0.06, 0.1, 0.4),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.3, 0.4, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(20.0),
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

    // Option 1: Resolution Selection Dropdown Card
    let option1 = commands
        .spawn((
            ChildOf(right_col),
            UInteraction::default(),
            Pickable::default(),
            AnimHover::default(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.06, 0.09, 0.14, 0.5),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.3, 0.4, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(option1),
        UTextLabel {
            text: "Screen Resolution".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(option1),
        UTextLabel {
            text: "1920 x 1080".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.0, 0.9, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    // Option 2: Texture Quality Card
    let option2 = commands
        .spawn((
            ChildOf(right_col),
            UInteraction::default(),
            Pickable::default(),
            AnimHover::default(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.06, 0.09, 0.14, 0.5),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.3, 0.4, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(option2),
        UTextLabel {
            text: "Texture Quality".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(option2),
        UTextLabel {
            text: "ULTRA".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.0, 0.9, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    // Option 3: Volume Slider Card
    let option3 = commands
        .spawn((
            ChildOf(right_col),
            UInteraction::default(),
            Pickable::default(),
            AnimHover::default(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.06, 0.09, 0.14, 0.5),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.3, 0.4, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    let slider_header = commands
        .spawn((
            ChildOf(option3),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(slider_header),
        UTextLabel {
            text: "Master Volume".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(slider_header),
        UTextLabel {
            text: "80%".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.0, 0.9, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(option3),
        USeekBar::sci_fi_style()
            .with_range(0.0, 100.0)
            .with_value(80.0),
    ));

    // Option 4: V-Sync Toggle Card
    let option4 = commands
        .spawn((
            ChildOf(right_col),
            UInteraction::default(),
            Pickable::default(),
            AnimHover::default(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.06, 0.09, 0.14, 0.5),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.3, 0.4, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(option4),
        UTextLabel {
            text: "V-Sync (Vertical Sync)".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((ChildOf(option4), UToggle::ios_style().with_checked(true)));

    // Instructions foot label on HUD
    let hud_root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::End,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(hud_root),
        UTextLabel {
            text: "Hover settings cards to see scale animations and glowing borders. [RMB + Drag] orbit camera.".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.6, 0.7, 0.8),
            ..default()
        },
        UNode::default(),
    ));
}

// System to smoothly lerp scale and border parameters based on UInteraction hover states
fn animate_hover_elements(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut UBorder, &UInteraction, &mut AnimHover)>,
) {
    let dt = time.delta_secs();
    let speed = 14.0; // Fast and snappy interpolation speed

    for (mut transform, mut border, interaction, mut anim) in &mut query {
        let (dest_scale, dest_border_w, dest_border_c) = match interaction {
            UInteraction::Hovered | UInteraction::Pressed => (
                anim.target_scale,
                anim.target_border_width,
                anim.target_border_color,
            ),
            _ => (
                anim.base_scale,
                anim.base_border_width,
                anim.base_border_color,
            ),
        };

        // Smoothly lerp Transform scale
        anim.current_scale = anim.current_scale.lerp(dest_scale, speed * dt);
        transform.scale = anim.current_scale;

        // Smoothly lerp UBorder width
        anim.current_border_width =
            anim.current_border_width + (dest_border_w - anim.current_border_width) * speed * dt;
        border.width = anim.current_border_width;

        // Smoothly lerp UBorder color
        let current_linear = anim.current_border_color.to_linear();
        let dest_linear = dest_border_c.to_linear();
        let r = current_linear.red + (dest_linear.red - current_linear.red) * speed * dt;
        let g = current_linear.green + (dest_linear.green - current_linear.green) * speed * dt;
        let b = current_linear.blue + (dest_linear.blue - current_linear.blue) * speed * dt;
        let a = current_linear.alpha + (dest_linear.alpha - current_linear.alpha) * speed * dt;
        anim.current_border_color = Color::srgba(r, g, b, a);
        border.color = anim.current_border_color;
    }
}

// Basic camera control system
fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.2;
        camera.orbit_distance = camera.orbit_distance.clamp(2.0, 8.0);
    }

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
    let target = Vec3::new(0.0, 0.6, 0.0);
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
