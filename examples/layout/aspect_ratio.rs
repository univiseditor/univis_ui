use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - CSS aspect-ratio Showcase".to_string(),
                resolution: (1240, 780).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<AspectRatioState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_telemetry))
        .run();
}

#[derive(Clone, Copy, PartialEq)]
enum PresetRatio {
    Cinema16x9,
    Classic4x3,
    Square1x1,
    Ultrawide21x9,
    Mobile9x16,
}

impl PresetRatio {
    fn ratio(self) -> f32 {
        match self {
            Self::Cinema16x9 => 16.0 / 9.0,
            Self::Classic4x3 => 4.0 / 3.0,
            Self::Square1x1 => 1.0,
            Self::Ultrawide21x9 => 21.0 / 9.0,
            Self::Mobile9x16 => 9.0 / 16.0,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Cinema16x9 => "16:9 Cinema",
            Self::Classic4x3 => "4:3 Classic",
            Self::Square1x1 => "1:1 Square",
            Self::Ultrawide21x9 => "21:9 Ultrawide",
            Self::Mobile9x16 => "9:16 Mobile",
        }
    }

    fn default_width(self) -> f32 {
        match self {
            Self::Cinema16x9 => 440.0,
            Self::Classic4x3 => 380.0,
            Self::Square1x1 => 270.0,
            Self::Ultrawide21x9 => 460.0,
            Self::Mobile9x16 => 170.0,
        }
    }

    fn width_bounds(self) -> (f32, f32, f32) {
        match self {
            Self::Cinema16x9 => (280.0, 480.0, 25.0),
            Self::Classic4x3 => (240.0, 410.0, 25.0),
            Self::Square1x1 => (160.0, 310.0, 25.0),
            Self::Ultrawide21x9 => (300.0, 490.0, 25.0),
            Self::Mobile9x16 => (110.0, 180.0, 15.0),
        }
    }
}

#[derive(Resource)]
struct AspectRatioState {
    current_preset: PresetRatio,
    hero_width: f32,
}

impl Default for AspectRatioState {
    fn default() -> Self {
        Self {
            current_preset: PresetRatio::Cinema16x9,
            hero_width: 440.0,
        }
    }
}

#[derive(Component)]
struct HeroFrame;

#[derive(Component)]
struct HeroBadge;

