use bevy::prelude::*;
use bevy::text::LineBreak;
use univis_ui::prelude::*;

const PANEL_TITLE_COPY: &[&str] = &[
    "Constraint Stress Lab",
    "Constraint Stress Lab With Aggressive Width Pulses",
    "Constraint Stress Lab For Mixed Autosize Rows",
];
const PANEL_SUBTITLE_COPY: &[&str] = &[
    "The center panel pulses between narrow and wide states while labels and widgets keep rotating.",
    "Watch autosize labels, ellipsis, selects, toggles, and nested cards as the available width breathes.",
    "This scene is meant to expose stale measurements, clipped labels, and rows that only account for text.",
];
const QUALITY_ROW_COPY: &[&str] = &[
    "Quality preset with speculative bloom and parallax overlays",
    "Quality preset tuned for cinematic review captures",
    "Quality preset routed through layered diagnostic passes",
];
const THEME_ROW_COPY: &[&str] = &[
    "Theme routing for mixed-density inspector regions",
    "Theme routing for narrow panels with long labels",
    "Theme routing for stacked cards and split telemetry rails",
];
const SYNC_ROW_COPY: &[&str] = &[
    "Enable VSync while temporal accumulation warms the swapchain",
    "Enable VSync while preview rows shrink and recover",
    "Enable VSync while status badges and toggles keep breathing",
];
const EXPORT_ROW_COPY: &[&str] = &[
    "Export overscan and supersample bias for review captures",
    "Export overscan for long-card thumbnails and audit boards",
    "Export overscan with fixed widgets inside pulsing rows",
];
const TAPE_HEADLINE_COPY: &[&str] = &[
    "Telemetry Tape",
    "Telemetry Tape For Post-Settle Visual Inspection",
    "Telemetry Tape For Width Recovery And Text Reflow",
];
const LOG_A_COPY: &[&str] = &[
    "Select trigger recovered from a narrow state and expanded back to the full inspector width.",
    "Constraint row kept the widget visible while the label shrank into a single ellipsized line.",
    "Nested panel reflow finished in the same frame after the pulsing width crossed its midpoint.",
];
const LOG_B_COPY: &[&str] = &[
    "Toggle track stayed centered after the label length changed and the row settled again.",
    "Status strip kept both badges and value cells aligned after a long subtitle reclaimed space.",
    "Meter labels remeasured after parent bounds changed instead of holding onto stale truncation.",
];
const LOG_C_COPY: &[&str] = &[
    "Grid cells maintained equal widths while the enclosing panel kept oscillating.",
    "Preview badges stayed readable without overlapping the action cluster below them.",
    "Inspector cards continued to respect content sizing after repeated width pulses.",
];
const PREVIEW_CAPTION_COPY: &[&str] = &[
    "Preview ribbon balancing dense labels against fixed controls",
    "Preview ribbon checking ellipsis recovery after parent growth",
    "Preview ribbon verifying nested rows after repeated pulses",
];

#[derive(Component)]
struct PulsingWidth {
    min: f32,
    max: f32,
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct StressPanel;

#[derive(Component, Clone, Copy)]
struct CyclingCopy {
    kind: CopyKind,
    speed: f32,
    phase: f32,
}

#[derive(Clone, Copy)]
enum CopyKind {
    PanelTitle,
    PanelSubtitle,
    QualityRow,
    ThemeRow,
    SyncRow,
    ExportRow,
    TapeHeadline,
    LogA,
    LogB,
    LogC,
    PreviewCaption,
}

#[derive(Component)]
struct CyclingToggle {
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct CyclingSelect {
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct CyclingProgress {
    mid: f32,
    amp: f32,
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct CyclingDrag {
    min: f32,
    max: f32,
    speed: f32,
    phase: f32,
}

#[derive(Component)]
struct LiveReadout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis Visual Regression Lab".to_string(),
                resolution: [1440, 920].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .insert_resource(UiSettlementConfig { max_iterations: 16 })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_pulsing_widths,
                animate_cycling_copy,
                animate_cycling_toggles,
                animate_cycling_selects,
                animate_cycling_progress,
                animate_cycling_drag_values,
                update_live_readout,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                background_color: Color::srgb(0.05, 0.06, 0.1),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Stretch,
                gap: 20.0,
                ..default()
            },
        ))
        .with_children(|root| {
            spawn_manual_panel(root);
            spawn_stress_panel(root);
            spawn_telemetry_tape(root);
        });
}

