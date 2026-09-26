//! # Visual Scrollbars & Kinetic Drag-to-Scroll Showcase
//!
//! Demonstrates:
//! 1. **`UScrollbarTrack` & `UScrollbarThumb`**: Proportional thumb sizing, 1:1 mouse drag
//!    synchronization, and track click-to-jump navigation.
//! 2. **`UScrollbarFade`**: Automatic smooth fade-out when idle, lighting up instantly on hover or scroll.
//! 3. **`UScrollKineticDrag`**: Touch-style content dragging with velocity momentum and exponential friction decay.
//! 4. **Bidirectional Support**: Both vertical and horizontal scrollbars running simultaneously.
//! 5. **Live Sizing & Diagnostics**: Dynamic item additions/removals demonstrating live thumb resizing.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct ScrollDemoState {
    card_count: usize,
    kinetic_enabled: bool,
}

impl Default for ScrollDemoState {
    fn default() -> Self {
        Self {
            card_count: 8,
            kinetic_enabled: true,
        }
    }
}

#[derive(Component)]
struct MainVerticalContainer;

#[derive(Component)]
struct MainVerticalContent;

#[derive(Component)]
struct HorizontalContainer;

#[derive(Component)]
enum ActionButton {
    ScrollTop,
    ScrollBottom,
    StepDown,
    StepUp,
    ToggleKinetic,
    AddCards,
    RemoveCards,
}

#[derive(Component)]
enum MetricLabel {
    OffsetY,
    MaxOffsetY,
    OffsetX,
    MaxOffsetX,
    VelocityY,
    ThumbHeight,
    KineticStatus,
    ItemCount,
}

fn border(color: Color, radius: f32, width: f32) -> UBorder {
    UBorder {
        color,
        width,
        radius: UCornerRadius::all(radius),
        offset: 0.0,
    }
}

fn flex_col(gap: f32) -> ULayout {
    ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        gap,
        ..default()
    }
}

