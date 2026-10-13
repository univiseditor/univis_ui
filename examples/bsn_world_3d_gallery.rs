//! Museum Exhibition Kiosk - 3D World-Space UI, Spatial Camera & BSN.
//!
//! Visual Theme: Prestigious Nordic Modernist Museum (Obsidian, Brushed Gold, Cream, Deep Charcoal).
//!
//! Features demonstrated:
//! - `URootUi::world_3d(Vec2::new(760.0, 500.0))`: Rendering interactive UI directly in 3D world coordinates
//! - 3D perspective scene with Camera3d, DirectionalLight, PointLight, exhibition plinth, and golden torus sculpture
//! - Spatial positioning and angle rotation of the UI kiosk in 3D space (`Transform::from_xyz(...)`)
//! - Built-in Theme typography (`Inter`, `FreeSerif`) and embedded Lucide vector icons
//!   (`Icon::LIBRARY`, `Icon::BOOK_OPEN`, `Icon::HEADPHONES`, `Icon::PLAY`, `Icon::PAUSE`, `Icon::GLOBE`, `Icon::ARROW_LEFT`, `Icon::ARROW_RIGHT`)
//! - Real-time SDF text outlines on 3D sculptural titles
//! - Interactive exhibition kiosk layout: Title, curatorial essay, dimension specs table, audio tour player
//! - Dynamic click observers updating tour guide playback state
//! - Clean, modern, accessible typography with zero visual distortion

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct ExhibitionKioskState {
    audio_playing: bool,
    #[allow(dead_code)]
    active_language: &'static str,
    status_notification: String,
}

impl Default for ExhibitionKioskState {
    fn default() -> Self {
        Self {
            audio_playing: false,
            active_language: "ENGLISH (EN)",
            status_notification: "Audio Guide ready • Select narration language".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct AudioGuideStatusText;

#[derive(Component, Clone, Copy, Default)]
struct AudioGuidePlayButtonText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Museum Exhibition Kiosk (URootUi::world_3d & BSN)".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<ExhibitionKioskState>()
        .add_systems(Startup, setup_museum_gallery)
        .add_systems(Update, (rotate_sculpture_system, sync_kiosk_ui))
        .run();
}

#[derive(Component)]
struct RotatingSculpture;

fn setup_museum_gallery(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>,
) {
    // 1. 3D Camera viewing the gallery and interactive kiosk
    commands.spawn((
        Camera3d::default(),
        AmbientLight {
            color: Color::srgb(0.9, 0.93, 1.0),
            brightness: 1_200.0,
            ..default()
        },
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.04, 0.05, 0.07)),
            ..default()
        },
        Transform::from_xyz(-0.10, 0.04, 1.25).looking_at(Vec3::new(-0.10, 0.0, 0.0), Vec3::Y),
    ));

    // 2. Exhibition Directional Lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 35_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 3.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Warm Accent Spotlight on the Sculpture
    commands.spawn((
        PointLight {
            intensity: 150_000.0,
            range: 6.0,
            color: Color::srgb(1.0, 0.92, 0.8),
            ..default()
        },
        Transform::from_xyz(-0.68, 1.2, 0.8),
    ));

    // Exhibition Kiosk Display Spotlight
    commands.spawn((
        PointLight {
            intensity: 220_000.0,
            range: 6.0,
            color: Color::srgb(0.95, 0.98, 1.0),
            ..default()
        },
        Transform::from_xyz(0.2, 0.5, 1.2),
    ));

    // 3. Gallery Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.07, 0.1),
            perceptual_roughness: 0.3,
            metallic: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.9, 0.0),
    ));

    // 4. Exhibition Pedestal (Plinth)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 0.95))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.14, 0.18),
            perceptual_roughness: 0.25,
            metallic: 0.05,
            ..default()
        })),
        Transform::from_xyz(-0.68, -0.42, -0.1),
    ));

    // 5. The Sculptural Artifact: Golden Celestial Torus
    commands.spawn((
        RotatingSculpture,
        Mesh3d(meshes.add(Torus::new(0.13, 0.27))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.76, 0.35),
            metallic: 0.92,
            perceptual_roughness: 0.22,
            ..default()
        })),
        Transform::from_xyz(-0.68, 0.25, -0.1).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.4,
            0.5,
            0.2,
        )),
    ));

    // 6. Interactive 3D World-Space UI Kiosk
    let kiosk_transform =
        Transform::from_xyz(0.44, 0.02, 0.0).with_rotation(Quat::from_rotation_y(-0.25));

    let text_font = theme.text.font.inter_regular.clone();
    let serif_font = theme.text.font.free_serif.clone();
    let icon_font = theme.icon.font.clone();

    commands
        .spawn_scene(museum_kiosk_scene(text_font, serif_font, icon_font))
        .insert(kiosk_transform);
}

/// Rotates the gallery sculpture slowly for subtle ambient life.
fn rotate_sculpture_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RotatingSculpture>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(0.25 * time.delta_secs());
    }
}

