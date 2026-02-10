use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        
        .add_systems(Startup, setup_seekbar_demo)
        .run();
}

fn setup_seekbar_demo(mut commands: Commands) {
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
        UClip::enabled(true),
            UScrollContainer::new(),
            UInteraction::default(),
            USelf {
                position_type: UPositionType::Absolute,
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
            text: "SeekBar Widget Gallery".to_string(),
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
                gap: 35.0,
                ..default()
            },
        )).with_children(|examples| {
            
            // === مثال 1: Volume Control ===
            create_section(examples, "Volume Control", |section| {
                section.spawn(
                    USeekBar::volume_style()
                        .with_value(0.7)
                        .with_range(0.0, 100.0)
                );
            });
            
            // === مثال 2: Video Progress ===
            create_section(examples, "Video Progress", |section| {
                section.spawn(
                    USeekBar::video_style()
                        .with_value(0.35)
                        .with_range(0.0, 300.0) // 5 minutes
                        .show_value()
                );
            });
            
            // === مثال 3: Brightness ===
            create_section(examples, "Brightness", |section| {
                section.spawn(
                    USeekBar::brightness_style()
                        .with_value(0.6)
                );
            });
            
            // === مثال 4: Sci-Fi Style ===
            create_section(examples, "Sci-Fi HUD", |section| {
                section.spawn(
                    USeekBar::sci_fi_style()
                        .with_value(0.45)
                        .with_range(0.0, 1000.0)
                );
            });
            
            // === مثال 5: Temperature ===
            create_section(examples, "Temperature (with steps)", |section| {
                section.spawn(
                    USeekBar::new()
                        .with_size(250.0, 8.0, 22.0)
                        .with_colors(
                            Color::srgb(0.2, 0.25, 0.35),
                            Color::srgb(1.0, 0.4, 0.2),
                            Color::srgb(1.0, 0.5, 0.3),
                        )
                        .with_range(16.0, 30.0)
                        .with_step(0.5)
                        .with_value(0.5)
                        .show_value()
                );
            });
            
            // === مثال 6: RGB Sliders ===
            create_section(examples, "RGB Color Mixer", |section| {
                
                // Red
                create_rgb_row(section, "R", Color::srgb(1.0, 0.2, 0.2), 0.8);
                
                // Green
                create_rgb_row(section, "G", Color::srgb(0.2, 1.0, 0.2), 0.5);
                
                // Blue
                create_rgb_row(section, "B", Color::srgb(0.3, 0.5, 1.0), 0.6);
            });
            
            // === مثال 7: Custom Minimal ===
            create_section(examples, "Minimal Design", |section| {
                section.spawn(
                    USeekBar::new()
                        .with_size(300.0, 2.0, 12.0)
                        .with_colors(
                            Color::srgba(1.0, 1.0, 1.0, 0.2),
                            Color::WHITE,
                            Color::WHITE,
                        )
                        .with_value(0.25)
                        .with_range(0.0, 10.0)
                        .show_value()
                );
            });
        });
        
        // Footer
        parent.spawn(UTextLabel {
            text: "Drag the sliders to interact!".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.5, 0.5, 0.6),
            ..default()
        });
    });
}

/// Helper لإنشاء قسم
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
            font_size: 16.0,
            color: Color::srgb(0.8, 0.85, 0.95),
            ..default()
        });
        
        // المحتوى
        content_fn(section);
    });
}

/// Helper لصف RGB
fn create_rgb_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    color: Color,
    value: f32,
) {
    parent.spawn((
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 15.0,
            ..default()
        },
    )).with_children(|row| {
        
        // Label
        row.spawn((
            UNode {
                width: UVal::Px(20.0),
                ..default()
            },
            ULayout::default(),
        )).with_children(|l| {
            l.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 14.0,
                color,
                ..default()
            });
        });
        
        // SeekBar
        row.spawn(
            USeekBar::new()
                .with_size(200.0, 4.0, 14.0)
                .with_colors(
                    Color::srgb(0.2, 0.2, 0.25),
                    color,
                    color,
                )
                .with_value(value)
                .with_range(0.0, 255.0)
                .show_value()
        );
    });
}

