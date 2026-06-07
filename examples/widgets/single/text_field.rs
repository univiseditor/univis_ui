use bevy::prelude::*;
use univis_ui::prelude::TextFieldInputType;
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
                width: UVal::Px(400.0),
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(panel), UTextLabel::new("Standard Text Field")));
    commands.spawn((
        ChildOf(panel),
        UTextField::new().with_placeholder("Type your name here..."),
    ));

    commands.spawn((ChildOf(panel), UTextLabel::new("Password Field")));
    commands.spawn((
        ChildOf(panel),
        UTextField::new()
            .with_placeholder("Enter password...")
            .input_type(TextFieldInputType::Password),
    ));

    commands.spawn((
        ChildOf(panel),
        UTextLabel::new("Pre-filled & Max Length (10)"),
    ));
    commands.spawn((
        ChildOf(panel),
        UTextField::new().with_text("Hello").with_max_length(10),
    ));
}
