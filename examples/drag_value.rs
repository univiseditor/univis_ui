use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_drag_value_events)
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
                background_color: Color::srgb(0.08, 0.09, 0.13),
                padding: USides::all(28.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 18.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel {
                text: "Drag Value Demo (Drag Left/Right)".to_string(),
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            });

            root.spawn(UTextLabel {
                text: "Hold Shift for fine precision".to_string(),
                font_size: 14.0,
                color: Color::srgb(0.7, 0.75, 0.85),
                ..default()
            });

            spawn_drag_row(
                root,
                "Speed",
                UDragValue::new()
                    .with_range(0.0, 100.0)
                    .with_value(45.0)
                    .with_step(1.0)
                    .with_decimals(0),
            );

            spawn_drag_row(
                root,
                "Opacity",
                UDragValue::new()
                    .with_range(0.0, 1.0)
                    .with_value(0.65)
                    .with_step(0.01)
                    .with_decimals(2)
                    .with_sensitivity_px(220.0),
            );

            spawn_drag_row(
                root,
                "Temperature",
                UDragValue::new()
                    .with_range(-50.0, 250.0)
                    .with_value(23.5)
                    .with_step(0.5)
                    .with_decimals(1),
            );
        });
}

fn spawn_drag_row(parent: &mut ChildSpawnerCommands, label: &str, drag: UDragValue) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(420.0),
                padding: USides::axes(14.0, 10.0),
                background_color: Color::srgba(0.14, 0.16, 0.2, 0.8),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 20.0,
                color: Color::WHITE,
                autosize: true,
                ..default()
            });

            row.spawn((
                drag,
                UNode {
                    width: UVal::Px(120.0),
                    ..default()
                },
            ));
        });
}

fn handle_drag_value_events(
    mut changed_events: MessageReader<DragValueChangedEvent>,
    mut commit_events: MessageReader<DragValueCommitEvent>,
) {
    for ev in changed_events.read() {
        info!(
            "changed: entity={:?} value={:.4} normalized={:.4}",
            ev.entity,
            ev.value,
            ev.normalized
        );
    }

    for ev in commit_events.read() {
        info!(
            "commit: entity={:?} value={:.4} normalized={:.4}",
            ev.entity,
            ev.value,
            ev.normalized
        );
    }
}
