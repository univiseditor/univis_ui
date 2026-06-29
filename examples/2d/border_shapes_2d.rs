//! # 2D Border & Shapes Showcase
//!
//! An interactive Bevy example showing off the SDF-based 2D border capabilities
//! of Univis UI, with various corner radiuses, shape modes (Round vs Cut), and dynamic animations.
//!
//! **Controls:**
//! - Click and drag sliders to dynamically tune 2D borders!

use bevy::prelude::*;
use univis_ui::prelude::*;

// ── App Settings ────────────────────────────────────────────────────────────
#[derive(Resource)]
struct AppSettings {
    border_width: f32,
    border_radius: f32,
    border_offset: f32,
    shape_mode: UShapeMode,
    pulse_enabled: bool,
    pulse_speed: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            border_width: 8.0,
            border_radius: 20.0,
            border_offset: 0.0,
            shape_mode: UShapeMode::Round,
            pulse_enabled: false,
            pulse_speed: 2.0,
        }
    }
}

// ── Components ───────────────────────────────────────────────────────────────
#[derive(Component)]
struct MainTargetPanel;

// UI marker components
#[derive(Component)]
struct BorderWidthSeekBar;

#[derive(Component)]
struct BorderRadiusSeekBar;

#[derive(Component)]
struct BorderOffsetSeekBar;

#[derive(Component)]
struct PulseSpeedSeekBar;

#[derive(Component)]
struct ShapeToggle;

#[derive(Component)]
struct PulseToggle;

#[derive(Component)]
struct StatusText;

// ── Main Entry ──────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI 2D Border & Corner Shapes Showcase".into(),
                    resolution: (1100, 800).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<AppSettings>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (sync_ui_to_settings, animate_pulse_border))
        .run();
}