fn spawn_manual_panel(root: &mut ChildSpawnerCommands) {
    root.spawn((
        UPanel::glass().with_gap(12.0),
        UNode {
            width: UVal::Px(380.0),
            height: UVal::Percent(1.0),
            ..default()
        },
    ))
    .with_children(|panel| {
        panel.spawn(UTextLabel {
            text: "Manual Comparison".to_string(),
            font_size: 28.0,
            color: Color::WHITE,
            ..default()
        });
        panel.spawn(UTextLabel {
            text: "Click these controls manually, then compare them against the automated pulse lab in the center."
                .to_string(),
            font_size: 14.0,
            color: Color::srgb(0.72, 0.78, 0.9),
            linebreak: LineBreak::WordBoundary,
            autosize: false,
            ..default()
        });
        panel.spawn(UDivider::horizontal().with_thickness(1.0));

        spawn_badge_strip(panel, &["Manual", "Stable Width", "Reference"]);

        spawn_select_row(
            panel,
            "Quality preset",
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Low", "low"),
                    USelectOption::new("Medium", "medium"),
                    USelectOption::new("High", "high"),
                    USelectOption::new("Ultra", "ultra"),
                ])
                .with_selected_value("high")
                .with_size(210.0, 38.0),
            None,
            None,
        );
        spawn_select_row(
            panel,
            "Theme routing",
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Nebula", "nebula"),
                    USelectOption::new("Slate", "slate"),
                    USelectOption::new("Signal", "signal"),
                    USelectOption::new("Graphite", "graphite"),
                ])
                .with_selected_value("signal")
                .with_size(210.0, 38.0),
            None,
            None,
        );
        spawn_toggle_row(
            panel,
            "Enable VSync",
            UToggle::material_style().with_checked(true),
            None,
            None,
        );
        spawn_drag_row(
            panel,
            "Exposure compensation",
            UDragValue::new()
                .with_range(0.0, 5.0)
                .with_value(1.35)
                .with_step(0.01)
                .with_decimals(2),
            None,
            None,
        );
        spawn_drag_row(
            panel,
            "Overscan bias",
            UDragValue::new()
                .with_range(0.0, 2.0)
                .with_value(0.42)
                .with_step(0.01)
                .with_decimals(2),
            None,
            None,
        );

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        panel.spawn(UTextLabel {
            text: "Manual Meters".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });
        spawn_meter(panel, "CPU Budget", Color::srgb(0.21, 0.78, 0.44), 0.58, Some(0.0));
        spawn_meter(panel, "GPU Budget", Color::srgb(0.28, 0.54, 0.94), 0.44, Some(1.2));
        spawn_meter(panel, "Clip Cache", Color::srgb(0.93, 0.68, 0.24), 0.71, Some(2.1));

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        panel.spawn(UTextLabel {
            text: "Notes".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });
        panel.spawn(UTextLabel {
            text: "The center panel keeps changing width and text density. If a label stays truncated after the panel grows again, that is a visual regression."
                .to_string(),
            font_size: 14.0,
            color: Color::srgb(0.72, 0.78, 0.9),
            linebreak: LineBreak::WordBoundary,
            autosize: false,
            ..default()
        });
    });
}