fn flex_row(gap: f32) -> ULayout {
    ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Row,
        gap,
        align_items: UAlignItems::Center,
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

fn spawn_label(commands: &mut Commands, parent: Entity, text: &str, color: Color, font_size: f32) {
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
        .add_plugins((DefaultPlugins, UnivisUiPlugin))
        .init_resource::<ScrollDemoState>()
        .add_systems(Startup, setup_demo_scene)
        .add_systems(Update, (handle_action_buttons, update_metrics_display))
        .run();
}

fn setup_demo_scene(mut commands: Commands, state: Res<ScrollDemoState>) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.12),
                padding: USides::all(20.0),
                ..default()
            },
            flex_col(16.0),
        ))
        .id();

    // Header Bar
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(56.0),
                background_color: Color::srgb(0.09, 0.12, 0.18),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::axes(12.0, 20.0),
                ..default()
            },
            border(Color::srgb(0.18, 0.24, 0.36), 10.0, 1.0),
            flex_row_between(),
        ))
        .id();

    let header_left = commands
        .spawn((ChildOf(header), UNode::default(), flex_row(12.0)))
        .id();
    spawn_label(
        &mut commands,
        header_left,
        "UNIVIS UI",
        Color::srgb(0.2, 0.8, 0.95),
        18.0,
    );
    spawn_label(
        &mut commands,
        header_left,
        "|",
        Color::srgb(0.3, 0.35, 0.45),
        16.0,
    );
    spawn_label(
        &mut commands,
        header_left,
        "Visual Scrollbars & Kinetic Drag-to-Scroll",
        Color::srgb(0.85, 0.9, 0.95),
        15.0,
    );

    let header_right = commands
        .spawn((ChildOf(header), UNode::default(), flex_row(10.0)))
        .id();
    for (badge, bg, fg) in [
        (
            "Proportional Thumb",
            Color::srgb(0.1, 0.4, 0.5),
            Color::srgb(0.4, 0.9, 1.0),
        ),
        (
            "Friction Momentum",
            Color::srgb(0.35, 0.15, 0.45),
            Color::srgb(0.9, 0.6, 1.0),
        ),
        (
            "Auto-Hide Fade",
            Color::srgb(0.15, 0.35, 0.25),
            Color::srgb(0.5, 0.9, 0.6),
        ),
    ] {
        spawn_badge(&mut commands, header_right, badge, bg, fg);
    }

    // Main workspace: Left controls & diagnostics, Right vertical feed
    let workspace = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(490.0),
                ..default()
            },
            flex_row(16.0),
        ))
        .id();

    // 1. Controls & Dashboard Panel (Left, 340px)
    let left_panel = commands
        .spawn((
            ChildOf(workspace),
            UNode {
                width: UVal::Px(340.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.10, 0.15),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::all(16.0),
                ..default()
            },
            border(Color::srgb(0.16, 0.20, 0.30), 12.0, 1.0),
            flex_col(10.0),
        ))
        .id();

    spawn_label(
        &mut commands,
        left_panel,
        "Controls & Telemetry",
        Color::srgb(0.9, 0.92, 0.96),
        15.0,
    );

    // Metrics Box
    let metrics_box = commands
        .spawn((
            ChildOf(left_panel),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.11),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(10.0),
                ..default()
            },
            border(Color::srgb(0.14, 0.18, 0.26), 8.0, 1.0),
            flex_col(4.0),
        ))
        .id();

    for (label, metric) in [
        ("Vertical Offset Y", MetricLabel::OffsetY),
        ("Vertical Max Y", MetricLabel::MaxOffsetY),
        ("Horizontal Offset X", MetricLabel::OffsetX),
        ("Horizontal Max X", MetricLabel::MaxOffsetX),
        ("Thumb Height", MetricLabel::ThumbHeight),
        ("Kinetic Velocity", MetricLabel::VelocityY),
        ("Kinetic Drag", MetricLabel::KineticStatus),
        ("Total Feed Cards", MetricLabel::ItemCount),
    ] {
        spawn_metric_row(&mut commands, metrics_box, label, metric);
    }

    // Buttons
    spawn_label(
        &mut commands,
        left_panel,
        "Navigation & Tests",
        Color::srgb(0.7, 0.75, 0.85),
        13.0,
    );

    for (b1, a1, b2, a2) in [
        (
            "Top",
            ActionButton::ScrollTop,
            "Bottom",
            ActionButton::ScrollBottom,
        ),
        (
            "+200px Down",
            ActionButton::StepDown,
            "-200px Up",
            ActionButton::StepUp,
        ),
        (
            "Add 3 Cards",
            ActionButton::AddCards,
            "Remove 3",
            ActionButton::RemoveCards,
        ),
    ] {
        let row = commands
            .spawn((ChildOf(left_panel), UNode::default(), flex_row(8.0)))
            .id();
        spawn_btn(&mut commands, row, b1, a1, 150.0);
        spawn_btn(&mut commands, row, b2, a2, 150.0);
    }

    spawn_btn(
        &mut commands,
        left_panel,
        "Toggle Kinetic Drag",
        ActionButton::ToggleKinetic,
        308.0,
    );

    // Instruction Box
    let instructions = commands
        .spawn((
            ChildOf(left_panel),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::srgba(0.12, 0.22, 0.35, 0.3),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(8.0),
                ..default()
            },
            border(Color::srgb(0.2, 0.35, 0.55), 8.0, 1.0),
            flex_col(3.0),
        ))
        .id();
    spawn_label(
        &mut commands,
        instructions,
        "Interaction Guide:",
        Color::srgb(0.4, 0.85, 1.0),
        12.0,
    );
    for line in [
        "- Wheel: Smooth glided scrolling (works on both lists)",
        "- Drag Content: Direct touch-style momentum fling",
        "- Drag Thumb: 1:1 proportional tracking",
        "- Track Click: Fast jump to section",
    ] {
        spawn_label(
            &mut commands,
            instructions,
            line,
            Color::srgb(0.75, 0.8, 0.9),
            11.0,
        );
    }

    // 2. Right Vertical Feed Container & Visual Scrollbar
    let feed_section = commands
        .spawn((
            ChildOf(workspace),
            UNode {
                width: UVal::Px(580.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.10, 0.15),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::all(14.0),
                ..default()
            },
            border(Color::srgb(0.16, 0.20, 0.30), 12.0, 1.0),
            flex_row(10.0),
        ))
        .id();

    // Scroll Viewport with rounded SDF clipping enabled
    let scroll_container = commands
        .spawn((
            ChildOf(feed_section),
            MainVerticalContainer,
            UNode {
                width: UVal::Px(526.0),
                height: UVal::Percent(1.0),
                border_radius: UCornerRadius::all(8.0),
                background_color: Color::srgb(0.05, 0.07, 0.10),
                padding: USides::all(12.0),
                ..default()
            },
            border(Color::srgb(0.14, 0.17, 0.24), 8.0, 1.0),
            UScrollContainer::vertical()
                .with_sensitivity(45.0)
                .with_smoothness(20.0),
            UClip { enabled: true },
            UScrollKineticDrag::default()
                .with_friction(7.5)
                .with_max_velocity(4500.0),
        ))
        .id();

    // Scroll Content
    let scroll_content = commands
        .spawn((
            ChildOf(scroll_container),
            MainVerticalContent,
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            UScrollContent::default(),
            flex_col(10.0),
        ))
        .id();

    for i in 1..=state.card_count {
        spawn_feed_card(&mut commands, scroll_content, i);
    }

    // Scrollbar Track & Thumb
    let track = commands
        .spawn((
            ChildOf(feed_section),
            UNode {
                width: UVal::Px(12.0),
                height: UVal::Percent(1.0),
                border_radius: UCornerRadius::all(6.0),
                background_color: Color::srgba(0.12, 0.15, 0.22, 0.6),
                ..default()
            },
            border(Color::srgba(0.2, 0.25, 0.35, 0.4), 6.0, 1.0),
            UScrollbarTrack::vertical(scroll_container).with_click_to_jump(true),
            UScrollbarFade::new(1.5, 0.4),
        ))
        .id();

    commands.spawn((
        ChildOf(track),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(50.0), // dynamically sized by sync_scrollbar_thumbs
            border_radius: UCornerRadius::all(6.0),
            background_color: Color::srgba(0.2, 0.7, 0.9, 0.85),
            ..default()
        },
        border(Color::srgba(0.4, 0.85, 1.0, 0.9), 6.0, 1.0),
        UScrollbarThumb::vertical(scroll_container).with_min_size(28.0),
        UScrollbarFade::new(1.5, 0.4),
    ));

    // 3. Horizontal Reel Section (Bottom)
    let bottom_section = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(160.0),
                background_color: Color::srgb(0.08, 0.10, 0.15),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::axes(10.0, 14.0),
                ..default()
            },
            border(Color::srgb(0.16, 0.20, 0.30), 12.0, 1.0),
            flex_col(8.0),
        ))
        .id();

    let bottom_header = commands
        .spawn((
            ChildOf(bottom_section),
            UNode::default(),
            flex_row_between(),
        ))
        .id();
    spawn_label(
        &mut commands,
        bottom_header,
        "Horizontal Scroll & Drag Reel (Clipped & Scrollable)",
        Color::srgb(0.85, 0.9, 0.95),
        14.0,
    );
    spawn_label(
        &mut commands,
        bottom_header,
        "Scroll wheel over reel, or drag directly / drag thumb",
        Color::srgb(0.5, 0.55, 0.65),
        12.0,
    );

    let h_container_box = commands
        .spawn((ChildOf(bottom_section), UNode::default(), flex_row(8.0)))
        .id();

    let h_container = commands
        .spawn((
            ChildOf(h_container_box),
            HorizontalContainer,
            UNode {
                width: UVal::Px(880.0),
                height: UVal::Px(90.0),
                border_radius: UCornerRadius::all(8.0),
                background_color: Color::srgb(0.05, 0.07, 0.10),
                padding: USides::all(8.0),
                ..default()
            },
            border(Color::srgb(0.14, 0.17, 0.24), 8.0, 1.0),
            UScrollContainer::horizontal().with_sensitivity(40.0),
            UClip { enabled: true },
            UScrollKineticDrag::default().with_friction(7.0),
        ))
        .id();

    let h_content = commands
        .spawn((
            ChildOf(h_container),
            UNode {
                min_width: 1824.0,
                height: UVal::Percent(1.0),
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(0.0),
                        flex_grow: Some(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            UScrollContent::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                container_ext: ULayoutContainerExt {
                    flex: ULayoutFlexContainer {
                        wrap: UFlexWrap::NoWrap,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    for i in 1..=12 {
        spawn_reel_card(&mut commands, h_content, i);
    }

    // Horizontal Track & Thumb
    let h_track = commands
        .spawn((
            ChildOf(bottom_section),
            UNode {
                width: UVal::Px(880.0),
                height: UVal::Px(10.0),
                border_radius: UCornerRadius::all(5.0),
                background_color: Color::srgba(0.12, 0.15, 0.22, 0.6),
                ..default()
            },
            border(Color::srgba(0.2, 0.25, 0.35, 0.4), 5.0, 1.0),
            UScrollbarTrack::horizontal(h_container).with_click_to_jump(true),
            UScrollbarFade::new(1.5, 0.4),
        ))
        .id();

    commands.spawn((
        ChildOf(h_track),
        UNode {
            width: UVal::Px(60.0), // dynamically sized by sync_scrollbar_thumbs
            height: UVal::Percent(1.0),
            border_radius: UCornerRadius::all(5.0),
            background_color: Color::srgba(0.8, 0.4, 0.9, 0.85),
            ..default()
        },
        border(Color::srgba(0.9, 0.6, 1.0, 0.9), 5.0, 1.0),
        UScrollbarThumb::horizontal(h_container).with_min_size(30.0),
        UScrollbarFade::new(1.5, 0.4),
    ));
}

fn spawn_metric_row(commands: &mut Commands, parent: Entity, label: &str, metric: MetricLabel) {
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

    spawn_label(commands, row, label, Color::srgb(0.6, 0.65, 0.75), 12.0);

    commands.spawn((
        ChildOf(row),
        UNode::default(),
        UTextLabel {
            text: "--".into(),
            color: Color::srgb(0.4, 0.85, 1.0),
            font_size: 12.0,
            ..default()
        },
        metric,
    ));
}

fn spawn_btn(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    action: ActionButton,
    width: f32,
) {
    let btn = commands
        .spawn((
            ChildOf(parent),
            action,
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(34.0),
                border_radius: UCornerRadius::all(6.0),
                background_color: Color::srgb(0.14, 0.18, 0.28),
                ..default()
            },
            border(Color::srgb(0.24, 0.32, 0.46), 6.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::srgb(0.14, 0.18, 0.28),
                hovered: Color::srgb(0.22, 0.28, 0.42),
                pressed: Color::srgb(0.08, 0.12, 0.20),
            },
        ))
        .id();

    spawn_label(commands, btn, title, Color::WHITE, 12.0);
}

fn spawn_badge(commands: &mut Commands, parent: Entity, title: &str, bg: Color, text_color: Color) {
    let badge = commands
        .spawn((
            ChildOf(parent),
            UNode {
                background_color: bg,
                border_radius: UCornerRadius::all(12.0),
                padding: USides::axes(4.0, 10.0),
                ..default()
            },
            border(text_color.with_alpha(0.3), 12.0, 1.0),
            flex_row(0.0),
        ))
        .id();
    spawn_label(commands, badge, title, text_color, 11.0);
}

fn spawn_feed_card(commands: &mut Commands, parent: Entity, index: usize) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(82.0),
                border_radius: UCornerRadius::all(8.0),
                background_color: Color::srgb(0.10, 0.13, 0.19),
                padding: USides::all(12.0),
                ..default()
            },
            border(Color::srgb(0.18, 0.22, 0.32), 8.0, 1.0),
            UInteraction::default(),
            UInteractionScale {
                normal: 1.0,
                hovered: 1.015,
                pressed: 0.985,
            },
            flex_col(4.0),
        ))
        .id();

    let top_row = commands
        .spawn((ChildOf(card), UNode::default(), flex_row_between()))
        .id();
    spawn_label(
        commands,
        top_row,
        &format!("Document Stream Item #{:02}", index),
        Color::srgb(0.9, 0.92, 0.96),
        13.0,
    );
    spawn_badge(
        commands,
        top_row,
        if index.is_multiple_of(2) {
            "Verified"
        } else {
            "Queued"
        },
        Color::srgba(0.15, 0.25, 0.35, 0.5),
        Color::srgb(0.4, 0.8, 1.0),
    );

    spawn_label(
        commands,
        card,
        "Smooth hardware-accelerated clipping and inertial momentum drag active.",
        Color::srgb(0.55, 0.6, 0.7),
        11.0,
    );
}

