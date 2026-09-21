//! # Scroll Containers & Interactive Spring Transitions Showcase
//!
//! Demonstrates:
//! 1. **`UScrollContainer` & `UScrollContent`**: Smooth mouse wheel scrolling with momentum
//!    and exponential smoothing, live extents synchronization, and edge bounds.
//! 2. **Hardware Rounded SDF Clipping (`UClip`)**: Content smoothly and precisely clips to the
//!    container's rounded boundaries in GPU fragment shader.
//! 3. **`UTransition` & `UInteractionScale`**: Spring-driven scale zoom on hover/press with
//!    configurable bounce physics and zero downward layout recalculation.
//! 4. **Live Controls**: Programmatic scroll jumping, top/bottom navigation, and dynamic item additions.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct DemoState {
    item_counter: usize,
    snappy_mode: bool,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            item_counter: 6,
            snappy_mode: false,
        }
    }
}

#[derive(Component)]
struct FeedContainer;

#[derive(Component)]
struct FeedContentRoot;

#[derive(Component)]
enum ControlButton {
    ScrollTop,
    ScrollBottom,
    ScrollStepDown,
    ScrollStepUp,
    ToggleSmoothness,
    AddItem,
}

#[derive(Component)]
struct ProgressBarFill;

#[derive(Component)]
enum StatLabel {
    Offset,
    Progress,
    Smoothness,
    ItemCount,
}

fn border(color: Color, r: f32, width: f32) -> UBorder {
    UBorder {
        color,
        width,
        radius: UCornerRadius::all(r),
        offset: 0.0,
    }
}

fn flex_box(dir: UFlexDirection, gap: f32) -> ULayout {
    ULayout {
        display: UDisplay::Flex,
        flex_direction: dir,
        gap,
        ..default()
    }
}

fn flex_row_between() -> ULayout {
    ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Row,
        justify_content: UJustifyContent::SpaceBetween,
        align_items: UAlignItems::Center,
        ..default()
    }
}