fn spawn_stress_panel(root: &mut ChildSpawnerCommands) {
    root.spawn((
        UPanel::glass().with_gap(12.0),
        StressPanel,
        PulsingWidth {
            min: 340.0,
            max: 540.0,
            speed: 0.38,
            phase: 0.0,
        },
        UNode {
            width: UVal::Px(480.0),
            height: UVal::Percent(1.0),
            ..default()
        },
    ))
    .with_children(|panel| {
        panel.spawn((
            UTextLabel {
                text: PANEL_TITLE_COPY[0].to_string(),
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
            CyclingCopy {
                kind: CopyKind::PanelTitle,
                speed: 0.45,
                phase: 0.0,
            },
        ));
        panel.spawn((
            UTextLabel {
                text: PANEL_SUBTITLE_COPY[0].to_string(),
                font_size: 14.0,
                color: Color::srgb(0.72, 0.78, 0.9),
                linebreak: LineBreak::WordBoundary,
                autosize: false,
                ..default()
            },
            CyclingCopy {
                kind: CopyKind::PanelSubtitle,
                speed: 0.35,
                phase: 0.8,
            },
        ));
        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        spawn_badge_strip(panel, &["Pulse Width", "Autosize", "Widget Mix"]);

        spawn_select_row(
            panel,
            QUALITY_ROW_COPY[0],
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Crisp", "crisp"),
                    USelectOption::new("Balanced", "balanced"),
                    USelectOption::new("Dense", "dense"),
                    USelectOption::new("Audit", "audit"),
                ])
                .with_selected_value("dense")
                .with_size(218.0, 38.0),
            Some(CyclingCopy {
                kind: CopyKind::QualityRow,
                speed: 0.30,
                phase: 0.1,
            }),
            Some(CyclingSelect {
                speed: 0.7,
                phase: 0.0,
            }),
        );
        spawn_select_row(
            panel,
            THEME_ROW_COPY[0],
            USelect::new()
                .with_options(vec![
                    USelectOption::new("Signal", "signal"),
                    USelectOption::new("Nebula", "nebula"),
                    USelectOption::new("Monochrome", "mono"),
                    USelectOption::new("Forest", "forest"),
                ])
                .with_selected_value("signal")
                .with_size(218.0, 38.0),
            Some(CyclingCopy {
                kind: CopyKind::ThemeRow,
                speed: 0.33,
                phase: 0.7,
            }),
            Some(CyclingSelect {
                speed: 0.55,
                phase: 1.2,
            }),
        );
        spawn_toggle_row(
            panel,
            SYNC_ROW_COPY[0],
            UToggle::material_style().with_checked(true),
            Some(CyclingCopy {
                kind: CopyKind::SyncRow,
                speed: 0.28,
                phase: 1.4,
            }),
            Some(CyclingToggle {
                speed: 1.35,
                phase: 0.0,
            }),
        );
        spawn_drag_row(
            panel,
            EXPORT_ROW_COPY[0],
            UDragValue::new()
                .with_range(0.0, 3.0)
                .with_value(1.1)
                .with_step(0.01)
                .with_decimals(2),
            Some(CyclingCopy {
                kind: CopyKind::ExportRow,
                speed: 0.31,
                phase: 2.1,
            }),
            Some(CyclingDrag {
                min: 0.2,
                max: 2.7,
                speed: 0.85,
                phase: 0.4,
            }),
        );

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        spawn_preview_cluster(panel);
    });
}

