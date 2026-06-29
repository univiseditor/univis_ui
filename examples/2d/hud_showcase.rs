//! # 2D Sci-Fi Gaming HUD Showcase
//!
//! A rich screen-space overlay demonstrating how to build game HUDs using Univis UI.
//! Includes a crosshair, status bars (health/shield), interactive sliders to adjust stats,
//! weapon select consoles, an animated radar sweep, and dynamic critical warning displays.

use bevy::prelude::*;
use univis_ui::prelude::*;

// ── Marker Components ────────────────────────────────────────────────────────
#[derive(Component)]
struct HealthSeekBar;

#[derive(Component)]
struct ShieldSeekBar;

#[derive(Component)]
struct WarningToggle;

#[derive(Component)]
struct WeaponSelectToggle;

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct ShieldBarFill;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ShieldText;

#[derive(Component)]
struct WeaponLabel;

#[derive(Component)]
struct AlertBox;

#[derive(Component)]
struct AlertText;

#[derive(Component)]
struct RadarDot {
    angle_offset: f32,
    distance: f32,
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI - Sci-Fi Tactical HUD Showcase".into(),
                    resolution: (1200, 800).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .add_systems(Startup, setup_hud)
        .add_systems(Update, animate_hud)
        .run();
}

fn setup_hud(mut commands: Commands) {
    // 1. Spawn Camera2d
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.03, 0.05)),
            ..default()
        },
    ));

    // 2. Fullscreen Screen-Space Root UI
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    // ── Center Reticle / Crosshair ───────────────────────────────────────────
    let crosshair_root = commands
        .spawn((
            ChildOf(root),
            absolute_pos(UVal::Percent(0.5), UVal::Percent(0.5), 0),
            UNode {
                width: UVal::Px(120.0),
                height: UVal::Px(120.0),
                margin: USides {
                    left: -60.0,
                    top: -60.0,
                    right: 0.0,
                    bottom: 0.0,
                },
                background_color: Color::NONE,
                border_radius: UCornerRadius::all(60.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 1.2, 2.5, 0.2), // Faint outer ring
                width: 2.0,
                radius: UCornerRadius::all(60.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Inner reticle circle
    commands.spawn((
        ChildOf(crosshair_root),
        UNode {
            width: UVal::Px(40.0),
            height: UVal::Px(40.0),
            background_color: Color::NONE,
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        UBorder {
            color: Color::srgb(0.0, 1.2, 2.5), // Inner cyan target ring
            width: 1.5,
            radius: UCornerRadius::all(20.0),
            offset: 0.0,
        },
    ));

    // Center target dot
    commands.spawn((
        ChildOf(crosshair_root),
        absolute_pos(UVal::Px(58.0), UVal::Px(58.0), 0),
        UNode {
            width: UVal::Px(4.0),
            height: UVal::Px(4.0),
            background_color: Color::srgb(0.0, 1.5, 3.0),
            border_radius: UCornerRadius::all(2.0),
            ..default()
        },
    ));

    // ── Top Bar: Alerts and Warning Banner ───────────────────────────────────
    let top_bar = commands
        .spawn((
            ChildOf(root),
            absolute_pos(UVal::Percent(0.5), UVal::Px(24.0), 0),
            AlertBox,
            UNode {
                width: UVal::Px(540.0),
                height: UVal::Px(50.0),
                margin: USides::left(-270.0), // Center align horizontally
                padding: USides::axes(24.0, 0.0),
                background_color: Color::srgba(0.04, 0.05, 0.08, 0.75),
                border_radius: UCornerRadius::all(10.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 1.2, 2.5, 0.4),
                width: 1.5,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(top_bar),
        AlertText,
        UNode::default(),
        UTextLabel {
            text: "◈ ALL SYSTEMS ACTIVE ◈".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.3, 0.8, 1.0),
            ..default()
        },
    ));

    // ── Bottom-Left Bar: Health and Shield bars ──────────────────────────────
    let stats_panel = commands
        .spawn((
            ChildOf(root),
            absolute_pos(UVal::Px(32.0), UVal::Auto, 0),
            UNode {
                width: UVal::Px(420.0),
                height: UVal::Px(160.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.75),
                border_radius: UCornerRadius::all(16.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
        ))
        .id();

    // Health Bar Row
    let health_row = commands
        .spawn((
            ChildOf(stats_panel),
            UNode::default(),
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
        ChildOf(health_row),
        UNode::default(),
        UTextLabel {
            text: "VIT / VITALS".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.9, 0.2, 0.3),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(health_row),
        HealthText,
        UNode::default(),
        UTextLabel {
            text: "HP: 100%".to_string(),
            font_size: 13.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    // Health Progress Bar Container
    let health_bg = commands
        .spawn((
            ChildOf(stats_panel),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(16.0),
                background_color: Color::srgba(1.0, 0.0, 0.0, 0.1),
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            ULayout::default(),
            UBorder {
                color: Color::srgba(1.0, 0.2, 0.3, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(4.0),
                offset: 0.0,
            },
        ))
        .id();

    // Inner Green/Red Fill
    commands.spawn((
        ChildOf(health_bg),
        HealthBarFill,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.9, 0.2, 0.3), // Start with red
            border_radius: UCornerRadius::all(4.0),
            ..default()
        },
    ));

    // Shield Bar Row
    let shield_row = commands
        .spawn((
            ChildOf(stats_panel),
            UNode {
                margin: USides::top(8.0),
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
        ChildOf(shield_row),
        UNode::default(),
        UTextLabel {
            text: "SHD / SHIELD".to_string(),
            font_size: 13.0,
            color: Color::srgb(0.2, 0.6, 1.0),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(shield_row),
        ShieldText,
        UNode::default(),
        UTextLabel {
            text: "SHIELD: 100%".to_string(),
            font_size: 13.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    // Shield Progress Bar Container
    let shield_bg = commands
        .spawn((
            ChildOf(stats_panel),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(16.0),
                background_color: Color::srgba(0.0, 0.5, 1.0, 0.1),
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            ULayout::default(),
            UBorder {
                color: Color::srgba(0.2, 0.6, 1.0, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(4.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(shield_bg),
        ShieldBarFill,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.2, 0.6, 1.0),
            border_radius: UCornerRadius::all(4.0),
            ..default()
        },
    ));

    // ── Bottom-Right Bar: Weapon status ──────────────────────────────────────
    let weapon_panel = commands
        .spawn((
            ChildOf(root),
            absolute_pos(UVal::Auto, UVal::Auto, 0),
            UNode {
                width: UVal::Px(380.0),
                height: UVal::Px(120.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.75),
                border_radius: UCornerRadius::all(16.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
        ))
        .id();

    commands.spawn((
        ChildOf(weapon_panel),
        WeaponLabel,
        UNode::default(),
        UTextLabel {
            text: "WEAPON: PULSE RAILGUN".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.3, 0.7, 1.0),
            ..default()
        },
    ));

    // Ammo Counter Row
    let ammo_row = commands
        .spawn((
            ChildOf(weapon_panel),
            UNode {
                margin: USides::top(6.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Baseline,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(ammo_row),
        UNode::default(),
        UTextLabel {
            text: "AMMO CAPACITY".to_string(),
            font_size: 12.0,
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(ammo_row),
        UNode::default(),
        UTextLabel {
            text: "32 / 128".to_string(),
            font_size: 24.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    // ── Top-Right Column: Animated Mini-Radar & Interactive Controls ────────
    let sidebar = commands
        .spawn((
            ChildOf(root),
            absolute_pos(UVal::Auto, UVal::Px(32.0), 0),
            UNode {
                width: UVal::Px(320.0),
                height: UVal::Px(520.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.01, 0.02, 0.04, 0.85),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                align_items: UAlignItems::Center,
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

    // Radar Header
    commands.spawn((
        ChildOf(sidebar),
        UNode::default(),
        UTextLabel {
            text: "TACTICAL RADAR".to_string(),
            font_size: 15.0,
            color: Color::srgb(0.3, 0.7, 1.0),
            ..default()
        },
    ));

    // Mini-Radar Screen Container
    let radar_screen = commands
        .spawn((
            ChildOf(sidebar),
            UNode {
                width: UVal::Px(160.0),
                height: UVal::Px(160.0),
                background_color: Color::srgba(0.0, 0.2, 0.4, 0.05),
                border_radius: UCornerRadius::all(80.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(80.0),
                offset: 0.0,
            },
            ULayout::default(),
        ))
        .id();

    // Radar Grid Rings (Static visual elements)
    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(20.0), UVal::Px(20.0), 0),
        UNode {
            width: UVal::Px(120.0),
            height: UVal::Px(120.0),
            background_color: Color::NONE,
            border_radius: UCornerRadius::all(60.0),
            ..default()
        },
        UBorder {
            color: Color::srgba(0.2, 0.5, 1.0, 0.15),
            width: 1.0,
            radius: UCornerRadius::all(60.0),
            offset: 0.0,
        },
    ));

    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(50.0), UVal::Px(50.0), 0),
        UNode {
            width: UVal::Px(60.0),
            height: UVal::Px(60.0),
            background_color: Color::NONE,
            border_radius: UCornerRadius::all(30.0),
            ..default()
        },
        UBorder {
            color: Color::srgba(0.2, 0.5, 1.0, 0.15),
            width: 1.0,
            radius: UCornerRadius::all(30.0),
            offset: 0.0,
        },
    ));

    // Center Cross-hairs
    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(0.0), UVal::Px(79.0), 0),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(1.0),
            background_color: Color::srgba(0.2, 0.5, 1.0, 0.1),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(79.0), UVal::Px(0.0), 0),
        UNode {
            width: UVal::Px(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.2, 0.5, 1.0, 0.1),
            ..default()
        },
    ));

    // Dynamic Rotating Threat Radar Dots (Orbiting targets)
    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(0.0), UVal::Px(0.0), 1),
        RadarDot {
            angle_offset: 0.0,
            distance: 54.0,
            speed: 1.2,
        },
        UNode {
            width: UVal::Px(6.0),
            height: UVal::Px(6.0),
            background_color: Color::srgb(1.0, 0.1, 0.2), // Enemy dot
            border_radius: UCornerRadius::all(3.0),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(radar_screen),
        absolute_pos(UVal::Px(0.0), UVal::Px(0.0), 1),
        RadarDot {
            angle_offset: 2.2,
            distance: 36.0,
            speed: -0.8,
        },
        UNode {
            width: UVal::Px(6.0),
            height: UVal::Px(6.0),
            background_color: Color::srgb(1.0, 0.1, 0.2), // Enemy dot 2
            border_radius: UCornerRadius::all(3.0),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(sidebar),
        UNode {
            height: UVal::Px(1.0),
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.2, 0.5, 1.0, 0.2),
            ..default()
        },
    ));

    // Dashboard Interactive controls title
    commands.spawn((
        ChildOf(sidebar),
        UNode {
            margin: USides::bottom(4.0),
            ..default()
        },
        UTextLabel {
            text: "◈ CONTROLS DASHBOARD ◈".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    // Closure to build control rows
    let mut add_hud_slider =
        |label: &str, min_val: f32, max_val: f32, initial_val: f32, marker: ComponentBox| {
            let row = commands
                .spawn((
                    ChildOf(sidebar),
                    UNode {
                        width: UVal::Percent(1.0),
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
                ChildOf(row),
                UNode::default(),
                UTextLabel {
                    text: label.to_string(),
                    color: Color::srgb(0.7, 0.8, 0.9),
                    font_size: 12.0,
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

    add_hud_slider(
        "Simulation Vitals (HP):",
        0.0,
        100.0,
        80.0,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(HealthSeekBar);
            },
        },
    );

    add_hud_slider(
        "Simulation Shields:",
        0.0,
        100.0,
        60.0,
        ComponentBox {
            apply: |c, e| {
                c.entity(e).insert(ShieldSeekBar);
            },
        },
    );

    // Control Toggles Row
    let toggles_row = commands
        .spawn((
            ChildOf(sidebar),
            UNode {
                width: UVal::Percent(1.0),
                margin: USides::top(8.0),
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

    // Warning alert toggle
    let alert_cell = commands
        .spawn((
            ChildOf(toggles_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(alert_cell),
        UNode::default(),
        UTextLabel {
            text: "Danger Alert".to_string(),
            font_size: 11.0,
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(alert_cell),
        WarningToggle,
        UToggle::sci_fi_style().with_checked(false),
    ));

    // Weapon select toggle
    let weapon_cell = commands
        .spawn((
            ChildOf(toggles_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(weapon_cell),
        UNode::default(),
        UTextLabel {
            text: "Weapon Mode".to_string(),
            font_size: 11.0,
            color: Color::WHITE,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(weapon_cell),
        WeaponSelectToggle,
        UToggle::sci_fi_style().with_checked(false),
    ));
}

fn animate_hud(
    time: Res<Time>,
    seekbar_health: Query<&USeekBar, With<HealthSeekBar>>,
    seekbar_shield: Query<&USeekBar, With<ShieldSeekBar>>,
    toggle_warning: Query<&UToggle, With<WarningToggle>>,
    toggle_weapon: Query<&UToggle, With<WeaponSelectToggle>>,
    mut fill_health: Query<&mut UNode, (With<HealthBarFill>, Without<ShieldBarFill>)>,
    mut fill_shield: Query<&mut UNode, (With<ShieldBarFill>, Without<HealthBarFill>)>,
    mut text_health: Query<
        &mut UTextLabel,
        (
            With<HealthText>,
            Without<ShieldText>,
            Without<WeaponLabel>,
            Without<AlertText>,
        ),
    >,
    mut text_shield: Query<
        &mut UTextLabel,
        (
            With<ShieldText>,
            Without<HealthText>,
            Without<WeaponLabel>,
            Without<AlertText>,
        ),
    >,
    mut weapon_label: Query<
        &mut UTextLabel,
        (
            With<WeaponLabel>,
            Without<HealthText>,
            Without<ShieldText>,
            Without<AlertText>,
        ),
    >,
    mut alert_box: Query<
        (&mut UBorder, &mut UNode),
        (
            With<AlertBox>,
            Without<HealthBarFill>,
            Without<ShieldBarFill>,
        ),
    >,
    mut alert_text: Query<
        &mut UTextLabel,
        (
            With<AlertText>,
            Without<HealthText>,
            Without<ShieldText>,
            Without<WeaponLabel>,
        ),
    >,
    mut radar_dots: Query<(&mut USelf, &RadarDot)>,
) {
    // 1. Get current values from seekbars & toggles
    let health_val = seekbar_health
        .iter()
        .next()
        .map_or(80.0, |sb| sb.real_value());
    let shield_val = seekbar_shield
        .iter()
        .next()
        .map_or(60.0, |sb| sb.real_value());
    let warning_active = toggle_warning.iter().next().map_or(false, |t| t.checked);
    let weapon_mode_secondary = toggle_weapon.iter().next().map_or(false, |t| t.checked);

    let health_pct = health_val / 100.0;
    let shield_pct = shield_val / 100.0;

    // 2. Update health bar fill & vitals text
    for mut node in &mut fill_health {
        node.width = UVal::Percent(health_pct);
        // Turn health bar red if health is low, green/orange otherwise
        if health_pct < 0.25 {
            node.background_color = Color::srgb(1.0, 0.1, 0.15);
        } else {
            node.background_color = Color::srgb(0.9, 0.2, 0.3);
        }
    }
    for mut label in &mut text_health {
        label.text = format!("HP: {:.0}%", health_val);
        if health_pct < 0.25 {
            label.color = Color::srgb(1.0, 0.2, 0.2);
        } else {
            label.color = Color::WHITE;
        }
    }

    // 3. Update shield bar fill & shields text
    for mut node in &mut fill_shield {
        node.width = UVal::Percent(shield_pct);
    }
    for mut label in &mut text_shield {
        label.text = format!("SHIELD: {:.0}%", shield_val);
    }

    // 4. Update weapon console selected text
    for mut label in &mut weapon_label {
        if weapon_mode_secondary {
            label.text = "WEAPON: PLASMA RIFLE".to_string();
            label.color = Color::srgb(0.9, 0.4, 0.15); // Neon Orange
        } else {
            label.text = "WEAPON: PULSE RAILGUN".to_string();
            label.color = Color::srgb(0.3, 0.7, 1.0); // Neon Cyan
        }
    }

    // 5. Update Alert Warning display (top center)
    let t = time.elapsed_secs();
    let is_alerting = warning_active || health_pct < 0.25;

    for (mut border, mut node) in &mut alert_box {
        if is_alerting {
            // Flashing Red Alert Box
            let alpha = 0.3 + (t * 6.0).sin().abs() * 0.6;
            border.color = Color::srgba(1.0, 0.15, 0.15, alpha);
            border.width = 2.0;
            node.background_color = Color::srgba(0.12, 0.01, 0.02, 0.9);
        } else {
            // Static Cyan Box
            border.color = Color::srgba(0.0, 1.2, 2.5, 0.4);
            border.width = 1.5;
            node.background_color = Color::srgba(0.04, 0.05, 0.08, 0.75);
        }
    }
    for mut label in &mut alert_text {
        if is_alerting {
            label.text = "◈ CRITICAL HULL INTEGRITY LOW ◈".to_string();
            label.color = Color::srgb(1.0, 0.2, 0.2);
        } else {
            label.text = "◈ ALL SYSTEMS OPERATIONAL ◈".to_string();
            label.color = Color::srgb(0.3, 0.8, 1.0);
        }
    }

    // 6. Animate Radar Dot Positions
    for (mut uself, dot) in &mut radar_dots {
        let angle = t * dot.speed + dot.angle_offset;
        // Calculate (x, y) relative to radar center (80, 80) minus dot radius (3)
        let x = 80.0 - 3.0 + angle.cos() * dot.distance;
        let y = 80.0 - 3.0 + angle.sin() * dot.distance;
        uself.left = UVal::Px(x);
        uself.top = UVal::Px(y);
    }
}

// Helper function to return absolute positioning coordinates
fn absolute_pos(left: UVal, top: UVal, order: i32) -> (USelf, UZIndex) {
    (
        USelf {
            left,
            top,
            position_type: UPositionType::Absolute,
            ..default()
        },
        UZIndex::Local(order),
    )
}
