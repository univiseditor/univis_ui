//! # 2D Multi-Stop Gradient Comprehensive Showcase
//!
//! Demonstrates every major gradient paradigm in Univis UI's SDF shader:
//! 1. **Dominant Color / Asymmetric Distribution**: Giving one color 75%-85% of the area with a thin rim.
//! 2. **Interactive Dominance Stepper**: Dynamically shifting color weight (25% vs 50% vs 80%).
//! 3. **Mixed Smooth & Hard Edges**: Smooth transition across the body followed by an instant sharp cut.
//! 4. **Hazard Stripes (Paired Hard Stops)**: Alternating sharp bands using duplicate stop positions.
//! 5. **Concentric Radial Target**: Stepped radial bullseye / radar rings (`.stepped()`).
//! 6. **Off-Center Spotlight**: Custom center coordinates (`radial_stops_custom`).
//! 7. **Full 8-Stop Optical Spectrum**: Maximum hardware stop capacity (`MAX_GRADIENT_STOPS = 8`).
//! 8. **Classic Smooth Synthwave**: Symmetrical multi-stop blending.

use bevy::prelude::*;
use core::f32::consts::{FRAC_PI_4, PI};
use univis_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PresetId {
    DominantRim,
    DominantSunset,
    MixedCut,
    HazardStripes,
    RadialTarget,
    RadialSpotlight,
    PrismSpectrum,
    Cyberpunk,
}

impl PresetId {
    const ALL: [PresetId; 8] = [
        PresetId::DominantRim,
        PresetId::DominantSunset,
        PresetId::MixedCut,
        PresetId::HazardStripes,
        PresetId::RadialTarget,
        PresetId::RadialSpotlight,
        PresetId::PrismSpectrum,
        PresetId::Cyberpunk,
    ];

    fn meta(self) -> (&'static str, &'static str, usize, Color) {
        match self {
            PresetId::DominantRim => (
                "DOMINANT DARK (80%) + NEON RIM",
                "Asymmetric stops: dark tone holds 0.0->0.80; neon rim occupies final 20%.",
                4,
                Color::srgb(0.0, 0.85, 1.0),
            ),
            PresetId::DominantSunset => (
                "DOMINANT GOLD (75%) + SUNSET",
                "Primary gold dominates 75% of surface before bleeding into sunset tones.",
                4,
                Color::srgb(1.0, 0.75, 0.2),
            ),
            PresetId::MixedCut => (
                "MIXED: SMOOTH FADE + HARD CUT",
                "0.0->0.70 smooth navy/purple blend, followed by an exact hard cut to cyan.",
                4,
                Color::srgb(0.0, 0.95, 0.75),
            ),
            PresetId::HazardStripes => (
                "HAZARD / RACING STRIPES",
                "Paired duplicate stops (0.25, 0.50, 0.75) creating alternating sharp bands.",
                8,
                Color::srgb(1.0, 0.80, 0.0),
            ),
            PresetId::RadialTarget => (
                "CONCENTRIC RADIAL TARGET",
                "Stepped radial interpolation (.stepped()) producing sharp radar rings.",
                4,
                Color::srgb(0.1, 0.85, 1.0),
            ),
            PresetId::RadialSpotlight => (
                "OFF-CENTER NEBULA SPOTLIGHT",
                "Radial gradient with offset center (-0.35, 0.35) and 1.2 radius.",
                3,
                Color::srgb(1.0, 0.85, 0.5),
            ),
            PresetId::PrismSpectrum => (
                "8-STOP OPTICAL SPECTRUM",
                "Full 8-stop hardware interpolation utilizing all GPU uniform stop registers.",
                8,
                Color::srgb(0.85, 0.35, 1.0),
            ),
            PresetId::Cyberpunk => (
                "CYBERPUNK NEON (BALANCED)",
                "Evenly distributed 4-stop synthwave linear gradient.",
                4,
                Color::srgb(0.0, 0.90, 1.0),
            ),
        }
    }