fn spawn_telemetry_tape(root: &mut ChildSpawnerCommands) {
    root.spawn((
        UPanel::glass().with_gap(12.0),
        UNode {
            width: UVal::Px(300.0),
            height: UVal::Percent(1.0),
            ..default()
        },
    ))
    .with_children(|panel| {
        panel.spawn((
            UTextLabel {
                text: TAPE_HEADLINE_COPY[0].to_string(),
                font_size: 26.0,
                color: Color::WHITE,
                ..default()
            },
            CyclingCopy {
                kind: CopyKind::TapeHeadline,
                speed: 0.42,
                phase: 0.0,
            },
        ));
        panel.spawn((
            UTextLabel {
                text: "pulse width: 480px | monitor labels recovering after expansion".to_string(),
                font_size: 13.0,
                color: Color::srgb(0.71, 0.79, 0.92),
                linebreak: LineBreak::WordBoundary,
                autosize: false,
                ..default()
            },
            LiveReadout,
        ));
        panel.spawn(UDivider::horizontal().with_thickness(1.0));

        spawn_log_row(
            panel,
            LOG_A_COPY[0],
            BadgeStyle::Info,
            Some(CyclingCopy {
                kind: CopyKind::LogA,
                speed: 0.36,
                phase: 0.0,
            }),
        );
        spawn_log_row(
            panel,
            LOG_B_COPY[0],
            BadgeStyle::Success,
            Some(CyclingCopy {
                kind: CopyKind::LogB,
                speed: 0.41,
                phase: 0.7,
            }),
        );
        spawn_log_row(
            panel,
            LOG_C_COPY[0],
            BadgeStyle::Warning,
            Some(CyclingCopy {
                kind: CopyKind::LogC,
                speed: 0.38,
                phase: 1.3,
            }),
        );

        panel.spawn(UDivider::horizontal().with_thickness(1.0));
        panel.spawn(UTextLabel {
            text: "Continuous Meters".to_string(),
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        });
        spawn_meter(
            panel,
            "Settle Throughput",
            Color::srgb(0.23, 0.56, 0.95),
            0.52,
            Some(0.2),
        );
        spawn_meter(
            panel,
            "Text Reflow",
            Color::srgb(0.93, 0.68, 0.24),
            0.63,
            Some(1.3),
        );
        spawn_meter(
            panel,
            "Widget Sync",
            Color::srgb(0.19, 0.78, 0.4),
            0.47,
            Some(2.0),
        );
    });
}

fn spawn_preview_cluster(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(360.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.14, 0.17, 0.24, 0.76),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|cluster| {
            cluster.spawn(UTextLabel {
                text: "Nested Preview Cluster".to_string(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            });
            spawn_badge_strip(cluster, &["Grid", "Stack", "Pulse"]);

            cluster
                .spawn((
                    UNode {
                        width: UVal::Px(320.0),
                        height: UVal::Auto,
                        padding: USides::all(10.0),
                        background_color: Color::srgba(0.18, 0.21, 0.31, 0.8),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                    PulsingWidth {
                        min: 240.0,
                        max: 420.0,
                        speed: 0.64,
                        phase: 1.2,
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                        gap: 12.0,
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        UTextLabel {
                            text: PREVIEW_CAPTION_COPY[0].to_string(),
                            font_size: 15.0,
                            color: Color::WHITE,
                            ..default()
                        },
                        CyclingCopy {
                            kind: CopyKind::PreviewCaption,
                            speed: 0.39,
                            phase: 0.4,
                        },
                    ));
                    row.spawn((
                        UToggle::material_style().with_checked(true),
                        CyclingToggle {
                            speed: 1.1,
                            phase: 0.8,
                        },
                    ));
                });

            cluster
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Grid,
                        grid_columns: 2,
                        gap: 10.0,
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    spawn_metric_card(
                        grid,
                        "Measure",
                        "Bounds update after parent width changes",
                        Color::srgb(0.23, 0.56, 0.95),
                        0.56,
                        0.0,
                    );
                    spawn_metric_card(
                        grid,
                        "Solve",
                        "Nested containers keep requeueing on the active generation",
                        Color::srgb(0.19, 0.78, 0.4),
                        0.49,
                        1.2,
                    );
                    spawn_metric_card(
                        grid,
                        "Render",
                        "Text cards should recover from truncation once width comes back",
                        Color::srgb(0.93, 0.68, 0.24),
                        0.63,
                        2.1,
                    );
                    spawn_metric_card(
                        grid,
                        "Inspect",
                        "Click the manual panel while this center cluster keeps breathing",
                        Color::srgb(0.82, 0.43, 0.92),
                        0.41,
                        2.8,
                    );
                });
        });
}

fn spawn_metric_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    caption: &str,
    color: Color,
    initial_value: f32,
    phase: f32,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgba(0.12, 0.15, 0.22, 0.82),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: title.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            card.spawn(UTextLabel {
                text: caption.to_string(),
                font_size: 12.0,
                color: Color::srgb(0.72, 0.78, 0.9),
                linebreak: LineBreak::WordBoundary,
                autosize: false,
                ..default()
            });
            spawn_meter(card, "Pulse", color, initial_value, Some(phase));
        });
}

