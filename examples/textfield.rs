use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_plugins(UnivisTextFieldPlugin)
        
        .add_systems(Startup, setup_textfield_demo)
        .add_systems(Update, (
            handle_textfield_changed,
            handle_textfield_submit,
        ))
        .run();
}

fn setup_textfield_demo(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
    
    // Screen Root
    commands.spawn((
        UScreenRoot,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.08, 0.08, 0.12),
            padding: USides::all(40.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 30.0,
            align_items: UAlignItems::Center,
            justify_content: UJustifyContent::Center,
            ..default()
        },
    )).with_children(|parent| {
        
        // العنوان
        parent.spawn(UTextLabel {
            text: "TextField Widget Gallery".to_string(),
            font_size: 36.0,
            color: Color::WHITE,
            ..default()
        });
        
        // Container الرئيسي
        parent.spawn((
            UNode {
                width: UVal::Px(500.0),
                padding: USides::all(40.0),
                background_color: Color::srgba(0.15, 0.15, 0.2, 0.9),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 30.0,
                ..default()
            },
        )).with_children(|examples| {
            
            // === مثال 1: TextField أساسي ===
            create_textfield_row(
                examples,
                "Basic Text",
                UTextField::new()
                    .with_placeholder("Type something...")
                    .with_size(400.0, 40.0),
            );
            
            // === مثال 2: مع نص أولي ===
            create_textfield_row(
                examples,
                "With Initial Text",
                UTextField::new()
                    .with_text("Hello World!")
                    .with_size(400.0, 40.0),
            );
            
            // === مثال 3: Email ===
            create_textfield_row(
                examples,
                "Email Address",
                UTextField::email_style(),
            );
            
            // === مثال 4: Password ===
            create_textfield_row(
                examples,
                "Password",
                UTextField::password_style(),
            );
            
            // === مثال 5: Number ===
            create_textfield_row(
                examples,
                "Number Only",
                UTextField::number_style()
                    .with_placeholder("Enter a number..."),
            );
            
            // === مثال 6: مع حد أقصى ===
            create_textfield_row(
                examples,
                "Max Length (10 chars)",
                UTextField::new()
                    .with_placeholder("Max 10 characters")
                    .with_max_length(10)
                    .with_size(400.0, 40.0),
            );
            
            // === مثال 7: Search ===
            create_textfield_row(
                examples,
                "Search",
                UTextField::search_style(),
            );
            
            // === مثال 8: Readonly ===
            create_textfield_row(
                examples,
                "Readonly Field",
                UTextField::new()
                    .with_text("Cannot edit this")
                    .readonly()
                    .with_size(400.0, 40.0),
            );
            
            // === مثال 9: Disabled ===
            create_textfield_row(
                examples,
                "Disabled Field",
                UTextField::new()
                    .with_text("Disabled")
                    .disabled()
                    .with_size(400.0, 40.0),
            );
            
            // === مثال 10: نموذج تسجيل كامل ===
            create_form_example(examples);
        });
        
        // Footer
        parent.spawn(UTextLabel {
            text: "Click to focus, type to input. Press Enter to submit.".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.5, 0.5, 0.6),
            ..default()
        });
    });
}

/// Helper لإنشاء صف TextField
fn create_textfield_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    textfield: UTextField,
) {
    parent.spawn((
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 8.0,
            ..default()
        },
    )).with_children(|row| {
        
        // Label
        row.spawn(UTextLabel {
            text: label.to_string(),
            font_size: 14.0,
            color: Color::srgb(0.7, 0.75, 0.85),
            ..default()
        });
        
        // TextField
        row.spawn(textfield);
    });
}

/// مثال نموذج تسجيل
fn create_form_example(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::all(25.0),
            background_color: Color::srgba(0.1, 0.12, 0.18, 0.6),
            border_radius: UCornerRadius::all(12.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 20.0,
            ..default()
        },
    )).with_children(|form| {
        
        // عنوان النموذج
        form.spawn(UTextLabel {
            text: "Registration Form".to_string(),
            font_size: 20.0,
            color: Color::srgb(0.9, 0.9, 0.95),
            ..default()
        });
        
        // الاسم
        create_textfield_row(
            form,
            "Full Name *",
            UTextField::new()
                .with_placeholder("John Doe")
                .with_size(400.0, 42.0),
        );
        
        // البريد
        create_textfield_row(
            form,
            "Email *",
            UTextField::email_style()
                .with_size(400.0, 42.0),
        );
        
        // كلمة المرور
        create_textfield_row(
            form,
            "Password *",
            UTextField::password_style()
                .with_size(400.0, 42.0),
        );
        
        // العمر
        create_textfield_row(
            form,
            "Age",
            UTextField::number_style()
                .with_placeholder("18")
                .with_size(100.0, 42.0),
        );
        
        // زر Submit
        form.spawn((
            UButton {
                padding: USides::axes(30.0, 12.0),
                background: Color::srgb(0.2, 0.6, 1.0),
                hover_color: Color::srgb(0.25, 0.65, 1.0),
                pressed_color: Color::srgb(0.15, 0.55, 0.95),
                border_radius: UCornerRadius::all(8.0),
            },
        )).with_children(|button| {
            button.spawn(UTextLabel {
                text: "Submit".to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
        });
    });
}

/// معالجة تغييرات النص
fn handle_textfield_changed(
    mut events: MessageReader<TextFieldChangedEvent>,
) {
    for event in events.read() {
        info!("TextField changed: '{}'", event.text);
    }
}

/// معالجة Submit (Enter)
fn handle_textfield_submit(
    mut events: MessageReader<TextFieldSubmitEvent>,
) {
    for event in events.read() {
        info!("TextField submitted: '{}'", event.text);
    }
}