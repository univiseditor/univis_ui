use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::picking::prelude::MeshPickingPlugin;
use bevy::prelude::*;
use univis_ui::prelude::*;

// Components for controlling the 3D scene
#[derive(Component)]
struct RotatingShape;

// Camera control component
#[derive(Component)]
struct MainCamera {
    orbit_distance: f32,
    pitch: f32,
    yaw: f32,
}

// Markers for UI
#[derive(Component)]
struct ShowcaseRotationToggle;

#[derive(Component)]
struct ShowcaseSpeedSlider;

#[derive(Component)]
struct ShowcaseProgress;

#[derive(Component)]
struct ShowcaseStatusText;

// Resource to store scene settings
#[derive(Resource)]
struct ShowcaseSettings {
    rotation_enabled: bool,
    rotation_speed: f32,
    progress: f32,
}

impl Default for ShowcaseSettings {
    fn default() -> Self {
        Self {
            rotation_enabled: true,
            rotation_speed: 1.0,
            progress: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_plugins(MeshPickingPlugin) // Enable picking for 3D meshes (PBR backend)
        .init_resource::<ShowcaseSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rotate_shapes,
                sync_ui_to_settings,
                camera_control,
                update_progress,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 1. Camera
    commands.spawn((
        Camera3d::default(),
        MainCamera {
            orbit_distance: 6.0,
            pitch: -0.2, // tilt down slightly
            yaw: -0.3,
        },
        Transform::from_xyz(0.0, 2.5, 6.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    // Overlay Camera for 2D HUD
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // 2. Lights
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: 2000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 1000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(-2.0, 3.0, 3.0),
    ));

    // 3. Spawns interactive rotating 3D shapes
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.8),
        metallic: 0.5,
        perceptual_roughness: 0.3,
        ..default()
    });