fn spawn_log_row(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    style: BadgeStyle,
    cycling_copy: Option<CyclingCopy>,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::all(10.0),
                background_color: Color::srgba(0.16, 0.18, 0.24, 0.78),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn((
                UBadge {
                    style,
                    size: BadgeSize::Small,
                },
                UNode::default(),
                ULayout::default(),
            ))
            .with_children(|badge| {
                badge.spawn(UTextLabel {
                    text: "live".to_string(),
                    font_size: 11.0,
                    color: Color::WHITE,
                    ..default()
                });
            });

            let mut entity = row.spawn(UTextLabel {
                text: text.to_string(),
                font_size: 13.0,
                color: Color::srgb(0.86, 0.9, 0.97),
                linebreak: LineBreak::WordBoundary,
                autosize: false,
                ..default()
            });
            if let Some(cycling_copy) = cycling_copy {
                entity.insert(cycling_copy);
            }
        });
}

fn spawn_meter(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    color: Color,
    initial_value: f32,
    phase: Option<f32>,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::all(10.0),
                background_color: Color::srgba(0.16, 0.18, 0.24, 0.74),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 13.0,
                color: Color::WHITE,
                ..default()
            });
            let mut meter = card.spawn((UProgressBar {
                value: initial_value,
                bar_color: color,
            },));
            if let Some(phase) = phase {
                meter.insert(CyclingProgress {
                    mid: initial_value,
                    amp: 0.22,
                    speed: 0.9,
                    phase,
                });
            }
        });
}

fn spawn_select_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    select: USelect,
    cycling_copy: Option<CyclingCopy>,
    cycling_select: Option<CyclingSelect>,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            let mut label_entity = row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            if let Some(cycling_copy) = cycling_copy {
                label_entity.insert(cycling_copy);
            }

            let mut select_entity = row.spawn(select);
            if let Some(cycling_select) = cycling_select {
                select_entity.insert(cycling_select);
            }
        });
}

fn spawn_toggle_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    toggle: UToggle,
    cycling_copy: Option<CyclingCopy>,
    cycling_toggle: Option<CyclingToggle>,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            let mut label_entity = row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            if let Some(cycling_copy) = cycling_copy {
                label_entity.insert(cycling_copy);
            }

            let mut toggle_entity = row.spawn(toggle);
            if let Some(cycling_toggle) = cycling_toggle {
                toggle_entity.insert(cycling_toggle);
            }
        });
}

fn spawn_drag_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    drag: UDragValue,
    cycling_copy: Option<CyclingCopy>,
    cycling_drag: Option<CyclingDrag>,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                padding: USides::axes(12.0, 8.0),
                background_color: Color::srgba(0.18, 0.21, 0.27, 0.72),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .with_children(|row| {
            let mut label_entity = row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            if let Some(cycling_copy) = cycling_copy {
                label_entity.insert(cycling_copy);
            }

            let mut drag_entity = row.spawn((
                drag,
                UNode {
                    width: UVal::Px(170.0),
                    ..default()
                },
            ));
            if let Some(cycling_drag) = cycling_drag {
                drag_entity.insert(cycling_drag);
            }
        });
}

