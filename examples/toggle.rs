use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UnivisUiPlugin,LayoutProfilingPlugin))
        // .add_plugins(UnivisTogglePlugin)
        
        // إضافة Event
        
        .add_systems(Update, emit_toggle_events)
        
        .add_systems(Startup, setup_toggle_demo)
        .add_observer(handle_toggle_events)
        .run();
}

fn setup_toggle_demo(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
    
    // Screen Root
    commands.spawn((
        UScreenRoot,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.1, 0.1, 0.15),
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
            text: "Toggle Widget Gallery".to_string(),
            font_size: 32.0,
            color: Color::WHITE,
            ..default()
        });
        
        // Container للأمثلة
        parent.spawn((
            UNode {
                width: UVal::Px(400.0),
                padding: USides::all(30.0),
                background_color: Color::srgba(0.2, 0.2, 0.25, 0.8),
                border_radius: UCornerRadius::all(15.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 25.0,
                ..default()
            },
        )).with_children(|examples| {
            
            // === مثال 1: Toggle افتراضي ===
            create_toggle_row(
                examples,
                "Default Toggle",
                UToggle::new(),
            );
            
            // === مثال 2: iOS Style ===
            create_toggle_row(
                examples,
                "iOS Style",
                UToggle::ios_style().with_checked(true),
            );
            
            // === مثال 3: Material Style ===
            create_toggle_row(
                examples,
                "Material Design",
                UToggle::material_style(),
            );
            
            // === مثال 4: Sci-Fi Style ===
            create_toggle_row(
                examples,
                "Sci-Fi Style",
                UToggle::sci_fi_style().with_checked(true),
            );
            
            // === مثال 5: Custom Colors ===
            create_toggle_row(
                examples,
                "Custom Purple",
                UToggle::new()
                    .with_colors(
                        Color::srgb(0.3, 0.2, 0.4),
                        Color::srgb(0.6, 0.3, 0.9),
                        Color::srgb(0.95, 0.9, 1.0),
                    ),
            );
            
            // === مثال 6: Large Toggle ===
            create_toggle_row(
                examples,
                "Large Size",
                UToggle::new()
                    .with_size(80.0, 40.0)
                    .with_colors(
                        Color::srgb(0.2, 0.2, 0.2),
                        Color::srgb(1.0, 0.5, 0.0),
                        Color::WHITE,
                    ),
            );
            
            // === مثال 7: Disabled ===
            create_toggle_row(
                examples,
                "Disabled State",
                UToggle::new()
                    .with_checked(true)
                    .disabled(),
            );
        });
        
        // Footer
        parent.spawn(UTextLabel {
            text: "Click on toggles to interact!".to_string(),
            font_size: 16.0,
            color: Color::srgb(0.6, 0.6, 0.7),
            ..default()
        });
    });
}

/// Helper لإنشاء صف Toggle مع Label
fn create_toggle_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    toggle: UToggle,
) {
    parent.spawn((
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(15.0, 10.0),
            background_color: Color::srgba(0.15, 0.15, 0.2, 0.5),
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            justify_content: UJustifyContent::SpaceBetween,
            ..default()
        },
    )).with_children(|row| {
        // Label
        row.spawn(UTextLabel {
            text: label.to_string(),
            font_size: 18.0,
            color: Color::srgb(0.9, 0.9, 0.95),
            ..default()
        });
        
        // Toggle
        row.spawn(toggle);
    });
}

/// معالجة أحداث Toggle
fn handle_toggle_events(
    event: On<Pointer<Click>>,
    // mut events: MessageReader<ToggleChangedEvent>,
    query: Query<&UToggle>,
) {

    if let Ok(toggle) = query.get(event.entity.entity()) {
        let status = if toggle.checked { "ON" } else { "OFF" };
        info!("Toggle changed to: {} (Entity: {:?})", status, event.entity);
    }
    // for event in events.read() {
        // if let Ok(toggle) = query.get(event.entity) {
            
            
            
            // يمكنك هنا إضافة منطق التطبيق
            // مثل: تفعيل/تعطيل ميزة، تغيير إعدادات، إلخ
        // }
    // }
}