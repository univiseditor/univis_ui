use bevy::prelude::*;
use univis_ui_engine::internal::ComputedSize;
use univis_ui_engine::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;

#[derive(Component)]
struct CenterProbe;

#[derive(Component, Clone, Copy)]
enum PulseAxis {
    Width,
    Height,
}

#[derive(Component, Clone, Copy)]
struct Pulse {
    axis: PulseAxis,
    base: f32,
    amp: f32,
    speed: f32,
    phase: f32,
}

#[derive(Resource)]
struct VerifyTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisEnginePlugin)
        .insert_resource(VerifyTimer(Timer::from_seconds(0.6, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_pulses, verify_center_stability))
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
                background_color: Color::srgb(0.04, 0.05, 0.09),
                padding: USides::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                UNode {
                    width: UVal::Px(1140.0),
                    height: UVal::Px(700.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgb(0.1, 0.13, 0.2),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.9, 0.95, 1.0, 0.35),
                    width: 1.0,
                    offset: 0.0,
                    radius: UCornerRadius::all(14.0),
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 16.0,
                    align_items: UAlignItems::Stretch,
                    ..default()
                },
                CenterProbe,
            ))
            .with_children(|frame| {
                spawn_left_stress_column(frame);
                spawn_right_reference_column(frame);
            });
        });
}

fn spawn_left_stress_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.7),
                height: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.09, 0.11, 0.16),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|left| {
            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(180.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.16, 0.19, 0.27),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                    ..default()
                },
            ))
            .with_children(|row| {
                spawn_top_tile(row, Color::srgb(0.22, 0.36, 0.62), 2.0, 0.0);
                spawn_top_tile(row, Color::srgb(0.2, 0.5, 0.4), 1.0, 0.7);
                spawn_top_tile(row, Color::srgb(0.56, 0.36, 0.23), 1.0, 1.4);
            });

            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.12, 0.15, 0.22),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Grid,
                    grid_columns: 4,
                    gap: 8.0,
                    container_ext: ULayoutContainerExt {
                        grid: ULayoutGridContainer {
                            template_columns: vec![
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                            ],
                            auto_rows: UTrackSize::Px(58.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|grid| {
                spawn_grid_tile(grid, Color::srgb(0.19, 0.57, 0.88), None, 2, None, 2);
                spawn_grid_tile(grid, Color::srgb(0.76, 0.34, 0.3), None, 1, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.22, 0.68, 0.42), None, 1, None, 2);
                spawn_grid_tile(grid, Color::srgb(0.86, 0.66, 0.24), None, 2, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.42, 0.48, 0.88), None, 1, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.26, 0.78, 0.78), None, 1, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.56, 0.38, 0.74), None, 2, None, 1);
            });

            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(130.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.16, 0.18, 0.25),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                    ..default()
                },
            ))
            .with_children(|bottom| {
                for i in 0..4 {
                    bottom.spawn((
                        UNode {
                            width: UVal::Flex(1.0),
                            height: UVal::Px(84.0),
                            background_color: Color::srgb(
                                0.24 + i as f32 * 0.05,
                                0.3 + i as f32 * 0.03,
                                0.45 + i as f32 * 0.02,
                            ),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
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
                        Pulse {
                            axis: PulseHeight::axis(),
                            base: 78.0 + i as f32 * 2.0,
                            amp: 10.0,
                            speed: 0.55 + i as f32 * 0.12,
                            phase: i as f32,
                        },
                    ));
                }
            });
        });
}