    fn title(self) -> &'static str {
        self.meta().0
    }
    fn description(self) -> &'static str {
        self.meta().1
    }
    fn stop_count(self) -> usize {
        self.meta().2
    }
    fn accent_color(self) -> Color {
        self.meta().3
    }

    fn to_gradient(self, angle: f32, stepped: bool, bias: f32) -> UGradient {
        let b = bias.clamp(0.15, 0.90);
        let grad = match self {
            PresetId::DominantRim => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.02, 0.035, 0.07, 0.98)),
                    (b, Color::srgba(0.02, 0.035, 0.07, 0.98)),
                    (b + (1.0 - b) * 0.45, Color::srgba(0.0, 0.65, 0.95, 0.95)),
                    (1.00, Color::srgba(0.3, 0.95, 1.0, 1.0)),
                ],
            ),
            PresetId::DominantSunset => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(1.0, 0.82, 0.35, 0.95)),
                    (b, Color::srgba(1.0, 0.82, 0.35, 0.95)),
                    (b + (1.0 - b) * 0.5, Color::srgba(0.95, 0.32, 0.15, 0.95)),
                    (1.00, Color::srgba(0.32, 0.04, 0.12, 0.98)),
                ],
            ),
            PresetId::MixedCut => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.03, 0.08, 0.22, 0.95)),
                    (b, Color::srgba(0.55, 0.10, 0.65, 0.95)),
                    (b, Color::srgba(0.0, 0.95, 0.75, 1.0)),
                    (1.00, Color::srgba(0.0, 0.95, 0.75, 1.0)),
                ],
            ),
            PresetId::HazardStripes => {
                let d = Color::srgba(0.05, 0.05, 0.06, 0.98);
                let w = Color::srgba(1.0, 0.78, 0.05, 0.98);
                UGradient::linear_stops(
                    angle,
                    [
                        (0.0, d),
                        (0.25, d),
                        (0.25, w),
                        (0.5, w),
                        (0.5, d),
                        (0.75, d),
                        (0.75, w),
                        (1.0, w),
                    ],
                )
            }
            PresetId::RadialTarget => UGradient::radial_stops([
                (0.00, Color::srgba(1.0, 1.0, 1.0, 1.0)),
                (0.20, Color::srgba(0.0, 0.85, 1.0, 0.95)),
                (0.48, Color::srgba(0.05, 0.15, 0.38, 0.95)),
                (0.80, Color::srgba(0.02, 0.04, 0.10, 0.98)),
            ])
            .stepped(),
            PresetId::RadialSpotlight => UGradient::radial_stops_custom(
                Vec2::new(-0.35, 0.35),
                1.2,
                [
                    (0.00, Color::srgba(1.0, 0.90, 0.60, 0.95)),
                    (0.50, Color::srgba(0.60, 0.25, 0.15, 0.85)),
                    (1.00, Color::srgba(0.03, 0.02, 0.06, 0.98)),
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
            PresetId::Cyberpunk => UGradient::linear_stops(
                angle,
                [
                    (0.00, Color::srgba(0.08, 0.02, 0.25, 0.95)),
                    (0.35, Color::srgba(0.55, 0.05, 0.70, 0.95)),
                    (0.70, Color::srgba(0.95, 0.10, 0.55, 0.95)),
                    (1.00, Color::srgba(0.00, 0.90, 1.00, 0.95)),
                ],
            ),
        };

        if stepped { grad.stepped() } else { grad }
    }
}

#[derive(Resource)]
struct GradientDemoState {
    current_preset: PresetId,
    angle: f32,
    auto_rotate: bool,
    shape_mode: UShapeMode,
    inner_glow_enabled: bool,
    stepped_mode: bool,
    dominance_ratio: f32,
}

