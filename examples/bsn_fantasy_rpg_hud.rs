//! # Fantasy RPG Game HUD (Bevy Scene Notation & Virtual Resolution)
//!
//! Demonstrates building an ornate, high-fantasy game HUD powered by Univis UI and
//! Bevy Scene Notation (`bsn!`).
//!
//! ## Architectural Features Demonstrated:
//! - **Virtual Resolution Scaling (`URootUi::virtual_screen`)**: Design against a canonical
//!   1920x1080 canvas with automatic height-matching scaling across arbitrary resolutions and aspect ratios.
//! - **Anchor Positioning (`USelf::anchor_*`)**: Corner and edge anchoring (`anchor_top_left`,
//!   `anchor_top_center`, `anchor_top_right`, `anchor_bottom_center`, `anchor_bottom_left`).
//! - **Built-in Theme & Typography**: High-definition `Theme` typography (`Inter`, `FreeSerif`)
//!   and embedded Lucide vector icons (`Icon::COMPASS`, `Icon::FLAME`, `Icon::HEART`, `Icon::DROPLETS`,
//!   `Icon::ZAP`, `Icon::SHIELD`, `Icon::CROSSHAIR`, `Icon::EYE`, `Icon::MAP_PIN`, `Icon::FEATHER`, `Icon::AWARD`).
//! - **Real-time SDF Text Outlines**: Outlined boss title and health/mana counters for dramatic legibility.
//! - **Classic RPG Status Orbs**: Ruby Health Orb and Sapphire Mana Orb with ornate filigree borders.
//! - **Action Hotbar**: 8 slotted abilities and consumable potions with keybind badges.
//! - **Interactive Potion Consumption**: `On<Pointer<Click>>` observers to heal and restore mana with reactive UI updates.
//! - **Boss Encounter Bar & Celestial Mini-Map**: Multi-layered health tracking and quest objectives.
//! - **3D World Backdrop**: Stylized ancient stone shrine with atmospheric lighting behind the HUD.

use bevy::prelude::*;
use univis_ui::prelude::*;

// ---------------------------------------------------------------------------
// Game State & Components
// ---------------------------------------------------------------------------

#[derive(Resource)]
struct RpgGameState {
    player_hp: f32,
    player_max_hp: f32,
    player_mp: f32,
    player_max_mp: f32,
    boss_hp: f32,
    boss_max_hp: f32,
    health_potions: u32,
    mana_potions: u32,
    last_log: String,
}

impl Default for RpgGameState {
    fn default() -> Self {
        Self {
            player_hp: 3150.0,
            player_max_hp: 4000.0,
            player_mp: 1420.0,
            player_max_mp: 2200.0,
            boss_hp: 68_400.0,
            boss_max_hp: 100_000.0,
            health_potions: 4,
            mana_potions: 6,
            last_log: "Encounter begun with Voidbringer Malakor!".to_string(),
        }
    }
}

/// Marker for the health text readout inside the Ruby Orb.
#[derive(Component, Clone, Default)]
struct HealthReadoutText;

/// Marker for the mana text readout inside the Sapphire Orb.
#[derive(Component, Clone, Default)]
struct ManaReadoutText;

/// Marker for the health potion counter text.
#[derive(Component, Clone, Default)]
struct HealthPotionCountText;

/// Marker for the mana potion counter text.
#[derive(Component, Clone, Default)]
struct ManaPotionCountText;

/// Marker for the boss health percentage bar fill.
#[derive(Component, Clone, Default)]
struct BossHealthBarFill;

/// Marker for the combat chronicle log message.
#[derive(Component, Clone, Default)]
struct CombatLogText;

/// Ambient rotation marker for the background 3D shrine rune.
#[derive(Component)]
struct FloatingShrineRune;

// ---------------------------------------------------------------------------
// Application Entry Point
// ---------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Fantasy RPG HUD (URootUi::virtual_screen & BSN)"
                        .to_string(),
                    resolution: (1280, 720).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<RpgGameState>()
        .add_systems(Startup, (setup_world_backdrop, setup_fantasy_hud).chain())
        .add_systems(Update, (rotate_shrine_rune, sync_hud_state))
        .run();
}

