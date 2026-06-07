use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
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

    let panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UNode {
                width: UVal::Px(450.0),
                padding: USides::all(32.0),
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

    commands.spawn((ChildOf(panel), UTextLabel::new("Default Seekbar")));
    commands.spawn((
        ChildOf(panel),
        USeekBar::new().with_value(50.0).with_range(0.0, 100.0),
    ));

    commands.spawn((ChildOf(panel), UTextLabel::new("Volume Style Seekbar")));
    commands.spawn((
        ChildOf(panel),
        USeekBar::volume_style().with_value(0.75), // Usually 0.0 to 1.0
    ));

    commands.spawn((
        ChildOf(panel),
        UTextLabel::new("Video/Timeline Style Seekbar"),
    ));
    commands.spawn((ChildOf(panel), USeekBar::video_style().with_value(0.3)));

    commands.spawn((ChildOf(panel), UTextLabel::new("Brightness Style Seekbar")));
    commands.spawn((ChildOf(panel), USeekBar::brightness_style().with_value(0.9)));
}
