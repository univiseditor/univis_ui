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
                gap: 32.0,
                ..default()
            },
        ))
        .id();

    // Glass Panel
    let glass_panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UNode {
                width: UVal::Px(250.0),
                height: UVal::Px(200.0),
                padding: USides::all(24.0),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(glass_panel),
        UTextLabel {
            text: "Glass Panel".into(),
            font_size: 24.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(glass_panel),
        UTextLabel::new("Transparent background with subtle border and blur effect."),
    ));

    // Card Panel
    let card_panel = commands
        .spawn((
            ChildOf(root),
            UPanel::card().with_gap(16.0),
            UNode {
                width: UVal::Px(250.0),
                height: UVal::Px(200.0),
                padding: USides::all(24.0),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(card_panel),
        UTextLabel {
            text: "Card Panel".into(),
            font_size: 24.0,
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(card_panel),
        UTextLabel::new("Solid background, meant for grouped content."),
    ));
}