// ---------------------------------------------------------------------------
// 3D Fantasy World Backdrop
// ---------------------------------------------------------------------------

fn setup_world_backdrop(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera overlooking an ancient shrine (renders first)
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.04, 0.05, 0.08)),
            ..default()
        },
        Transform::from_xyz(0.0, 3.2, 7.5).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));

    // 2D Camera for the Fantasy RPG HUD overlay (renders on top of 3D)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // Directional Moonlight
    commands.spawn((
        DirectionalLight {
            illuminance: 1200.0,
            color: Color::srgb(0.65, 0.75, 1.0),
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Warm Torchlight Left
    commands.spawn((
        PointLight {
            intensity: 80_000.0,
            range: 8.0,
            color: Color::srgb(1.0, 0.65, 0.3),
            ..default()
        },
        Transform::from_xyz(-2.2, 1.8, 0.5),
    ));

    // Warm Torchlight Right
    commands.spawn((
        PointLight {
            intensity: 80_000.0,
            range: 8.0,
            color: Color::srgb(1.0, 0.65, 0.3),
            ..default()
        },
        Transform::from_xyz(2.2, 1.8, 0.5),
    ));

    // Stone Ground Platform
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.09, 0.12),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Shrine Altar Base
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.0, 0.8, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.16, 0.22),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.4, 0.0),
    ));

    // Left Stone Pillar
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 3.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.13, 0.18),
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(-2.2, 1.75, 0.5),
    ));

    // Right Stone Pillar
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 3.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.13, 0.18),
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(2.2, 1.75, 0.5),
    ));

    // Floating Ancient Shrine Torus Rune
    commands.spawn((
        FloatingShrineRune,
        Mesh3d(meshes.add(Torus::new(0.12, 0.45))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.75, 0.3),
            emissive: LinearRgba::new(0.4, 0.25, 0.05, 1.0),
            metallic: 0.9,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.6, 0.0).with_rotation(Quat::from_rotation_x(0.6)),
    ));
}

fn rotate_shrine_rune(time: Res<Time>, mut query: Query<&mut Transform, With<FloatingShrineRune>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.4 * time.delta_secs());
        transform.rotate_z(0.2 * time.delta_secs());
    }
}

// ---------------------------------------------------------------------------
// Fantasy RPG HUD via BSN
// ---------------------------------------------------------------------------

fn setup_fantasy_hud(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
    theme: Res<Theme>,
) {
    let ui_cam = camera_query
        .single()
        .expect("Camera2d entity required for HUD");
    let mut root_ui = URootUi::virtual_screen(Vec2::new(1920.0, 1080.0));
    root_ui.camera = UiCameraRef::Entity(ui_cam);

    let text_font = theme.text.font.inter_regular.clone();
    let serif_font = theme.text.font.free_serif.clone();
    let icon_font = theme.icon.font.clone();

    commands
        .spawn_scene(fantasy_hud_scene(text_font, serif_font, icon_font))
        .insert(root_ui);
}

/// The complete HUD scene rendered on a virtual 1920x1080 canvas.
fn fantasy_hud_scene(
    text_font: Handle<Font>,
    serif_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1920.0),
            height: UVal::Px(1080.0),
            background_color: Color::NONE,
        }
        Children [
            // Top Center: Compass Ribbon & Zone Banner
            (hud_top_compass(icon_font.clone(), text_font.clone())),

            // Top Center Sub: Boss Encounter Health Bar
            (hud_boss_encounter_bar(icon_font.clone(), text_font.clone())),

            // Top Left: Character Medallion, Level, Currencies, and Active Buffs
            (hud_character_crest(icon_font.clone(), text_font.clone(), serif_font)),

            // Top Right: Mini-Map Frame & Quest Objective Tracker
            (hud_minimap_and_quests(icon_font.clone(), text_font.clone())),

            // Bottom Left: Combat Chronicle & Event Feed
            (hud_combat_log_box(icon_font.clone(), text_font.clone())),

            // Bottom Center: Master Action Bar (Ruby HP, Hotbar, Sapphire MP)
            (hud_bottom_action_cluster(icon_font, text_font)),
        ]
    }
}