fn spawn_badge_strip(parent: &mut ChildSpawnerCommands, labels: &[&str]) {
    parent
        .spawn((
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
        .with_children(|row| {
            for (index, label) in labels.iter().enumerate() {
                let style = match index % 3 {
                    0 => BadgeStyle::Info,
                    1 => BadgeStyle::Success,
                    _ => BadgeStyle::Warning,
                };
                row.spawn((
                    UBadge {
                        style,
                        size: BadgeSize::Small,
                    },
                    UNode::default(),
                    ULayout::default(),
                ))
                .with_children(|badge| {
                    badge.spawn(UTextLabel {
                        text: (*label).to_string(),
                        font_size: 11.0,
                        color: Color::WHITE,
                        ..default()
                    });
                });
            }
        });
}

fn animate_pulsing_widths(time: Res<Time>, mut query: Query<(&PulsingWidth, &mut UNode)>) {
    let t = time.elapsed_secs();
    for (pulse, mut node) in query.iter_mut() {
        node.width = UVal::Px(pulse_value(
            pulse.min,
            pulse.max,
            pulse.speed,
            pulse.phase,
            t,
        ));
    }
}

fn animate_cycling_copy(time: Res<Time>, mut query: Query<(&CyclingCopy, &mut UTextLabel)>) {
    let t = time.elapsed_secs();
    for (cycling, mut label) in query.iter_mut() {
        let options = copy_options(cycling.kind);
        let index = cycle_index(options.len(), t, cycling.speed, cycling.phase);
        let next = options[index];
        if label.text != next {
            label.text = next.to_string();
        }
    }
}

fn animate_cycling_toggles(time: Res<Time>, mut query: Query<(&CyclingToggle, &mut UToggle)>) {
    let t = time.elapsed_secs();
    for (cycling, mut toggle) in query.iter_mut() {
        let next = (t * cycling.speed + cycling.phase).sin() > 0.0;
        if toggle.checked != next {
            toggle.checked = next;
        }
    }
}

fn animate_cycling_selects(time: Res<Time>, mut query: Query<(&CyclingSelect, &mut USelect)>) {
    let t = time.elapsed_secs();
    for (cycling, mut select) in query.iter_mut() {
        if select.options.is_empty() {
            continue;
        }
        let index = cycle_index(select.options.len(), t, cycling.speed, cycling.phase);
        let next = Some(index);
        if select.selected_index != next {
            select.selected_index = next;
            select.highlighted_index = next;
        }
    }
}

fn animate_cycling_progress(
    time: Res<Time>,
    mut query: Query<(&CyclingProgress, &mut UProgressBar)>,
) {
    let t = time.elapsed_secs();
    for (cycling, mut bar) in query.iter_mut() {
        let wave = (t * cycling.speed + cycling.phase).sin();
        bar.value = (cycling.mid + cycling.amp * wave).clamp(0.0, 1.0);
    }
}

fn animate_cycling_drag_values(time: Res<Time>, mut query: Query<(&CyclingDrag, &mut UDragValue)>) {
    let t = time.elapsed_secs();
    for (cycling, mut drag) in query.iter_mut() {
        let wave = ((t * cycling.speed) + cycling.phase).sin() * 0.5 + 0.5;
        drag.value = cycling.min + (cycling.max - cycling.min) * wave;
    }
}

fn update_live_readout(
    time: Res<Time>,
    stress_panels: Query<&UNode, With<StressPanel>>,
    mut labels: Query<&mut UTextLabel, With<LiveReadout>>,
) {
    let width = stress_panels
        .iter()
        .find_map(|node| match node.width {
            UVal::Px(value) => Some(value),
            _ => None,
        })
        .unwrap_or(0.0);

    for mut label in labels.iter_mut() {
        let next = format!(
            "pulse width: {width:.0}px | t={:.1}s | look for rows that recover after expansion",
            time.elapsed_secs()
        );
        if label.text != next {
            label.text = next;
        }
    }
}

fn pulse_value(min: f32, max: f32, speed: f32, phase: f32, t: f32) -> f32 {
    let wave = (t * speed + phase).sin() * 0.5 + 0.5;
    min + (max - min) * wave
}

fn cycle_index(len: usize, t: f32, speed: f32, phase: f32) -> usize {
    ((t * speed + phase).floor() as usize) % len.max(1)
}

fn copy_options(kind: CopyKind) -> &'static [&'static str] {
    match kind {
        CopyKind::PanelTitle => PANEL_TITLE_COPY,
        CopyKind::PanelSubtitle => PANEL_SUBTITLE_COPY,
        CopyKind::QualityRow => QUALITY_ROW_COPY,
        CopyKind::ThemeRow => THEME_ROW_COPY,
        CopyKind::SyncRow => SYNC_ROW_COPY,
        CopyKind::ExportRow => EXPORT_ROW_COPY,
        CopyKind::TapeHeadline => TAPE_HEADLINE_COPY,
        CopyKind::LogA => LOG_A_COPY,
        CopyKind::LogB => LOG_B_COPY,
        CopyKind::LogC => LOG_C_COPY,
        CopyKind::PreviewCaption => PREVIEW_CAPTION_COPY,
    }
}
