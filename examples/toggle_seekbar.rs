use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Component)]
struct DemoToggle;

#[derive(Component)]
struct DemoSeekBar;

#[derive(Component)]
struct ToggleStatusLabel;

#[derive(Component)]
struct SeekBarStatusLabel;

#[derive(Component)]
struct PreviewStatusLabel;

#[derive(Component)]
struct PreviewLight;

#[derive(Component)]
struct PreviewCard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, sync_demo_ui)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.1),
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

    let shell = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(960.0),
                height: UVal::Px(620.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.98),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.68, 0.76, 0.88, 0.18),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Stretch,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    let controls = commands
        .spawn((
            ChildOf(shell),
            panel_node(340.0, 572.0, Color::srgba(0.1, 0.12, 0.17, 0.96)),
            panel_border(Color::srgba(0.78, 0.84, 0.94, 0.18), 24.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(controls),
        label_node(Color::NONE),
        text("Toggle + SeekBar Demo", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(controls),
        label_node(Color::NONE),
        text(
            "The toggle enables the accent output. The seekbar controls intensity.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let toggle_card = commands
        .spawn((
            ChildOf(controls),
            panel_node(308.0, 160.0, Color::srgba(0.12, 0.15, 0.2, 0.95)),
            panel_border(Color::srgba(0.76, 0.83, 0.93, 0.15), 20.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(toggle_card),
        label_node(Color::NONE),
        text("Accent Toggle", 22.0, Color::srgb(0.97, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(toggle_card),
        ToggleStatusLabel,
        label_node(Color::srgba(0.13, 0.2, 0.15, 0.92)),
        text("Accent mode: ON", 16.0, Color::srgb(0.82, 1.0, 0.9)),
    ));
    commands.spawn((
        ChildOf(toggle_card),
        DemoToggle,
        UToggle::ios_style().with_checked(true),
    ));

    let seekbar_card = commands
        .spawn((
            ChildOf(controls),
            panel_node(308.0, 216.0, Color::srgba(0.12, 0.15, 0.2, 0.95)),
            panel_border(Color::srgba(0.76, 0.83, 0.93, 0.15), 20.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(seekbar_card),
        label_node(Color::NONE),
        text("Intensity SeekBar", 22.0, Color::srgb(0.97, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(seekbar_card),
        SeekBarStatusLabel,
        label_node(Color::srgba(0.14, 0.16, 0.22, 0.92)),
        text("Intensity: 68%", 16.0, Color::srgb(0.86, 0.92, 1.0)),
    ));
    commands.spawn((
        ChildOf(seekbar_card),
        DemoSeekBar,
        USeekBar::brightness_style()
            .with_range(0.0, 100.0)
            .with_value(0.68)
            .with_size(280.0, 8.0, 24.0)
            .show_value(),
    ));
    commands.spawn((
        ChildOf(seekbar_card),
        label_node(Color::NONE),
        text(
            "Drag the thumb to change the preview output.",
            15.0,
            Color::srgb(0.76, 0.82, 0.9),
        ),
    ));

    let preview = commands
        .spawn((
            ChildOf(shell),
            PreviewCard,
            panel_node(572.0, 572.0, Color::srgba(0.08, 0.15, 0.27, 0.98)),
            panel_border(Color::srgba(0.62, 0.8, 1.0, 0.2), 24.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(preview),
        label_node(Color::NONE),
        text("Live Preview", 32.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(preview),
        PreviewStatusLabel,
        label_node(Color::srgba(0.08, 0.2, 0.31, 0.86)),
        text("Signal live at 68%", 18.0, Color::srgb(0.87, 0.96, 1.0)),
    ));
    commands.spawn((
        ChildOf(preview),
        PreviewLight,
        UNode {
            width: UVal::Px(145.0),
            height: UVal::Px(145.0),
            background_color: Color::srgb(0.52, 0.69, 1.0),
            border_radius: UCornerRadius::all(72.5),
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(preview),
        label_node(Color::NONE),
        text(
            "When the toggle is off, the preview dims but the slider still updates the target intensity.",
            16.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));
}

fn sync_demo_ui(
    toggle_query: Query<&UToggle, With<DemoToggle>>,
    seekbar_query: Query<&USeekBar, With<DemoSeekBar>>,
    mut label_query: Query<(
        Entity,
        &mut UTextLabel,
        Option<&ToggleStatusLabel>,
        Option<&SeekBarStatusLabel>,
        Option<&PreviewStatusLabel>,
    )>,
    mut preview_node_query: Query<(&mut UNode, Option<&PreviewCard>, Option<&PreviewLight>)>,
) {
    let Some(toggle) = toggle_query.iter().next() else {
        return;
    };
    let Some(seekbar) = seekbar_query.iter().next() else {
        return;
    };

    let intensity = seekbar.real_value().round().clamp(0.0, 100.0);
    let normalized = seekbar.value.clamp(0.0, 1.0);
    let enabled = toggle.checked;

    for (_, mut label, toggle_label, seekbar_label, preview_label) in &mut label_query {
        if toggle_label.is_some() {
            label.text = format!("Accent mode: {}", if enabled { "ON" } else { "OFF" });
            label.color = if enabled {
                Color::srgb(0.82, 1.0, 0.9)
            } else {
                Color::srgb(0.86, 0.87, 0.9)
            };
        }

        if seekbar_label.is_some() {
            label.text = format!("Intensity: {:.0}%", intensity);
        }

        if preview_label.is_some() {
            label.text = if enabled {
                format!("Signal live at {:.0}%", intensity)
            } else {
                format!("Signal muted, target still at {:.0}%", intensity)
            };
            label.color = if enabled {
                Color::srgb(0.87, 0.96, 1.0)
            } else {
                Color::srgb(0.82, 0.84, 0.88)
            };
        }
    }

    for (mut node, is_preview_card, is_preview_light) in &mut preview_node_query {
        if is_preview_card.is_some() {
            node.background_color = if enabled {
                Color::srgba(
                    0.08,
                    0.12 + (0.08 * normalized),
                    0.22 + (0.26 * normalized),
                    0.98,
                )
            } else {
                Color::srgba(0.1, 0.11, 0.13 + (0.06 * normalized), 0.98)
            };
        }

        if is_preview_light.is_some() {
            let size = 72.0 + (normalized * 110.0);
            node.width = UVal::Px(size);
            node.height = UVal::Px(size);
            node.border_radius = UCornerRadius::all(size / 2.0);
            node.background_color = if enabled {
                Color::srgb(0.34 + (0.36 * normalized), 0.52 + (0.26 * normalized), 1.0)
            } else {
                Color::srgba(
                    0.24 + (0.1 * normalized),
                    0.26 + (0.1 * normalized),
                    0.32,
                    0.92,
                )
            };
        }
    }
}

fn panel_node(width: f32, height: f32, background_color: Color) -> UNode {
    UNode {
        width: UVal::Px(width),
        height: UVal::Px(height),
        padding: USides::all(18.0),
        background_color,
        border_radius: UCornerRadius::all(24.0),
        ..default()
    }
}

fn panel_border(color: Color, radius: f32) -> UBorder {
    UBorder {
        color,
        width: 1.0,
        radius: UCornerRadius::all(radius),
        offset: 0.0,
    }
}

fn label_node(background_color: Color) -> UNode {
    UNode {
        background_color,
        padding: USides::axes(10.0, 6.0),
        border_radius: UCornerRadius::all(12.0),
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