// ---------------------------------------------------------------------------
// Top Center: Compass Ribbon & Zone Banner
// ---------------------------------------------------------------------------

fn hud_top_compass(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        USelf::anchor_top_center(14.0)
        UNode {
            width: UVal::Px(500.0),
            height: UVal::Px(60.0),
            background_color: Color::srgba(0.05, 0.06, 0.09, 0.88),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(16.0, 6.0),
        }
        UBorder {
            color: Color::srgba(0.85, 0.72, 0.35, 0.5),
            width: 1.5,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Zone Name with Compass Icon
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::COMPASS.to_string() },
                        font_size: 13.0,
                        font: { icon_font },
                        color: Color::srgb(0.95, 0.82, 0.45),
                    }),
                    (UText {
                        text: { "SANCTUARY OF ELDORIA • CHAPTER IV".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.95, 0.82, 0.45),
                    })
                ]
            ),
            // Compass Ribbon Bearings
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText { text: { "W".to_string() }, font_size: 11.5, font: { text_font.clone() }, color: Color::srgb(0.55, 0.6, 0.7) }),
                    (UText { text: { "• •".to_string() }, font_size: 9.0, font: { text_font.clone() }, color: Color::srgb(0.35, 0.4, 0.5) }),
                    (UText { text: { "NW".to_string() }, font_size: 11.5, font: { text_font.clone() }, color: Color::srgb(0.55, 0.6, 0.7) }),
                    (UText { text: { "• •".to_string() }, font_size: 9.0, font: { text_font.clone() }, color: Color::srgb(0.35, 0.4, 0.5) }),
                    (UText { text: { "▼ [ N ]".to_string() }, font_size: 13.0, font: { text_font.clone() }, color: Color::srgb(1.0, 0.9, 0.4) }),
                    (UText { text: { "• •".to_string() }, font_size: 9.0, font: { text_font.clone() }, color: Color::srgb(0.35, 0.4, 0.5) }),
                    (UText { text: { "NE".to_string() }, font_size: 11.5, font: { text_font.clone() }, color: Color::srgb(0.55, 0.6, 0.7) }),
                    (UText { text: { "• •".to_string() }, font_size: 9.0, font: { text_font.clone() }, color: Color::srgb(0.35, 0.4, 0.5) }),
                    (UText { text: { "E".to_string() }, font_size: 11.5, font: { text_font }, color: Color::srgb(0.55, 0.6, 0.7) }),
                ]
            )
        ]
    }
}

// ---------------------------------------------------------------------------
// Top Center Sub: Boss Encounter Health Bar
// ---------------------------------------------------------------------------

fn hud_boss_encounter_bar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        USelf::anchor_top_center(82.0)
        UNode {
            width: UVal::Px(640.0),
            height: UVal::Px(46.0),
            background_color: Color::srgba(0.04, 0.05, 0.08, 0.92),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(14.0, 6.0),
        }
        UBorder {
            color: Color::srgba(0.75, 0.2, 0.25, 0.7),
            width: 1.5,
            radius: UCornerRadius::all(6.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            // Boss Title
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::FLAME.to_string() },
                                font_size: 13.0,
                                font: { icon_font },
                                color: Color::srgb(1.0, 0.35, 0.35),
                            }),
                            (UText {
                                text: { "MALAKOR THE VOIDBRINGER (ANCIENT WYRM)".to_string() },
                                font_size: 12.5,
                                font: { text_font.clone() },
                                color: Color::srgb(1.0, 0.45, 0.45),
                                outline_width: 1.5,
                                outline_color: Color::srgba(0.0, 0.0, 0.0, 0.8),
                            })
                        ]
                    ),
                    (UText {
                        text: { "68% • PHASE II".to_string() },
                        font_size: 11.5,
                        font: { text_font },
                        color: Color::srgb(0.95, 0.8, 0.4),
                    })
                ]
            ),
            // Boss Health Track & Filled Meter
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(12.0),
                    background_color: Color::srgb(0.12, 0.14, 0.18),
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    (
                        BossHealthBarFill
                        UNode {
                            width: UVal::Percent(0.684),
                            height: UVal::Percent(1.0),
                            background_color: Color::srgb(0.85, 0.18, 0.22),
                            border_radius: UCornerRadius::all(4.0),
                        }
                    )
                ]
            )
        ]
    }
}