    // Cube
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(2.0, 1.0, 0.0),
            RotatingShape,
            Pickable::default(),
        ))
        .observe(
            |trigger: On<Pointer<Click>>, mut settings: ResMut<ShowcaseSettings>| {
                if trigger.button == PointerButton::Primary {
                    settings.rotation_enabled = !settings.rotation_enabled;
                }
            },
        );

    // Sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3),
            metallic: 0.1,
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(2.0, 3.0, 0.0),
        RotatingShape,
        Pickable::default(),
    ));

    // 4. Spawn World-Space UI
    let root = commands
        .spawn((
            URootUi::world_3d(Vec2::new(700.0, 800.0)),
            Transform::from_xyz(-1.5, 2.0, 0.0).with_rotation(Quat::from_rotation_y(0.35)),
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

    // 5. Main ScrollView to hold all components
    let scroll_view = commands
        .spawn((
            ChildOf(root),
            UScrollContainer::new(),
            UPanel::glass(),
            UNode {
                width: UVal::Px(600.0),
                height: UVal::Px(700.0),
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

    // -- Header Section --
    let header = commands
        .spawn((
            ChildOf(scroll_view),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(header),
        UTextLabel {
            text: "Complex Component Showcase".to_string(),
            font_size: 28.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(header),
        ShowcaseStatusText,
        UTextLabel {
            text: "Explore all Univis UI components working together in 3D.".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((ChildOf(scroll_view), UDivider::horizontal()));

    // -- Controls Section --
    let section_controls = create_section(commands.reborrow(), scroll_view, "Basic Controls");

    // Buttons
    let row_buttons = commands
        .spawn((
            ChildOf(section_controls),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_button(
        commands.reborrow(),
        row_buttons,
        "Primary",
        UButton::primary(),
    );
    spawn_button(
        commands.reborrow(),
        row_buttons,
        "Outline",
        UButton::secondary(),
    );
    spawn_button(
        commands.reborrow(),
        row_buttons,
        "Ghost",
        UButton::success(),
    );
    spawn_button(
        commands.reborrow(),
        row_buttons,
        "Danger",
        UButton::danger(),
    );

    // Badges
    let row_badges = commands
        .spawn((
            ChildOf(section_controls),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_badge(commands.reborrow(), row_badges, "New", UBadge::primary());
    spawn_badge(commands.reborrow(), row_badges, "Active", UBadge::success());
    spawn_badge(
        commands.reborrow(),
        row_badges,
        "Warning",
        UBadge::warning(),
    );
    spawn_badge(commands.reborrow(), row_badges, "Error", UBadge::danger());

    // Toggles & Checkboxes
    let row_toggles = commands
        .spawn((
            ChildOf(section_controls),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 24.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_toggle_with_label(
        commands.reborrow(),
        row_toggles,
        "iOS Toggle:",
        UToggle::ios_style().with_checked(true),
        Some(ShowcaseRotationToggle),
    );
    spawn_toggle_with_label(
        commands.reborrow(),
        row_toggles,
        "Material:",
        UToggle::material_style(),
        None::<ShowcaseRotationToggle>,
    );

    let checkbox_wrap = commands
        .spawn((
            ChildOf(row_toggles),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                gap: 8.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((ChildOf(checkbox_wrap), UCheckbox::default()));
    commands.spawn((
        ChildOf(checkbox_wrap),
        UNode::default(),
        UTextLabel {
            text: "Checkbox".into(),
            color: Color::WHITE,
            ..default()
        },
    ));

    // -- Inputs Section --
    commands.spawn((ChildOf(scroll_view), UDivider::horizontal()));
    let section_inputs = create_section(commands.reborrow(), scroll_view, "Inputs & Sliders");

    // Text Field
    commands.spawn((
        ChildOf(section_inputs),
        UTextField::default().with_placeholder("Enter some text here..."),
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
    ));

    // Drag Value
    let row_drag = commands
        .spawn((
            ChildOf(section_inputs),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                gap: 12.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(row_drag),
        UNode::default(),
        UTextLabel {
            text: "Drag Value:".into(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(row_drag),
        UDragValue::default().with_value(42.0).with_step(0.5),
    ));

    // SeekBars
    commands.spawn((
        ChildOf(section_inputs),
        UNode::default(),
        UTextLabel {
            text: "Sci-Fi Slider".into(),
            color: Color::WHITE,
            font_size: 12.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(section_inputs),
        ShowcaseSpeedSlider,
        USeekBar::sci_fi_style()
            .with_range(0.0, 5.0)
            .with_value(1.0)
            .show_value(),
    ));

    commands.spawn((
        ChildOf(section_inputs),
        UNode::default(),
        UTextLabel {
            text: "Material Slider".into(),
            color: Color::WHITE,
            font_size: 12.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(section_inputs),
        USeekBar::brightness_style()
            .with_range(0.0, 100.0)
            .with_value(50.0)
            .show_value(),
    ));

    // Progress
    commands.spawn((
        ChildOf(section_inputs),
        UNode::default(),
        UTextLabel {
            text: "Progress Bar".into(),
            color: Color::WHITE,
            font_size: 12.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(section_inputs),
        ShowcaseProgress,
        UProgressBar {
            value: 0.3,
            ..default()
        },
    ));

    // -- Media Section --
    commands.spawn((ChildOf(scroll_view), UDivider::horizontal()));
    let section_media = create_section(commands.reborrow(), scroll_view, "Media & Icons");

    let row_media = commands
        .spawn((
            ChildOf(section_media),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Image
    let handle = asset_server.load("textures/univis_logo.png"); // Assuming there's a logo or it'll just show empty/fallback if not found, usually examples provide images. We use a generic node with background if missing.
    commands.spawn((
        ChildOf(row_media),
        UImage::new(handle),
        UNode {
            width: UVal::Px(64.0),
            height: UVal::Px(64.0),
            background_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        },
    ));

    // Circular Progress
    commands.spawn((
        ChildOf(row_media),
        ShowcaseProgress,
        UProgressBar {
            value: 0.7,
            ..default()
        },
    ));

    // -- Panels & Layouts Section --
    commands.spawn((ChildOf(scroll_view), UDivider::horizontal()));
    let section_layouts = create_section(commands.reborrow(), scroll_view, "Nested Panels");

    let row_panels = commands
        .spawn((
            ChildOf(section_layouts),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                gap: 16.0,
                flex_direction: UFlexDirection::Row,
                ..default()
            },
        ))
        .id();

    // Card Panel
    let card = commands
        .spawn((
            ChildOf(row_panels),
            UPanel::card(),
            UNode {
                width: UVal::Px(250.0),
                padding: USides::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(card),
        UNode::default(),
        UTextLabel {
            text: "Card Panel".into(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(card),
        UNode::default(),
        UTextLabel {
            text: "A simple card with solid background.".into(),
            font_size: 12.0,
            color: Color::srgb(0.8, 0.8, 0.8),
            ..default()
        },
    ));
    spawn_button(commands.reborrow(), card, "Action", UButton::primary());

    // Glass Panel
    let glass = commands
        .spawn((
            ChildOf(row_panels),
            UPanel::glass(),
            UNode {
                width: UVal::Px(250.0),
                padding: USides::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(glass),
        UNode::default(),
        UTextLabel {
            text: "Glass Panel".into(),
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(glass),
        UNode::default(),
        UTextLabel {
            text: "A panel with blur/glass effect.".into(),
            font_size: 12.0,
            color: Color::srgb(0.8, 0.8, 0.8),
            ..default()
        },
    ));
    spawn_button(commands.reborrow(), glass, "Explore", UButton::secondary());

    // -- Screen-Space HUD Overlay --
    let hud_root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Start,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            ChildOf(hud_root),
            UPanel::glass(),
            UNode {
                padding: USides::axes(16.0, 12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                UNode::default(),
                UTextLabel {
                    text: "HUD Overlay".into(),
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            p.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Screen-space UI always on top".into(),
                    font_size: 14.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        });
}

fn create_section(mut commands: Commands, parent: Entity, title: &str) -> Entity {
    let section = commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(section),
        UNode::default(),
        UTextLabel {
            text: title.to_string(),
            font_size: 20.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    ));

    section
}

fn spawn_button(mut commands: Commands, parent: Entity, label: &str, btn: UButton) {
    commands
        .spawn((
            ChildOf(parent),
            btn,
            UNode {
                padding: USides::axes(16.0, 8.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: label.to_string(),
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn spawn_badge(mut commands: Commands, parent: Entity, label: &str, badge: UBadge) {
    commands
        .spawn((
            ChildOf(parent),
            badge,
            UNode {
                padding: USides::axes(8.0, 4.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: label.to_string(),
                    font_size: 12.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn spawn_toggle_with_label(
    mut commands: Commands,
    parent: Entity,
    label: &str,
    toggle: UToggle,
    marker: Option<impl Component>,
) {
    let wrap = commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                gap: 8.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(wrap),
        UNode::default(),
        UTextLabel {
            text: label.to_string(),
            color: Color::WHITE,
            ..default()
        },
    ));

    let mut t_cmd = commands.spawn((ChildOf(wrap), toggle));
    if let Some(m) = marker {
        t_cmd.insert(m);
    }
}

fn update_progress(time: Res<Time>, mut settings: ResMut<ShowcaseSettings>) {
    settings.progress = (time.elapsed_secs() * 0.5).sin() * 0.5 + 0.5;
}

fn rotate_shapes(
    time: Res<Time>,
    settings: Res<ShowcaseSettings>,
    mut query: Query<&mut Transform, With<RotatingShape>>,
) {
    if settings.rotation_enabled {
        let delta = time.delta_secs() * settings.rotation_speed;
        for mut transform in &mut query {
            transform.rotate_y(delta);
            transform.rotate_x(delta * 0.3);
        }
    }
}

fn sync_ui_to_settings(
    toggle_query: Query<&UToggle, With<ShowcaseRotationToggle>>,
    slider_query: Query<&USeekBar, With<ShowcaseSpeedSlider>>,
    mut progress_query: Query<&mut UProgressBar, With<ShowcaseProgress>>,
    mut settings: ResMut<ShowcaseSettings>,
) {
    // Read from UI
    if let Some(toggle) = toggle_query.iter().next() {
        settings.rotation_enabled = toggle.checked;
    }
    if let Some(slider) = slider_query.iter().next() {
        settings.rotation_speed = slider.real_value();
    }

    // Write to UI (Progress)
    for mut progress in &mut progress_query {
        progress.value = settings.progress;
    }
}

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
        camera.orbit_distance = camera.orbit_distance.clamp(2.0, 15.0);
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
    let target = Vec3::new(0.0, 1.0, 0.0);
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
