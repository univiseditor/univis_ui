use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (log_select_changes, log_select_open_state))
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
                background_color: Color::srgb(0.07, 0.09, 0.13),
                padding: USides::all(28.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel {
                text: "USelect Demo".to_string(),
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            });

            root.spawn(UTextLabel {
                text: "Mouse + Keyboard: ArrowUp/ArrowDown + Enter + Escape".to_string(),
                font_size: 14.0,
                color: Color::srgb(0.7, 0.75, 0.85),
                ..default()
            });

            spawn_select_row(
                root,
                "Quality",
                USelect::new()
                    .with_options(make_options(&[
                        ("Low", "low", false),
                        ("Medium", "medium", false),
                        ("High", "high", false),
                        ("Ultra", "ultra", false),
                    ]))
                    .with_selected_value("high")
                    .with_size(260.0, 38.0),
            );

            spawn_select_row(
                root,
                "Language",
                USelect::new()
                    .with_options(make_options(&[
                        ("Rust", "rust", false),
                        ("C++", "cpp", true),
                        ("Go", "go", false),
                        ("Python", "python", false),
                    ]))
                    .with_placeholder("Pick language")
                    .with_size(260.0, 38.0),
            );

            spawn_select_row(
                root,
                "Theme",
                USelect::new()
                    .with_options(make_options(&[
                        ("Ocean", "ocean", false),
                        ("Sunset", "sunset", false),
                        ("Forest", "forest", false),
                    ]))
                    .with_placeholder("No selection")
                    .with_size(260.0, 38.0),
            );
        });
}

fn spawn_select_row(parent: &mut ChildSpawnerCommands, label: &str, select: USelect) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(460.0),
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

            row.spawn(select);
        });
}

fn make_options(items: &[(&str, &str, bool)]) -> Vec<USelectOption> {
    items
        .iter()
        .map(|(label, value, disabled)| {
            let option = USelectOption::new(*label, *value);
            if *disabled {
                option.disabled()
            } else {
                option
            }
        })
        .collect()
}

fn log_select_changes(mut reader: MessageReader<SelectChangedEvent>) {
    for event in reader.read() {
        info!(
            "select changed: entity={:?} index={} value={} label={}",
            event.entity, event.selected_index, event.value, event.label
        );
    }
}

fn log_select_open_state(mut reader: MessageReader<SelectOpenStateChangedEvent>) {
    for event in reader.read() {
        info!(
            "select open state: entity={:?} is_open={}",
            event.entity, event.is_open
        );
    }
}