// ---------------------------------------------------------------------------
// Top Left: Character Crest, Currencies & Buffs
// ---------------------------------------------------------------------------

fn hud_character_crest(
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
    serif_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        USelf::anchor_top_left(Vec2::new(28.0, 20.0))
        UNode {
            background_color: Color::srgba(0.05, 0.06, 0.1, 0.9),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(12.0),
        }
        UBorder {
            color: Color::srgba(0.85, 0.72, 0.35, 0.45),
            width: 1.5,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            // Top Row: Avatar Frame + Info
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    // Medallion Avatar Circle
                    (
                        UNode {
                            width: UVal::Px(52.0),
                            height: UVal::Px(52.0),
                            background_color: Color::srgb(0.16, 0.2, 0.3),
                            border_radius: UCornerRadius::all(26.0),
                        }
                        UBorder {
                            color: Color::srgb(0.92, 0.76, 0.35),
                            width: 2.0,
                            radius: UCornerRadius::all(26.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::AWARD.to_string() },
                                font_size: 22.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(1.0, 0.88, 0.45),
                            })
                        ]
                    ),
                    // Name, Title, and Exp Bar
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 3.0,
                        }
                        Children [
                            (UText {
                                text: { "Archmage Valerius".to_string() },
                                font_size: 15.0,
                                font: { serif_font },
                                color: Color::srgb(0.95, 0.96, 0.98),
                            }),
                            (UText {
                                text: { "Level 58 Grand Invoker".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.65, 0.75, 0.9),
                            }),
                            // Experience Progress Track
                            (
                                UNode {
                                    width: UVal::Px(120.0),
                                    height: UVal::Px(5.0),
                                    background_color: Color::srgb(0.1, 0.12, 0.16),
                                    border_radius: UCornerRadius::all(2.5),
                                }
                                Children [
                                    (
                                        UNode {
                                            width: UVal::Percent(0.74),
                                            height: UVal::Percent(1.0),
                                            background_color: Color::srgb(0.5, 0.4, 0.85),
                                            border_radius: UCornerRadius::all(2.5),
                                        }
                                    )
                                ]
                            )
                        ]
                    )
                ]
            ),
            // Currency Row
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.03, 0.04, 0.07, 0.7),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(10.0, 5.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    gap: 16.0,
                }
                Children [
                    (UText { text: { "[G] 4,820 Gold".to_string() }, font_size: 12.0, font: { text_font.clone() }, color: Color::srgb(0.95, 0.85, 0.35) }),
                    (UText { text: { "[V] 128 Void Shards".to_string() }, font_size: 12.0, font: { text_font.clone() }, color: Color::srgb(0.65, 0.88, 1.0) }),
                ]
            ),
            // Active Buffs Row
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 6.0,
                }
                Children [
                    (buff_badge(Icon::FLAME, Color::srgb(1.0, 0.45, 0.2), "24s", icon_font.clone(), text_font.clone())),
                    (buff_badge(Icon::SHIELD, Color::srgb(0.95, 0.8, 0.4), "18s", icon_font.clone(), text_font.clone())),
                    (buff_badge(Icon::ZAP, Color::srgb(0.5, 0.8, 1.0), "52m", icon_font.clone(), text_font.clone())),
                    (buff_badge(Icon::HEART, Color::srgb(0.4, 0.9, 0.5), "1h", icon_font, text_font)),
                ]
            )
        ]
    }
}

fn buff_badge(
    icon_glyph: &'static str,
    color: Color,
    time: &'static str,
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(36.0),
            height: UVal::Px(36.0),
            background_color: Color::srgba(0.1, 0.14, 0.22, 0.8),
            border_radius: UCornerRadius::all(5.0),
            padding: USides::all(2.0),
        }
        UBorder {
            color: Color::srgba(0.7, 0.6, 0.3, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(5.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText { text: { icon_glyph.to_string() }, font_size: 13.0, font: { icon_font }, color: { color } }),
            (UText { text: { time.to_string() }, font_size: 9.0, font: { text_font }, color: Color::srgb(0.8, 0.85, 0.95) }),
        ]
    }
}

