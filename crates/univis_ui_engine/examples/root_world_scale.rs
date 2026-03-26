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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 1000.0)));

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|hud| {
            hud.spawn(UTextLabel {
                text: "Fixed Logical Canvas With Explicit World Scale".into(),
                font_size: 30.0,
                color: Color::srgb(0.94, 0.97, 1.0),
                ..default()
            });

            hud.spawn(UTextLabel {
                text: "Both roots below use a 320 x 200 logical canvas. Only meters_per_unit changes their physical size in the world."
                    .into(),
                font_size: 19.0,
                color: Color::srgb(0.72, 0.82, 0.9),
                ..default()
            });
        });

    spawn_world_scale_panel(
        &mut commands,
        Vec3::new(-230.0, -40.0, 0.0),
        0.35,
        Color::srgb(0.18, 0.24, 0.32),
        Color::srgb(0.48, 0.84, 0.96),
    );

    spawn_world_scale_panel(
        &mut commands,
        Vec3::new(220.0, -40.0, 0.0),
        0.9,
        Color::srgb(0.22, 0.17, 0.13),
        Color::srgb(0.98, 0.76, 0.34),
    );
}

fn spawn_world_scale_panel(
    commands: &mut Commands,
    position: Vec3,
    meters_per_unit: f32,
    background_color: Color,
    accent_color: Color,
) {
    let world_width = 320.0 * meters_per_unit;
    let world_height = 200.0 * meters_per_unit;

    commands
        .spawn((
            URootUi {
                meters_per_unit,
                ..URootUi::world_2d(Vec2::new(320.0, 200.0))
            },
            Transform::from_translation(position),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color,
                border_radius: UCornerRadius::all(20.0),
                padding: USides::all(18.0),
                ..default()
            },
            UBorder {
                width: 2.0,
                color: accent_color,
                ..default()
            },
            ULayout {
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn(UTextLabel {
                text: format!("meters_per_unit = {:.2}", meters_per_unit),
                font_size: 28.0,
                color: accent_color,
                ..default()
            });

            panel.spawn(UTextLabel {
                text: "Logical canvas = 320 x 200 UI units".into(),
                font_size: 18.0,
                color: Color::srgb(0.94, 0.96, 1.0),
                ..default()
            });

            panel
                .spawn((
                    UNode {
                        width: UVal::Px(220.0),
                        height: UVal::Px(72.0),
                        background_color: accent_color.with_alpha(0.12),
                        border_radius: UCornerRadius::all(16.0),
                        padding: USides::all(12.0),
                        ..default()
                    },
                    UBorder {
                        width: 1.0,
                        color: accent_color.with_alpha(0.6),
                        ..default()
                    },
                    ULayout {
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|badge| {
                    badge.spawn(UTextLabel {
                        text: format!(
                            "Physical size = {:.0} x {:.0}\nworld units",
                            world_width, world_height
                        ),
                        font_size: 17.0,
                        justify: Justify::Center,
                        color: Color::srgb(0.96, 0.98, 1.0),
                        ..default()
                    });
                });
        });
}
