use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    let font = theme.text.font.inter_regular.clone();
    commands.spawn(Camera2d);

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.11),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 20.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                UPanel::glass().with_gap(12.0),
                UPanelWindow::default().with_min_size(240.0, 160.0),
                UNode {
                    width: UVal::Px(460.0),
                    height: UVal::Px(280.0),
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(UTextLabel {
                    text: "Resizable Panel (Borders + Corners)".to_string(),
                    font_size: 24.0,
                    color: Color::WHITE,
                    font: font.clone(),
                    ..default()
                });

                panel.spawn(UDivider::horizontal().with_thickness(2.0));

                panel.spawn(UTextLabel {
                    text: "Drag panel borders to resize.\nMove/bring-to-front are intentionally out of scope in this example."
                        .to_string(),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.86, 0.96),
                    font: font.clone(),
                    ..default()
                });
            });

            root.spawn((
                UPanel::card().with_gap(10.0),
                UNode {
                    width: UVal::Px(280.0),
                    height: UVal::Px(200.0),
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(UTextLabel {
                    text: "Static Panel".to_string(),
                    font_size: 22.0,
                    color: Color::WHITE,
                    font: font.clone(),
                    ..default()
                });
                panel.spawn(UTextLabel {
                    text: "This one has no UPanelWindow component.".to_string(),
                    font_size: 15.0,
                    color: Color::srgb(0.78, 0.84, 0.92),
                    font,
                    ..default()
                });
            });
        });
}
