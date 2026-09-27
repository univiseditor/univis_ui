//! # Interactive Text Input & Caret System Showcase
//!
//! Demonstrates:
//! 1. **Text Input Fields**: Interactive editable fields with responsive focus rings.
//! 2. **Animated Caret**: Blinking cursor synchronized with UTF-8 character boundaries.
//! 3. **Input Modes**: Plaintext vs Masked Password inputs (`*`).
//! 4. **Character Filters**: Numeric-only, Alphanumeric, and Single-line constraints.
//! 5. **Sequential Tab Cycling**: Seamless Tab / Shift+Tab navigation between inputs.
//! 6. **Event Subscriptions**: Real-time [`UTextInputChanged`] and [`UTextInputSubmit`] events.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource, Default)]
struct InputDemoState {
    last_changed_field: String,
    last_changed_value: String,
    last_submitted_field: String,
    last_submitted_value: String,
    active_field: String,
    active_selection: String,
}

#[derive(Component)]
struct FieldIdentifier {
    name: &'static str,
}

#[derive(Component)]
enum TelemetryField {
    ActiveField,
    ActiveSelection,
    LastChange,
    LastSubmit,
}

fn border(color: Color, r: f32, width: f32) -> UBorder {
    UBorder {
        color,
        width,
        radius: UCornerRadius::all(r),
        offset: 0.0,
    }
}

