use bevy::prelude::*;
use univis_ui::prelude::*; // Assuming univis is your crate name
// Import the module we created in the previous step
// mod text_field;
// use text_field::{UnivisTextFieldPlugin, UTextFieldBuilder, UTextFieldChangeEvent, UTextFieldSubmitEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 1. Add the Core Univis Plugin
        .add_plugins(UnivisUiPlugin)
        // 2. Add our new Text Field Plugin
        .add_plugins(UnivisTextFieldVisualPlugin)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, handle_text_events)
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Spawn Camera
    commands.spawn(Camera2d::default());

    // 1. Create a Root Container (World Root or Screen Root)
    commands
        .spawn((
            UWorldRoot {
                size: Vec2::new(800.0, 600.0),
                is_3d: false, // 2D Mode
                ..default()
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.1, 0.1, 0.15), // Dark background
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                // Arrange fields vertically
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 20.0, // Space between fields
                ..default()
            },
        ))
        .with_children(|parent| {
            // --- Example 1: Standard Text Field ---
            UTextFieldBuilder::new()
                .label("Username")
                .value("Admin") // Initial value
                .spawn(parent);

            // --- Example 2: Password Field ---
            UTextFieldBuilder::new()
                .label("Password")
                .password() // Masks input with *
                .spawn(parent);

            // --- Example 3: Search Field (Empty start) ---
            UTextFieldBuilder::new().label("Search...").spawn(parent);
        });

    // // Instructions
    // commands.spawn(Text::from_section(
    //     "Click a field to focus. Type to enter text. Press Enter to submit.",
    //     TextStyle {
    //         font_size: 20.0,
    //         color: Color::WHITE,
    //         ..default()
    //     }
    // ).with_style(Style {
    //     position_type: PositionType::Absolute,
    //     bottom: Val::Px(20.0),
    //     left: Val::Px(20.0),
    //     ..default()
    // }));
}

// System to listen to events emitted by the text fields
fn handle_text_events(
    mut change_events: MessageReader<UTextFieldChangeEvent>,
    mut submit_events: MessageReader<UTextFieldSubmitEvent>,
) {
    // 1. Listen for typing (Real-time)
    for ev in change_events.read() {
        // You can filter by entity if you stored the IDs
        println!("Entity {:?} Changed: {}", ev.entity, ev.value);
    }

    // 2. Listen for Submit (Enter Key)
    for ev in submit_events.read() {
        info!(
            "FORM SUBMITTED! Entity: {:?} | Final Value: {}",
            ev.entity, ev.value
        );
    }
}