// ---------------------------------------------------------------------------
// Top Right: Mini-Map Frame & Quest Objectives
// ---------------------------------------------------------------------------

fn hud_minimap_and_quests(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        USelf::anchor_top_right(Vec2::new(28.0, 20.0))
        UNode {
            width: UVal::Px(240.0),
            background_color: Color::NONE,
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::FlexEnd,
            gap: 12.0,
        }
        Children [
            // Circular Mini-Map Frame
            (
                UNode {
                    width: UVal::Px(160.0),
                    height: UVal::Px(160.0),
                    background_color: Color::srgba(0.06, 0.08, 0.12, 0.9),
                    border_radius: UCornerRadius::all(80.0),
                }
                UBorder {
                    color: Color::srgb(0.85, 0.72, 0.35),
                    width: 2.0,
                    radius: UCornerRadius::all(80.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                }
                Children [
                    // Player Blip
                    (
                        UNode {
                            width: UVal::Px(8.0),
                            height: UVal::Px(8.0),
                            background_color: Color::srgb(0.2, 0.9, 0.4),
                            border_radius: UCornerRadius::all(4.0),
                        }
                    ),
                    (UText {
                        text: { "YOU".to_string() },
                        font_size: 9.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.2, 0.9, 0.4),
                    }),
                    (UText {
                        text: { "X: 384  Y: 712".to_string() },
                        font_size: 9.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.6, 0.7, 0.8),
                    })
                ]
            ),
            // Quest Objective Card
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.05, 0.06, 0.09, 0.85),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(10.0),
                }
                UBorder {
                    color: Color::srgba(0.85, 0.72, 0.35, 0.35),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 4.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::MAP_PIN.to_string() },
                                font_size: 11.0,
                                font: { icon_font },
                                color: Color::srgb(0.85, 0.72, 0.35),
                            }),
                            (UText {
                                text: { "CURRENT QUEST".to_string() },
                                font_size: 9.5,
                                font: { text_font.clone() },
                                color: Color::srgb(0.85, 0.72, 0.35),
                            })
                        ]
                    ),
                    (UText {
                        text: { "The Sunken Relic of Eldoria".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::WHITE,
                    }),
                    (UText {
                        text: { "• Retrieve the Sunstone Core (2/3)".to_string() },
                        font_size: 10.5,
                        font: { text_font.clone() },
                        color: Color::srgb(0.7, 0.8, 0.9),
                    }),
                    (UText {
                        text: { "• Defeat Voidbringer Malakor (0/1)".to_string() },
                        font_size: 10.5,
                        font: { text_font },
                        color: Color::srgb(1.0, 0.45, 0.45),
                    })
                ]
            )
        ]
    }
}

// ---------------------------------------------------------------------------
// Bottom Left: Combat Chronicle & Event Feed
// ---------------------------------------------------------------------------

fn hud_combat_log_box(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        USelf::anchor_bottom_left(Vec2::new(28.0, 20.0))
        UNode {
            width: UVal::Px(320.0),
            background_color: Color::srgba(0.04, 0.05, 0.08, 0.85),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(10.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.35, 0.5, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 6.0,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 5.0,
                }
                Children [
                    (UText {
                        text: { Icon::FEATHER.to_string() },
                        font_size: 11.0,
                        font: { icon_font },
                        color: Color::srgb(0.55, 0.65, 0.8),
                    }),
                    (UText {
                        text: { "COMBAT CHRONICLE".to_string() },
                        font_size: 10.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.55, 0.65, 0.8),
                    })
                ]
            ),
            (
                CombatLogText
                UText {
                    text: { "[Log] Encounter begun with Voidbringer Malakor!\n[Combat] Boss HP: 68400 / 100000\n[Stats] HP: 3150/4000  MP: 1420/2200".to_string() },
                    font_size: 10.5,
                    font: { text_font.clone() },
                    color: Color::srgb(0.85, 0.9, 0.95),
                }
            ),
            (UText {
                text: { "Click Q / E to drink potions or 1-6 for abilities".to_string() },
                font_size: 9.5,
                font: { text_font },
                color: Color::srgb(0.45, 0.5, 0.6),
            })
        ]
    }
}

