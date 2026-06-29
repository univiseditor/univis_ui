use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI 2D Picking Example".into(),
                    resolution: (1024, 768).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<SceneSettings>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (sync_ui_to_settings, camera_control, animate_sprites),
        )
        .run();
}

/// Global state to sync UI with the scene
#[derive(Resource)]
struct SceneSettings {
    sprite_speed: f32,
    enable_animation: bool,
    background_color: Color,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            sprite_speed: 2.0,
            enable_animation: true,
            background_color: Color::srgb(0.1, 0.1, 0.15),
        }
    }
}

// Marker components for UI syncing
#[derive(Component)]
struct AnimationToggle;

#[derive(Component)]
struct SpeedSeekBar;

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct InteractiveSprite;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<SceneSettings>,
) {
    // 1. Camera
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(settings.background_color),
            ..default()
        },
    ));

    // 2. Interactive 2D Sprites
    // Spawning a few colored shapes
    let circle_mesh = meshes.add(Circle::new(50.0));

    let colors = [
        Color::srgb(0.8, 0.2, 0.2),
        Color::srgb(0.2, 0.8, 0.2),
        Color::srgb(0.2, 0.2, 0.8),
    ];

    for (i, color) in colors.iter().enumerate() {
        commands
            .spawn((
                Mesh2d(circle_mesh.clone()),
                MeshMaterial2d(materials.add(*color)),
                Transform::from_xyz((i as f32 - 1.0) * 150.0, 0.0, 0.0),
                InteractiveSprite,
                // Bevy Picking requirement for 2D meshes
                Pickable::default(),
            ))
            .observe(handle_sprite_clicks);
    }

    // 3. Spawns a screen-space UI root
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(30.0),
                // We want the UI to be in the bottom-left corner
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Start, // Align to left
                justify_content: UJustifyContent::End, // Align to bottom
                ..default()
            },
        ))
        .id();

    // 4. Build the UI Panel
    let panel = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(450.0),
                padding: USides::all(24.0),
                border_radius: UCornerRadius::all(20.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
                ..default()
            },
            // The panel has picking block so clicks don't pass through to the 2D scene behind it
            Pickable::default(),
        ))
        .id();

    // A. Title
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Univis 2D UI & Picking".to_string(),
            color: Color::WHITE,
            font_size: 24.0,
            ..default()
        },
    ));

    // B. Status
    commands.spawn((
        ChildOf(panel),
        StatusText,
        UNode::default(),
        UTextLabel {
            text: "Animation Speed: 2.0".to_string(),
            color: Color::srgb(0.7, 0.8, 0.9),
            font_size: 16.0,
            ..default()
        },
    ));

    // Divider
    commands.spawn((
        ChildOf(panel),
        UNode {
            height: UVal::Px(1.0),
            background_color: Color::srgba(1.0, 1.0, 1.0, 0.2),
            ..default()
        },
    ));

    // C. Animation Toggle
    let row_anim = commands
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
        ChildOf(row_anim),
        UNode::default(),
        UTextLabel {
            text: "Enable Animation:".to_string(),
            color: Color::WHITE,
            font_size: 16.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(row_anim),
        AnimationToggle,
        UToggle::ios_style().with_checked(settings.enable_animation),
    ));

    // D. Speed Slider
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Speed Multiplier:".to_string(),
            color: Color::WHITE,
            font_size: 16.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(panel),
        SpeedSeekBar,
        USeekBar::sci_fi_style()
            .with_range(0.0, 5.0)
            .with_value(settings.sprite_speed)
            .show_value(),
    ));

    // Helper hint text
    commands.spawn((
        ChildOf(panel),
        UNode::default(),
        UTextLabel {
            text: "Tip: Drag mouse (Right click) to move camera! Scroll to zoom.\nClick the circles directly to change their color!".to_string(),
            color: Color::srgb(0.5, 0.8, 0.5),
            font_size: 14.0,
            ..default()
        },
    ));
}

/// Simple camera pan
fn camera_control(
    mut q_camera: Query<&mut Transform, With<Camera2d>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    // Pan with right mouse button
    if mouse_button.pressed(MouseButton::Right) {
        for mut transform in &mut q_camera {
            transform.translation.x -= mouse_motion.delta.x;
            transform.translation.y += mouse_motion.delta.y;
        }
    }
}

/// Animate the interactive sprites
fn animate_sprites(
    time: Res<Time>,
    settings: Res<SceneSettings>,
    mut q_sprites: Query<(Entity, &mut Transform), With<InteractiveSprite>>,
) {
    if !settings.enable_animation {
        return;
    }

    for (entity, mut transform) in q_sprites.iter_mut() {
        // Just make them bob up and down with different phases based on entity id
        let phase = (entity.to_bits() % 100) as f32;
        let y_offset = (time.elapsed_secs() * settings.sprite_speed + phase).sin() * 50.0;
        transform.translation.y = y_offset;
    }
}

/// Read clicks on the interactive sprites using Bevy's observers
fn handle_sprite_clicks(
    click: On<Pointer<Click>>,
    mut q_materials: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_sprites: Query<Entity, With<InteractiveSprite>>,
) {
    let target = click.entity.entity();
    if q_sprites.contains(target) {
        // Pick a random color
        let r = (target.to_bits() % 100) as f32 * 0.01;
        let g = (target.to_bits() % 50) as f32 * 0.02;
        let b = (target.to_bits() % 25) as f32 * 0.04;

        let random_color = Color::srgb(r, g, b);

        if let Ok(mut mat_handle) = q_materials.get_mut(target) {
            mat_handle.0 = materials.add(random_color);
        }
    }
}

fn sync_ui_to_settings(
    mut settings: ResMut<SceneSettings>,
    toggle_anim: Query<&UToggle, With<AnimationToggle>>,
    seek_speed: Query<&USeekBar, With<SpeedSeekBar>>,
    mut label_query: Query<&mut UTextLabel, With<StatusText>>,
) {
    if let Some(toggle) = toggle_anim.iter().next() {
        settings.enable_animation = toggle.checked;
    }

    if let Some(seek) = seek_speed.iter().next() {
        settings.sprite_speed = seek.value;
    }

    if let Some(mut label) = label_query.iter_mut().next() {
        label.text = format!("Animation Speed: {:.1}", settings.sprite_speed);
    }
}