impl Default for GradientDemoState {
    fn default() -> Self {
        Self {
            current_preset: PresetId::DominantRim,
            angle: FRAC_PI_4,
            auto_rotate: false,
            shape_mode: UShapeMode::Cut,
            inner_glow_enabled: true,
            stepped_mode: false,
            dominance_ratio: 0.80,
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
struct EdgeButtonText;

#[derive(Component)]
struct ShapeButtonText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Multi-Stop Gradient Comprehensive Showcase".into(),
                    resolution: (1260, 890).into(),
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
                padding: USides::axes(20.0, 14.0),
                background_color: Color::NONE,
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

    // ── Header Bar ──────────────────────────────────────────────────────────
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(16.0, 8.0),
                background_color: Color::srgba(0.04, 0.07, 0.12, 0.9),
                border_radius: UCornerRadius::all(8.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
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

    spawn_label(
        &mut commands,
        header,
        "UNIVIS UI : MULTI-STOP GRADIENT PARADIGMS",
        Color::srgb(0.0, 0.9, 1.0),
        16.0,
    );
    spawn_label(
        &mut commands,
        header,
        "Dominant Colors - Asymmetric Stops - Mixed Smooth/Sharp Cuts - 8-Stop Spectrum",
        Color::srgba(0.6, 0.75, 0.9, 0.85),
        12.0,
    );

    // ── Main Content Body ───────────────────────────────────────────────────
    let body = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(390.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    // Hero Preview Card
    let hero = commands
        .spawn((
            ChildOf(body),
            HeroCard,
            UNode {
                width: UVal::Percent(0.60),
                height: UVal::Percent(1.0),
                padding: USides::all(18.0),
                background_color: Color::srgba(0.05, 0.08, 0.15, 0.95),
                border_radius: UCornerRadius::all(16.0),
                shape_mode: state.shape_mode,
                ..default()
            },
            state.current_preset.to_gradient(
                state.angle,
                state.stepped_mode,
                state.dominance_ratio,
            ),
            UBorder {
                color: state.current_preset.accent_color().with_alpha(0.6),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            UShadow::glow(state.current_preset.accent_color().with_alpha(0.3), 18.0),
            UInnerGlow::new(state.current_preset.accent_color().with_alpha(0.25), 10.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    let hero_top = spawn_row(&mut commands, hero, true);
    commands.spawn((
        ChildOf(hero_top),
        HeroPresetTitle,
        UNode::default(),
        UTextLabel {
            text: state.current_preset.title().to_string(),
            color: Color::WHITE,
            font_size: 20.0,
            ..default()
        },
    ));

    let tag = commands
        .spawn((
            ChildOf(hero_top),
            UNode {
                padding: USides::axes(8.0, 4.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.65),
                border_radius: UCornerRadius::all(6.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            ULayout::default(),
        ))
        .id();
    spawn_label(
        &mut commands,
        tag,
        "SDF WGSL PIPELINE",
        Color::srgb(0.0, 0.9, 1.0),
        11.0,
    );

    let hero_bottom = commands
        .spawn((
            ChildOf(hero),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.6),
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
            color: Color::srgba(0.9, 0.95, 1.0, 0.92),
            font_size: 12.0,
            ..default()
        },
    ));

    // Right Inspector & Controls Column
    let controls_col = commands
        .spawn((
            ChildOf(body),
            UNode {
                width: UVal::Percent(0.40),
                height: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.92),
                border_radius: UCornerRadius::all(12.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.25),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 10.0,
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
                padding: USides::all(10.0),
                background_color: Color::srgba(0.02, 0.03, 0.06, 0.85),
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
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        inspector_panel,
        "LIVE SHADER INSPECTOR",
        Color::srgb(0.0, 0.85, 1.0),
        12.0,
    );
    commands.spawn((
        ChildOf(inspector_panel),
        InspectorText,
        UNode::default(),
        UTextLabel {
            text: "Initializing...".to_string(),
            color: Color::srgba(0.85, 0.92, 0.98, 0.9),
            font_size: 12.0,
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
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    // Row for Dominance Shift (25% | 50% | 80%)
    let dom_row = spawn_row(&mut commands, btn_bar, false);
    for (label, val) in [("25%", 0.25), ("50%", 0.50), ("80%", 0.80)] {
        let btn = spawn_labeled_btn(
            &mut commands,
            dom_row,
            &format!("Dominance {label}"),
            Color::srgb(0.12, 0.35, 0.55),
        );
        commands.entity(btn).observe(
            move |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
                state.dominance_ratio = val;
            },
        );
    }

    // Button: Toggle Edge Mode
    let edge_btn = spawn_labeled_btn(
        &mut commands,
        btn_bar,
        "Edge Mode: SMOOTH",
        Color::srgb(0.65, 0.25, 0.15),
    );
    commands.entity(edge_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.stepped_mode = !state.stepped_mode;
        },
    );
    commands.entity(edge_btn).insert(EdgeButtonText);

    // Row for Rotate + Angle Step
    let rot_row = spawn_row(&mut commands, btn_bar, false);
    let rotate_btn = spawn_labeled_btn(
        &mut commands,
        rot_row,
        "Rotate: OFF",
        Color::srgb(0.0, 0.45, 0.7),
    );
    commands.entity(rotate_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.auto_rotate = !state.auto_rotate;
        },
    );
    commands.entity(rotate_btn).insert(RotateButtonText);

    let angle_step_btn = spawn_labeled_btn(
        &mut commands,
        rot_row,
        "Angle +45°",
        Color::srgb(0.18, 0.45, 0.4),
    );
    commands.entity(angle_step_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.angle = (state.angle + FRAC_PI_4) % (PI * 2.0);
        },
    );

    // Button: Toggle Shape Mode
    let shape_btn = spawn_labeled_btn(
        &mut commands,
        btn_bar,
        "Shape Mode: CUT",
        Color::srgb(0.2, 0.35, 0.6),
    );
    commands.entity(shape_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.shape_mode = match state.shape_mode {
                UShapeMode::Cut => UShapeMode::Round,
                UShapeMode::Round => UShapeMode::Cut,
            };
        },
    );
    commands.entity(shape_btn).insert(ShapeButtonText);

    // ── Bottom Gallery Grid: 8 Preset Cards ─────────────────────────────────
    let gallery_header = spawn_row(&mut commands, root, true);
    spawn_label(
        &mut commands,
        gallery_header,
        "PARADIGM GALLERY (CLICK ANY CARD TO SELECT PRESET)",
        Color::srgb(0.0, 0.9, 1.0),
        13.0,
    );

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
                        column_gap: Some(10.0),
                        row_gap: Some(10.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: vec![UTrackSize::repeat_fit(UTrackRepeat::minmax(
                            UTrackBound::px(135.0),
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
                    height: UVal::Px(105.0),
                    padding: USides::all(8.0),
                    background_color: Color::srgba(0.04, 0.07, 0.12, 0.9),
                    border_radius: UCornerRadius::all(8.0),
                    shape_mode: UShapeMode::Cut,
                    ..default()
                },
                preset.to_gradient(FRAC_PI_4, false, 0.75),
                UBorder {
                    color: preset.accent_color().with_alpha(0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
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

        spawn_label(&mut commands, card, preset.title(), Color::WHITE, 10.0);

        let badge = commands
            .spawn((
                ChildOf(card),
                UNode {
                    padding: USides::axes(5.0, 2.0),
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.55),
                    border_radius: UCornerRadius::all(4.0),
                    shape_mode: UShapeMode::Cut,
                    ..default()
                },
                ULayout::default(),
            ))
            .id();
        spawn_label(
            &mut commands,
            badge,
            &format!("{} STOPS", preset.stop_count()),
            preset.accent_color(),
            9.0,
        );
    }
}

fn spawn_row(commands: &mut Commands, parent: Entity, space_between: bool) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: if space_between {
                    UJustifyContent::SpaceBetween
                } else {
                    UJustifyContent::Start
                },
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id()
}

fn spawn_label(commands: &mut Commands, parent: Entity, text: &str, color: Color, font_size: f32) {
    commands.spawn((
        ChildOf(parent),
        UNode::default(),
        UTextLabel {
            text: text.to_string(),
            color,
            font_size,
            ..default()
        },
    ));
}

fn spawn_labeled_btn(commands: &mut Commands, parent: Entity, label: &str, bg: Color) -> Entity {
    let btn = commands
        .spawn((
            ChildOf(parent),
            UNode {
                padding: USides::axes(10.0, 7.0),
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
        .id();

    commands.spawn((
        ChildOf(btn),
        UNode::default(),
        UTextLabel {
            text: label.to_string(),
            color: Color::WHITE,
            font_size: 11.0,
            ..default()
        },
    ));
    btn
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
    mut title_query: Query<
        &mut UTextLabel,
        (
            With<HeroPresetTitle>,
            Without<HeroPresetDesc>,
            Without<InspectorText>,
        ),
    >,
    mut desc_query: Query<
        &mut UTextLabel,
        (
            With<HeroPresetDesc>,
            Without<HeroPresetTitle>,
            Without<InspectorText>,
        ),
    >,
    mut inspector_query: Query<
        &mut UTextLabel,
        (
            With<InspectorText>,
            Without<HeroPresetTitle>,
            Without<HeroPresetDesc>,
        ),
    >,
    rotate_btn_query: Query<&Children, With<RotateButtonText>>,
    edge_btn_query: Query<&Children, With<EdgeButtonText>>,
    shape_btn_query: Query<&Children, With<ShapeButtonText>>,
    mut labels: Query<
        &mut UTextLabel,
        (
            Without<HeroPresetTitle>,
            Without<HeroPresetDesc>,
            Without<InspectorText>,
        ),
    >,
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
        *grad = preset.to_gradient(state.angle, state.stepped_mode, state.dominance_ratio);
        border.color = accent.with_alpha(0.65);
        shadow.color = accent.with_alpha(0.35);

        *inner_glow = if state.inner_glow_enabled {
            UInnerGlow::new(accent.with_alpha(0.3), 12.0)
        } else {
            UInnerGlow::new(Color::NONE, 0.0)
        };
    }

    // 2. Update UI labels
    if let Ok(mut title) = title_query.single_mut() {
        title.text = preset.title().to_string();
    }
    if let Ok(mut desc) = desc_query.single_mut() {
        desc.text = preset.description().to_string();
    }
    if let Ok(mut inspector) = inspector_query.single_mut() {
        let edge_str = if state.stepped_mode {
            "Hard (Stepped Bands)"
        } else {
            "Smooth (Blended)"
        };
        let dom_pct = (state.dominance_ratio * 100.0).round() as i32;
        let angle_deg = (state.angle.to_degrees() % 360.0).round() as i32;
        inspector.text = format!(
            "Active: {}\nStops: {} | Dominance: {}% | {}\nAngle: {}°\nBehavior: {}",
            preset.title(),
            preset.stop_count(),
            dom_pct,
            edge_str,
            angle_deg,
            preset.description(),
        );
    }

    // 3. Update button labels
    if let Ok(children) = rotate_btn_query.single() {
        if let Some(&child) = children.first() {
            if let Ok(mut label) = labels.get_mut(child) {
                label.text = format!("Rotate: {}", if state.auto_rotate { "ON" } else { "OFF" });
            }
        }
    }
    if let Ok(children) = edge_btn_query.single() {
        if let Some(&child) = children.first() {
            if let Ok(mut label) = labels.get_mut(child) {
                label.text = format!(
                    "Edge Mode: {}",
                    if state.stepped_mode {
                        "HARD (STEPPED)"
                    } else {
                        "SMOOTH"
                    }
                );
            }
        }
    }
    if let Ok(children) = shape_btn_query.single() {
        if let Some(&child) = children.first() {
            if let Ok(mut label) = labels.get_mut(child) {
                label.text = format!(
                    "Shape: {}",
                    match state.shape_mode {
                        UShapeMode::Cut => "CUT",
                        UShapeMode::Round => "ROUND",
                    }
                );
            }
        }
    }
}