fn spawn_reel_card(commands: &mut Commands, parent: Entity, index: usize) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(140.0),
                min_width: 140.0,
                height: UVal::Percent(1.0),
                border_radius: UCornerRadius::all(8.0),
                background_color: Color::srgb(0.11, 0.14, 0.22),
                padding: USides::all(10.0),
                ..default()
            },
            border(Color::srgb(0.2, 0.26, 0.38), 8.0, 1.0),
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(0.0),
                        flex_grow: Some(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            UInteraction::default(),
            UInteractionScale {
                normal: 1.0,
                hovered: 1.03,
                pressed: 0.97,
            },
            flex_col(4.0),
        ))
        .id();

    spawn_label(
        commands,
        card,
        &format!("Reel #{:02}", index),
        Color::srgb(0.85, 0.9, 0.95),
        12.0,
    );
    spawn_label(
        commands,
        card,
        "Horizontal Card",
        Color::srgb(0.5, 0.55, 0.65),
        10.0,
    );
}

fn handle_action_buttons(
    mut state: ResMut<ScrollDemoState>,
    mut commands: Commands,
    clicks: Query<(&ActionButton, &UInteraction), Changed<UInteraction>>,
    mut v_container: Query<
        (&mut UScrollContainer, &mut UScrollKineticDrag),
        With<MainVerticalContainer>,
    >,
    content_query: Query<Entity, With<MainVerticalContent>>,
    card_children: Query<&Children, With<MainVerticalContent>>,
) {
    let Ok((mut container, mut kinetic)) = v_container.single_mut() else {
        return;
    };

    for (action, interaction) in clicks.iter() {
        if *interaction != UInteraction::Clicked {
            continue;
        }

        match action {
            ActionButton::ScrollTop => {
                kinetic.stop();
                let x = container.target_offset.x;
                container.scroll_to(Vec2::new(x, 0.0));
            }
            ActionButton::ScrollBottom => {
                kinetic.stop();
                let x = container.target_offset.x;
                let max_y = container.max_offset.y;
                container.scroll_to(Vec2::new(x, max_y));
            }
            ActionButton::StepDown => {
                kinetic.stop();
                container.scroll_by(Vec2::new(0.0, 200.0));
            }
            ActionButton::StepUp => {
                kinetic.stop();
                container.scroll_by(Vec2::new(0.0, -200.0));
            }
            ActionButton::ToggleKinetic => {
                state.kinetic_enabled = !state.kinetic_enabled;
                kinetic.enabled = state.kinetic_enabled;
                if !kinetic.enabled {
                    kinetic.stop();
                }
            }
            ActionButton::AddCards => {
                if let Ok(content_entity) = content_query.single() {
                    let start = state.card_count + 1;
                    for i in start..=(start + 2) {
                        spawn_feed_card(&mut commands, content_entity, i);
                    }
                    state.card_count += 3;
                }
            }
            ActionButton::RemoveCards => {
                if state.card_count > 3 {
                    let Ok(content_entity) = content_query.single() else {
                        continue;
                    };
                    let Ok(children) = card_children.get(content_entity) else {
                        continue;
                    };
                    for child in children.iter().rev().take(3) {
                        commands.entity(child).despawn();
                    }
                    state.card_count -= 3;
                }
            }
        }
    }
}

