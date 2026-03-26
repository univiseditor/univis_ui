//! Explores text measurement, overflow, clipping, and autosize for `UTextLabel`.
//!
//! Related docs:
//! - `docs/src/en/widgets/overview.md`
//! - `docs/src/ar/widgets/overview.md`
//! - `docs/src/en/examples/index.md#text_label`

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
#[allow(unused_imports)]
use univis_ui_interaction::prelude::*;
#[allow(unused_imports)]
use univis_ui_style::prelude::*;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

const MIN_CAMERA_SCALE: f32 = 0.2;
const MAX_CAMERA_SCALE: f32 = 8.0;
const ZOOM_FACTOR_PER_STEP: f32 = 0.85;
const SAMPLE_TEXT: &str =
    "UTextLabel now understands wrapping, clipping, ellipsis, max-lines, and parent clipping.";

#[derive(Component)]
struct ZoomCamera;

#[derive(Component)]
struct ZoomReadout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            UnivisUiStylePlugin,
            UnivisEnginePlugin,
            UnivisInteractionPlugin,
            UnivisWidgetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_zoom_controls, update_zoom_readout))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        ZoomCamera,
    ));

    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_2d(Vec2::new(1680.0, 960.0))
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.10),
                padding: USides::all(28.0),
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
                    width: UVal::Px(1440.0),
                    height: UVal::Px(820.0),
                    padding: USides::all(26.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(30.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.95, 0.74, 0.30, 0.28),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 18.0,
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(UTextLabel {
                    text: "UTextLabel Feature Playground".into(),
                    font_size: 40.0,
                    color: Color::srgb(0.97, 0.90, 0.78),
                    ..default()
                });

                panel.spawn((
                    UNode {
                        width: UVal::Px(1040.0),
                        ..default()
                    },
                    UTextLabel {
                        text: "Inspect every mode in one place: mouse wheel or +/- for zoom, 0 to reset. The cards below show autosize, wrapping, ellipsis, hard clip, max-lines, visible overflow, and parent clipping."
                            .into(),
                        font_size: 18.0,
                        color: Color::srgba(0.90, 0.94, 0.97, 0.86),
                        autosize: false,
                        linebreak: LineBreak::WordBoundary,
                        ..default()
                    },
                ));

                panel.spawn((
                    ZoomReadout,
                    UTextLabel {
                        text: "Zoom: 1.00x | camera scale: 1.000".into(),
                        font_size: 18.0,
                        color: Color::srgb(0.48, 0.86, 1.0),
                        ..default()
                    },
                ));

                panel.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        padding: USides::all(18.0),
                        background_color: Color::srgb(0.08, 0.09, 0.12),
                        border_radius: UCornerRadius::all(24.0),
                        ..default()
                    },
                    UBorder {
                        width: 1.0,
                        color: Color::srgba(0.55, 0.69, 0.80, 0.35),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        gap: 16.0,
                        container_ext: ULayoutContainerExt {
                            box_align: ULayoutBoxAlignContainer {
                                row_gap: Some(16.0),
                                column_gap: Some(16.0),
                                ..default()
                            },
                            flex: ULayoutFlexContainer {
                                wrap: UFlexWrap::Wrap,
                                align_content: Some(UContentAlignExt::Center),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    spawn_demo_card(
                        grid,
                        "Autosize",
                        "The node grows to the measured text automatically.",
                        |card| {
                        card.spawn(UTextLabel {
                            text: "Autosize grows\nto fit content.".into(),
                            font_size: 28.0,
                            color: Color::srgb(0.98, 0.95, 0.90),
                            ..default()
                        });
                    });

                    spawn_demo_card(
                        grid,
                        "Wrap",
                        "Word wrapping inside a fixed-width node.",
                        |card| {
                        spawn_fixed_label(
                            card,
                            SAMPLE_TEXT,
                            280.0,
                            120.0,
                            UTextOverflow::Visible,
                            LineBreak::WordBoundary,
                            None,
                        );
                    });

                    spawn_demo_card(
                        grid,
                        "Ellipsis",
                        "Single-line ellipsis when width is too small.",
                        |card| {
                        spawn_fixed_label(
                            card,
                            "This single-line label should end with three dots when it reaches the node boundary.",
                            280.0,
                            56.0,
                            UTextOverflow::Ellipsis,
                            LineBreak::NoWrap,
                            Some(1),
                        );
                    });

                    spawn_demo_card(
                        grid,
                        "Clip",
                        "Hard local clip inside the node without ellipsis.",
                        |card| {
                        spawn_fixed_label(
                            card,
                            "The end of this sentence is clipped exactly at the node boundary.",
                            280.0,
                            56.0,
                            UTextOverflow::Clip,
                            LineBreak::NoWrap,
                            Some(1),
                        );
                    });

                    spawn_demo_card(
                        grid,
                        "Max Lines",
                        "Two wrapped lines and then ellipsis.",
                        |card| {
                        spawn_fixed_label(
                            card,
                            "This paragraph wraps naturally, but it is limited to two lines, so the renderer must keep only the visible portion and append an ellipsis when needed.",
                            280.0,
                            88.0,
                            UTextOverflow::Ellipsis,
                            LineBreak::WordBoundary,
                            Some(2),
                        );
                    });

                    spawn_demo_card(
                        grid,
                        "Visible Overflow",
                        "Overflow remains visible when clipping is disabled.",
                        |card| {
                        spawn_fixed_label(
                            card,
                            "Visible overflow can continue past the inner box.",
                            240.0,
                            54.0,
                            UTextOverflow::Visible,
                            LineBreak::NoWrap,
                            Some(1),
                        );
                    });

                    spawn_demo_card(
                        grid,
                        "Parent UClip",
                        "The parent mask trims only the overflowing area.",
                        |card| {
                        card.spawn((
                            UClip::enabled(true),
                            UNode {
                                width: UVal::Px(280.0),
                                height: UVal::Px(90.0),
                                padding: USides::all(14.0),
                                background_color: Color::srgb(0.12, 0.15, 0.18),
                                border_radius: UCornerRadius::all(16.0),
                                ..default()
                            },
                            UBorder {
                                width: 1.0,
                                color: Color::srgba(0.35, 0.75, 0.95, 0.45),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Flex,
                                justify_content: UJustifyContent::Center,
                                align_items: UAlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|clip_box| {
                            clip_box.spawn(UTextLabel {
                                text: "Parent clipping now trims the text smoothly instead of hiding the whole label entity."
                                    .into(),
                                font_size: 24.0,
                                color: Color::srgb(0.88, 0.96, 1.0),
                                overflow: UTextOverflow::Visible,
                                linebreak: LineBreak::NoWrap,
                                ..default()
                            });
                        });
                    });
                });
            });
        });
}