// ---------------------------------------------------------------------------
// Bottom Center: Master Action Bar (Health Orb, Hotbar, Mana Orb)
// ---------------------------------------------------------------------------

fn hud_bottom_action_cluster(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        USelf::anchor_bottom_center(20.0)
        UNode {
            background_color: Color::NONE,
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::FlexEnd,
            gap: 16.0,
        }
        Children [
            // Left: Ruby Health Orb
            (hud_health_orb(icon_font.clone(), text_font.clone())),

            // Center: 8-Slot Action Hotbar
            (hud_hotbar_strip(icon_font.clone(), text_font.clone())),

            // Right: Sapphire Mana Orb
            (hud_mana_orb(icon_font, text_font)),
        ]
    }
}

/// Circular Ruby Health Orb with gold filigree ring.
fn hud_health_orb(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(130.0),
            height: UVal::Px(130.0),
            background_color: Color::srgb(0.72, 0.1, 0.16),
            border_radius: UCornerRadius::all(65.0),
        }
        UBorder {
            color: Color::srgb(0.92, 0.76, 0.35),
            width: 3.5,
            radius: UCornerRadius::all(65.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 2.0,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 3.0,
                }
                Children [
                    (UText {
                        text: { Icon::HEART.to_string() },
                        font_size: 11.0,
                        font: { icon_font },
                        color: Color::srgb(1.0, 0.8, 0.8),
                    }),
                    (UText {
                        text: { "HEALTH".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(1.0, 0.8, 0.8),
                    })
                ]
            ),
            (
                HealthReadoutText
                UText {
                    text: { "3,150".to_string() },
                    font_size: 24.0,
                    font: { text_font.clone() },
                    color: Color::WHITE,
                    outline_width: 1.5,
                    outline_color: Color::BLACK,
                }
            ),
            (UText {
                text: { "/ 4,000".to_string() },
                font_size: 12.0,
                font: { text_font },
                color: Color::srgb(1.0, 0.8, 0.8),
            })
        ]
    }
}

/// Circular Sapphire Mana Orb with silver-gold filigree ring.
fn hud_mana_orb(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(130.0),
            height: UVal::Px(130.0),
            background_color: Color::srgb(0.1, 0.32, 0.78),
            border_radius: UCornerRadius::all(65.0),
        }
        UBorder {
            color: Color::srgb(0.78, 0.84, 0.95),
            width: 3.5,
            radius: UCornerRadius::all(65.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 2.0,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 3.0,
                }
                Children [
                    (UText {
                        text: { Icon::DROPLETS.to_string() },
                        font_size: 11.0,
                        font: { icon_font },
                        color: Color::srgb(0.8, 0.9, 1.0),
                    }),
                    (UText {
                        text: { "MANA".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.8, 0.9, 1.0),
                    })
                ]
            ),
            (
                ManaReadoutText
                UText {
                    text: { "1,420".to_string() },
                    font_size: 24.0,
                    font: { text_font.clone() },
                    color: Color::WHITE,
                    outline_width: 1.5,
                    outline_color: Color::BLACK,
                }
            ),
            (UText {
                text: { "/ 2,200".to_string() },
                font_size: 12.0,
                font: { text_font },
                color: Color::srgb(0.8, 0.9, 1.0),
            })
        ]
    }
}

