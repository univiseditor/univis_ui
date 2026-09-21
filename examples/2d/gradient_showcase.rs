//! # 2D Multi-Stop Gradient Comprehensive Showcase
//!
//! Demonstrates every major gradient paradigm in Univis UI's SDF shader:
//! 1. **Sweep / Shimmer (المرور / الشيمر)**: Translating gradient phase smoothly across nodes.
//! 2. **Rotation (الدوران)**: Continuously rotating gradient angles in real-time.
//! 3. **Dominant Color / Asymmetric Distribution**: 75%-80% surface held by a primary tone.
//! 4. **Interactive Dominance Stepper**: Dynamically shifting color boundary (25% / 50% / 80%).
//! 5. **Mixed Smooth & Hard Edges**: Continuous body fade followed by an exact hard cut.
//! 6. **Hazard Stripes (Paired Hard Stops)**: Alternating sharp bands.
//! 7. **Concentric Radial Target**: Stepped radial bullseye / radar rings (`.stepped()`).
//! 8. **Off-Center Spotlight**: Custom center coordinates (`radial_stops_custom`).
//! 9. **Full 8-Stop Optical Spectrum**: Maximum hardware stop capacity (`MAX_GRADIENT_STOPS = 8`).

use bevy::prelude::*;
use core::f32::consts::{FRAC_PI_4, PI};
use univis_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AnimMode {
    Sweep,  // المرور الخطي / الشيمر والنبض الدائري
    Rotate, // دوران زاوية التدرج
    Off,    // إيقاف الحركة
}

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
    #[rustfmt::skip]
    const ALL: [PresetId; 8] = [
        PresetId::DominantRim, PresetId::DominantSunset, PresetId::MixedCut, PresetId::HazardStripes,
        PresetId::RadialTarget, PresetId::RadialSpotlight, PresetId::PrismSpectrum, PresetId::Cyberpunk,
    ];

    #[rustfmt::skip]
    fn meta(self) -> (&'static str, &'static str, usize, Color) {
        match self {
            PresetId::DominantRim => ("DOMINANT DARK (80%) + NEON RIM", "Dark 0.0->0.80 with neon rim in final 20%.", 4, Color::srgb(0.0, 0.85, 1.0)),
            PresetId::DominantSunset => ("DOMINANT GOLD (75%) + SUNSET", "Gold dominates 75% before sunset bleed.", 4, Color::srgb(1.0, 0.75, 0.2)),
            PresetId::MixedCut => ("MIXED: SMOOTH FADE + HARD CUT", "Smooth blend 0.0->0.70, sharp cut to cyan.", 4, Color::srgb(0.0, 0.95, 0.75)),
            PresetId::HazardStripes => ("HAZARD / RACING STRIPES", "Paired duplicate stops produce sharp bands.", 8, Color::srgb(1.0, 0.80, 0.0)),
            PresetId::RadialTarget => ("CONCENTRIC RADIAL TARGET", "Stepped radial interpolation (.stepped()).", 4, Color::srgb(0.1, 0.85, 1.0)),
            PresetId::RadialSpotlight => ("OFF-CENTER SPOTLIGHT", "Offset center (-0.35, 0.35) and 1.2 radius.", 3, Color::srgb(1.0, 0.85, 0.5)),
            PresetId::PrismSpectrum => ("8-STOP OPTICAL SPECTRUM", "Full 8-stop hardware interpolation.", 8, Color::srgb(0.85, 0.35, 1.0)),
            PresetId::Cyberpunk => ("CYBERPUNK NEON (BALANCED)", "Evenly distributed 4-stop synthwave linear.", 4, Color::srgb(0.0, 0.90, 1.0)),
        }
    }

    #[rustfmt::skip]
    fn title(self) -> &'static str { self.meta().0 }
    #[rustfmt::skip]
    fn description(self) -> &'static str { self.meta().1 }
    #[rustfmt::skip]
    fn stop_count(self) -> usize { self.meta().2 }
    #[rustfmt::skip]
    fn accent_color(self) -> Color { self.meta().3 }

    #[rustfmt::skip]
    fn to_gradient(self, angle: f32, offset: f32, stepped: bool, bias: f32) -> UGradient {
        let b = bias.clamp(0.15, 0.90);
        let grad = match self {
            PresetId::DominantRim => UGradient::linear_stops(angle, [
                (0.00, Color::srgba(0.02, 0.035, 0.07, 0.98)),
                (b, Color::srgba(0.02, 0.035, 0.07, 0.98)),
                (b + (1.0 - b) * 0.45, Color::srgba(0.0, 0.65, 0.95, 0.95)),
                (1.00, Color::srgba(0.3, 0.95, 1.0, 1.0)),
            ]),
            PresetId::DominantSunset => UGradient::linear_stops(angle, [
                (0.00, Color::srgba(1.0, 0.82, 0.35, 0.95)),
                (b, Color::srgba(1.0, 0.82, 0.35, 0.95)),
                (b + (1.0 - b) * 0.5, Color::srgba(0.95, 0.32, 0.15, 0.95)),
                (1.00, Color::srgba(0.32, 0.04, 0.12, 0.98)),
            ]),
            PresetId::MixedCut => UGradient::linear_stops(angle, [
                (0.00, Color::srgba(0.03, 0.08, 0.22, 0.95)),
                (b, Color::srgba(0.55, 0.10, 0.65, 0.95)),
                (b, Color::srgba(0.0, 0.95, 0.75, 1.0)),
                (1.00, Color::srgba(0.0, 0.95, 0.75, 1.0)),
            ]),
            PresetId::HazardStripes => {
                let (d, w) = (Color::srgba(0.05, 0.05, 0.06, 0.98), Color::srgba(1.0, 0.78, 0.05, 0.98));
                UGradient::linear_stops(angle, [
                    (0.00, d), (0.25, d), (0.25, w), (0.50, w),
                    (0.50, d), (0.75, d), (0.75, w), (1.00, w),
                ])
            }
            PresetId::RadialTarget => UGradient::radial_stops([
                (0.00, Color::WHITE),
                (0.20, Color::srgba(0.0, 0.85, 1.0, 0.95)),
                (0.48, Color::srgba(0.05, 0.15, 0.38, 0.95)),
                (0.80, Color::srgba(0.02, 0.04, 0.10, 0.98)),
            ]).stepped(),
            PresetId::RadialSpotlight => UGradient::radial_stops_custom(
                Vec2::new(-0.35, 0.35), 1.2,
                [
                    (0.00, Color::srgba(1.0, 0.90, 0.60, 0.95)),
                    (0.50, Color::srgba(0.60, 0.25, 0.15, 0.85)),
                    (1.00, Color::srgba(0.03, 0.02, 0.06, 0.98)),
                ],
            ),
            PresetId::PrismSpectrum => UGradient::linear_stops(angle, [
                (0.00, Color::srgb(1.0, 0.1, 0.1)),
                (0.14, Color::srgb(1.0, 0.5, 0.0)),
                (0.28, Color::srgb(1.0, 0.9, 0.1)),
                (0.42, Color::srgb(0.1, 0.9, 0.3)),
                (0.57, Color::srgb(0.0, 0.8, 1.0)),
                (0.71, Color::srgb(0.2, 0.3, 1.0)),
                (0.85, Color::srgb(0.7, 0.2, 1.0)),
                (1.00, Color::srgb(1.0, 0.2, 0.7)),
            ]),
            PresetId::Cyberpunk => UGradient::linear_stops(angle, [
                (0.00, Color::srgba(0.08, 0.02, 0.25, 0.95)),
                (0.35, Color::srgba(0.55, 0.05, 0.70, 0.95)),
                (0.70, Color::srgba(0.95, 0.10, 0.55, 0.95)),
                (1.00, Color::srgba(0.00, 0.90, 1.00, 0.95)),
            ]),
        };

        let grad = if stepped { grad.stepped() } else { grad };
        grad.with_offset(offset)
    }
}

