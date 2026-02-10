use bevy::prelude::*;
use univis_ui::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_plugins(UnivisTextFieldPlugin)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, handle_text_events)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.1, 0.1, 0.15),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 20.0,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Text Field 1
            parent.spawn((
                UTextField::new()
                    .with_placeholder("text...")
                    .with_size(300.0, 50.0),
            ));

            // Text Field 2
            parent.spawn((
                UTextField::new()
                    .with_placeholder("email")
                    .input_type(TextFieldInputType::Email)
                    .with_size(300.0, 50.0),
            ));
            
            parent.spawn((
                UTextField::new()
                    .with_placeholder("password")
                    .input_type(TextFieldInputType::Password)
                    .with_size(300.0, 50.0),
            ));
            
        });
}

fn handle_text_events(
    mut change_events: MessageReader<TextFieldChangedEvent>,
    mut submit_events: MessageReader<TextFieldSubmitEvent>,
) {
    for ev in change_events.read() {
        println!("Changed: {}", ev.text);
    }

    for ev in submit_events.read() {
        info!("SUBMITTED: {}", ev.text);
    }
}