#[derive(Component)]
struct TelemetryLabel;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Screen-space root
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.03, 0.04, 0.07),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Start,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    // -------------------------------------------------------------
    // Header
    // -------------------------------------------------------------
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
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
        ChildOf(header),
        label_node(),
        text(
            "Univis UI - CSS aspect-ratio Showcase",
            25.0,
            Color::srgb(0.95, 0.98, 1.0),
        ),
    ));

    commands.spawn((
        ChildOf(header),
        label_node(),
        text(
            "Automatic proportional sizing complying with CSS Box Sizing Level 4 standards.",
            13.0,
            Color::srgb(0.60, 0.70, 0.85),
        ),
    ));

    commands.spawn((
        ChildOf(header),
        label_node(),
        text(
            "Controls: [1] 16:9 | [2] 4:3 | [3] 1:1 | [4] 21:9 | [5] 9:16  ---  [W/Up]: Scale Up | [S/Down]: Scale Down",
            12.5,
            Color::srgb(0.30, 0.85, 0.95),
        ),
    ));

    // -------------------------------------------------------------
    // Main Showcase Row
    // -------------------------------------------------------------
    let main_row = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Start,
                gap: 28.0,
                ..default()
            },
        ))
        .id();

    // =============================================================
    // Left: Interactive Hero Card
    // =============================================================
    let hero_card = commands
        .spawn((
            ChildOf(main_row),
            UNode {
                width: UVal::Px(560.0),
                height: UVal::Auto,
                background_color: Color::srgba(0.08, 0.10, 0.16, 0.85),
                border_radius: UCornerRadius::all(16.0),
                padding: USides::all(20.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.20, 0.65, 0.95, 0.35),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(hero_card),
        label_node(),
        text(
            "Interactive Dynamic Frame",
            18.0,
            Color::srgb(0.35, 0.85, 0.95),
        ),
    ));

    // Container for the hero frame
    let frame_viewport = commands
        .spawn((
            ChildOf(hero_card),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(350.0),
                background_color: Color::srgba(0.04, 0.06, 0.10, 0.9),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.25, 0.40, 0.60, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
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

    // The hero frame with aspect_ratio
    let hero = commands
        .spawn((
            ChildOf(frame_viewport),
            HeroFrame,
            UNode {
                width: UVal::Px(440.0),
                height: UVal::Auto,
                aspect_ratio: Some(16.0 / 9.0),
                background_color: Color::srgba(0.12, 0.28, 0.48, 0.9),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.40, 0.85, 1.0, 0.6),
                width: 1.5,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(hero),
        HeroBadge,
        label_node(),
        text("16:9 Cinema", 15.0, Color::WHITE),
    ));

    commands.spawn((
        ChildOf(hero),
        label_node(),
        text("h = w / ratio", 11.0, Color::srgb(0.70, 0.85, 0.95)),
    ));

    // Telemetry display box
    let telemetry_box = commands
        .spawn((
            ChildOf(hero_card),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                background_color: Color::srgba(0.04, 0.06, 0.10, 0.7),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(10.0),
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

    commands.spawn((
        ChildOf(telemetry_box),
        TelemetryLabel,
        label_node(),
        text("Calculating...", 11.5, Color::srgb(0.45, 0.95, 0.65)),
    ));

    // =============================================================
    // Right: Standard Proportions Gallery
    // =============================================================
    let gallery_card = commands
        .spawn((
            ChildOf(main_row),
            UNode {
                width: UVal::Px(520.0),
                height: UVal::Auto,
                background_color: Color::srgba(0.08, 0.10, 0.16, 0.85),
                border_radius: UCornerRadius::all(16.0),
                padding: USides::all(20.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.75, 0.35, 0.95, 0.35),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(gallery_card),
        label_node(),
        text(
            "Standard Aspect Ratios Gallery",
            18.0,
            Color::srgb(0.85, 0.50, 1.0),
        ),
    ));

    // Gallery Row 1: 16:9 Video Player & 1:1 Avatar
    let gallery_row1 = commands
        .spawn((
            ChildOf(gallery_card),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    // 16:9 Video Player card (Width 280, Height Auto)
    let video_item = commands
        .spawn((
            ChildOf(gallery_row1),
            UNode {
                width: UVal::Px(280.0),
                height: UVal::Auto,
                aspect_ratio: Some(16.0 / 9.0),
                background_color: Color::srgba(0.20, 0.12, 0.30, 0.9),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(10.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.75, 0.40, 0.95, 0.4),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(video_item),
        label_node(),
        text("[16:9] Video Card", 14.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(video_item),
        label_node(),
        text("280 x 157.5 px", 11.0, Color::srgb(0.7, 0.6, 0.8)),
    ));

    // 1:1 Square Avatar (Width 120, Height Auto)
    let avatar_item = commands
        .spawn((
            ChildOf(gallery_row1),
            UNode {
                width: UVal::Px(130.0),
                height: UVal::Auto,
                aspect_ratio: Some(1.0),
                background_color: Color::srgba(0.12, 0.28, 0.22, 0.9),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(8.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.35, 0.85, 0.60, 0.4),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(avatar_item),
        label_node(),
        text("1:1 Square", 14.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(avatar_item),
        label_node(),
        text("130 x 130 px", 11.0, Color::srgb(0.6, 0.8, 0.7)),
    ));

    // Gallery Row 2: 3:4 Poster & 2-Column Grid of 1:1 Cells
    let gallery_row2 = commands
        .spawn((
            ChildOf(gallery_card),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    // 3:4 Portrait Poster (Height 150, Width Auto => 112.5x150)
    let poster_item = commands
        .spawn((
            ChildOf(gallery_row2),
            UNode {
                width: UVal::Auto,
                height: UVal::Px(150.0),
                aspect_ratio: Some(3.0 / 4.0),
                background_color: Color::srgba(0.35, 0.22, 0.12, 0.9),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(8.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.95, 0.65, 0.30, 0.4),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(poster_item),
        label_node(),
        text("3:4 Poster", 14.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(poster_item),
        label_node(),
        text("112.5 x 150 px", 10.5, Color::srgb(0.9, 0.75, 0.6)),
    ));

    // Mini 2-Column Grid with 1:1 Cells (Auto Width & Height inside Grid)
    let mini_grid = commands
        .spawn((
            ChildOf(gallery_row2),
            UNode {
                width: UVal::Px(280.0),
                height: UVal::Px(150.0),
                background_color: Color::srgba(0.04, 0.07, 0.12, 0.8),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.5, 0.4, 0.9, 0.25),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 2,
                gap: 12.0,
                align_items: UAlignItems::Center,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        justify_items: Some(UAlignItemsExt::Center),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    for i in 1..=2 {
        let grid_box = commands
            .spawn((
                ChildOf(mini_grid),
                UNode {
                    width: UVal::Auto,
                    height: UVal::Auto,
                    aspect_ratio: Some(1.0),
                    background_color: Color::srgba(0.18, 0.16, 0.32, 0.9),
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.5, 0.4, 0.9, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
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

        commands.spawn((
            ChildOf(grid_box),
            label_node(),
            text(&format!("Grid 1:1 #{i}"), 12.0, Color::WHITE),
        ));
    }

    // -------------------------------------------------------------
    // Footer
    // -------------------------------------------------------------
    commands.spawn((
        ChildOf(root),
        label_node(),
        text(
            "Powered by Univis UI CSS Box Sizing Level 4 engine with zero heap reallocations.",
            12.0,
            Color::srgb(0.45, 0.50, 0.60),
        ),
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<AspectRatioState>,
    mut q_hero: Query<&mut UNode, With<HeroFrame>>,
    mut q_badge: Query<&mut UTextLabel, With<HeroBadge>>,
) {
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::Digit1) {
        state.current_preset = PresetRatio::Cinema16x9;
        state.hero_width = PresetRatio::Cinema16x9.default_width();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        state.current_preset = PresetRatio::Classic4x3;
        state.hero_width = PresetRatio::Classic4x3.default_width();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        state.current_preset = PresetRatio::Square1x1;
        state.hero_width = PresetRatio::Square1x1.default_width();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        state.current_preset = PresetRatio::Ultrawide21x9;
        state.hero_width = PresetRatio::Ultrawide21x9.default_width();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        state.current_preset = PresetRatio::Mobile9x16;
        state.hero_width = PresetRatio::Mobile9x16.default_width();
        changed = true;
    }

    let (min_w, max_w, step) = state.current_preset.width_bounds();
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        state.hero_width = (state.hero_width + step).min(max_w);
        changed = true;
    } else if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        state.hero_width = (state.hero_width - step).max(min_w);
        changed = true;
    }

    if changed {
        for mut node in &mut q_hero {
            node.width = UVal::Px(state.hero_width);
            node.aspect_ratio = Some(state.current_preset.ratio());
        }

        for mut badge in &mut q_badge {
            badge.text = state.current_preset.name().to_string();
        }
    }
}

fn update_telemetry(
    state: Res<AspectRatioState>,
    q_hero: Query<&ComputedSize, With<HeroFrame>>,
    mut q_telemetry: Query<&mut UTextLabel, With<TelemetryLabel>>,
) {
    let Ok(size) = q_hero.single() else {
        return;
    };

    let target_ratio = state.current_preset.ratio();
    let measured_ratio = if size.height > 0.001 {
        size.width / size.height
    } else {
        0.0
    };
    let delta = (measured_ratio - target_ratio).abs();

    for mut label in &mut q_telemetry {
        label.text = format!(
            "Target: {:.4} | Size: {:.1}x{:.1}px | Measured: {:.4} (Err: {:.5})",
            target_ratio, size.width, size.height, measured_ratio, delta
        );
    }
}

fn label_node() -> UNode {
    UNode {
        background_color: Color::NONE,
        ..default()
    }
}

fn text(value: &str, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: value.to_string(),
        font_size,
        color,
        ..default()
    }
}