#[derive(Resource)]
struct GradientDemoState {
    current_preset: PresetId,
    angle: f32,
    offset: f32,
    anim_mode: AnimMode,
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
            offset: 0.0,
            anim_mode: AnimMode::Sweep,
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
enum HeroLabelType {
    Title,
    Desc,
    Inspector,
}

#[derive(Component)]
enum ControlBtnType {
    Anim,
    Edge,
    Shape,
}

#[rustfmt::skip]
fn cut_border(color: Color, r: f32, width: f32) -> UBorder {
    UBorder { color, width, radius: UCornerRadius::all(r), offset: 0.0 }
}

#[rustfmt::skip]
fn cut_panel(w: UVal, h: UVal, p: USides, bg: Color, r: f32) -> UNode {
    UNode { width: w, height: h, padding: p, background_color: bg, border_radius: UCornerRadius::all(r), shape_mode: UShapeMode::Cut, ..default() }
}

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

    // Header Bar
    let header = commands
        .spawn((
            ChildOf(root),
            cut_panel(
                UVal::Percent(1.0),
                UVal::Auto,
                USides::axes(16.0, 8.0),
                Color::srgba(0.04, 0.07, 0.12, 0.9),
                8.0,
            ),
            cut_border(Color::srgba(0.0, 0.8, 1.0, 0.3), 8.0, 1.0),
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
        "Sweep Shimmer (مرور) - Rotation (دوران) - Dominant Areas - Mixed Cuts",
        Color::srgba(0.6, 0.75, 0.9, 0.85),
        12.0,
    );

    // Main Content Body
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
                state.offset,
                state.stepped_mode,
                state.dominance_ratio,
            ),
            cut_border(
                state.current_preset.accent_color().with_alpha(0.6),
                16.0,
                1.5,
            ),
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
    spawn_typed_label(
        &mut commands,
        hero_top,
        HeroLabelType::Title,
        state.current_preset.title(),
        Color::WHITE,
        20.0,
    );