fn spawn_right_reference_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.1, 0.12, 0.18),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|right| {
            right.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    padding: USides::all(18.0),
                    background_color: Color::srgb(0.2, 0.22, 0.38),
                    border_radius: UCornerRadius::all(20.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                    ..default()
                },
            ))
            .with_children(|card| {
                card.spawn((
                    UNode {
                        width: UVal::Px(92.0),
                        height: UVal::Px(92.0),
                        background_color: Color::srgb(0.73, 0.78, 0.9),
                        border_radius: UCornerRadius::all(46.0),
                        ..default()
                    },
                    UBorder {
                        color: Color::WHITE,
                        width: 2.0,
                        offset: 5.0,
                        radius: UCornerRadius::all(46.0),
                    },
                ));

                card.spawn(UNode {
                    width: UVal::Px(240.0),
                    height: UVal::Px(30.0),
                    background_color: Color::srgba(0.95, 0.96, 1.0, 0.86),
                    border_radius: UCornerRadius::all(6.0),
                    ..default()
                });

                card.spawn(UNode {
                    width: UVal::Px(170.0),
                    height: UVal::Px(18.0),
                    background_color: Color::srgba(0.74, 0.78, 0.88, 0.75),
                    border_radius: UCornerRadius::all(5.0),
                    ..default()
                });

                card.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Auto,
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
                    for color in [
                        Color::srgb(0.28, 0.66, 0.96),
                        Color::srgb(0.86, 0.68, 0.22),
                        Color::srgb(0.24, 0.76, 0.4),
                    ] {
                        tags.spawn(UNode {
                            width: UVal::Px(56.0),
                            height: UVal::Px(22.0),
                            background_color: color,
                            border_radius: UCornerRadius::all(11.0),
                            ..default()
                        });
                    }
                });

                card.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(90.0),
                        padding: USides::all(8.0),
                        background_color: Color::srgba(0.08, 0.09, 0.15, 0.4),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                        ..default()
                    },
                ))
                .with_children(|stats| {
                    for i in 0..3 {
                        stats.spawn((
                            UNode {
                                width: UVal::Flex(1.0),
                                height: UVal::Px(70.0),
                                background_color: Color::srgb(
                                    0.18 + i as f32 * 0.02,
                                    0.2 + i as f32 * 0.02,
                                    0.28 + i as f32 * 0.02,
                                ),
                                border_radius: UCornerRadius::all(8.0),
                                ..default()
                            },
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
                        ));
                    }
                });

                card.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Auto,
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
                        UNode {
                            width: UVal::Flex(1.0),
                            height: UVal::Px(44.0),
                            background_color: Color::srgb(0.21, 0.5, 0.92),
                            border_radius: UCornerRadius::all(9.0),
                            ..default()
                        },
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
                    ));

                    actions.spawn((
                        UNode {
                            width: UVal::Flex(1.0),
                            height: UVal::Px(44.0),
                            background_color: Color::srgb(0.45, 0.47, 0.52),
                            border_radius: UCornerRadius::all(9.0),
                            ..default()
                        },
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
                    ));
                });

                // Absolute corner probes to exercise out-of-flow math.
                card.spawn((
                    UNode {
                        width: UVal::Px(18.0),
                        height: UVal::Px(18.0),
                        background_color: Color::srgb(0.98, 0.35, 0.35),
                        border_radius: UCornerRadius::all(9.0),
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        right: UVal::Px(12.0),
                        top: UVal::Px(12.0),
                        ..default()
                    },
                ));

                card.spawn((
                    UNode {
                        width: UVal::Px(18.0),
                        height: UVal::Px(18.0),
                        background_color: Color::srgb(0.35, 0.98, 0.68),
                        border_radius: UCornerRadius::all(9.0),
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Px(12.0),
                        bottom: UVal::Px(12.0),
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_top_tile(parent: &mut ChildSpawnerCommands, color: Color, grow: f32, phase: f32) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(8.0),
                background_color: color,
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(grow),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
        ))
        .with_children(|tile| {
            tile.spawn(UNode {
                width: UVal::Px(86.0),
                height: UVal::Px(12.0),
                background_color: Color::srgba(0.95, 0.97, 1.0, 0.7),
                border_radius: UCornerRadius::all(5.0),
                ..default()
            });

            for i in 0..3 {
                tile.spawn((
                    UNode {
                        width: UVal::Px(70.0 + i as f32 * 26.0),
                        height: UVal::Px(10.0),
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.45),
                        border_radius: UCornerRadius::all(5.0),
                        ..default()
                    },
                    Pulse {
                        axis: PulseAxis::Width,
                        base: 82.0 + i as f32 * 18.0,
                        amp: 28.0 - i as f32 * 6.0,
                        speed: 0.9 + i as f32 * 0.25,
                        phase: phase + i as f32 * 0.6,
                    },
                ));
            }
        });
}

fn spawn_grid_tile(
    parent: &mut ChildSpawnerCommands,
    color: Color,
    col_start: Option<u32>,
    col_span: u32,
    row_start: Option<u32>,
    row_span: u32,
) {
    parent.spawn((
        UNode {
            width: UVal::Auto,
            height: UVal::Auto,
            background_color: color,
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        USelf {
            item_ext: ULayoutItemExt {
                grid: ULayoutGridItem {
                    column_start: col_start,
                    column_span: col_span.max(1),
                    row_start,
                    row_span: row_span.max(1),
                },
                ..default()
            },
            ..default()
        },
    ));
}

fn animate_pulses(time: Res<Time>, mut query: Query<(&mut UNode, &Pulse)>) {
    for (mut node, pulse) in query.iter_mut() {
        let value = (pulse.base + pulse.amp * (time.elapsed_secs() * pulse.speed + pulse.phase).sin())
            .max(4.0);
        match pulse.axis {
            PulseAxis::Width => node.width = UVal::Px(value),
            PulseAxis::Height => node.height = UVal::Px(value),
        }
    }
}

fn verify_center_stability(
    time: Res<Time>,
    mut timer: ResMut<VerifyTimer>,
    query: Query<(&GlobalTransform, &ComputedSize), With<CenterProbe>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok((transform, size)) = query.single() else {
        return;
    };

    let pos = transform.translation();
    let dx = pos.x.abs();
    let dy = pos.y.abs();

    if dx > 1.0 || dy > 1.0 {
        warn!(
            "CENTER DRIFT: probe at ({:.2}, {:.2}), size=({:.1}, {:.1})",
            pos.x, pos.y, size.width, size.height
        );
    } else {
        info!(
            "center stable: ({:.2}, {:.2}), size=({:.1}, {:.1})",
            pos.x, pos.y, size.width, size.height
        );
    }
}

struct PulseHeight;

impl PulseHeight {
    fn axis() -> PulseAxis {
        PulseAxis::Height
    }
}