fn spawn_text(commands: &mut Commands, parent: Entity, text: &str, color: Color, font_size: f32) {
    commands.spawn((
        ChildOf(parent),
        UNode::default(),
        UTextLabel {
            text: text.into(),
            color,
            font_size,
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Scroll Containers & Interactive Transitions".into(),
                    resolution: (1260, 880).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<DemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_control_clicks, update_scroll_diagnostics))
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
                padding: USides::all(24.0),
                background_color: Color::srgb(0.04, 0.05, 0.09),
                ..default()
            },
            flex_box(UFlexDirection::Row, 28.0),
        ))
        .id();

    // LEFT COLUMN: Scrollable Feed Viewport
    let left_col = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(640.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Column, 12.0),
        ))
        .id();

    let header = commands
        .spawn((
            ChildOf(left_col),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Column, 4.0),
        ))
        .id();

    spawn_text(
        &mut commands,
        header,
        "INTERACTIVE FEED VIEWPORT",
        Color::srgb(0.0, 0.9, 1.0),
        22.0,
    );
    spawn_text(
        &mut commands,
        header,
        "Scroll with mouse wheel. Hover cards for physics spring scale zoom.",
        Color::srgb(0.6, 0.65, 0.75),
        13.0,
    );

    let viewport = commands
        .spawn((
            ChildOf(left_col),
            FeedContainer,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(660.0),
                border_radius: UCornerRadius::all(16.0),
                background_color: Color::srgb(0.07, 0.09, 0.14),
                ..default()
            },
            border(Color::srgb(0.18, 0.22, 0.32), 16.0, 1.5),
            UScrollContainer::vertical().with_smoothness(16.0),
            UClip { enabled: true },
            UInteraction::default(),
        ))
        .id();

    let feed_content = commands
        .spawn((
            ChildOf(viewport),
            FeedContentRoot,
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                ..default()
            },
            flex_box(UFlexDirection::Column, 14.0),
            UScrollContent::default(),
        ))
        .id();

    spawn_initial_cards(&mut commands, feed_content);

    // RIGHT COLUMN: Diagnostics & Controls Panel
    let right_col = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(490.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Column, 14.0),
        ))
        .id();

    let r_header = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Column, 4.0),
        ))
        .id();

    spawn_text(
        &mut commands,
        r_header,
        "VIEWPORT CONTROLS & TELEMETRY",
        Color::srgb(1.0, 0.8, 0.2),
        20.0,
    );
    spawn_text(
        &mut commands,
        r_header,
        "Live inspection of UScrollContainer & UInteractionScale",
        Color::srgb(0.6, 0.65, 0.75),
        13.0,
    );

    // Telemetry Card
    let telemetry = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                border_radius: UCornerRadius::all(14.0),
                background_color: Color::srgb(0.08, 0.10, 0.16),
                ..default()
            },
            border(Color::srgb(0.18, 0.22, 0.32), 14.0, 1.0),
            flex_box(UFlexDirection::Column, 10.0),
        ))
        .id();

    spawn_text(
        &mut commands,
        telemetry,
        "SCROLL EXTENTS & PROGRESS",
        Color::srgb(0.9, 0.95, 1.0),
        14.0,
    );

    let track = commands
        .spawn((
            ChildOf(telemetry),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(10.0),
                border_radius: UCornerRadius::all(5.0),
                background_color: Color::srgb(0.12, 0.15, 0.22),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(track),
        ProgressBarFill,
        UNode {
            width: UVal::Percent(0.0),
            height: UVal::Percent(1.0),
            border_radius: UCornerRadius::all(5.0),
            background_color: Color::srgb(0.0, 0.85, 1.0),
            ..default()
        },
    ));

    spawn_stat_row(
        &mut commands,
        telemetry,
        "Offset:",
        "0.0 / 0.0 px",
        StatLabel::Offset,
    );
    spawn_stat_row(
        &mut commands,
        telemetry,
        "Progress:",
        "0.0%",
        StatLabel::Progress,
    );
    spawn_stat_row(
        &mut commands,
        telemetry,
        "Smoothness:",
        "16.0 (Glide)",
        StatLabel::Smoothness,
    );
    spawn_stat_row(
        &mut commands,
        telemetry,
        "Feed Items:",
        "6 Cards",
        StatLabel::ItemCount,
    );

    // Navigation Controls Card
    let controls = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                border_radius: UCornerRadius::all(14.0),
                background_color: Color::srgb(0.08, 0.10, 0.16),
                ..default()
            },
            border(Color::srgb(0.18, 0.22, 0.32), 14.0, 1.0),
            flex_box(UFlexDirection::Column, 10.0),
        ))
        .id();

    spawn_text(
        &mut commands,
        controls,
        "NAVIGATION COMMANDS",
        Color::srgb(0.9, 0.95, 1.0),
        14.0,
    );

    let row1 = commands
        .spawn((
            ChildOf(controls),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Row, 10.0),
        ))
        .id();
    spawn_button(
        &mut commands,
        row1,
        "▲ Top",
        ControlButton::ScrollTop,
        Color::srgb(0.15, 0.22, 0.35),
    );
    spawn_button(
        &mut commands,
        row1,
        "▼ Bottom",
        ControlButton::ScrollBottom,
        Color::srgb(0.15, 0.22, 0.35),
    );

    let row2 = commands
        .spawn((
            ChildOf(controls),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Row, 10.0),
        ))
        .id();
    spawn_button(
        &mut commands,
        row2,
        "↓ Glide +160px",
        ControlButton::ScrollStepDown,
        Color::srgb(0.18, 0.24, 0.32),
    );
    spawn_button(
        &mut commands,
        row2,
        "↑ Glide -160px",
        ControlButton::ScrollStepUp,
        Color::srgb(0.18, 0.24, 0.32),
    );

    let row3 = commands
        .spawn((
            ChildOf(controls),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_box(UFlexDirection::Row, 10.0),
        ))
        .id();
    spawn_button(
        &mut commands,
        row3,
        "⚡ Toggle Speed",
        ControlButton::ToggleSmoothness,
        Color::srgb(0.24, 0.18, 0.35),
    );
    spawn_button(
        &mut commands,
        row3,
        "➕ Add Card",
        ControlButton::AddItem,
        Color::srgb(0.14, 0.28, 0.22),
    );

    // Technical Architecture Info Card
    let info = commands
        .spawn((
            ChildOf(right_col),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                border_radius: UCornerRadius::all(12.0),
                background_color: Color::srgb(0.06, 0.07, 0.11),
                ..default()
            },
            border(Color::srgb(0.14, 0.18, 0.26), 12.0, 1.0),
            flex_box(UFlexDirection::Column, 6.0),
        ))
        .id();

    spawn_text(
        &mut commands,
        info,
        "ARCHITECTURE HIGHLIGHTS",
        Color::srgb(0.0, 0.85, 0.65),
        13.0,
    );
    spawn_text(
        &mut commands,
        info,
        "• UScrollContainer moves Transform.translation with zero layout re-solves.",
        Color::srgb(0.65, 0.70, 0.80),
        11.5,
    );
    spawn_text(
        &mut commands,
        info,
        "• UClip provides GPU rounded SDF clipping for both rendering and picking.",
        Color::srgb(0.65, 0.70, 0.80),
        11.5,
    );
    spawn_text(
        &mut commands,
        info,
        "• UInteractionScale drives UTransition physics spring zoom automatically.",
        Color::srgb(0.65, 0.70, 0.80),
        11.5,
    );
}

