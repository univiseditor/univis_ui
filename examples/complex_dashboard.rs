use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Component)]
struct AnimatedProgress {
    mid: f32,
    amp: f32,
    speed: f32,
    phase: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UnivisUiPlugin, UnivisBadgePlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_progress_bars,
                log_select_changes,
                log_drag_value_commits,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(26.0),
                background_color: Color::srgb(0.05, 0.06, 0.1),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 24.0,
                ..default()
            },
        ))
        .with_children(|root| {
            spawn_control_panel(root);
            spawn_preview_panel(root);
        });
}

fn spawn_control_panel(root: &mut ChildSpawnerCommands) {
    root.spawn((
        UPanel::glass().with_gap(12.0),
        UPanelWindow::default().with_min_size(460.0, 520.0),
        UNode {
            width: UVal::Px(560.0),
            height: UVal::Px(700.0),
            ..default()
        },
    ))
    .with_children(|panel| {
        panel.spawn(UTextLabel {
            text: "Complex Dashboard Stress Test".to_string(),
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        });

        panel.spawn(UTextLabel {
            text: "Flex + Grid-like groups + runtime values + interactions".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.73, 0.79, 0.9),
            ..default()
        });

        panel.spawn(UDivider::horizontal().with_thickness(2.0));

        panel.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Start,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_badge(row, "Live", BadgeStyle::Success);
            spawn_badge(row, "UI2D", BadgeStyle::Info);
            spawn_badge(row, "Cached Layout", BadgeStyle::Warning);
        });

        panel.spawn(UDivider::horizontal().with_thickness(1.0));

        panel.spawn(UTextLabel {
            text: "Display Settings".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });

        spawn_select_row(
            panel,
            "Quality",
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Low", "low"),
                    USelectOption::new("Medium", "medium"),
                    USelectOption::new("High", "high"),
                    USelectOption::new("Ultra", "ultra"),
                ])
                .with_selected_value("high")
                .with_size(260.0, 38.0),
        );

        spawn_select_row(
            panel,
            "Theme",
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Nebula", "nebula"),
                    USelectOption::new("Ocean", "ocean"),
                    USelectOption::new("Graphite", "graphite"),
                    USelectOption::new("Forest", "forest"),
                ])
                .with_selected_value("nebula")
                .with_size(260.0, 38.0),
        );

        panel.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: "Enable VSync".to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            row.spawn(UToggle::material_style().with_checked(true));
        });

        panel.spawn(UCheckbox::new("High quality shadows").checked(true));
        panel.spawn(UCheckbox::new("Enable post-processing").checked(true));
        panel.spawn(UCheckbox::new("Show debug overlays"));

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        panel.spawn(UTextLabel {
            text: "Runtime Metrics".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });

        spawn_progress_row(
            panel,
            "CPU",
            Color::srgb(0.19, 0.78, 0.4),
            0.62,
            AnimatedProgress {
                mid: 0.62,
                amp: 0.18,
                speed: 0.9,
                phase: 0.0,
            },
        );
        spawn_progress_row(
            panel,
            "GPU",
            Color::srgb(0.23, 0.56, 0.95),
            0.48,
            AnimatedProgress {
                mid: 0.48,
                amp: 0.24,
                speed: 1.2,
                phase: 1.1,
            },
        );
        spawn_progress_row(
            panel,
            "VRAM",
            Color::srgb(0.9, 0.67, 0.22),
            0.72,
            AnimatedProgress {
                mid: 0.72,
                amp: 0.14,
                speed: 0.7,
                phase: 2.0,
            },
        );

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        panel.spawn(UTextLabel {
            text: "Advanced Tuning".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });

        spawn_drag_row(
            panel,
            "Exposure",
            UDragValue::new()
                .with_range(0.0, 5.0)
                .with_value(1.35)
                .with_step(0.01)
                .with_decimals(2),
        );
        spawn_drag_row(
            panel,
            "Sharpness",
            UDragValue::new()
                .with_range(0.0, 2.0)
                .with_value(0.7)
                .with_step(0.01)
                .with_decimals(2),
        );
        spawn_drag_row(
            panel,
            "Bloom",
            UDragValue::new()
                .with_range(0.0, 3.0)
                .with_value(1.1)
                .with_step(0.05)
                .with_decimals(2),
        );

        panel.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|actions| {
            actions.spawn((
                UButton::primary(),
                USelf {
                    item_ext: ULayoutItemExt {
                        flex: ULayoutFlexItem {
                            flex_grow: Some(1.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                ULayout {
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|btn| {
                btn.spawn(UTextLabel {
                    text: "Apply".to_string(),
                    font_size: 16.0,
                    ..default()
                });
            });

            actions.spawn((
                UButton::secondary(),
                USelf {
                    item_ext: ULayoutItemExt {
                        flex: ULayoutFlexItem {
                            flex_grow: Some(1.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                ULayout {
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|btn| {
                btn.spawn(UTextLabel {
                    text: "Reset".to_string(),
                    font_size: 16.0,
                    ..default()
                });
            });
        });
    });
}

fn spawn_preview_panel(root: &mut ChildSpawnerCommands) {
    root.spawn((
        UPanel::card().with_gap(14.0),
        UNode {
            width: UVal::Px(460.0),
            height: UVal::Px(700.0),
            background_color: Color::srgb(0.1, 0.12, 0.18),
            ..default()
        },
    ))
    .with_children(|panel| {
        panel.spawn(UTextLabel {
            text: "Live Preview".to_string(),
            font_size: 28.0,
            color: Color::WHITE,
            ..default()
        });

        panel.spawn(UTextLabel {
            text: "Reference card used for layout consistency checks".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.76, 0.8, 0.9),
            ..default()
        });

        panel.spawn(UDivider::horizontal().with_thickness(2.0));

        panel.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(540.0),
                background_color: Color::srgb(0.16, 0.18, 0.28),
                border_radius: UCornerRadius::all(22.0),
                padding: USides::all(22.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Start,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn((
                UNode {
                    width: UVal::Px(92.0),
                    height: UVal::Px(92.0),
                    background_color: Color::srgb(0.78, 0.83, 0.95),
                    border_radius: UCornerRadius::all(46.0),
                    ..default()
                },
                UBorder {
                    color: Color::WHITE,
                    width: 2.0,
                    offset: 6.0,
                    ..default()
                },
            ));

            card.spawn(UTextLabel {
                text: "Univis QA Runner".to_string(),
                font_size: 26.0,
                color: Color::WHITE,
                ..default()
            });

            card.spawn(UTextLabel {
                text: "@layout-regression".to_string(),
                font_size: 14.0,
                color: Color::srgb(0.66, 0.7, 0.8),
                ..default()
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Center,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|tags| {
                spawn_badge(tags, "Flex", BadgeStyle::Info);
                spawn_badge(tags, "Cache", BadgeStyle::Warning);
                spawn_badge(tags, "Interaction", BadgeStyle::Success);
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(0.0, 10.0),
                    background_color: Color::srgba(0.05, 0.06, 0.1, 0.35),
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceEvenly,
                    ..default()
                },
            ))
            .with_children(|stats| {
                spawn_stat(stats, "87", "Checks");
                spawn_stat(stats, "0", "Regressions");
                spawn_stat(stats, "99.4%", "Pass");
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|actions| {
                actions.spawn((
                    UButton::primary(),
                    USelf {
                        item_ext: ULayoutItemExt {
                            flex: ULayoutFlexItem {
                                flex_grow: Some(1.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    btn.spawn(UTextLabel {
                        text: "Inspect".to_string(),
                        font_size: 15.0,
                        ..default()
                    });
                });

                actions.spawn((
                    UButton::secondary(),
                    USelf {
                        item_ext: ULayoutItemExt {
                            flex: ULayoutFlexItem {
                                flex_grow: Some(1.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    btn.spawn(UTextLabel {
                        text: "Export".to_string(),
                        font_size: 15.0,
                        ..default()
                    });
                });
            });
        });
    });
}

fn spawn_select_row(parent: &mut ChildSpawnerCommands, label: &str, select: USelect) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                autosize: true,
                ..default()
            });
            row.spawn(select);
        });
}

fn spawn_drag_row(parent: &mut ChildSpawnerCommands, label: &str, drag: UDragValue) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                autosize: true,
                ..default()
            });
            row.spawn((
                drag,
                UNode {
                    width: UVal::Px(160.0),
                    ..default()
                },
            ));
        });
}

fn spawn_progress_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    color: Color,
    value: f32,
    anim: AnimatedProgress,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                autosize: true,
                ..default()
            });
            row.spawn((
                UProgressBar {
                    value,
                    bar_color: color,
                },
                anim,
                UNode {
                    width: UVal::Px(220.0),
                    ..default()
                },
            ));
        });
}