fn update_metrics_display(
    state: Res<ScrollDemoState>,
    v_container: Query<(&UScrollContainer, &UScrollKineticDrag), With<MainVerticalContainer>>,
    h_container: Query<&UScrollContainer, With<HorizontalContainer>>,
    thumbs: Query<(&UScrollbarThumb, &ComputedSize)>,
    mut labels: Query<(&MetricLabel, &mut UTextLabel)>,
) {
    let Ok((container, kinetic)) = v_container.single() else {
        return;
    };

    let h_sc = h_container.single().ok();

    let thumb_h = thumbs
        .iter()
        .find(|(t, _)| t.axis == UScrollbarAxis::Vertical)
        .map(|(_, c)| c.height)
        .unwrap_or(0.0);

    for (metric, mut label) in labels.iter_mut() {
        match metric {
            MetricLabel::OffsetY => {
                label.text = format!("{:.1} px", container.scroll_offset.y);
            }
            MetricLabel::MaxOffsetY => {
                label.text = format!("{:.1} px", container.max_offset.y);
            }
            MetricLabel::OffsetX => {
                label.text = format!("{:.1} px", h_sc.map_or(0.0, |c| c.scroll_offset.x));
            }
            MetricLabel::MaxOffsetX => {
                label.text = format!("{:.1} px", h_sc.map_or(0.0, |c| c.max_offset.x));
            }
            MetricLabel::VelocityY => {
                label.text = format!("{:.1} px/s", kinetic.velocity.y);
            }
            MetricLabel::ThumbHeight => {
                label.text = format!("{:.1} px", thumb_h);
            }
            MetricLabel::KineticStatus => {
                label.text = if state.kinetic_enabled {
                    "Enabled".into()
                } else {
                    "Disabled".into()
                };
                label.color = if state.kinetic_enabled {
                    Color::srgb(0.4, 0.9, 0.5)
                } else {
                    Color::srgb(0.9, 0.4, 0.4)
                };
            }
            MetricLabel::ItemCount => {
                label.text = format!("{} cards", state.card_count);
            }
        }
    }
}