/// 8-Slot Horizontal Action Hotbar
fn hud_hotbar_strip(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.04, 0.05, 0.08, 0.92),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(8.0),
        }
        UBorder {
            color: Color::srgba(0.85, 0.72, 0.35, 0.5),
            width: 1.5,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 8.0,
        }
        Children [
            (ability_slot("1", Icon::FLAME, Color::srgb(1.0, 0.45, 0.2), "Fireball", 1, icon_font.clone(), text_font.clone())),
            (ability_slot("2", Icon::CLOUD_SNOW, Color::srgb(0.4, 0.85, 1.0), "Frost Nova", 2, icon_font.clone(), text_font.clone())),
            (ability_slot("3", Icon::ZAP, Color::srgb(1.0, 0.9, 0.3), "Lightning", 3, icon_font.clone(), text_font.clone())),
            (ability_slot("4", Icon::SHIELD, Color::srgb(0.95, 0.8, 0.4), "Aegis", 4, icon_font.clone(), text_font.clone())),
            (ability_slot("5", Icon::CROSSHAIR, Color::srgb(1.0, 0.3, 0.3), "Meteor", 5, icon_font.clone(), text_font.clone())),
            (ability_slot("6", Icon::EYE, Color::srgb(0.75, 0.5, 1.0), "Blink", 6, icon_font.clone(), text_font.clone())),
            (potion_slot_health(icon_font.clone(), text_font.clone())),
            (potion_slot_mana(icon_font, text_font)),
        ]
    }
}

/// Helper for standard ability slots (1 - 6)
fn ability_slot(
    key: &'static str,
    icon_glyph: &'static str,
    glyph_color: Color,
    name: &'static str,
    slot_id: u32,
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(78.0),
            height: UVal::Px(68.0),
            background_color: Color::srgb(0.1, 0.12, 0.18),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::all(4.0),
        }
        UBorder {
            color: Color::srgba(0.65, 0.55, 0.35, 0.5),
            width: 1.0,
            radius: UCornerRadius::all(6.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgb(0.1, 0.12, 0.18),
            hovered: Color::srgb(0.2, 0.25, 0.35),
            pressed: Color::srgb(0.3, 0.15, 0.18),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Top Row: Keybind Badge
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Start,
                }
                Children [
                    (UText {
                        text: { key.to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.85, 0.72, 0.35),
                    })
                ]
            ),
            // Center Glyph Icon
            (UText {
                text: { icon_glyph.to_string() },
                font_size: 18.0,
                font: { icon_font },
                color: { glyph_color },
            }),
            // Bottom Name Micro-label
            (UText {
                text: { name.to_string() },
                font_size: 8.5,
                font: { text_font },
                color: Color::srgb(0.75, 0.8, 0.9),
            })
        ]
        on(move |_event: On<Pointer<Click>>, mut state: ResMut<RpgGameState>| {
            let dmg = slot_id as f32 * 680.0;
            state.boss_hp = (state.boss_hp - dmg).max(0.0);
            state.player_mp = (state.player_mp - 80.0).max(0.0);
            state.last_log = format!("Cast {}! Struck Voidbringer for {:.0} dmg (-80 MP)", name, dmg);
            info!("{}", state.last_log);
        })
    }
}

/// Interactive Slot Q: Health Potion
fn potion_slot_health(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(78.0),
            height: UVal::Px(68.0),
            background_color: Color::srgb(0.22, 0.08, 0.1),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::all(4.0),
        }
        UBorder {
            color: Color::srgb(0.9, 0.25, 0.3),
            width: 1.5,
            radius: UCornerRadius::all(6.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgb(0.22, 0.08, 0.1),
            hovered: Color::srgb(0.35, 0.12, 0.16),
            pressed: Color::srgb(0.5, 0.1, 0.15),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Top Row: Keybind 'Q'
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Start,
                }
                Children [
                    (UText {
                        text: { "Q".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(1.0, 0.5, 0.5),
                    })
                ]
            ),
            // Center Potion Icon
            (UText {
                text: { Icon::HEART.to_string() },
                font_size: 18.0,
                font: { icon_font },
                color: Color::srgb(1.0, 0.35, 0.35),
            }),
            // Bottom Count Text
            (
                HealthPotionCountText
                UText {
                    text: { "x4".to_string() },
                    font_size: 10.5,
                    font: { text_font },
                    color: Color::srgb(1.0, 0.85, 0.85),
                }
            )
        ]
        on(|_event: On<Pointer<Click>>, mut state: ResMut<RpgGameState>| {
            if state.health_potions > 0 {
                state.health_potions -= 1;
                state.player_hp = (state.player_hp + 1000.0).min(state.player_max_hp);
                state.last_log = format!("Consumed Health Potion! Restored 1000 HP ({} left)", state.health_potions);
                info!("{}", state.last_log);
            }
        })
    }
}

