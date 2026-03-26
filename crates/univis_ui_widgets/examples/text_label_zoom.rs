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
                ..URootUi::world_2d(Vec2::new(1600.0, 900.0))
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.10),
                padding: USides::all(36.0),
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
                    width: UVal::Px(1120.0),
                    height: UVal::Px(700.0),
                    padding: USides::all(28.0),
                    background_color: Color::srgb(0.11, 0.13, 0.16),
                    border_radius: UCornerRadius::all(28.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.95, 0.74, 0.30, 0.35),
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
                    text: "UTextLabel Zoom Playground".into(),
                    font_size: 42.0,
                    color: Color::srgb(0.97, 0.90, 0.78),
                    ..default()
                });

                panel.spawn((
                    UNode {
                        width: UVal::Px(860.0),
                        ..default()
                    },
                    UTextLabel {
                        text: "Use the mouse wheel to zoom. Press + or - for fine control, and 0 to reset the camera."
                            .into(),
                        font_size: 18.0,
                        color: Color::srgba(0.90, 0.94, 0.97, 0.84),
                        autosize: false,
                        linebreak: LineBreak::WordBoundary,
                        ..default()
                    },
                ));

                panel.spawn((
                    ZoomReadout,
                    UTextLabel {
                        text: "Zoom: 1.00x | camera scale: 1.000".into(),
                        font_size: 20.0,
                        color: Color::srgb(0.48, 0.86, 1.0),
                        ..default()
                    },
                ));

                panel.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        padding: USides::all(28.0),
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
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|stage| {
                    stage.spawn((
                        UNode {
                            width: UVal::Px(820.0),
                            height: UVal::Px(460.0),
                            padding: USides::all(24.0),
                            background_color: Color::srgb(0.13, 0.16, 0.20),
                            border_radius: UCornerRadius::all(18.0),
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 12.0,
                            ..default()
                        },
                    ))
                    .with_children(|content| {
                        content.spawn(UTextLabel {
                            text: "UNIVIS UI".into(),
                            font_size: 112.0,
                            color: Color::srgb(0.96, 0.79, 0.42),
                            ..default()
                        });

                        content.spawn(UTextLabel {
                            text: "Sharp while zooming in".into(),
                            font_size: 42.0,
                            color: Color::srgb(0.93, 0.96, 0.99),
                            ..default()
                        });

                        content.spawn(UTextLabel {
                            text: "0123456789  The quick brown fox jumps over the lazy dog".into(),
                            font_size: 24.0,
                            color: Color::srgb(0.58, 0.86, 1.0),
                            ..default()
                        });

                        content.spawn((
                            UNode {
                                width: UVal::Px(680.0),
                                ..default()
                            },
                            UTextLabel {
                                text: "This block is intentionally wrapped so you can inspect edge quality on small text, long lines, and paragraph-style layout while moving the camera closer and farther."
                                    .into(),
                                font_size: 18.0,
                                color: Color::srgba(0.88, 0.91, 0.94, 0.90),
                                autosize: false,
                                linebreak: LineBreak::WordBoundary,
                                justify: Justify::Center,
                                ..default()
                            },
                        ));
                    });
                });
            });
        });
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
