use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        
        .add_systems(Startup, setup_radio_demo)
        .add_systems(Update, handle_radio_events)
        .run();
}

fn setup_radio_demo(mut commands: Commands) {
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
            text: "RadioButton & RadioGroup Gallery".to_string(),
            font_size: 36.0,
            color: Color::WHITE,
            ..default()
        });
        
        // Container الرئيسي
        parent.spawn((
            UNode {
                width: UVal::Px(600.0),
                padding: USides::all(40.0),
                background_color: Color::srgba(0.15, 0.15, 0.2, 0.9),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 35.0,
                ..default()
            },
        )).with_children(|examples| {
            examples.spawn(URadioGroup::new());
            // === مثال 1: Radio Group - اختيار الحجم ===
            create_radio_group_section(
                examples,
                "Select Size",
                vec![
                    ("small", "Small (S)"),
                    ("medium", "Medium (M)"),
                    ("large", "Large (L)"),
                    ("xlarge", "Extra Large (XL)"),
                ],
                Some("medium"),
                URadioButton::primary_style,
            );
            
            // === مثال 2: Radio Group - طريقة الدفع ===
            create_radio_group_section(
                examples,
                "Payment Method",
                vec![
                    ("credit", "Credit Card"),
                    ("paypal", "PayPal"),
                    ("crypto", "Cryptocurrency"),
                ],
                Some("credit"),
                |value| URadioButton::success_style(value).with_size(22.0),
            );
            
            // === مثال 3: Radio Group - مستوى الصعوبة ===
            create_radio_group_section(
                examples,
                "Difficulty Level",
                vec![
                    ("easy", "Easy"),
                    ("normal", "Normal"),
                    ("hard", "Hard"),
                    ("extreme", "Extreme"),
                ],
                Some("normal"),
                |value| {
                    if value == "extreme" {
                        URadioButton::danger_style(value).with_size(20.0)
                    } else {
                        URadioButton::primary_style(value).with_size(20.0)
                    }
                },
            );
            
            // === مثال 4: Radio Buttons منفردة (بدون Group) ===
            create_section(examples, "Individual Radio Buttons", |section| {
                
                create_radio_with_label(
                    section,
                    URadioButton::new("option1"),
                    "Standalone Option 1",
                );
                
                create_radio_with_label(
                    section,
                    URadioButton::new("option2").checked(),
                    "Standalone Option 2 (Pre-selected)",
                );
                
                create_radio_with_label(
                    section,
                    URadioButton::new("option3").disabled(),
                    "Disabled Option",
                );
            });
            
            // === مثال 5: Radio Group مع خيار إلغاء الاختيار ===
            create_custom_radio_group(
                examples,
                "Optional Selection (Click again to deselect)",
                vec![
                    ("red", "Red Theme", Color::srgb(0.9, 0.2, 0.2)),
                    ("green", "Green Theme", Color::srgb(0.2, 0.8, 0.3)),
                    ("blue", "Blue Theme", Color::srgb(0.2, 0.6, 1.0)),
                ],
                false, // allow_deselect = true
            );
            
            // === مثال 6: Compact Radio Group ===
            create_compact_radio_group(
                examples,
                "Text Alignment",
                vec!["Left", "Center", "Right", "Justify"],
            );
        });
        
        // Footer
        parent.spawn(UTextLabel {
            text: "Click on radio buttons to select!".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.5, 0.5, 0.6),
            ..default()
        });
    });
}

/// Helper: إنشاء RadioGroup قياسي
fn create_radio_group_section<F>(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    options: Vec<(&str, &str)>,
    default: Option<&str>,
    radio_factory: F,
) where
    F: Fn(String) -> URadioButton,
{
    create_section(parent, title, |section| {
        
        let mut group = URadioGroup::new();
        if let Some(def) = default {
            group = group.with_default(def);
        }
        
        section.spawn(group).with_children(|group_parent| {
            for (value, label) in options {
                create_radio_with_label(
                    group_parent,
                    radio_factory(value.to_string()),
                    label,
                );
            }
        });
    });
}

/// Helper: إنشاء RadioGroup مخصص
fn create_custom_radio_group(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    options: Vec<(&str, &str, Color)>,
    require_selection: bool,
) {
    create_section(parent, title, |section| {
        
        let group = if require_selection {
            URadioGroup::new()
        } else {
            URadioGroup::new().allow_deselect()
        };
        
        section.spawn(group).with_children(|group_parent| {
            for (value, label, color) in options {
                create_radio_with_label(
                    group_parent,
                    URadioButton::new(value)
                        .with_colors(
                            Color::srgb(0.3, 0.3, 0.35),
                            color,
                            color,
                        ),
                    label,
                );
            }
        });
    });
}

/// Helper: إنشاء Compact RadioGroup
fn create_compact_radio_group(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    options: Vec<&str>,
) {
    create_section(parent, title, |section| {
        
        section.spawn((
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(15.0),
                background_color: Color::srgba(0.1, 0.12, 0.15, 0.5),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 20.0,
                justify_content: UJustifyContent::SpaceEvenly,
                ..default()
            },
            URadioGroup::new().with_default(options[0]),
        )).with_children(|group_parent| {
            for option in options {
                create_radio_with_label(
                    group_parent,
                    URadioButton::new(option).with_size(18.0),
                    option,
                );
            }
        });
    });
}

/// Helper: إنشاء قسم
fn create_section(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    content_fn: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent.spawn((
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::all(20.0),
            background_color: Color::srgba(0.1, 0.12, 0.18, 0.6),
            border_radius: UCornerRadius::all(12.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 15.0,
            ..default()
        },
    )).with_children(|section| {
        
        // العنوان
        section.spawn(UTextLabel {
            text: title.to_string(),
            font_size: 18.0,
            color: Color::srgb(0.8, 0.85, 0.95),
            ..default()
        });
        
        // المحتوى
        content_fn(section);
    });
}

/// معالجة الأحداث
fn handle_radio_events(
    mut events: MessageReader<RadioButtonChangedEvent>,
) {
    for event in events.read() {
        if let Some(group_value) = &event.group_value {
            info!(
                "RadioGroup selection changed to: {} (from button: {})",
                group_value,
                event.value
            );
        } else {
            info!(
                "Individual RadioButton '{}' changed to: {}",
                event.value,
                event.checked
            );
        }
    }
}