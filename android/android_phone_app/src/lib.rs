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
                background_color: Color::srgb(0.07, 0.09, 0.13),
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
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    max_width: 430.0,
                    padding: USides::all(16.0),
                    background_color: Color::srgb(0.95, 0.97, 1.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 12.0,
                    ..default()
                },
            ))
            .with_children(|phone| {
                spawn_top_bar(phone);
                spawn_focus_card(phone);

                phone.spawn(
                    UTextField::new()
                        .with_text("Mix review assets")
                        .with_placeholder("Search scenes, widgets, and notes")
                        .with_size(366.0, 48.0),
                );

                phone
                    .spawn((
                        fill_vertical_space(),
                        UClip::enabled(true),
                        UScrollContainer::new(),
                        UInteraction::default(),
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(0.0),
                            padding: USides::all(8.0),
                            background_color: Color::srgba(0.89, 0.93, 0.99, 0.92),
                            border_radius: UCornerRadius::all(24.0),
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            ..default()
                        },
                    ))
                    .with_children(|viewport| {
                        viewport
                            .spawn((
                                USelf::default(),
                                UNode {
                                    width: UVal::Percent(1.0),
                                    height: UVal::Content,
                                    background_color: Color::NONE,
                                    ..default()
                                },
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 12.0,
                                    ..default()
                                },
                            ))
                            .with_children(|content| {
                                spawn_quick_actions(content);
                                spawn_quick_controls(content);
                                spawn_sync_status(content);
                                spawn_release_queue(content);
                            });
                    });

                spawn_bottom_nav(phone);
            });
        });
}

fn fill_vertical_space() -> USelf {
    USelf {
        item_ext: ULayoutItemExt {
            flex: ULayoutFlexItem {
                flex_grow: Some(1.0),
                flex_shrink: Some(1.0),
                flex_basis: Some(UVal::Px(0.0)),
            },
            ..default()
        },
        ..default()
    }
}

fn spawn_top_bar(parent: &mut ChildSpawnerCommands) {
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
            row.spawn((
                UNode {
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                    ..default()
                },
            ))
            .with_children(|labels| {
                labels.spawn(UTextLabel {
                    text: "Univis Mobile".into(),
                    font_size: 24.0,
                    color: Color::srgb(0.1, 0.14, 0.22),
                    ..default()
                });
                labels.spawn(UTextLabel {
                    text: "Android control room".into(),
                    font_size: 13.0,
                    color: Color::srgb(0.41, 0.47, 0.56),
                    ..default()
                });
            });

            row.spawn((
                UNode {
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|actions| {
                spawn_badge(
                    actions,
                    "beta",
                    UBadge::info().small(),
                    Color::srgb(0.06, 0.12, 0.2),
                );
                spawn_badge(actions, "5G", UBadge::success().small(), Color::WHITE);
            });
        });
}

fn spawn_focus_card(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UPanel::card()
                .with_background(Color::srgb(0.18, 0.31, 0.82))
                .with_padding(USides::all(16.0))
                .with_gap(10.0),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn((
                UNode {
                    background_color: Color::NONE,
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
                spawn_badge(row, "Live sync", UBadge::success().small(), Color::WHITE);
                spawn_badge(
                    row,
                    "12 updates",
                    UBadge::info().small(),
                    Color::srgb(0.06, 0.12, 0.2),
                );
            });

            card.spawn(UTextLabel {
                text: "Studio handheld dashboard".into(),
                font_size: 22.0,
                color: Color::WHITE,
                ..default()
            });
            card.spawn(UTextLabel {
                text: "A narrow Android scene with search, scroll, quick toggles, and release actions."
                    .into(),
                font_size: 14.0,
                color: Color::srgba(0.93, 0.96, 1.0, 0.92),
                ..default()
            });
        });
}

fn spawn_quick_actions(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UPanel::glass()
                .with_background(Color::srgba(0.1, 0.14, 0.2, 0.9))
                .with_padding(USides::all(14.0))
                .with_gap(12.0),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_heading(
                card,
                "Quick actions",
                "High-value commands at the top of the feed.",
            );

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|row| {
                spawn_action_button(row, "Review", UButton::primary(), Color::WHITE);
                spawn_action_button(row, "Capture", UButton::success(), Color::WHITE);
                spawn_action_button(
                    row,
                    "Offline",
                    UButton::secondary(),
                    Color::srgb(0.93, 0.95, 0.98),
                );
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|chips| {
                spawn_badge(
                    chips,
                    "Touch ready",
                    UBadge::info().small(),
                    Color::srgb(0.06, 0.12, 0.2),
                );
                spawn_badge(chips, "Widgets", UBadge::success().small(), Color::WHITE);
                spawn_badge(chips, "Android", UBadge::primary().small(), Color::WHITE);
            });
        });
}