fn spawn_stat_row(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    initial_val: &str,
    tag: StatLabel,
) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_row_between(),
        ))
        .id();
    spawn_text(commands, row, label, Color::srgb(0.55, 0.60, 0.70), 12.5);
    commands.spawn((
        ChildOf(row),
        tag,
        UNode::default(),
        UTextLabel {
            text: initial_val.into(),
            color: Color::srgb(0.90, 0.95, 1.0),
            font_size: 12.5,
            ..default()
        },
    ));
}

fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    tag: ControlButton,
    bg_color: Color,
) {
    let btn = commands
        .spawn((
            ChildOf(parent),
            tag,
            UNode {
                width: UVal::Percent(0.5),
                height: UVal::Px(38.0),
                border_radius: UCornerRadius::all(8.0),
                background_color: bg_color,
                ..default()
            },
            border(Color::srgb(0.28, 0.35, 0.48), 8.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UInteractionColors {
                normal: bg_color,
                hovered: Color::srgb(
                    (bg_color.to_srgba().red + 0.08).min(1.0),
                    (bg_color.to_srgba().green + 0.08).min(1.0),
                    (bg_color.to_srgba().blue + 0.08).min(1.0),
                ),
                pressed: Color::srgb(
                    (bg_color.to_srgba().red - 0.05).max(0.0),
                    (bg_color.to_srgba().green - 0.05).max(0.0),
                    (bg_color.to_srgba().blue - 0.05).max(0.0),
                ),
            },
            UInteractionScale::bouncy(),
            UTransition::bouncy(300.0, 0.7),
            UInteraction::default(),
        ))
        .id();

    spawn_text(commands, btn, label, Color::WHITE, 12.5);
}

fn spawn_feed_card(
    commands: &mut Commands,
    parent: Entity,
    index: usize,
    title: &str,
    desc: &str,
    gradient: UGradient,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(120.0),
                padding: USides::all(16.0),
                border_radius: UCornerRadius::all(12.0),
                background_color: Color::srgb(0.10, 0.12, 0.18),
                ..default()
            },
            border(Color::srgb(0.20, 0.25, 0.36), 12.0, 1.2),
            gradient,
            flex_box(UFlexDirection::Column, 8.0),
            UInteractionColors {
                normal: Color::srgb(0.10, 0.12, 0.18),
                hovered: Color::srgb(0.14, 0.17, 0.26),
                pressed: Color::srgb(0.08, 0.10, 0.15),
            },
            UInteractionScale::bouncy(),
            UTransition::bouncy(260.0, 0.65),
            UInteraction::default(),
        ))
        .id();

    let top_row = commands
        .spawn((
            ChildOf(card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            flex_row_between(),
        ))
        .id();
    spawn_text(
        commands,
        top_row,
        &format!("#{index:02} {title}"),
        Color::WHITE,
        14.5,
    );

    let badge = commands
        .spawn((
            ChildOf(top_row),
            UNode {
                padding: USides::axes(3.0, 8.0),
                border_radius: UCornerRadius::all(6.0),
                background_color: Color::srgba(0.0, 0.8, 1.0, 0.15),
                ..default()
            },
        ))
        .id();
    spawn_text(
        commands,
        badge,
        "SPRING SCALE",
        Color::srgb(0.0, 0.85, 1.0),
        10.0,
    );

    spawn_text(commands, card, desc, Color::srgb(0.70, 0.75, 0.85), 12.5);

    let bottom_row = commands
        .spawn((
            ChildOf(card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::End,
                ..default()
            },
        ))
        .id();
    spawn_text(
        commands,
        bottom_row,
        "Hover to zoom 1.10x",
        Color::srgb(0.45, 0.50, 0.60),
        11.0,
    );
}