/// Scene constructing the interactive 3D kiosk UI panel.
fn museum_kiosk_scene(
    text_font: Handle<Font>,
    serif_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        URootUi::world_3d(Vec2::new(760.0, 500.0))
        UPbr {
            metallic: 0.0,
            roughness: 0.85,
            emissive: LinearRgba::BLACK,
        }
        UNode {
            width: UVal::Px(760.0),
            height: UVal::Px(500.0),
            background_color: Color::srgba(0.06, 0.07, 0.11, 0.94),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(20.0),
        }
        UBorder {
            color: Color::srgba(0.92, 0.76, 0.35, 0.45),
            width: 1.5,
            radius: UCornerRadius::all(14.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            (kiosk_header(icon_font.clone(), text_font.clone())),
            (kiosk_artifact_body(icon_font.clone(), text_font.clone(), serif_font)),
            (kiosk_audio_tour_bar(icon_font.clone(), text_font.clone())),
            (kiosk_action_footer(icon_font, text_font)),
        ]
    }
}

/// Kiosk Top Header: Gallery wing and catalog ID.
fn kiosk_header(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(42.0),
            background_color: Color::srgba(0.04, 0.05, 0.08, 0.8),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(14.0, 8.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
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
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::LIBRARY.to_string() },
                        font_size: 14.0,
                        font: { icon_font },
                        color: Color::srgb(1.0, 0.88, 0.5),
                    }),
                    (UText {
                        text: { "NORDIC MODERNISM WING • GALLERY 4".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::srgb(1.0, 0.88, 0.5),
                    })
                ]
            ),
            (
                UNode {
                    padding: USides::axes(8.0, 3.0),
                    background_color: Color::srgba(0.15, 0.2, 0.28, 0.6),
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    (UText {
                        text: { "CATALOG REF: MMA-1924-CT".to_string() },
                        font_size: 11.0,
                        font: { text_font },
                        color: Color::srgb(0.7, 0.75, 0.85),
                    })
                ]
            )
        ]
    }
}

/// Main Artifact Details Body: Title, description, and physical specifications.
fn kiosk_artifact_body(
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
    serif_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            gap: 12.0,
        }
        Children [
            // Title & Artist
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 3.0,
                }
                Children [
                    (UText {
                        text: { "THE CELESTIAL TORUS".to_string() },
                        font_size: 24.0,
                        font: { serif_font },
                        color: Color::WHITE,
                        outline_width: 1.5,
                        outline_color: Color::srgba(0.85, 0.65, 0.2, 0.45),
                    }),
                    (UText {
                        text: { "Astrid Lindholm (Danish, 1892–1968) • Cast Bronze & Obsidian (1924)".to_string() },
                        font_size: 13.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.85, 0.72, 0.4),
                    })
                ]
            ),

            // Curatorial Note
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.05, 0.08, 0.6),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(12.0),
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
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::BOOK_OPEN.to_string() },
                                font_size: 12.0,
                                font: { icon_font },
                                color: Color::srgb(0.92, 0.76, 0.35),
                            }),
                            (UText {
                                text: { "CURATORIAL ESSAY".to_string() },
                                font_size: 11.5,
                                font: { text_font.clone() },
                                color: Color::srgb(0.92, 0.76, 0.35),
                            })
                        ]
                    ),
                    (UText {
                        text: { "Cast during the early Nordic Modernist renaissance in Copenhagen.\nThe continuous single-surface topology explores non-Euclidean curvature\nand mathematical equilibrium, balancing brushed bronze luster against\ndeep obsidian shadows.".to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.92, 0.94, 0.98),
                    })
                ]
            ),

            // Dimension Specs Row
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.05, 0.08, 0.6),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(12.0, 8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (spec_pill("DIMENSIONS", "68 x 45 x 45 cm", text_font.clone())),
                    (spec_pill("MEDIUM", "Gilt Cast Bronze", text_font.clone())),
                    (spec_pill("WEIGHT", "42.5 kg", text_font.clone())),
                    (spec_pill("PROVENANCE", "Lindholm Estate", text_font)),
                ]
            )
        ]
    }
}

/// Helper for specification key-value items.
fn spec_pill(label: &'static str, value: &'static str, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 2.0,
        }
        Children [
            (UText {
                text: { label.to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.72, 0.76, 0.86),
            }),
            (UText {
                text: { value.to_string() },
                font_size: 13.5,
                font: { text_font },
                color: Color::WHITE,
            })
        ]
    }
}