    let tag = commands
        .spawn((
            ChildOf(hero_top),
            cut_panel(
                UVal::Auto,
                UVal::Auto,
                USides::axes(8.0, 4.0),
                Color::srgba(0.0, 0.0, 0.0, 0.65),
                6.0,
            ),
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
            cut_panel(
                UVal::Percent(1.0),
                UVal::Auto,
                USides::axes(12.0, 8.0),
                Color::srgba(0.0, 0.0, 0.0, 0.6),
                8.0,
            ),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_typed_label(
        &mut commands,
        hero_bottom,
        HeroLabelType::Desc,
        state.current_preset.description(),
        Color::srgba(0.9, 0.95, 1.0, 0.92),
        12.0,
    );

    // Controls Column
    let controls_col = commands
        .spawn((
            ChildOf(body),
            cut_panel(
                UVal::Percent(0.40),
                UVal::Percent(1.0),
                USides::all(14.0),
                Color::srgba(0.03, 0.05, 0.09, 0.92),
                12.0,
            ),
            cut_border(Color::srgba(0.0, 0.8, 1.0, 0.25), 12.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    let inspector_panel = commands
        .spawn((
            ChildOf(controls_col),
            cut_panel(
                UVal::Percent(1.0),
                UVal::Auto,
                USides::all(10.0),
                Color::srgba(0.02, 0.03, 0.06, 0.85),
                8.0,
            ),
            cut_border(Color::srgba(0.0, 0.85, 1.0, 0.2), 8.0, 1.0),
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
    spawn_typed_label(
        &mut commands,
        inspector_panel,
        HeroLabelType::Inspector,
        "Initializing...",
        Color::srgba(0.85, 0.92, 0.98, 0.9),
        12.0,
    );

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

    // Dominance Row (25% | 50% | 80%)
    let dom_row = spawn_row(&mut commands, btn_bar, false);
    for (label, val) in [("25%", 0.25), ("50%", 0.50), ("80%", 0.80)] {
        let (btn, _) = spawn_action_btn(
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

    // Edge Mode Button
    let (edge_btn, edge_lbl) = spawn_action_btn(
        &mut commands,
        btn_bar,
        "Edge Mode: SMOOTH",
        Color::srgb(0.65, 0.25, 0.15),
    );
    commands.entity(edge_lbl).insert(ControlBtnType::Edge);
    commands.entity(edge_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.stepped_mode = !state.stepped_mode;
        },
    );

    // Animation Controls Row (Mode + Offset Step + Angle Step)
    let anim_row = spawn_row(&mut commands, btn_bar, false);
    let (anim_btn, anim_lbl) = spawn_action_btn(
        &mut commands,
        anim_row,
        "Anim: SWEEP (مرور)",
        Color::srgb(0.0, 0.45, 0.7),
    );
    commands.entity(anim_lbl).insert(ControlBtnType::Anim);
    commands.entity(anim_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.anim_mode = match state.anim_mode {
                AnimMode::Sweep => AnimMode::Rotate,
                AnimMode::Rotate => AnimMode::Off,
                AnimMode::Off => AnimMode::Sweep,
            };
        },
    );

    let (offset_btn, _) = spawn_action_btn(
        &mut commands,
        anim_row,
        "Offset +0.2",
        Color::srgb(0.12, 0.42, 0.48),
    );
    commands.entity(offset_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.offset = (state.offset + 0.2) % 2.0;
        },
    );

    let (angle_btn, _) = spawn_action_btn(
        &mut commands,
        anim_row,
        "Angle +45°",
        Color::srgb(0.18, 0.45, 0.4),
    );
    commands.entity(angle_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.angle = (state.angle + FRAC_PI_4) % (PI * 2.0);
        },
    );

    // Shape Mode Button
    let (shape_btn, shape_lbl) = spawn_action_btn(
        &mut commands,
        btn_bar,
        "Shape Mode: CUT",
        Color::srgb(0.2, 0.35, 0.6),
    );
    commands.entity(shape_lbl).insert(ControlBtnType::Shape);
    commands.entity(shape_btn).observe(
        |_click: On<Pointer<Click>>, mut state: ResMut<GradientDemoState>| {
            state.shape_mode = match state.shape_mode {
                UShapeMode::Cut => UShapeMode::Round,
                UShapeMode::Round => UShapeMode::Cut,
            };
        },
    );

    // Bottom Gallery Grid
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
        spawn_gallery_card(&mut commands, gallery_grid, preset);
    }
}

#[rustfmt::skip]
fn card_interactions(bg: Color) -> (UInteraction, UInteractionColors) {
    (UInteraction::default(), UInteractionColors { normal: bg, hovered: bg.with_alpha(0.85), pressed: bg.with_alpha(0.6) })
}

fn spawn_gallery_card(commands: &mut Commands, parent: Entity, preset: PresetId) {
    let card = commands
        .spawn((
            ChildOf(parent),
            cut_panel(
                UVal::Auto,
                UVal::Px(105.0),
                USides::all(8.0),
                Color::srgba(0.04, 0.07, 0.12, 0.9),
                8.0,
            ),
            preset.to_gradient(FRAC_PI_4, 0.0, false, 0.75),
            cut_border(preset.accent_color().with_alpha(0.4), 8.0, 1.0),
            card_interactions(Color::srgba(0.04, 0.07, 0.12, 0.9)),
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

    spawn_label(commands, card, preset.title(), Color::WHITE, 10.0);
    let badge = commands
        .spawn((
            ChildOf(card),
            cut_panel(
                UVal::Auto,
                UVal::Auto,
                USides::axes(5.0, 2.0),
                Color::srgba(0.0, 0.0, 0.0, 0.55),
                4.0,
            ),
            ULayout::default(),
        ))
        .id();
    spawn_label(
        commands,
        badge,
        &format!("{} STOPS", preset.stop_count()),
        preset.accent_color(),
        9.0,
    );
}

#[rustfmt::skip]
fn spawn_row(commands: &mut Commands, parent: Entity, space_between: bool) -> Entity {
    let justify = if space_between { UJustifyContent::SpaceBetween } else { UJustifyContent::Start };
    commands.spawn((
        ChildOf(parent),
        UNode { width: UVal::Percent(1.0), ..default() },
        ULayout { display: UDisplay::Flex, flex_direction: UFlexDirection::Row, justify_content: justify, align_items: UAlignItems::Center, gap: 6.0, ..default() },
    )).id()
}

#[rustfmt::skip]
fn spawn_label(commands: &mut Commands, parent: Entity, text: &str, color: Color, font_size: f32) -> Entity {
    commands.spawn((ChildOf(parent), UNode::default(), UTextLabel { text: text.to_string(), color, font_size, ..default() })).id()
}

#[rustfmt::skip]
fn spawn_typed_label(commands: &mut Commands, parent: Entity, label_type: HeroLabelType, text: &str, color: Color, font_size: f32) {
    commands.spawn((ChildOf(parent), label_type, UNode::default(), UTextLabel { text: text.to_string(), color, font_size, ..default() }));
}

fn spawn_action_btn(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    bg: Color,
) -> (Entity, Entity) {
    let btn = commands
        .spawn((
            ChildOf(parent),
            cut_panel(UVal::Auto, UVal::Auto, USides::axes(10.0, 7.0), bg, 6.0),
            cut_border(Color::srgba(0.0, 0.9, 1.0, 0.35), 6.0, 1.0),
            card_interactions(bg),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let text_entity = commands
        .spawn((
            ChildOf(btn),
            UNode::default(),
            UTextLabel {
                text: label.to_string(),
                color: Color::WHITE,
                font_size: 11.0,
                ..default()
            },
        ))
        .id();

    (btn, text_entity)
}

fn update_demo(time: Res<Time>, mut state: ResMut<GradientDemoState>) {
    let dt = time.delta_secs();
    match state.anim_mode {
        AnimMode::Sweep => {
            state.offset += dt * 0.75;
            if state.offset > 1.25 {
                state.offset = -1.25;
            }
        }
        AnimMode::Rotate => {
            state.angle += dt * 0.75;
            if state.angle > PI * 2.0 {
                state.angle -= PI * 2.0;
            }
        }
        AnimMode::Off => {}
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
    mut hero_labels: Query<(&mut UTextLabel, &HeroLabelType)>,
    mut btn_labels: Query<(&mut UTextLabel, &ControlBtnType)>,
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
        *grad = preset.to_gradient(
            state.angle,
            state.offset,
            state.stepped_mode,
            state.dominance_ratio,
        );
        border.color = accent.with_alpha(0.65);
        shadow.color = accent.with_alpha(0.35);
        *inner_glow = if state.inner_glow_enabled {
            UInnerGlow::new(accent.with_alpha(0.3), 12.0)
        } else {
            UInnerGlow::new(Color::NONE, 0.0)
        };
    }

    // 2. Update UI labels
    for (mut label, label_type) in &mut hero_labels {
        match label_type {
            HeroLabelType::Title => label.text = preset.title().to_string(),
            HeroLabelType::Desc => label.text = preset.description().to_string(),
            HeroLabelType::Inspector => {
                let edge_str = if state.stepped_mode {
                    "Hard (Stepped Bands)"
                } else {
                    "Smooth (Blended)"
                };
                let anim_str = match state.anim_mode {
                    AnimMode::Sweep => "SWEEP (مرور / شيمر)",
                    AnimMode::Rotate => "ROTATE (دوران)",
                    AnimMode::Off => "OFF (ثابت)",
                };
                let dom_pct = (state.dominance_ratio * 100.0).round() as i32;
                let angle_deg = (state.angle.to_degrees() % 360.0).round() as i32;
                label.text = format!(
                    "Active: {}\nStops: {} | Dominance: {}% | {}\nAnim: {} | Offset: {:.2} | Angle: {}°\nBehavior: {}",
                    preset.title(),
                    preset.stop_count(),
                    dom_pct,
                    edge_str,
                    anim_str,
                    state.offset,
                    angle_deg,
                    preset.description(),
                );
            }
        }
    }

    // 3. Update button labels
    for (mut label, btn_type) in &mut btn_labels {
        match btn_type {
            ControlBtnType::Anim => {
                label.text = match state.anim_mode {
                    AnimMode::Sweep => "Anim: SWEEP (مرور)".to_string(),
                    AnimMode::Rotate => "Anim: ROTATE (دوران)".to_string(),
                    AnimMode::Off => "Anim: OFF (إيقاف)".to_string(),
                };
            }
            ControlBtnType::Edge => {
                label.text = format!(
                    "Edge Mode: {}",
                    if state.stepped_mode {
                        "HARD (STEPPED)"
                    } else {
                        "SMOOTH"
                    }
                );
            }
            ControlBtnType::Shape => {
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
