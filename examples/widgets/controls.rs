use bevy::picking::Pickable;
use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, emit_toggle_events)
        .add_systems(Update, (log_toggle_events, log_radio_events))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.1),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let shell = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(1120.0),
                height: UVal::Px(760.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.98),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.72, 0.8, 0.9, 0.18),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(shell),
        label_node(),
        text("Widgets: controls", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Buttons, checkboxes, toggles, and radio groups. Toggle and radio events are written to the console.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1072.0),
                height: UVal::Px(640.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    let left = panel(&mut commands, content, 527.0, 640.0, "Buttons + booleans");
    let right = panel(&mut commands, content, 527.0, 640.0, "Radio groups");

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "Buttons rely on hover/press feedback from `UButton`. Checkboxes and toggles demonstrate binary state widgets.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let button_row = commands
        .spawn((
            ChildOf(left),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_button(&mut commands, button_row, "Primary", UButton::primary());
    spawn_button(&mut commands, button_row, "Secondary", UButton::secondary());
    spawn_button(&mut commands, button_row, "Success", UButton::success());
    spawn_button(&mut commands, button_row, "Danger", UButton::danger());

    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    let bool_grid = commands
        .spawn((
            ChildOf(left),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(bool_grid),
        UCheckbox::new("Auto-save").checked(true),
    ));
    commands.spawn((ChildOf(bool_grid), UCheckbox::new("Offline cache")));
    commands.spawn((
        ChildOf(bool_grid),
        UCheckbox::new("Experimental mode").with_color(Color::srgb(0.92, 0.62, 0.18)),
    ));

    spawn_toggle_row(
        &mut commands,
        bool_grid,
        "Alerts",
        UToggle::ios_style().with_checked(true),
    );
    spawn_toggle_row(
        &mut commands,
        bool_grid,
        "Compact mode",
        UToggle::material_style(),
    );
    spawn_toggle_row(
        &mut commands,
        bool_grid,
        "Sci-fi HUD",
        UToggle::sci_fi_style(),
    );

    commands.spawn((
        ChildOf(right),
        label_node(),
        text(
            "Radio buttons can be nested under row wrappers. The group scans descendants recursively and keeps one selected value.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let quality_group = commands
        .spawn((
            ChildOf(right),
            URadioGroup::new().with_default("balanced").with_gap(12.0),
        ))
        .id();
    spawn_radio_row(
        &mut commands,
        quality_group,
        URadioButton::primary_style("performance"),
        "Performance",
    );
    spawn_radio_row(
        &mut commands,
        quality_group,
        URadioButton::primary_style("balanced").checked(),
        "Balanced",
    );
    spawn_radio_row(
        &mut commands,
        quality_group,
        URadioButton::success_style("quality"),
        "Quality",
    );

    commands.spawn((ChildOf(right), UDivider::horizontal().with_thickness(2.0)));

    let view_group = commands
        .spawn((
            ChildOf(right),
            URadioGroup::new()
                .horizontal()
                .with_gap(16.0)
                .with_default("dock"),
        ))
        .id();
    spawn_radio_card(
        &mut commands,
        view_group,
        URadioButton::new("dock"),
        "Dock",
        Color::srgba(0.2, 0.37, 0.82, 0.22),
    );
    spawn_radio_card(
        &mut commands,
        view_group,
        URadioButton::new("float"),
        "Float",
        Color::srgba(0.18, 0.68, 0.48, 0.22),
    );
    spawn_radio_card(
        &mut commands,
        view_group,
        URadioButton::danger_style("fullscreen"),
        "Fullscreen",
        Color::srgba(0.77, 0.27, 0.31, 0.22),
    );

    commands.spawn((ChildOf(right), UDivider::horizontal().with_thickness(2.0)));

    let optional_group = commands
        .spawn((
            ChildOf(right),
            URadioGroup::new().allow_deselect().with_gap(12.0),
        ))
        .id();
    spawn_radio_row(
        &mut commands,
        optional_group,
        URadioButton::new("logs"),
        "Logs pane",
    );
    spawn_radio_row(
        &mut commands,
        optional_group,
        URadioButton::new("metrics"),
        "Metrics pane",
    );
    spawn_radio_row(
        &mut commands,
        optional_group,
        URadioButton::new("notifications"),
        "Notifications pane",
    );
}

fn panel(commands: &mut Commands, parent: Entity, width: f32, height: f32, title: &str) -> Entity {
    let panel = commands
        .spawn((
            ChildOf(parent),
            UPanel::glass().with_gap(12.0),
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(title, 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));

    panel
}

fn spawn_button(commands: &mut Commands, parent: Entity, label: &str, button: UButton) {
    let entity = commands
        .spawn((
            ChildOf(parent),
            button,
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(entity),
        UTextLabel {
            text: label.to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        },
        Pickable::IGNORE,
    ));
}

fn spawn_toggle_row(commands: &mut Commands, parent: Entity, label: &str, toggle: UToggle) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 10.0),
                background_color: Color::srgba(0.12, 0.15, 0.2, 0.6),
                border_radius: UCornerRadius::all(14.0),
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
        .id();

    commands.spawn((ChildOf(row), label_node(), text(label, 16.0, Color::WHITE)));
    commands.spawn((ChildOf(row), toggle));
}

fn spawn_radio_row(commands: &mut Commands, parent: Entity, radio: URadioButton, label: &str) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 10.0),
                background_color: Color::srgba(0.12, 0.15, 0.2, 0.55),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(row), radio));
    commands.spawn((ChildOf(row), label_node(), text(label, 16.0, Color::WHITE)));
}

fn spawn_radio_card(
    commands: &mut Commands,
    parent: Entity,
    radio: URadioButton,
    title: &str,
    accent: Color,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(155.0),
                padding: USides::all(14.0),
                background_color: accent,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(card), radio));
    commands.spawn((ChildOf(card), label_node(), text(title, 15.0, Color::WHITE)));
}

fn log_toggle_events(mut reader: MessageReader<ToggleChangedEvent>) {
    for event in reader.read() {
        info!(
            "toggle changed: entity={:?} checked={}",
            event.entity, event.checked
        );
    }
}

fn log_radio_events(mut reader: MessageReader<RadioButtonChangedEvent>) {
    for event in reader.read() {
        info!(
            "radio changed: entity={:?} value={} checked={} group_value={:?}",
            event.entity, event.value, event.checked, event.group_value
        );
    }
}

fn label_node() -> UNode {
    UNode {
        background_color: Color::NONE,
        ..default()
    }
}

fn text(value: &str, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: value.to_string(),
        font_size,
        color,
        ..default()
    }
}
