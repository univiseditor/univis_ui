//! # 2D Multi-Stop Gradient Showcase
//!
//! Demonstrates Univis UI's SDF-based multi-stop linear and radial gradients (`UGradient`).
//!
//! Features demonstrated:
//! - Up to 8 color stops with normalized positions (`[0.0, 1.0]`).
//! - Linear gradients at arbitrary angles (`UGradient::linear_stops`).
//! - Centered radial gradients (`UGradient::radial_stops`).
//! - Off-center radial spotlights (`UGradient::radial_stops_custom`).
//! - Real-time angle rotation and interactive preset switching.

use bevy::prelude::*;
use core::f32::consts::{FRAC_PI_4, PI};
use univis_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PresetId {
    Cyberpunk,
    Aurora,
    SolarFlare,
    PrismSpectrum,
    QuantumCore,
    OffsetSpotlight,
}

impl PresetId {
    const ALL: [PresetId; 6] = [
        PresetId::Cyberpunk,
        PresetId::Aurora,
        PresetId::SolarFlare,
        PresetId::PrismSpectrum,
        PresetId::QuantumCore,
        PresetId::OffsetSpotlight,
    ];

    fn title(self) -> &'static str {
        match self {
            PresetId::Cyberpunk => "CYBERPUNK NEON",
            PresetId::Aurora => "AURORA BOREALIS",
            PresetId::SolarFlare => "SOLAR FLARE",
            PresetId::PrismSpectrum => "PRISM SPECTRUM",
            PresetId::QuantumCore => "QUANTUM REACTOR",
            PresetId::OffsetSpotlight => "NEBULA SPOTLIGHT",
        }
    }

    fn description(self) -> &'static str {
        match self {
            PresetId::Cyberpunk => "4-Stop Linear (Indigo -> Violet -> Magenta -> Cyan)",
            PresetId::Aurora => "5-Stop Linear (Night Blue -> Teal -> Emerald -> Lime -> Purple)",
            PresetId::SolarFlare => {
                "4-Stop Linear (Deep Crimson -> Blazing Red -> Amber -> Solar White)"
            }
            PresetId::PrismSpectrum => "8-Stop Linear (Full Optical Spectrum: Red through Violet)",
            PresetId::QuantumCore => {
                "4-Stop Radial (White-Hot Core -> Cyan Plasma -> Indigo -> Void)"
            }
            PresetId::OffsetSpotlight => "3-Stop Radial with Custom Center (-0.35, 0.35)",
        }
    }

    fn stop_count(self) -> usize {
        match self {
            PresetId::Cyberpunk => 4,
            PresetId::Aurora => 5,
            PresetId::SolarFlare => 4,
            PresetId::PrismSpectrum => 8,
            PresetId::QuantumCore => 4,
            PresetId::OffsetSpotlight => 3,
        }
    }

    fn to_gradient(self, angle: f32) -> UGradient {
        match self {
            PresetId::Cyberpunk => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.08, 0.02, 0.25, 0.95)),
                    (0.35, Color::srgba(0.55, 0.05, 0.70, 0.95)),
                    (0.70, Color::srgba(0.95, 0.10, 0.55, 0.95)),
                    (1.00, Color::srgba(0.00, 0.90, 1.00, 0.95)),
                ],
            ),
            PresetId::Aurora => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.01, 0.03, 0.10, 0.98)),
                    (0.25, Color::srgba(0.02, 0.25, 0.35, 0.95)),
                    (0.55, Color::srgba(0.05, 0.80, 0.55, 0.95)),
                    (0.80, Color::srgba(0.45, 0.95, 0.30, 0.95)),
                    (1.00, Color::srgba(0.30, 0.08, 0.45, 0.95)),
                ],
            ),
            PresetId::SolarFlare => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.35, 0.02, 0.05, 0.95)),
                    (0.35, Color::srgba(0.90, 0.15, 0.05, 0.95)),
                    (0.70, Color::srgba(1.00, 0.65, 0.05, 0.95)),
                    (1.00, Color::srgba(1.00, 0.98, 0.85, 0.98)),
                ],
            ),
            PresetId::PrismSpectrum => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgb(1.0, 0.1, 0.1)),
                    (0.14, Color::srgb(1.0, 0.5, 0.0)),
                    (0.28, Color::srgb(1.0, 0.9, 0.1)),
                    (0.42, Color::srgb(0.1, 0.9, 0.3)),
                    (0.57, Color::srgb(0.0, 0.8, 1.0)),
                    (0.71, Color::srgb(0.2, 0.3, 1.0)),
                    (0.85, Color::srgb(0.7, 0.2, 1.0)),
                    (1.00, Color::srgb(1.0, 0.2, 0.7)),
                ],
            ),
            PresetId::QuantumCore => UGradient::radial_stops([
                (0.00, Color::srgba(1.0, 1.0, 1.0, 1.0)),
                (0.25, Color::srgba(0.0, 0.95, 1.0, 0.95)),
                (0.65, Color::srgba(0.05, 0.15, 0.45, 0.95)),
                (1.00, Color::srgba(0.01, 0.02, 0.06, 0.98)),
            ]),
            PresetId::OffsetSpotlight => UGradient::radial_stops_custom(
                Vec2::new(-0.35, 0.35),
                1.2,
                [
                    (0.00, Color::srgba(1.0, 0.90, 0.60, 0.95)),
                    (0.50, Color::srgba(0.60, 0.25, 0.15, 0.85)),
                    (1.00, Color::srgba(0.03, 0.02, 0.06, 0.98)),
                ],
            ),
        }
    }

    fn accent_color(self) -> Color {
        match self {
            PresetId::Cyberpunk => Color::srgb(0.0, 0.9, 1.0),
            PresetId::Aurora => Color::srgb(0.1, 0.9, 0.5),
            PresetId::SolarFlare => Color::srgb(1.0, 0.65, 0.1),
            PresetId::PrismSpectrum => Color::srgb(0.8, 0.4, 1.0),
            PresetId::QuantumCore => Color::srgb(0.2, 0.8, 1.0),
            PresetId::OffsetSpotlight => Color::srgb(1.0, 0.8, 0.4),
        }
    }
}

