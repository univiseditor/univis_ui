use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                log_text_changes,
                log_text_submits,
                log_select_changes,
                log_drag_changes,
                log_drag_commits,
                log_seekbar_changes,
            ),
        )
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
                width: UVal::Px(1160.0),
                height: UVal::Px(780.0),
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
        text("Widgets: inputs", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Text fields, selects, drag values, and seek bars. Interactions are written to the console.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1112.0),
                height: UVal::Px(660.0),
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

    let left = panel(
        &mut commands,
        content,
        547.0,
        660.0,
        "Text + select widgets",
    );
    let right = panel(&mut commands, content, 547.0, 660.0, "Numeric inputs");

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "Click a field to focus it. Select widgets support mouse and keyboard navigation.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let fields = commands
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
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(fields),
        UTextField::new()
            .with_placeholder("Project name")
            .with_text("Telemetry board")
            .with_size(340.0, 46.0),
    ));
    commands.spawn((
        ChildOf(fields),
        UTextField::new()
            .with_placeholder("Email")
            .input_type(TextFieldInputType::Email)
            .with_size(340.0, 46.0),
    ));
    commands.spawn((
        ChildOf(fields),
        UTextField::new()
            .with_placeholder("PIN")
            .input_type(TextFieldInputType::Number)
            .with_max_length(4)
            .with_size(180.0, 46.0),
    ));
    commands.spawn((
        ChildOf(fields),
        UTextField::new()
            .with_placeholder("Password")
            .input_type(TextFieldInputType::Password)
            .with_size(340.0, 46.0),
    ));

    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    let selects = commands
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

    spawn_select_row(
        &mut commands,
        selects,
        "Quality",
        USelect::new()
            .with_options(vec![
                USelectOption::new("Low", "low"),
                USelectOption::new("Balanced", "balanced"),
                USelectOption::new("High", "high"),
                USelectOption::new("Ultra", "ultra"),
            ])
            .with_selected_value("high")
            .with_size(280.0, 40.0),
    );
    spawn_select_row(
        &mut commands,
        selects,
        "Language",
        USelect::new()
            .with_options(vec![
                USelectOption::new("Rust", "rust"),
                USelectOption::new("C++", "cpp").disabled(),
                USelectOption::new("Go", "go"),
                USelectOption::new("Python", "python"),
            ])
            .with_placeholder("Pick language")
            .with_max_visible_options(4)
            .with_size(280.0, 40.0),
    );

    commands.spawn((
        ChildOf(right),
        label_node(),
        text(
            "Drag values react to horizontal mouse motion. Seek bars map normalized values into configured real ranges.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let drags = commands
        .spawn((
            ChildOf(right),
            UNode {
                width: UVal::Percent(1.0),
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
        .id();

    spawn_drag_row(
        &mut commands,
        drags,
        "Exposure",
        UDragValue::new()
            .with_range(-2.0, 2.0)
            .with_value(0.25)
            .with_step(0.05)
            .with_decimals(2),
    );
    spawn_drag_row(
        &mut commands,
        drags,
        "Zoom",
        UDragValue::new()
            .with_range(10.0, 400.0)
            .with_value(120.0)
            .with_step(5.0)
            .with_decimals(0)
            .with_sensitivity_px(320.0),
    );
    spawn_drag_row(
        &mut commands,
        drags,
        "Temperature",
        UDragValue::new()
            .with_range(16.0, 30.0)
            .with_value(22.5)
            .with_step(0.5)
            .with_decimals(1),
    );

    commands.spawn((ChildOf(right), UDivider::horizontal().with_thickness(2.0)));

    let seekbars = commands
        .spawn((
            ChildOf(right),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    spawn_seekbar_row(
        &mut commands,
        seekbars,
        "Volume",
        USeekBar::volume_style()
            .with_value(0.72)
            .with_range(0.0, 100.0),
    );
    spawn_seekbar_row(
        &mut commands,
        seekbars,
        "Timeline",
        USeekBar::video_style()
            .with_value(0.35)
            .with_range(0.0, 300.0)
            .show_value(),
    );
    spawn_seekbar_row(
        &mut commands,
        seekbars,
        "Brightness",
        USeekBar::brightness_style().with_value(0.58),
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

fn spawn_select_row(commands: &mut Commands, parent: Entity, label: &str, select: USelect) {
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
    commands.spawn((ChildOf(row), select));
}

fn spawn_drag_row(commands: &mut Commands, parent: Entity, label: &str, drag: UDragValue) {
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
    commands.spawn((
        ChildOf(row),
        drag,
        UNode {
            width: UVal::Px(140.0),
            ..default()
        },
    ));
}

fn spawn_seekbar_row(commands: &mut Commands, parent: Entity, label: &str, seekbar: USeekBar) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.12, 0.15, 0.2, 0.6),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(row), label_node(), text(label, 16.0, Color::WHITE)));
    commands.spawn((ChildOf(row), seekbar));
}

fn log_text_changes(mut reader: MessageReader<TextFieldChangedEvent>) {
    for event in reader.read() {
        info!(
            "textfield changed: entity={:?} text={}",
            event.entity, event.text
        );
    }
}

fn log_text_submits(mut reader: MessageReader<TextFieldSubmitEvent>) {
    for event in reader.read() {
        info!(
            "textfield submit: entity={:?} text={}",
            event.entity, event.text
        );
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

fn log_drag_changes(mut reader: MessageReader<DragValueChangedEvent>) {
    for event in reader.read() {
        info!(
            "drag changed: entity={:?} value={} normalized={}",
            event.entity, event.value, event.normalized
        );
    }
}

fn log_drag_commits(mut reader: MessageReader<DragValueCommitEvent>) {
    for event in reader.read() {
        info!(
            "drag commit: entity={:?} value={} normalized={}",
            event.entity, event.value, event.normalized
        );
    }
}

fn log_seekbar_changes(mut reader: MessageReader<SeekBarChangedEvent>) {
    for event in reader.read() {
        info!(
            "seekbar changed: entity={:?} normalized={} real_value={}",
            event.entity, event.value, event.real_value
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