/// Audio Guide Player Bar: Play/Pause, scrubber, and time indicator.
fn kiosk_audio_tour_bar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(50.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.85),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(14.0, 8.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.35, 0.5, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Left: Play Button + Narration Info
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    // Play Button Circle
                    (
                        UNode {
                            width: UVal::Px(32.0),
                            height: UVal::Px(32.0),
                            background_color: Color::srgb(0.92, 0.76, 0.35),
                            border_radius: UCornerRadius::all(16.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.92, 0.76, 0.35),
                            hovered: Color::srgb(1.0, 0.86, 0.45),
                            pressed: Color::srgb(0.8, 0.65, 0.25),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (
                                AudioGuidePlayButtonText
                                UText {
                                    text: { Icon::PLAY.to_string() },
                                    font_size: 14.0,
                                    font: { icon_font.clone() },
                                    color: Color::srgb(0.06, 0.07, 0.1),
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<ExhibitionKioskState>| {
                            state.audio_playing = !state.audio_playing;
                            state.status_notification = if state.audio_playing {
                                "Playing: 'The Sculptural Balance of Lindholm' (Narrated by Dr. H. Vogel)".to_string()
                            } else {
                                "Audio guide paused".to_string()
                            };
                            info!("{}", state.status_notification);
                        })
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (
                                AudioGuideStatusText
                                UText {
                                    text: { "OFFICIAL AUDIO TOUR: 03:42 / 12:15".to_string() },
                                    font_size: 11.0,
                                    font: { text_font.clone() },
                                    color: Color::WHITE,
                                }
                            ),
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
                                        text: { Icon::HEADPHONES.to_string() },
                                        font_size: 10.0,
                                        font: { icon_font },
                                        color: Color::srgb(0.5, 0.55, 0.65),
                                    }),
                                    (UText {
                                        text: { "Narrated by Chief Curator Dr. Henrik Vogel".to_string() },
                                        font_size: 10.0,
                                        font: { text_font.clone() },
                                        color: Color::srgb(0.5, 0.55, 0.65),
                                    })
                                ]
                            )
                        ]
                    )
                ]
            ),
            // Right: Scrubber Track Mock
            (
                UNode {
                    width: UVal::Px(160.0),
                    height: UVal::Px(6.0),
                    background_color: Color::srgba(0.2, 0.25, 0.35, 0.7),
                    border_radius: UCornerRadius::all(3.0),
                }
                Children [
                    (
                        UNode {
                            width: UVal::Percent(0.35),
                            height: UVal::Percent(1.0),
                            background_color: Color::srgb(0.92, 0.76, 0.35),
                            border_radius: UCornerRadius::all(3.0),
                        }
                    )
                ]
            )
        ]
    }
}

/// Action Footer Bar: Gallery navigation and language selector.
fn kiosk_action_footer(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(44.0),
            padding: USides::axes(0.0, 4.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Left: Language Pill
            (
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    background_color: Color::srgba(0.12, 0.16, 0.24, 0.7),
                    border_radius: UCornerRadius::all(6.0),
                }
                UBorder {
                    color: Color::srgba(0.3, 0.35, 0.5, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::GLOBE.to_string() },
                        font_size: 11.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.7, 0.75, 0.85),
                    }),
                    (UText {
                        text: { "LANGUAGE: ENGLISH (EN)".to_string() },
                        font_size: 10.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.7, 0.75, 0.85),
                    })
                ]
            ),
            // Right: Navigation Buttons
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    (
                        UNode {
                            background_color: Color::srgba(0.12, 0.16, 0.24, 0.8),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(14.0, 8.0),
                        }
                        UInteraction::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::ARROW_LEFT.to_string() },
                                font_size: 12.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.7, 0.75, 0.85),
                            }),
                            (UText {
                                text: { "PREVIOUS".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.7, 0.75, 0.85),
                            })
                        ]
                    ),
                    (
                        UNode {
                            background_color: Color::srgb(0.92, 0.76, 0.35),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(16.0, 8.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.92, 0.76, 0.35),
                            hovered: Color::srgb(1.0, 0.86, 0.45),
                            pressed: Color::srgb(0.8, 0.65, 0.25),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { "NEXT ARTIFACT".to_string() },
                                font_size: 11.0,
                                font: { text_font },
                                color: Color::srgb(0.06, 0.07, 0.1),
                            }),
                            (UText {
                                text: { Icon::ARROW_RIGHT.to_string() },
                                font_size: 12.0,
                                font: { icon_font },
                                color: Color::srgb(0.06, 0.07, 0.1),
                            })
                        ]
                    )
                ]
            )
        ]
    }
}

/// Synchronizes audio player state to UI buttons and text.
fn sync_kiosk_ui(
    state: Res<ExhibitionKioskState>,
    mut status_q: Query<
        &mut UText,
        (
            With<AudioGuideStatusText>,
            Without<AudioGuidePlayButtonText>,
        ),
    >,
    mut play_btn_q: Query<
        &mut UText,
        (
            With<AudioGuidePlayButtonText>,
            Without<AudioGuideStatusText>,
        ),
    >,
) {
    if state.is_changed() {
        for mut text in &mut status_q {
            text.text = if state.audio_playing {
                "NOW PLAYING: 03:42 / 12:15 • TRACK 4".to_string()
            } else {
                "OFFICIAL AUDIO TOUR: 03:42 / 12:15".to_string()
            };
        }
        for mut text in &mut play_btn_q {
            text.text = if state.audio_playing {
                Icon::PAUSE.to_string()
            } else {
                Icon::PLAY.to_string()
            };
        }
    }
}