fn spawn_demo_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    note: &str,
    content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(328.0),
                height: UVal::Px(220.0),
                padding: USides::all(18.0),
                background_color: Color::srgb(0.13, 0.16, 0.20),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
            UBorder {
                width: 1.0,
                color: Color::srgba(0.90, 0.95, 1.0, 0.18),
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(0.0),
                        flex_basis: Some(UVal::Px(328.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: title.into(),
                font_size: 22.0,
                color: Color::srgb(0.96, 0.79, 0.42),
                ..default()
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    ..default()
                },
                UTextLabel {
                    text: note.into(),
                    font_size: 14.0,
                    color: Color::srgba(0.85, 0.89, 0.92, 0.75),
                    autosize: false,
                    linebreak: LineBreak::WordBoundary,
                    ..default()
                },
            ));

            content(card);
        });
}

fn spawn_fixed_label(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    width: f32,
    height: f32,
    overflow: UTextOverflow,
    linebreak: LineBreak,
    max_lines: Option<usize>,
) {
    parent.spawn((
        UNode {
            width: UVal::Px(width),
            height: UVal::Px(height),
            padding: USides::all(12.0),
            background_color: Color::srgb(0.11, 0.13, 0.16),
            border_radius: UCornerRadius::all(14.0),
            ..default()
        },
        UBorder {
            width: 1.0,
            color: Color::srgba(0.45, 0.78, 0.98, 0.35),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            ..default()
        },
        UTextLabel {
            text: text.into(),
            font_size: 18.0,
            color: Color::srgb(0.92, 0.95, 0.99),
            autosize: false,
            overflow,
            linebreak,
            max_lines,
            justify: Justify::Center,
            ..default()
        },
    ));
}

fn camera_zoom_controls(
    mut mouse_wheel: MessageReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Projection, With<ZoomCamera>>,
) {
    let mut zoom_steps = 0.0;

    for event in mouse_wheel.read() {
        zoom_steps += event.y;
    }

    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        zoom_steps += time.delta_secs() * 8.0;
    }
    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        zoom_steps -= time.delta_secs() * 8.0;
    }

    let reset = keyboard.just_pressed(KeyCode::Digit0);
    if zoom_steps == 0.0 && !reset {
        return;
    }

    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };

    let Projection::Orthographic(orthographic) = &mut *projection else {
        return;
    };

    if reset {
        orthographic.scale = 1.0;
    }

    if zoom_steps != 0.0 {
        orthographic.scale = (orthographic.scale * ZOOM_FACTOR_PER_STEP.powf(zoom_steps))
            .clamp(MIN_CAMERA_SCALE, MAX_CAMERA_SCALE);
    }
}

fn update_zoom_readout(
    camera_query: Query<Ref<Projection>, With<ZoomCamera>>,
    mut readout_query: Query<&mut UTextLabel, With<ZoomReadout>>,
) {
    let Ok(projection) = camera_query.single() else {
        return;
    };

    if !projection.is_changed() {
        return;
    }

    let Projection::Orthographic(orthographic) = &*projection else {
        return;
    };

    let zoom = 1.0 / orthographic.scale;
    let text = format!(
        "Zoom: {:.2}x | camera scale: {:.3} | wheel / +/- / 0",
        zoom, orthographic.scale
    );

    for mut label in readout_query.iter_mut() {
        if label.text != text {
            label.text = text.clone();
        }
    }
}