fn spawn_quick_controls(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UPanel::glass()
                .with_background(Color::srgba(0.08, 0.11, 0.16, 0.9))
                .with_padding(USides::all(14.0))
                .with_gap(12.0),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_heading(
                card,
                "Quick controls",
                "Small interactive controls sized for a phone feed.",
            );

            spawn_setting_row(card, "Focus mode", SettingControl::Toggle(true));
            spawn_setting_row(card, "Cloud backup", SettingControl::Toggle(false));
            spawn_setting_row(card, "Brightness", SettingControl::Brightness(0.72));
            spawn_setting_row(card, "Monitoring", SettingControl::Volume(0.44));
        });
}

fn spawn_sync_status(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UPanel::glass()
                .with_background(Color::srgba(0.09, 0.13, 0.18, 0.9))
                .with_padding(USides::all(14.0))
                .with_gap(12.0),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_heading(
                card,
                "Sync status",
                "Compact status rows with badges for mobile operations screens.",
            );

            spawn_status_row(card, "Asset mirror", "Healthy", UBadge::success().small());
            spawn_status_row(card, "Build cache", "Warm", UBadge::info().small());
            spawn_status_row(card, "Audio stems", "Review", UBadge::warning().small());
        });
}

fn spawn_release_queue(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UPanel::glass()
                .with_background(Color::srgba(0.08, 0.11, 0.16, 0.9))
                .with_padding(USides::all(14.0))
                .with_gap(12.0),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_heading(
                card,
                "Release queue",
                "A second card so the scroll container has real content to move through.",
            );

            spawn_queue_item(card, "Patch notes", "Ready for localization");
            spawn_queue_item(card, "Hero scene", "Needs final color pass");
            spawn_queue_item(card, "Screenshots", "Awaiting Android export");
        });
}

fn spawn_bottom_nav(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.12, 0.16, 0.23),
                border_radius: UCornerRadius::all(22.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|nav| {
            spawn_action_button(nav, "Home", UButton::primary(), Color::WHITE);
            spawn_action_button(
                nav,
                "Tasks",
                UButton::secondary(),
                Color::srgb(0.93, 0.95, 0.98),
            );
            spawn_action_button(
                nav,
                "Sync",
                UButton::secondary(),
                Color::srgb(0.93, 0.95, 0.98),
            );
            spawn_action_button(
                nav,
                "Profile",
                UButton::secondary(),
                Color::srgb(0.93, 0.95, 0.98),
            );
        });
}

fn spawn_section_heading(parent: &mut ChildSpawnerCommands, title: &str, subtitle: &str) {
    parent.spawn(UTextLabel {
        text: title.into(),
        font_size: 18.0,
        color: Color::WHITE,
        ..default()
    });
    parent.spawn(UTextLabel {
        text: subtitle.into(),
        font_size: 13.0,
        color: Color::srgba(0.84, 0.89, 0.96, 0.9),
        ..default()
    });
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    button: UButton,
    text_color: Color,
) {
    parent.spawn(button).with_children(|slot| {
        slot.spawn(UTextLabel {
            text: label.into(),
            font_size: 13.0,
            color: text_color,
            ..default()
        });
    });
}

fn spawn_badge(parent: &mut ChildSpawnerCommands, label: &str, badge: UBadge, text_color: Color) {
    parent.spawn(badge).with_children(|slot| {
        slot.spawn(UTextLabel {
            text: label.into(),
            font_size: 12.0,
            color: text_color,
            ..default()
        });
    });
}

fn spawn_status_row(parent: &mut ChildSpawnerCommands, label: &str, status: &str, badge: UBadge) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 10.0),
                background_color: Color::srgba(0.15, 0.19, 0.27, 0.72),
                border_radius: UCornerRadius::all(16.0),
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
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            });
            spawn_badge(row, status, badge, Color::WHITE);
        });
}

fn spawn_queue_item(parent: &mut ChildSpawnerCommands, title: &str, detail: &str) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.16, 0.2, 0.28, 0.72),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
        ))
        .with_children(|item| {
            item.spawn(UTextLabel {
                text: title.into(),
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            });
            item.spawn(UTextLabel {
                text: detail.into(),
                font_size: 13.0,
                color: Color::srgba(0.84, 0.89, 0.96, 0.9),
                ..default()
            });
        });
}

enum SettingControl {
    Toggle(bool),
    Brightness(f32),
    Volume(f32),
}

fn spawn_setting_row(parent: &mut ChildSpawnerCommands, label: &str, control: SettingControl) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 10.0),
                background_color: Color::srgba(0.15, 0.19, 0.27, 0.72),
                border_radius: UCornerRadius::all(16.0),
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
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            });

            match control {
                SettingControl::Toggle(checked) => {
                    row.spawn(UToggle::material_style().with_checked(checked));
                }
                SettingControl::Brightness(value) => {
                    row.spawn(
                        USeekBar::brightness_style()
                            .with_value(value)
                            .with_size(168.0, 6.0, 18.0),
                    );
                }
                SettingControl::Volume(value) => {
                    row.spawn(
                        USeekBar::volume_style()
                            .with_value(value)
                            .with_size(168.0, 6.0, 18.0),
                    );
                }
            }
        });
}
