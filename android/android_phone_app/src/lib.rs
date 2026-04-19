use bevy::prelude::*;
use univis_ui::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.94, 0.96, 0.99),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                UNode {
                    width: UVal::Px(360.0),
                    min_height: 760.0,
                    padding: USides::all(20.0),
                    background_color: Color::srgb(0.98, 0.99, 1.0),
                    border_radius: UCornerRadius::all(28.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 18.0,
                    ..default()
                },
            ))
            .with_children(|phone| {
                phone.spawn(UTextLabel {
                    text: "Univis Mobile".into(),
                    font_size: 28.0,
                    color: Color::srgb(0.11, 0.16, 0.24),
                    ..default()
                });

                phone.spawn(UTextLabel {
                    text: "Search, quick toggles, and compact controls in one narrow screen.".into(),
                    font_size: 14.0,
                    color: Color::srgb(0.43, 0.49, 0.58),
                    ..default()
                });

                phone.spawn(
                    UTextField::new()
                        .with_placeholder("Search apps and settings")
                        .with_size(320.0, 46.0),
                );

                phone.spawn((
                    UPanel::card()
                        .with_background(Color::srgb(0.91, 0.95, 1.0))
                        .with_padding(USides::all(16.0))
                        .with_gap(14.0),
                    UNode {
                        width: UVal::Px(320.0),
                        ..default()
                    },
                ))
                .with_children(|settings| {
                    settings.spawn(UTextLabel {
                        text: "Quick settings".into(),
                        font_size: 18.0,
                        color: Color::srgb(0.11, 0.16, 0.24),
                        ..default()
                    });

                    spawn_setting_row(settings, "Focus mode", SettingControl::Toggle(true));
                    spawn_setting_row(settings, "Notifications", SettingControl::Toggle(false));
                    spawn_setting_row(settings, "Brightness", SettingControl::SeekBar(0.68));
                });

                phone.spawn((
                    UNode {
                        width: UVal::Px(320.0),
                        padding: USides::all(12.0),
                        background_color: Color::srgb(0.93, 0.96, 1.0),
                        border_radius: UCornerRadius::all(20.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|nav| {
                    spawn_nav_button(nav, "Home");
                    spawn_nav_button(nav, "Explore");
                    spawn_nav_button(nav, "Profile");
                });
            });
        });
}

enum SettingControl {
    Toggle(bool),
    SeekBar(f32),
}

fn spawn_setting_row(parent: &mut ChildSpawnerCommands, label: &str, control: SettingControl) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: label.into(),
                font_size: 16.0,
                color: Color::srgb(0.17, 0.22, 0.31),
                ..default()
            });

            match control {
                SettingControl::Toggle(checked) => {
                    row.spawn(UToggle::material_style().with_checked(checked));
                }
                SettingControl::SeekBar(value) => {
                    row.spawn(
                        USeekBar::brightness_style()
                            .with_value(value)
                            .with_size(160.0, 6.0, 18.0),
                    );
                }
            }
        });
}

fn spawn_nav_button(parent: &mut ChildSpawnerCommands, label: &str) {
    parent
        .spawn(UButton::secondary())
        .with_children(|button| {
            button.spawn(UTextLabel {
                text: label.into(),
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            });
        });
}