fn spawn_label(
    commands: &mut Commands,
    parent: Entity,
    text: &str,
    color: Color,
    font_size: f32,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            UTextLabel {
                text: text.into(),
                color,
                font_size,
                ..default()
            },
        ))
        .id()
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Interactive Text Input & Caret System".into(),
                    resolution: (1280, 800).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<InputDemoState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_telemetry, update_active_focus))
        .add_observer(on_demo_input_changed)
        .add_observer(on_demo_input_submit)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(32.0),
                background_color: Color::srgb(0.04, 0.05, 0.08),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // 1. Header
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(64.0),
                padding: USides::axes(12.0, 24.0),
                background_color: Color::srgba(0.07, 0.09, 0.14, 0.9),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(Color::srgba(0.18, 0.24, 0.35, 0.7), 8.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let title_col = commands
        .spawn((
            ChildOf(header),
            UNode::default(),
            ULayout {
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        title_col,
        "UNIVIS UI // TEXT INPUT & CARET SYSTEM",
        Color::srgb(0.0, 0.9, 1.0),
        22.0,
    );
    spawn_label(
        &mut commands,
        title_col,
        "Interactive editable fields, UTF-8 cursor navigation, password masking & filters",
        Color::srgb(0.6, 0.68, 0.78),
        13.0,
    );

    let status_badge = commands
        .spawn((
            ChildOf(header),
            UNode {
                padding: USides::axes(14.0, 8.0),
                background_color: Color::srgba(0.0, 0.8, 0.4, 0.15),
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            border(Color::srgb(0.0, 0.8, 0.4), 4.0, 1.0),
        ))
        .id();

    spawn_label(
        &mut commands,
        status_badge,
        "INPUT ENGINE ACTIVE",
        Color::srgb(0.0, 0.9, 0.5),
        12.0,
    );

    // 2. Middle Content: Form Panel & Telemetry Card
    let content = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Start,
                gap: 32.0,
                ..default()
            },
        ))
        .id();

    // Form Container
    let form_panel = commands
        .spawn((
            ChildOf(content),
            UNode {
                width: UVal::Px(580.0),
                padding: USides::all(28.0),
                background_color: Color::srgba(0.06, 0.08, 0.12, 0.95),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            border(Color::srgba(0.18, 0.24, 0.35, 0.8), 12.0, 1.5),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        form_panel,
        "STATION ACCESS PROTOCOL",
        Color::srgb(0.9, 0.92, 0.96),
        18.0,
    );

    // Field 1: Callsign / Username (AlphaNumeric, max 16)
    spawn_form_field(
        &mut commands,
        form_panel,
        "PILOT CALLSIGN",
        "Allowed: A-Z, 0-9 (Max 16 chars)",
        UTextInput::new("Enter callsign (e.g. Maverick)...")
            .with_filter(TextInputFilter::AlphaNumeric)
            .with_max_length(16),
        "Pilot Callsign",
        1,
    );

    // Field 2: Password (Masked)
    spawn_form_field(
        &mut commands,
        form_panel,
        "ENCRYPTION KEY",
        "Characters masked for security",
        UTextInput::new("Enter authorization password...")
            .with_mode(TextInputMode::Password)
            .with_max_length(24),
        "Encryption Key",
        2,
    );

    // Field 3: Port / Frequency (Numeric only)
    spawn_form_field(
        &mut commands,
        form_panel,
        "COMM FREQUENCY",
        "Digits only (0-9, Max 5 digits)",
        UTextInput::new("Enter frequency (e.g. 1420)...")
            .with_filter(TextInputFilter::NumericOnly)
            .with_max_length(5),
        "Comm Frequency",
        3,
    );

    // Field 4: Console Command (Single Line, clears on submit)
    spawn_form_field(
        &mut commands,
        form_panel,
        "ORBITAL COMMAND",
        "Press Enter to transmit (clears on submit)",
        UTextInput::new("Type mission command and press Enter...")
            .with_filter(TextInputFilter::SingleLine)
            .with_clear_on_submit(true),
        "Orbital Command",
        4,
    );

    // Telemetry Panel
    let telemetry_panel = commands
        .spawn((
            ChildOf(content),
            UNode {
                width: UVal::Px(420.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.06, 0.08, 0.12, 0.95),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            border(Color::srgba(0.18, 0.24, 0.35, 0.8), 12.0, 1.5),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        telemetry_panel,
        "INPUT TELEMETRY HUD",
        Color::srgb(0.0, 0.9, 1.0),
        16.0,
    );

    spawn_telemetry_row(
        &mut commands,
        telemetry_panel,
        "ACTIVE FOCUS",
        "None (Click field or press Tab)",
        TelemetryField::ActiveField,
    );
    spawn_telemetry_row(
        &mut commands,
        telemetry_panel,
        "SELECTION RANGE",
        "None (Click or drag to select)",
        TelemetryField::ActiveSelection,
    );
    spawn_telemetry_row(
        &mut commands,
        telemetry_panel,
        "LAST CHANGED",
        "No changes detected",
        TelemetryField::LastChange,
    );
    spawn_telemetry_row(
        &mut commands,
        telemetry_panel,
        "LAST TRANSMISSION",
        "None (Press Enter inside input)",
        TelemetryField::LastSubmit,
    );

    // Instructions Box inside Telemetry
    let guide_box = commands
        .spawn((
            ChildOf(telemetry_panel),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.09, 0.12, 0.18, 0.8),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            border(Color::srgba(0.2, 0.28, 0.4, 0.5), 6.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        guide_box,
        "INPUT & MOUSE CONTROLS",
        Color::srgb(0.85, 0.88, 0.94),
        13.0,
    );
    spawn_label(
        &mut commands,
        guide_box,
        "- Click: Position caret directly at character\n- Drag: Select text range with mouse\n- Double Click: Select word under cursor\n- Tab / Shift+Tab: Cycle between input fields\n- Enter: Submit input event\n- Arrows / Home / End: Move caret (Shift to select)\n- Ctrl + A: Select all text\n- Ctrl + Backspace: Delete whole word",
        Color::srgb(0.65, 0.72, 0.82),
        12.0,
    );

    // 3. Footer Bar
    let footer = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(48.0),
                padding: USides::axes(20.0, 10.0),
                background_color: Color::srgba(0.05, 0.07, 0.1, 0.9),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            border(Color::srgba(0.14, 0.18, 0.28, 0.6), 6.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    spawn_label(
        &mut commands,
        footer,
        "UNIVIS UI ENGINE v0.3.1 // HEADLESS BEVY ECS PRIMITIVES",
        Color::srgb(0.45, 0.52, 0.62),
        12.0,
    );
    spawn_label(
        &mut commands,
        footer,
        "[Click or Tab to Focus] [Enter to Submit] [Esc to Cancel]",
        Color::srgb(0.0, 0.8, 1.0),
        12.0,
    );
}

fn spawn_form_field(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    description: &str,
    input_cfg: UTextInput,
    field_name: &'static str,
    tab_index: i32,
) {
    let field_group = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    let label_row = commands
        .spawn((
            ChildOf(field_group),
            UNode {
                width: UVal::Percent(1.0),
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

    spawn_label(
        commands,
        label_row,
        title,
        Color::srgb(0.85, 0.9, 0.96),
        14.0,
    );
    spawn_label(
        commands,
        label_row,
        description,
        Color::srgb(0.5, 0.58, 0.68),
        11.0,
    );

    let input_entity = spawn_text_input(commands, input_cfg);
    commands
        .entity(input_entity)
        .insert((ChildOf(field_group), FieldIdentifier { name: field_name }));

    // Set tab index for sequential navigation
    commands
        .entity(input_entity)
        .insert(UFocusable::new().with_tab_index(tab_index));
}

fn spawn_telemetry_row(
    commands: &mut Commands,
    parent: Entity,
    header: &str,
    initial: &str,
    field_type: TelemetryField,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.1, 0.15, 0.7),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            border(Color::srgba(0.18, 0.22, 0.32, 0.6), 6.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    spawn_label(commands, card, header, Color::srgb(0.55, 0.62, 0.72), 11.0);

    commands.spawn((
        ChildOf(card),
        UNode::default(),
        UTextLabel {
            text: initial.to_string(),
            font_size: 13.0,
            color: Color::srgb(0.0, 0.9, 1.0),
            ..default()
        },
        field_type,
    ));
}

fn on_demo_input_changed(
    trigger: On<UTextInputChanged>,
    mut state: ResMut<InputDemoState>,
    query: Query<&FieldIdentifier>,
) {
    let name = query.get(trigger.entity).map(|f| f.name).unwrap_or("Input");
    state.last_changed_field = name.to_string();
    state.last_changed_value = trigger.value.clone();
}

fn on_demo_input_submit(
    trigger: On<UTextInputSubmit>,
    mut state: ResMut<InputDemoState>,
    query: Query<&FieldIdentifier>,
) {
    let name = query.get(trigger.entity).map(|f| f.name).unwrap_or("Input");
    state.last_submitted_field = name.to_string();
    state.last_submitted_value = trigger.value.clone();
}

fn update_active_focus(
    focus_state: Res<UFocusState>,
    query: Query<(&FieldIdentifier, &UTextInput)>,
    mut state: ResMut<InputDemoState>,
) {
    if let Some(focused) = focus_state.focused {
        if let Ok((field, input)) = query.get(focused) {
            state.active_field = field.name.to_string();
            if let Some((start, end)) = selection_bounds(input) {
                let selected_text = if start < end && end <= input.value.len() {
                    &input.value[start..end]
                } else {
                    ""
                };
                state.active_selection = format!("[{start}..{end}]: \"{selected_text}\"");
            } else {
                state.active_selection = format!("None (Cursor at byte {})", input.cursor_position);
            }
        } else {
            state.active_field = "Other UI element".to_string();
            state.active_selection = "None".to_string();
        }
    } else {
        state.active_field = "None (Click field or press Tab)".to_string();
        state.active_selection = "None (No focused input)".to_string();
    }
}

fn update_telemetry(
    state: Res<InputDemoState>,
    mut query: Query<(&mut UTextLabel, &TelemetryField)>,
) {
    for (mut label, field) in query.iter_mut() {
        match field {
            TelemetryField::ActiveField => {
                let target = state.active_field.clone();
                if label.text != target {
                    label.text = target;
                }
            }
            TelemetryField::ActiveSelection => {
                let target = state.active_selection.clone();
                if label.text != target {
                    label.text = target;
                }
            }
            TelemetryField::LastChange => {
                let target = if state.last_changed_field.is_empty() {
                    "No changes yet".to_string()
                } else {
                    format!(
                        "{}: \"{}\"",
                        state.last_changed_field, state.last_changed_value
                    )
                };
                if label.text != target {
                    label.text = target;
                }
            }
            TelemetryField::LastSubmit => {
                let target = if state.last_submitted_field.is_empty() {
                    "None (Press Enter inside input)".to_string()
                } else {
                    format!(
                        "Transmitted {}: \"{}\"",
                        state.last_submitted_field, state.last_submitted_value
                    )
                };
                if label.text != target {
                    label.text = target;
                }
            }
        }
    }
}