// ── Setup System ─────────────────────────────────────────────────────────────
fn setup_scene(mut commands: Commands, settings: Res<AppSettings>) {
    // 1. Camera2d with clear color
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.03, 0.04, 0.06)),
            ..default()
        },
    ));

    // 2. Screen Space UI Root
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(30.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row, // Left-to-right columns
                gap: 32.0,
                ..default()
            },
        ))
        .id();

    // 3. Left Column: Control Panel
    let control_panel = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(460.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.25),
                width: 2.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            Pickable::default(),
        ))
        .id();

    // Control Headers
    commands.spawn((
        ChildOf(control_panel),
        UNode::default(),
        UTextLabel {
            text: "◈ 2D BORDER & SHAPES ◈".to_string(),
            color: Color::srgb(0.3, 0.7, 1.0),
            font_size: 22.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(control_panel),
        StatusText,
        UNode {
            margin: USides::bottom(8.0),
            ..default()
        },
        UTextLabel {
            text: "".to_string(),
            color: Color::srgb(0.7, 0.85, 0.95),
            font_size: 15.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(control_panel),
        UNode {
            height: UVal::Px(2.0),
            background_color: Color::srgba(0.2, 0.5, 1.0, 0.2),
            margin: USides::bottom(6.0),
            ..default()
        },
    ));

    // Helper closure to build sliders
    let mut add_slider = |parent_entity: Entity,
                          label: &str,
                          min_val: f32,
                          max_val: f32,
                          initial_val: f32,
                          marker: ComponentBox| {
        let row = commands
            .spawn((
                ChildOf(parent_entity),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(row),
            UNode {
                width: UVal::Px(160.0),
                ..default()
            },
            UTextLabel {
                text: label.to_string(),
                color: Color::WHITE,
                font_size: 14.0,
                ..default()
            },
        ));

        let seek_val = (initial_val - min_val) / (max_val - min_val);
        let seekbar_entity = commands
            .spawn((
                ChildOf(row),
                USeekBar::sci_fi_style()
                    .with_range(min_val, max_val)
                    .with_value(seek_val)
                    .show_value(),
            ))
            .id();

        marker.add_to(&mut commands, seekbar_entity);
    };

    struct ComponentBox {
        apply: fn(&mut Commands, Entity),
    }
    impl ComponentBox {
        fn add_to(&self, commands: &mut Commands, entity: Entity) {
            (self.apply)(commands, entity);
        }
    }

    add_slider(
        control_panel,
        "Border Width:",
        0.0,
        30.0,
        settings.border_width,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(BorderWidthSeekBar);
            },
        },
    );

    add_slider(
        control_panel,
        "Border Radius:",
        0.0,
        60.0,
        settings.border_radius,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(BorderRadiusSeekBar);
            },
        },
    );

    add_slider(
        control_panel,
        "Border Offset:",
        -15.0,
        30.0,
        settings.border_offset,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(BorderOffsetSeekBar);
            },
        },
    );

    add_slider(
        control_panel,
        "Pulse Speed:",
        0.5,
        5.0,
        settings.pulse_speed,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(PulseSpeedSeekBar);
            },
        },
    );

    // Control Toggles Row
    let toggles_row = commands
        .spawn((
            ChildOf(control_panel),
            UNode {
                margin: USides::top(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceAround,
                ..default()
            },
        ))
        .id();

    // Shape Toggle (Round vs Cut)
    let shape_cell = commands
        .spawn((
            ChildOf(toggles_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(shape_cell),
        UNode::default(),
        UTextLabel {
            text: "Cut Corners".to_string(),
            color: Color::WHITE,
            font_size: 13.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(shape_cell),
        ShapeToggle,
        UToggle::sci_fi_style().with_checked(false),
    ));

    // Pulse Animation Toggle
    let pulse_cell = commands
        .spawn((
            ChildOf(toggles_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(pulse_cell),
        UNode::default(),
        UTextLabel {
            text: "Pulse Border".to_string(),
            color: Color::WHITE,
            font_size: 13.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(pulse_cell),
        PulseToggle,
        UToggle::sci_fi_style().with_checked(settings.pulse_enabled),
    ));

    // 4. Right Column: Preview Area and Presets
    let right_column = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    // A. Top Preview Card (Interactive)
    let preview_card = commands
        .spawn((
            ChildOf(right_column),
            MainTargetPanel,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(320.0),
                padding: USides::all(30.0),
                background_color: Color::srgba(0.06, 0.08, 0.14, 0.5),
                border_radius: UCornerRadius::all(settings.border_radius),
                shape_mode: settings.shape_mode,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
            UBorder {
                color: Color::srgb(0.0, 1.5, 3.0), // Neon Cyan
                width: settings.border_width,
                radius: UCornerRadius::all(settings.border_radius),
                offset: settings.border_offset,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preview_card),
        UNode::default(),
        UTextLabel {
            text: "2D INTERACTIVE PREVIEW".to_string(),
            color: Color::WHITE,
            font_size: 26.0,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(preview_card),
        UNode::default(),
        UTextLabel {
            text: "Adjust sliders to see 2D SDF border properties transform live!\nTurn on 'Pulse Border' to trigger dynamic border thickness and offset animations.".to_string(),
            color: Color::srgb(0.7, 0.8, 0.95),
            font_size: 15.0,
            ..default()
        },
    ));

    // B. Bottom Presets Row
    let presets_row = commands
        .spawn((
            ChildOf(right_column),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
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

    // Helper closure to build preset cards
    let mut add_preset_card = |label: &str,
                               radius_val: f32,
                               width_val: f32,
                               offset_val: f32,
                               shape_val: UShapeMode,
                               border_color: Color,
                               bg_color: Color,
                               desc: &str| {
        let card = commands
            .spawn((
                ChildOf(presets_row),
                UNode {
                    width: UVal::Percent(0.33),
                    height: UVal::Px(280.0),
                    padding: USides::all(20.0),
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(radius_val),
                    shape_mode: shape_val,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                    ..default()
                },
                UBorder {
                    color: border_color,
                    width: width_val,
                    radius: UCornerRadius::all(radius_val),
                    offset: offset_val,
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            UNode::default(),
            UTextLabel {
                text: label.to_string(),
                color: border_color,
                font_size: 16.0,
                ..default()
            },
        ));

        commands.spawn((
            ChildOf(card),
            UNode::default(),
            UTextLabel {
                text: desc.to_string(),
                color: Color::srgb(0.8, 0.82, 0.85),
                font_size: 13.0,
                ..default()
            },
        ));
    };

    // Preset 1: Tech Card (Small Rounded corners)
    add_preset_card(
        "TECH TERMINAL",
        6.0,
        3.0,
        0.0,
        UShapeMode::Round,
        Color::srgb(0.0, 1.0, 0.4), // Emerald Green
        Color::srgba(0.01, 0.05, 0.02, 0.85),
        "Radius: 6.0px (Round)\nWidth: 3.0px\nOffset: 0.0px\n\nSharp, clean cyber look for micro-widgets.",
    );

    // Preset 2: Sci-Fi Armor (Medium Chamfered corners)
    add_preset_card(
        "CHAMFERED SHIELD",
        16.0,
        6.0,
        0.0,
        UShapeMode::Cut,
        Color::srgb(1.0, 0.5, 0.0), // Neon Amber
        Color::srgba(0.06, 0.03, 0.01, 0.85),
        "Radius: 16.0px (Cut)\nWidth: 6.0px\nOffset: 0.0px\n\nChamfered corners creating a protective frame.",
    );

    // Preset 3: Floating Ring (Large Rounded offset)
    add_preset_card(
        "FLOATING RING",
        28.0,
        4.0,
        12.0,
        UShapeMode::Round,
        Color::srgb(0.0, 0.9, 1.0), // Cyan Halo
        Color::srgba(0.01, 0.04, 0.06, 0.8),
        "Radius: 28.0px (Round)\nWidth: 4.0px\nOffset: 12.0px\n\nFloating border halo offset outside layout boundaries.",
    );
}

// ── Sync UI to Settings System ───────────────────────────────────────────────
fn sync_ui_to_settings(
    mut settings: ResMut<AppSettings>,
    toggle_shape: Query<&UToggle, With<ShapeToggle>>,
    toggle_pulse: Query<&UToggle, With<PulseToggle>>,
    seekbar_width: Query<&USeekBar, With<BorderWidthSeekBar>>,
    seekbar_radius: Query<&USeekBar, With<BorderRadiusSeekBar>>,
    seekbar_offset: Query<&USeekBar, With<BorderOffsetSeekBar>>,
    seekbar_pulse_speed: Query<&USeekBar, With<PulseSpeedSeekBar>>,
    mut target_panel_query: Query<(&mut UBorder, &mut UNode), With<MainTargetPanel>>,
    mut status_text_query: Query<&mut UTextLabel, With<StatusText>>,
) {
    if let Some(toggle) = toggle_shape.iter().next() {
        settings.shape_mode = if toggle.checked {
            UShapeMode::Cut
        } else {
            UShapeMode::Round
        };
    }
    if let Some(toggle) = toggle_pulse.iter().next() {
        settings.pulse_enabled = toggle.checked;
    }
    if let Some(seekbar) = seekbar_width.iter().next() {
        settings.border_width = seekbar.real_value();
    }
    if let Some(seekbar) = seekbar_radius.iter().next() {
        settings.border_radius = seekbar.real_value();
    }
    if let Some(seekbar) = seekbar_offset.iter().next() {
        settings.border_offset = seekbar.real_value();
    }
    if let Some(seekbar) = seekbar_pulse_speed.iter().next() {
        settings.pulse_speed = seekbar.real_value();
    }

    // Apply metallic, roughness, shape mode, and border changes to the main target panel
    // Only apply static border settings when NOT pulsing
    if !settings.pulse_enabled {
        for (mut border, mut node) in &mut target_panel_query {
            border.width = settings.border_width;
            border.radius = UCornerRadius::all(settings.border_radius);
            border.offset = settings.border_offset;

            node.border_radius = UCornerRadius::all(settings.border_radius);
            node.shape_mode = settings.shape_mode;
        }
    } else {
        // Keep the shape mode and border radius synced even when pulsing
        for (mut border, mut node) in &mut target_panel_query {
            border.radius = UCornerRadius::all(settings.border_radius);
            node.border_radius = UCornerRadius::all(settings.border_radius);
            node.shape_mode = settings.shape_mode;
        }
    }

    // Update status text on the control panel
    for mut label in &mut status_text_query {
        label.text = format!(
            "Width: {:.1}px | Radius: {:.1}px | Offset: {:.1}px\nCorner Shape: {:?} | Pulse Anim: {}",
            settings.border_width,
            settings.border_radius,
            settings.border_offset,
            settings.shape_mode,
            if settings.pulse_enabled {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
    }
}

// ── Pulse Animation System ───────────────────────────────────────────────────
fn animate_pulse_border(
    time: Res<Time>,
    settings: Res<AppSettings>,
    mut target_panel_query: Query<&mut UBorder, With<MainTargetPanel>>,
) {
    if !settings.pulse_enabled {
        return;
    }
    let t = time.elapsed_secs() * settings.pulse_speed;
    // Oscillate border width from 2.0 to 18.0 using sine
    let width_osc = 2.0 + (t.sin() * 0.5 + 0.5) * 16.0;
    // Oscillate border offset from -8.0 to 16.0 using cosine
    let offset_osc = -8.0 + (t.cos() * 0.5 + 0.5) * 24.0;

    for mut border in &mut target_panel_query {
        border.width = width_osc;
        border.offset = offset_osc;
    }
}