fn spawn_initial_cards(commands: &mut Commands, parent: Entity) {
    let cards = [
        (
            "CYBERPUNK NEON GLIDE",
            "Multi-stop linear gradient transitioning cyan to violet.",
            UGradient::linear_stops(
                0.785,
                [
                    (0.0, Color::srgb(0.0, 0.8, 1.0)),
                    (0.4, Color::srgb(0.2, 0.1, 0.4)),
                    (1.0, Color::srgb(0.7, 0.1, 0.8)),
                ],
            ),
        ),
        (
            "EMERALD QUANTUM MATRIX",
            "Precision smooth gradient with green-tinted phosphor glow.",
            UGradient::linear_stops(
                1.57,
                [
                    (0.0, Color::srgb(0.0, 0.85, 0.5)),
                    (0.5, Color::srgb(0.05, 0.20, 0.15)),
                    (1.0, Color::srgb(0.0, 0.4, 0.3)),
                ],
            ),
        ),
        (
            "SOLAR FLARE SUNSET",
            "Dominant warm amber fading into deep twilight purple.",
            UGradient::linear_stops(
                0.35,
                [
                    (0.0, Color::srgb(1.0, 0.65, 0.1)),
                    (0.6, Color::srgb(0.8, 0.2, 0.4)),
                    (1.0, Color::srgb(0.2, 0.05, 0.3)),
                ],
            ),
        ),
        (
            "DEEP OCEANIC TRENCH",
            "Subtle aquatic depth with crisp cyan edge highlights.",
            UGradient::linear_stops(
                2.35,
                [
                    (0.0, Color::srgb(0.05, 0.15, 0.3)),
                    (0.7, Color::srgb(0.08, 0.25, 0.45)),
                    (1.0, Color::srgb(0.0, 0.75, 0.9)),
                ],
            ),
        ),
        (
            "CRIMSON VOLCANIC CORE",
            "High-contrast dynamic magma gradient with molten finish.",
            UGradient::linear_stops(
                0.0,
                [
                    (0.0, Color::srgb(0.9, 0.15, 0.15)),
                    (0.5, Color::srgb(0.4, 0.05, 0.1)),
                    (1.0, Color::srgb(0.15, 0.02, 0.05)),
                ],
            ),
        ),
        (
            "STELLAR NEBULA PRISM",
            "Balanced chromatic sweep simulating optical dispersion.",
            UGradient::linear_stops(
                1.1,
                [
                    (0.0, Color::srgb(0.8, 0.2, 0.9)),
                    (0.5, Color::srgb(0.1, 0.3, 0.7)),
                    (1.0, Color::srgb(0.1, 0.8, 0.8)),
                ],
            ),
        ),
    ];

    for (i, (title, desc, grad)) in cards.into_iter().enumerate() {
        spawn_feed_card(commands, parent, i + 1, title, desc, grad);
    }
}

fn handle_control_clicks(
    mut interaction_q: Query<(&UInteraction, &ControlButton), Changed<UInteraction>>,
    mut container_q: Query<&mut UScrollContainer, With<FeedContainer>>,
    content_q: Query<Entity, With<FeedContentRoot>>,
    mut commands: Commands,
    mut state: ResMut<DemoState>,
) {
    let Ok(mut container) = container_q.single_mut() else {
        return;
    };

    for (interaction, btn) in interaction_q.iter_mut() {
        if *interaction != UInteraction::Clicked {
            continue;
        }

        match btn {
            ControlButton::ScrollTop => {
                container.scroll_to(Vec2::ZERO);
            }
            ControlButton::ScrollBottom => {
                let bottom_y = container.max_offset.y;
                container.scroll_to(Vec2::new(0.0, bottom_y));
            }
            ControlButton::ScrollStepDown => {
                container.scroll_by(Vec2::new(0.0, 160.0));
            }
            ControlButton::ScrollStepUp => {
                container.scroll_by(Vec2::new(0.0, -160.0));
            }
            ControlButton::ToggleSmoothness => {
                state.snappy_mode = !state.snappy_mode;
                container.smoothness = if state.snappy_mode { 28.0 } else { 16.0 };
            }
            ControlButton::AddItem => {
                state.item_counter += 1;
                let count = state.item_counter;
                if let Ok(content_entity) = content_q.single() {
                    let grad = UGradient::linear_stops(
                        0.5,
                        [
                            (0.0, Color::srgb(0.2, 0.8, 0.6)),
                            (0.5, Color::srgb(0.1, 0.2, 0.4)),
                            (1.0, Color::srgb(0.8, 0.3, 0.7)),
                        ],
                    );
                    spawn_feed_card(
                        &mut commands,
                        content_entity,
                        count,
                        "DYNAMICALLY SPAWNED CARD",
                        "Added live to UScrollContent. Extents auto-recalculated.",
                        grad,
                    );
                }
            }
        }
    }
}

fn update_scroll_diagnostics(
    container_q: Query<&UScrollContainer, With<FeedContainer>>,
    mut progress_bar: Query<&mut UNode, With<ProgressBarFill>>,
    mut stat_labels: Query<(&StatLabel, &mut UTextLabel)>,
    state: Res<DemoState>,
) {
    let Ok(container) = container_q.single() else {
        return;
    };

    let progress = container.progress_y();

    if let Ok(mut bar_node) = progress_bar.single_mut() {
        bar_node.width = UVal::Percent(progress);
    }

    for (stat, mut label) in stat_labels.iter_mut() {
        match stat {
            StatLabel::Offset => {
                label.text = format!(
                    "{:.1} / {:.1} px",
                    container.scroll_offset.y, container.max_offset.y
                );
            }
            StatLabel::Progress => {
                label.text = format!("{:.1}%", progress * 100.0);
            }
            StatLabel::Smoothness => {
                label.text = if state.snappy_mode {
                    "28.0 (Snappy)".into()
                } else {
                    "16.0 (Glide)".into()
                };
            }
            StatLabel::ItemCount => {
                label.text = format!("{} Cards", state.item_counter);
            }
        }
    }
}