/// Interactive Slot E: Mana Potion
fn potion_slot_mana(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(78.0),
            height: UVal::Px(68.0),
            background_color: Color::srgb(0.08, 0.14, 0.26),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::all(4.0),
        }
        UBorder {
            color: Color::srgb(0.3, 0.55, 0.95),
            width: 1.5,
            radius: UCornerRadius::all(6.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgb(0.08, 0.14, 0.26),
            hovered: Color::srgb(0.12, 0.22, 0.4),
            pressed: Color::srgb(0.15, 0.3, 0.6),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Top Row: Keybind 'E'
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Start,
                }
                Children [
                    (UText {
                        text: { "E".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.5, 0.75, 1.0),
                    })
                ]
            ),
            // Center Potion Icon
            (UText {
                text: { Icon::DROPLETS.to_string() },
                font_size: 18.0,
                font: { icon_font },
                color: Color::srgb(0.4, 0.8, 1.0),
            }),
            // Bottom Count Text
            (
                ManaPotionCountText
                UText {
                    text: { "x6".to_string() },
                    font_size: 10.5,
                    font: { text_font },
                    color: Color::srgb(0.8, 0.92, 1.0),
                }
            )
        ]
        on(|_event: On<Pointer<Click>>, mut state: ResMut<RpgGameState>| {
            if state.mana_potions > 0 {
                state.mana_potions -= 1;
                state.player_mp = (state.player_mp + 750.0).min(state.player_max_mp);
                state.last_log = format!("Consumed Mana Potion! Restored 750 MP ({} left)", state.mana_potions);
                info!("{}", state.last_log);
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Reactive HUD State Synchronization System
// ---------------------------------------------------------------------------

fn sync_hud_state(
    state: Res<RpgGameState>,
    mut hp_text_query: Query<
        &mut UText,
        (
            With<HealthReadoutText>,
            Without<ManaReadoutText>,
            Without<HealthPotionCountText>,
            Without<ManaPotionCountText>,
            Without<CombatLogText>,
        ),
    >,
    mut mp_text_query: Query<
        &mut UText,
        (
            With<ManaReadoutText>,
            Without<HealthReadoutText>,
            Without<HealthPotionCountText>,
            Without<ManaPotionCountText>,
            Without<CombatLogText>,
        ),
    >,
    mut hp_pot_query: Query<
        &mut UText,
        (
            With<HealthPotionCountText>,
            Without<HealthReadoutText>,
            Without<ManaReadoutText>,
            Without<ManaPotionCountText>,
            Without<CombatLogText>,
        ),
    >,
    mut mp_pot_query: Query<
        &mut UText,
        (
            With<ManaPotionCountText>,
            Without<HealthReadoutText>,
            Without<ManaReadoutText>,
            Without<HealthPotionCountText>,
            Without<CombatLogText>,
        ),
    >,
    mut log_query: Query<
        &mut UText,
        (
            With<CombatLogText>,
            Without<HealthReadoutText>,
            Without<ManaReadoutText>,
            Without<HealthPotionCountText>,
            Without<ManaPotionCountText>,
        ),
    >,
    mut boss_bar_query: Query<&mut UNode, With<BossHealthBarFill>>,
) {
    if !state.is_changed() {
        return;
    }

    for mut text in &mut hp_text_query {
        text.text = format!("{:.0}", state.player_hp);
    }

    for mut text in &mut mp_text_query {
        text.text = format!("{:.0}", state.player_mp);
    }

    for mut text in &mut hp_pot_query {
        text.text = format!("x{}", state.health_potions);
    }

    for mut text in &mut mp_pot_query {
        text.text = format!("x{}", state.mana_potions);
    }

    for mut text in &mut log_query {
        text.text = format!(
            "[Log] {}\n[Combat] Boss HP: {:.0} / {:.0}\n[Stats] HP: {:.0}/4000  MP: {:.0}/2200",
            state.last_log, state.boss_hp, state.boss_max_hp, state.player_hp, state.player_mp
        );
    }

    for mut node in &mut boss_bar_query {
        let pct = (state.boss_hp / state.boss_max_hp).clamp(0.0, 1.0);
        node.width = UVal::Percent(pct);
    }
}
