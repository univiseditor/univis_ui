use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use univis_ui::prelude::*;

// 1. Magical rune enum
#[derive(Clone, Copy, Debug, PartialEq)]
enum MagicRune {
    Fire,
    Frost,
    Chaos,
    Nature,
    Storm,
    Void,
}

// 2. Resource to store spellcrafting state
#[derive(Resource)]
struct RuneState {
    active_rune: MagicRune,
    mana: f32,
}

impl Default for RuneState {
    fn default() -> Self {
        Self {
            active_rune: MagicRune::Frost,
            mana: 0.8,
        }
    }
}

// 3. Components
#[derive(Component)]
struct MagicCrystal;

#[derive(Component)]
struct RuneStatusText;

#[derive(Component)]
struct ManaBar;

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
                title: "Univis UI - Magic Rune Spellweaver Table".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<RuneState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_crystal,
                update_crystal_material,
                update_status_text,
                update_mana_bar,
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
    // --- 1. CAMERAS & LIGHTING ---
    // 3D Camera for magic table observation
    let camera_3d = commands
        .spawn((
            Camera3d::default(),
            MainCamera {
                orbit_distance: 4.5,
                pitch: -0.22,
                yaw: -0.4,
            },
            Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
        ))
        .id();

    // 2D Camera for RPG Screen HUD overlay
    let camera_2d = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
        ))
        .id();

    // Soft global ambient lighting
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.08, 0.05, 0.12),
        brightness: 80.0,
        ..default()
    });

    // Golden pedestal light
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.8, 0.4),
            intensity: 600.0,
            range: 8.0,
            ..default()
        },
        Transform::from_xyz(1.5, 0.2, -1.0),
    ));

    // Dynamic rune glow light (placed near crystal)
    commands.spawn((
        PointLight {
            color: Color::srgb(0.15, 0.75, 0.95),
            intensity: 1500.0,
            range: 12.0,
            ..default()
        },
        Transform::from_xyz(1.5, 0.8, -1.0),
    ));

    // Directional moon-like light
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: 1200.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // --- 2. 3D WORLD OBJECTS (THE MAGIC CRYSTAL) ---
    // We spawn a floating crystal and magical orbital rings
    let crystal_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.75, 0.95),
        emissive: LinearRgba::rgb(0.3, 1.5, 2.0),
        metallic: 0.95,
        perceptual_roughness: 0.05,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(crystal_material.clone()),
        Transform::from_xyz(1.5, 0.8, -1.0),
        MagicCrystal,
    ));

    // Spawn orbital magical rings
    let gold_ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.75, 0.2),
        metallic: 0.9,
        perceptual_roughness: 0.2,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.65, 0.02))),
        MeshMaterial3d(gold_ring_mat),
        Transform::from_xyz(1.5, 0.8, -1.0).with_rotation(Quat::from_rotation_x(1.2)),
        MagicCrystal,
    ));

    // --- 3. HOLOGRAPHIC SPELLWEAVER PANEL (WORLD SPACE UI) ---
    let terminal_root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera_3d),
                ..URootUi::world_3d(Vec2::new(760.0, 560.0))
            },
            Transform::from_xyz(-0.7, 0.8, 0.0).with_rotation(Quat::from_rotation_y(0.45)),
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

    // Magic tablet frame using premium SDF Glassmorphism
    let main_frame = commands
        .spawn((
            ChildOf(terminal_root),
            UPanel::glass(),
            UNode {
                width: UVal::Px(720.0),
                height: UVal::Px(520.0),
                padding: USides::all(20.0),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.8, 0.5, 1.0, 0.25),
                width: 1.5,
                radius: UCornerRadius::all(28.0),
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

    // LEFT PANEL: The Spellweaver Rune Ring (Radial Layout)
    let left_col = commands
        .spawn((
            ChildOf(main_frame),
            UNode {
                width: UVal::Px(310.0),
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
            text: "RUNE CIRCLE ASSEMBLY".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.8, 0.5, 1.0),
            ..default()
        },
        UNode::default(),
    ));

    // Radial Container: Distributes spells orbital style
    let radial_ring = commands
        .spawn((
            ChildOf(left_col),
            UNode {
                width: UVal::Px(230.0),
                height: UVal::Px(230.0),
                background_color: Color::srgba(0.08, 0.04, 0.15, 0.4),
                border_radius: UCornerRadius::all(115.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.8, 0.5, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(115.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Radial,
                ..default()
            },
        ))
        .id();

    // Spawning 6 different magical runes inside the radial layout
    let runes = [
        (MagicRune::Fire, "FIRE", Color::srgb(0.95, 0.25, 0.15)),
        (MagicRune::Frost, "FROST", Color::srgb(0.15, 0.75, 0.95)),
        (MagicRune::Chaos, "CHAOS", Color::srgb(0.75, 0.15, 0.85)),
        (MagicRune::Nature, "LIFE", Color::srgb(0.25, 0.85, 0.35)),
        (MagicRune::Storm, "STORM", Color::srgb(0.95, 0.85, 0.15)),
        (MagicRune::Void, "VOID", Color::srgb(0.4, 0.4, 0.5)),
    ];

    for (rune_type, label, color) in runes {
        // Subtle color adaptations for hover/press states
        let hover_color = Color::srgba(
            color.to_linear().red * 1.2,
            color.to_linear().green * 1.2,
            color.to_linear().blue * 1.2,
            0.9,
        );
        let press_color = Color::srgba(
            color.to_linear().red * 0.7,
            color.to_linear().green * 0.7,
            color.to_linear().blue * 0.7,
            1.0,
        );

        let chip = commands
            .spawn((
                ChildOf(radial_ring),
                UButton::secondary(),
                UNode {
                    width: UVal::Px(56.0),
                    height: UVal::Px(56.0),
                    border_radius: UCornerRadius::all(28.0),
                    background_color: color,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    ..default()
                },
                UInteractionColors {
                    normal: color,
                    hovered: hover_color,
                    pressed: press_color,
                },
            ))
            .observe(move |_: On<Pointer<Click>>, mut state: ResMut<RuneState>| {
                state.active_rune = rune_type;
            })
            .id();

        commands.spawn((
            ChildOf(chip),
            UTextLabel {
                text: label.to_string(),
                font_size: 10.0,
                color: Color::WHITE,
                ..default()
            },
            UNode::default(),
        ));
    }

    commands.spawn((
        ChildOf(left_col),
        UTextLabel {
            text: "Orbit click to select focus matrix".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.6, 0.5, 0.7),
            ..default()
        },
        UNode::default(),
    ));

    // Vertical Divider
    commands.spawn((ChildOf(main_frame), UDivider::horizontal()));

    // RIGHT PANEL: Diagnostic telemetry and Masonry inventory
    let right_col = commands
        .spawn((
            ChildOf(main_frame),
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

    // Active spell indicator block
    let top_status = commands
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
        ChildOf(top_status),
        RuneStatusText,
        UTextLabel {
            text: "ACTIVE RUNE: FROST MATRIX".to_string(),
            font_size: 20.0,
            color: Color::srgb(0.15, 0.75, 0.95),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(top_status),
        UTextLabel {
            text: "Spellcraft terminal active. Magic crystal synced to ECS.".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.6, 0.65, 0.75),
            ..default()
        },
        UNode::default(),
    ));

    // Inventory Title
    commands.spawn((
        ChildOf(right_col),
        UTextLabel {
            text: "RUNE INVENTORY (MASONRY GRID)".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.8, 0.7, 0.9),
            ..default()
        },
        UNode::default(),
    ));

    // Masonry Container: Lays out children of different heights in columns automatically
    let masonry_inventory = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(280.0),
                padding: USides::all(10.0),
                background_color: Color::srgba(0.08, 0.05, 0.15, 0.4),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.8, 0.5, 1.0, 0.1),
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Masonry,
                grid_columns: 3,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(8.0),
                        column_gap: Some(8.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // Spawn cards with varied heights (which standard Bevy Flexbox cannot fit neatly without gaps)
    let inventory_items = [
        ("Scroll", 95.0, Color::srgb(0.8, 0.3, 0.9)),
        ("Potion", 70.0, Color::srgb(0.2, 0.8, 0.4)),
        ("Shard", 115.0, Color::srgb(0.1, 0.7, 0.9)),
        ("Relic", 80.0, Color::srgb(0.9, 0.6, 0.1)),
        ("Gem", 60.0, Color::srgb(0.95, 0.2, 0.3)),
        ("Tome", 125.0, Color::srgb(0.5, 0.4, 0.9)),
    ];

    for (name, height, item_color) in inventory_items {
        let card = commands
            .spawn((
                ChildOf(masonry_inventory),
                UNode {
                    width: UVal::Flex(1.0),
                    height: UVal::Px(height),
                    padding: USides::all(8.0),
                    background_color: Color::srgba(
                        item_color.to_linear().red,
                        item_color.to_linear().green,
                        item_color.to_linear().blue,
                        0.15,
                    ),
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(
                        item_color.to_linear().red,
                        item_color.to_linear().green,
                        item_color.to_linear().blue,
                        0.4,
                    ),
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                    offset: 0.0,
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            UTextLabel {
                text: name.to_string(),
                font_size: 11.0,
                color: item_color,
                ..default()
            },
            UNode::default(),
        ));
        commands.spawn((
            ChildOf(card),
            UTextLabel {
                text: format!("{height:.0}px"),
                font_size: 9.0,
                color: Color::srgba(1.0, 1.0, 1.0, 0.6),
                ..default()
            },
            UNode::default(),
        ));
    }

    // --- 4. SCREEN-SPACE RPG HUD ---
    let hud_root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Entity(camera_2d),
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

    // Top archmage panel
    let top_hud = commands
        .spawn((
            ChildOf(hud_root),
            UPanel::glass(),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(20.0, 12.0),
                border_radius: UCornerRadius::all(16.0),
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

    let character_info = commands
        .spawn((
            ChildOf(top_hud),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Glowing level badge
    let badge = commands
        .spawn((
            ChildOf(character_info),
            UNode {
                padding: USides::axes(10.0, 4.0),
                background_color: Color::srgb(0.8, 0.5, 1.0),
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
        ChildOf(badge),
        UTextLabel {
            text: "LVL 50".to_string(),
            font_size: 11.0,
            color: Color::BLACK,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(character_info),
        UTextLabel {
            text: "Archmage Eldrin".to_string(),
            font_size: 15.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    // Mana container (uses UProgressBar widget)
    let mana_wrap = commands
        .spawn((
            ChildOf(top_hud),
            UNode {
                width: UVal::Px(240.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(mana_wrap),
        UTextLabel {
            text: "SPELLWEAVING MANA RESONANCE".to_string(),
            font_size: 10.0,
            color: Color::srgb(0.7, 0.6, 0.8),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(mana_wrap),
        ManaBar,
        UProgressBar {
            value: 0.8,
            ..default()
        },
    ));

    // Bottom camera manual
    let bottom_hud = commands
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
        ChildOf(bottom_hud),
        UTextLabel {
            text: "[RMB + Drag] Orbit Table Camera  |  [Scroll] Zoom Crystal".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.6, 0.5, 0.7),
            ..default()
        },
        UNode::default(),
    ));
}

// System to animate floating magic crystal rotation and scale
fn animate_crystal(
    time: Res<Time>,
    state: Res<RuneState>,
    mut query: Query<&mut Transform, With<MagicCrystal>>,
) {
    let t = time.elapsed_secs();
    // Animation rotation speed adapts dynamically to selected rune
    let spin_speed = match state.active_rune {
        MagicRune::Fire => 3.5,
        MagicRune::Frost => 0.8,
        MagicRune::Chaos => 5.0,
        MagicRune::Nature => 1.5,
        MagicRune::Storm => 6.0,
        MagicRune::Void => 0.4,
    };

    for (i, mut transform) in query.iter_mut().enumerate() {
        if i == 0 {
            // Main sphere moves up/down gently
            transform.translation.y = 0.8 + (t * 2.0).sin() * 0.08;
            transform.rotate_y(time.delta_secs() * spin_speed * 0.4);
        } else {
            // Gold rings spin opposite
            transform.rotate_y(-time.delta_secs() * spin_speed);
            transform.rotate_x(time.delta_secs() * spin_speed * 0.3);
        }
    }
}

// System to update standard material parameters when active rune changes
fn update_crystal_material(
    state: Res<RuneState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<MagicCrystal>>,
) {
    if state.is_changed() {
        for mat_handle in &query {
            if let Some(mut material) = materials.get_mut(&mat_handle.0) {
                let (base, emissive) = match state.active_rune {
                    MagicRune::Fire => (
                        Color::srgb(0.95, 0.25, 0.15),
                        LinearRgba::rgb(2.5, 0.6, 0.2),
                    ),
                    MagicRune::Frost => (
                        Color::srgb(0.15, 0.75, 0.95),
                        LinearRgba::rgb(0.3, 1.6, 2.5),
                    ),
                    MagicRune::Chaos => (
                        Color::srgb(0.75, 0.15, 0.85),
                        LinearRgba::rgb(1.8, 0.3, 2.5),
                    ),
                    MagicRune::Nature => (
                        Color::srgb(0.25, 0.85, 0.35),
                        LinearRgba::rgb(0.4, 2.5, 0.6),
                    ),
                    MagicRune::Storm => (
                        Color::srgb(0.95, 0.85, 0.15),
                        LinearRgba::rgb(2.5, 2.0, 0.3),
                    ),
                    MagicRune::Void => {
                        (Color::srgb(0.1, 0.08, 0.18), LinearRgba::rgb(0.2, 0.1, 0.4))
                    }
                };
                material.base_color = base;
                material.emissive = emissive;
            }
        }
    }
}

// System to update spellweaving label text
fn update_status_text(
    state: Res<RuneState>,
    mut query: Query<&mut UTextLabel, With<RuneStatusText>>,
) {
    if state.is_changed() {
        for mut label in &mut query {
            let (text, color) = match state.active_rune {
                MagicRune::Fire => ("ACTIVE RUNE: FIRE MATRIX", Color::srgb(0.95, 0.25, 0.15)),
                MagicRune::Frost => ("ACTIVE RUNE: FROST MATRIX", Color::srgb(0.15, 0.75, 0.95)),
                MagicRune::Chaos => ("ACTIVE RUNE: CHAOS MATRIX", Color::srgb(0.75, 0.15, 0.85)),
                MagicRune::Nature => ("ACTIVE RUNE: LIFE MATRIX", Color::srgb(0.25, 0.85, 0.35)),
                MagicRune::Storm => ("ACTIVE RUNE: STORM MATRIX", Color::srgb(0.95, 0.85, 0.15)),
                MagicRune::Void => ("ACTIVE RUNE: VOID MATRIX", Color::srgb(0.5, 0.4, 0.6)),
            };
            label.text = text.to_string();
            label.color = color;
        }
    }
}

// System to simulate Mana fluctuations in HUD
fn update_mana_bar(
    time: Res<Time>,
    mut state: ResMut<RuneState>,
    mut query: Query<&mut UProgressBar, With<ManaBar>>,
) {
    state.mana = (time.elapsed_secs() * 0.12).cos() * 0.4 + 0.6;
    for mut progress in &mut query {
        progress.value = state.mana;
    }
}

// Camera orbiting system
fn camera_control(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Some((mut transform, mut camera)) = camera_query.iter_mut().next() else {
        return;
    };

    // Scroll zoom
    let zoom_delta = scroll.delta.y;
    if zoom_delta.abs() > 0.0 {
        camera.orbit_distance -= zoom_delta * 0.2;
        camera.orbit_distance = camera.orbit_distance.clamp(2.0, 8.0);
    }

    // Right drag orbit
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
    let target = Vec3::new(0.0, 0.8, 0.0);
    transform.translation = target + rotation.mul_vec3(Vec3::new(0.0, 0.0, camera.orbit_distance));
    transform.look_at(target, Vec3::Y);
}