fn spawn_badge(parent: &mut ChildSpawnerCommands, text: &str, style: BadgeStyle) {
    parent
        .spawn((UBadge { style, size: BadgeSize::Small }, UNode::default(), ULayout::default()))
        .with_children(|badge| {
            badge.spawn(UTextLabel {
                text: text.to_string(),
                font_size: 12.0,
                color: Color::WHITE,
                ..default()
            });
        });
}

fn spawn_stat(parent: &mut ChildSpawnerCommands, value: &str, label: &str) {
    parent
        .spawn((
            UNode::default(),
            ULayout {
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|col| {
            col.spawn(UTextLabel {
                text: value.to_string(),
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            });
            col.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 12.0,
                color: Color::srgb(0.66, 0.7, 0.78),
                ..default()
            });
        });
}

fn animate_progress_bars(time: Res<Time>, mut query: Query<(&mut UProgressBar, &AnimatedProgress)>) {
    for (mut bar, anim) in query.iter_mut() {
        let wave = (time.elapsed_secs() * anim.speed + anim.phase).sin();
        bar.value = (anim.mid + anim.amp * wave).clamp(0.0, 1.0);
    }
}

fn log_select_changes(mut reader: MessageReader<SelectChangedEvent>) {
    for event in reader.read() {
        info!(
            "select changed: entity={:?} index={} value={} label={}",
            event.entity, event.selected_index, event.value, event.label
        );
    }
}

fn log_drag_value_commits(mut reader: MessageReader<DragValueCommitEvent>) {
    for event in reader.read() {
        info!(
            "drag commit: entity={:?} value={:.3} normalized={:.3}",
            event.entity, event.value, event.normalized
        );
    }
}