#[derive(Resource)]
struct GradientDemoState {
    current_preset: PresetId,
    angle: f32,
    auto_rotate: bool,
    shape_mode: UShapeMode,
    inner_glow_enabled: bool,
}

impl Default for GradientDemoState {
    fn default() -> Self {
        Self {
            current_preset: PresetId::Cyberpunk,
            angle: FRAC_PI_4,
            auto_rotate: true,
            shape_mode: UShapeMode::Cut,
            inner_glow_enabled: true,
        }
    }
}

#[derive(Component)]
struct HeroCard;

#[derive(Component)]
struct HeroPresetTitle;

#[derive(Component)]
struct HeroPresetDesc;

#[derive(Component)]
struct InspectorText;

#[derive(Component)]
struct RotateButtonText;

#[derive(Component)]
struct ShapeButtonText;

#[derive(Component)]
struct GlowButtonText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Multi-Stop Gradient Showcase".into(),
                    resolution: (1240, 860).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<GradientDemoState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_demo, sync_ui_state))
        .run();
}

fn setup_scene(mut commands: Commands, state: Res<GradientDemoState>) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.015, 0.02, 0.035)),
            ..default()
        },
    ));

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::axes(24.0, 18.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    // ── Header Bar ──────────────────────────────────────────────────────────
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(16.0, 10.0),
                background_color: Color::srgba(0.04, 0.07, 0.12, 0.9),
                border_radius: UCornerRadius::all(10.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
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
        ChildOf(header),
        UNode::default(),
        UTextLabel {
            text: "UNIVIS UI : MULTI-STOP GRADIENT SHOWCASE".to_string(),
            color: Color::srgb(0.0, 0.9, 1.0),
            font_size: 17.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(header),
        UNode::default(),
        UTextLabel {
            text: "Up to 8 stops - Linear & Radial SDF Shading".to_string(),
            color: Color::srgba(0.6, 0.75, 0.9, 0.8),
            font_size: 13.0,
            ..default()
        },
    ));

    // ── Main Content Body: Left = Hero Panel, Right = Inspector & Controls ──
    let body = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(420.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    let hero = commands
        .spawn((
            ChildOf(body),
            HeroCard,
            UNode {
                width: UVal::Percent(0.62),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.05, 0.08, 0.15, 0.95),
                border_radius: UCornerRadius::all(18.0),
                shape_mode: state.shape_mode,
                ..default()
            },
            state.current_preset.to_gradient(state.angle),
            UBorder {
                color: state.current_preset.accent_color().with_alpha(0.6),
                width: 1.5,
                radius: UCornerRadius::all(18.0),
                offset: 0.0,
            },
            UShadow::glow(state.current_preset.accent_color().with_alpha(0.3), 20.0),
            UInnerGlow::new(state.current_preset.accent_color().with_alpha(0.25), 12.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    let hero_top = commands
        .spawn((
            ChildOf(hero),
            UNode {
                width: UVal::Percent(1.0),
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
        ChildOf(hero_top),
        HeroPresetTitle,
        UNode::default(),
        UTextLabel {
            text: state.current_preset.title().to_string(),
            color: Color::WHITE,
            font_size: 22.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(hero_top),
        UNode {
            padding: USides::axes(8.0, 4.0),
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.6),
            border_radius: UCornerRadius::all(6.0),
            shape_mode: UShapeMode::Cut,
            ..default()
        },
        UTextLabel {
            text: "LIVE SDF RENDER".to_string(),
            color: Color::srgb(0.0, 0.9, 1.0),
            font_size: 11.0,
            ..default()
        },
    ));

    let hero_bottom = commands
        .spawn((
            ChildOf(hero),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.55),
                border_radius: UCornerRadius::all(8.0),
                shape_mode: UShapeMode::Cut,
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
        ChildOf(hero_bottom),
        HeroPresetDesc,
        UNode::default(),
        UTextLabel {
            text: state.current_preset.description().to_string(),
            color: Color::srgba(0.9, 0.95, 1.0, 0.9),
            font_size: 12.0,
            ..default()
        },
    ));

    // Right Inspector & Controls Column
    let controls_col = commands
        .spawn((
            ChildOf(body),
            UNode {
                width: UVal::Percent(0.38),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.9),
                border_radius: UCornerRadius::all(14.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.25),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    // Top section: Inspector info
    let inspector_panel = commands
        .spawn((
            ChildOf(controls_col),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.02, 0.03, 0.06, 0.8),
                border_radius: UCornerRadius::all(8.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.85, 1.0, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
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

    commands.spawn((
        ChildOf(inspector_panel),
        UNode::default(),
        UTextLabel {
            text: "INSPECTOR / UNIFORM METRICS".to_string(),
            color: Color::srgb(0.0, 0.85, 1.0),
            font_size: 13.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(inspector_panel),
        InspectorText,
        UNode::default(),
        UTextLabel {
            text: "Initializing...".to_string(),
            color: Color::srgba(0.85, 0.9, 0.98, 0.9),
            font_size: 13.0,
            ..default()
        },
    ));

    // Bottom section: Control Buttons
    let btn_bar = commands
        .spawn((
            ChildOf(controls_col),
            UNode {
                width: UVal::Percent(1.0),
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

    // Button 1: Toggle Auto-Rotate
    let rotate_btn = spawn_button(&mut commands, btn_bar, Color::srgb(0.0, 0.45, 0.7));
    commands.entity(rotate_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.auto_rotate = !state.auto_rotate;
        },
    );
    commands.spawn((
        ChildOf(rotate_btn),
        RotateButtonText,
        UNode::default(),
        UTextLabel {
            text: "Rotate Angle: ON".to_string(),
            color: Color::WHITE,
            font_size: 13.0,
            ..default()
        },
    ));

    // Button 2: Toggle Shape Mode
    let shape_btn = spawn_button(&mut commands, btn_bar, Color::srgb(0.2, 0.35, 0.6));
    commands.entity(shape_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.shape_mode = match state.shape_mode {
                UShapeMode::Cut => UShapeMode::Round,
                UShapeMode::Round => UShapeMode::Cut,
            };
        },
    );
    commands.spawn((
        ChildOf(shape_btn),
        ShapeButtonText,
        UNode::default(),
        UTextLabel {
            text: "Shape Mode: CUT".to_string(),
            color: Color::WHITE,
            font_size: 13.0,
            ..default()
        },
    ));

    // Button 3: Toggle Inner Glow
    let glow_btn = spawn_button(&mut commands, btn_bar, Color::srgb(0.3, 0.2, 0.5));
    commands.entity(glow_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.inner_glow_enabled = !state.inner_glow_enabled;
        },
    );
    commands.spawn((
        ChildOf(glow_btn),
        GlowButtonText,
        UNode::default(),
        UTextLabel {
            text: "Inner Glow: ON".to_string(),
            color: Color::WHITE,
            font_size: 13.0,
            ..default()
        },
    ));

    // ── Bottom Gallery Row: 6 Preset Cards ──────────────────────────────────
    let gallery_header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
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
        ChildOf(gallery_header),
        UNode::default(),
        UTextLabel {
            text: "PRESET GALLERY (CLICK TO SELECT PRESET)".to_string(),
            color: Color::srgb(0.0, 0.9, 1.0),
            font_size: 14.0,
            ..default()
        },
    ));

    let gallery_grid = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        column_gap: Some(12.0),
                        row_gap: Some(12.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: vec![UTrackSize::repeat_fit(UTrackRepeat::minmax(
                            UTrackBound::px(175.0),
                            UTrackBound::fr(1.0),
                        ))],
                        template_rows: vec![UTrackSize::Auto],
                        auto_rows: UTrackSize::Auto,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    for preset in PresetId::ALL {
        let card = commands
            .spawn((
                ChildOf(gallery_grid),
                UNode {
                    height: UVal::Px(120.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgba(0.04, 0.07, 0.12, 0.9),
                    border_radius: UCornerRadius::all(10.0),
                    shape_mode: UShapeMode::Cut,
                    ..default()
                },
                preset.to_gradient(FRAC_PI_4),
                UBorder {
                    color: preset.accent_color().with_alpha(0.35),
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
                    offset: 0.0,
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: Color::srgba(0.04, 0.07, 0.12, 0.9),
                    hovered: Color::srgba(0.08, 0.14, 0.22, 0.95),
                    pressed: Color::srgba(0.02, 0.04, 0.08, 0.95),
                },
                UShadow::glow(preset.accent_color().with_alpha(0.2), 8.0),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .observe(
                move |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
                    state.current_preset = preset;
                },
            )
            .id();

        commands.spawn((
            ChildOf(card),
            UNode::default(),
            UTextLabel {
                text: preset.title().to_string(),
                color: Color::WHITE,
                font_size: 12.0,
                ..default()
            },
        ));

        commands.spawn((
            ChildOf(card),
            UNode {
                padding: USides::axes(6.0, 3.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                border_radius: UCornerRadius::all(4.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UTextLabel {
                text: format!("{} STOPS", preset.stop_count()),
                color: preset.accent_color(),
                font_size: 10.0,
                ..default()
            },
        ));
    }
}

fn spawn_button(commands: &mut Commands, parent: Entity, bg: Color) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                padding: USides::axes(12.0, 9.0),
                background_color: bg,
                border_radius: UCornerRadius::all(6.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.9, 1.0, 0.35),
                width: 1.0,
                radius: UCornerRadius::all(6.0),
                offset: 0.0,
            },
            UInteraction::default(),
            UInteractionColors {
                normal: bg,
                hovered: bg.with_alpha(0.85),
                pressed: bg.with_alpha(0.6),
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id()
}

fn update_demo(time: Res<Time>, mut state: ResMut<GradientDemoState>) {
    if state.auto_rotate {
        state.angle += time.delta_secs() * 0.75;
        if state.angle > PI * 2.0 {
            state.angle -= PI * 2.0;
        }
    }
}

fn sync_ui_state(
    state: Res<GradientDemoState>,
    mut hero_query: Query<
        (
            &mut UNode,
            &mut UGradient,
            &mut UBorder,
            &mut UShadow,
            &mut UInnerGlow,
        ),
        With<HeroCard>,
    >,
    mut text_query: Query<(
        &mut UTextLabel,
        Option<&HeroPresetTitle>,
        Option<&HeroPresetDesc>,
        Option<&InspectorText>,
        Option<&RotateButtonText>,
        Option<&ShapeButtonText>,
        Option<&GlowButtonText>,
    )>,
) {
    if !state.is_changed() {
        return;
    }

    let preset = state.current_preset;
    let accent = preset.accent_color();

    // 1. Update Hero Card
    if let Ok((mut node, mut grad, mut border, mut shadow, mut inner_glow)) =
        hero_query.single_mut()
    {
        node.shape_mode = state.shape_mode;
        *grad = preset.to_gradient(state.angle);
        border.color = accent.with_alpha(0.65);
        shadow.color = accent.with_alpha(0.35);

        *inner_glow = if state.inner_glow_enabled {
            UInnerGlow::new(accent.with_alpha(0.3), 12.0)
        } else {
            UInnerGlow::new(Color::NONE, 0.0)
        };
    }

    // 2. Update UI labels
    let angle_deg = (state.angle.to_degrees() % 360.0).round() as i32;
    for (mut label, title, desc, inspector, rotate, shape, glow) in &mut text_query {
        if title.is_some() {
            label.text = preset.title().to_string();
        } else if desc.is_some() {
            label.text = preset.description().to_string();
        } else if inspector.is_some() {
            label.text = format!(
                "Active Preset: {}\nStop Count: {} / 8 stops\nAngle / Radius: {} deg\nDetails: {}",
                preset.title(),
                preset.stop_count(),
                angle_deg,
                preset.description(),
            );
        } else if rotate.is_some() {
            label.text = format!(
                "Rotate Angle: {}",
                if state.auto_rotate { "ON" } else { "OFF" }
            );
        } else if shape.is_some() {
            label.text = format!(
                "Shape Mode: {}",
                match state.shape_mode {
                    UShapeMode::Cut => "CUT",
                    UShapeMode::Round => "ROUND",
                }
            );
        } else if glow.is_some() {
            label.text = format!(
                "Inner Glow: {}",
                if state.inner_glow_enabled {
                    "ON"
                } else {
                    "OFF"
                }
            );
        }
    }
}
