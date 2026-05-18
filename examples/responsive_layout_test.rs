use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Responsive Layout Demo".to_string(),
                resolution: (1280, 860).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<LayoutState>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_responsive_layout)
        .run();
}

#[derive(Component)]
struct ResponsiveRoot;

#[derive(Component)]
struct FrameShell;

#[derive(Component)]
struct BreakpointTitle;

#[derive(Component)]
struct BreakpointHint;

#[derive(Component)]
struct HeroChart;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum SectionKind {
    Header,
    LeftNav,
    Hero,
    Story,
    Stats,
    MiniCards,
    RightRail,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Breakpoint {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Clone, Copy)]
struct Placement {
    column_start: u32,
    column_span: u32,
    row_start: u32,
    row_span: u32,
}

#[derive(Resource, Default)]
struct LayoutState {
    breakpoint: Option<Breakpoint>,
    last_width: f32,
    last_height: f32,
}

impl Breakpoint {
    fn from_width(width: f32) -> Self {
        if width < 540.0 {
            Self::One
        } else if width < 760.0 {
            Self::Two
        } else if width < 980.0 {
            Self::Three
        } else if width < 1240.0 {
            Self::Four
        } else {
            Self::Five
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::One => "Breakpoint 1",
            Self::Two => "Breakpoint 2",
            Self::Three => "Breakpoint 3",
            Self::Four => "Breakpoint 4",
            Self::Five => "Breakpoint 5",
        }
    }

    fn range(self) -> &'static str {
        match self {
            Self::One => "< 540 px",
            Self::Two => "540 - 759 px",
            Self::Three => "760 - 979 px",
            Self::Four => "980 - 1239 px",
            Self::Five => ">= 1240 px",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::One => "Phone: stacked flow with only the essential cards visible.",
            Self::Two => "Tablet: hero and metrics split into two columns.",
            Self::Three => "Small desktop: utility rail appears beside the main flow.",
            Self::Four => "Dashboard: content expands into a balanced three-column composition.",
            Self::Five => "Wide desktop: full navigation, main content, metrics, and utility rail.",
        }
    }

    fn shell_width(self, window_width: f32) -> f32 {
        let available = (window_width - 40.0).max(240.0);
        match self {
            Self::One => available.min(340.0),
            Self::Two => available.min(620.0),
            Self::Three => available.min(780.0),
            Self::Four => available.min(980.0),
            Self::Five => available.min(1240.0),
        }
    }

    fn shell_height(self, window_height: f32) -> f32 {
        let available = (window_height - 120.0).max(320.0);
        match self {
            Self::One => available.min(640.0),
            Self::Two => available.min(440.0),
            Self::Three => available.min(520.0),
            Self::Four => available.min(560.0),
            Self::Five => available.min(620.0),
        }
    }

    fn shell_padding(self) -> f32 {
        match self {
            Self::One => 14.0,
            Self::Two => 16.0,
            Self::Three => 18.0,
            Self::Four => 18.0,
            Self::Five => 20.0,
        }
    }

    fn hero_chart_height(self) -> f32 {
        match self {
            Self::One => 120.0,
            Self::Two => 132.0,
            Self::Three => 146.0,
            Self::Four => 158.0,
            Self::Five => 164.0,
        }
    }

    fn grid_columns(self) -> u32 {
        match self {
            Self::One => 1,
            Self::Two | Self::Three => 2,
            Self::Four => 3,
            Self::Five => 4,
        }
    }

    fn template_columns(self) -> Vec<UTrackSize> {
        match self {
            Self::One => vec![UTrackSize::Fr(1.0)],
            Self::Two => vec![UTrackSize::Fr(1.9), UTrackSize::Fr(1.0)],
            Self::Three => vec![UTrackSize::Fr(2.05), UTrackSize::Fr(1.0)],
            Self::Four => vec![
                UTrackSize::Fr(1.75),
                UTrackSize::Fr(1.0),
                UTrackSize::Fr(0.95),
            ],
            Self::Five => vec![
                UTrackSize::Px(132.0),
                UTrackSize::Fr(1.85),
                UTrackSize::Fr(1.0),
                UTrackSize::Fr(0.92),
            ],
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            URootUi::screen(),
            ResponsiveRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                background_color: Color::srgb(0.92, 0.92, 0.91),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(root),
        BreakpointTitle,
        label_node(),
        text(
            "Responsive Layout · Breakpoint 5 (>= 1240 px)",
            26.0,
            text_dark(),
        ),
    ));

    commands.spawn((
        ChildOf(root),
        BreakpointHint,
        label_node(),
        text(
            "Resize the window to see the same detailed dashboard rearrange itself.",
            14.0,
            text_muted(),
        ),
    ));

    let shell = commands
        .spawn((
            ChildOf(root),
            FrameShell,
            UNode {
                width: UVal::Px(Breakpoint::Five.shell_width(1280.0)),
                height: UVal::Px(Breakpoint::Five.shell_height(860.0)),
                padding: USides::all(Breakpoint::Five.shell_padding()),
                background_color: Color::srgb(0.985, 0.99, 0.985),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.74, 0.81, 0.74, 0.5),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Grid,
                gap: 14.0,
                grid_columns: Breakpoint::Five.grid_columns(),
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(14.0),
                        column_gap: Some(14.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: Breakpoint::Five.template_columns(),
                        auto_rows: UTrackSize::Auto,
                        auto_columns: UTrackSize::Auto,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    spawn_header_section(&mut commands, shell);
    spawn_left_nav_section(&mut commands, shell);
    spawn_hero_section(&mut commands, shell);
    spawn_story_section(&mut commands, shell);
    spawn_stats_section(&mut commands, shell);
    spawn_mini_cards_section(&mut commands, shell);
    spawn_right_rail_section(&mut commands, shell);
}

fn update_responsive_layout(
    windows: Query<&Window, With<PrimaryWindow>>,
    root_query: Query<Entity, With<ResponsiveRoot>>,
    mut labels: Query<(
        &mut UTextLabel,
        Option<&BreakpointTitle>,
        Option<&BreakpointHint>,
    )>,
    mut layout_nodes: Query<(
        Entity,
        Option<&FrameShell>,
        Option<&SectionKind>,
        Option<&HeroChart>,
        &mut UNode,
        &mut ULayout,
        Option<&mut USelf>,
    )>,
    mut state: ResMut<LayoutState>,
    mut invalidate_queue: ResMut<UiInvalidateRequestQueue>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let width = window.width();
    let height = window.height();
    let breakpoint = Breakpoint::from_width(width);
    let resized =
        (width - state.last_width).abs() > 0.5 || (height - state.last_height).abs() > 0.5;

    state.last_width = width;
    state.last_height = height;

    if resized {
        if let Ok(root_entity) = root_query.single() {
            invalidate_queue.request(root_entity, UiLayoutInvalidation::intrinsic_change());
        }
    }

    if state.breakpoint == Some(breakpoint) && !resized {
        return;
    }

    for (mut label, title, hint) in &mut labels {
        if title.is_some() {
            label.text = format!(
                "Responsive Layout · {} ({})",
                breakpoint.label(),
                breakpoint.range()
            );
        } else if hint.is_some() {
            label.text = breakpoint.description().to_string();
        }
    }

    let mut changed = Vec::new();

    for (entity, frame_shell, section_kind, hero_chart, mut node, mut layout, maybe_self) in
        &mut layout_nodes
    {
        if frame_shell.is_some() {
            apply_shell_breakpoint(breakpoint, width, height, &mut node, &mut layout);
            changed.push(entity);
        }

        if let Some(kind) = section_kind.copied() {
            let Some(mut uself) = maybe_self else {
                continue;
            };

            apply_section_breakpoint(breakpoint, kind, &mut node, &mut layout, &mut uself);
            changed.push(entity);
        }

        if hero_chart.is_some() {
            node.height = UVal::Px(breakpoint.hero_chart_height());
            changed.push(entity);
        }
    }

    for entity in changed {
        invalidate_queue.request(entity, UiLayoutInvalidation::intrinsic_change());
    }

    state.breakpoint = Some(breakpoint);
}

fn apply_shell_breakpoint(
    breakpoint: Breakpoint,
    window_width: f32,
    window_height: f32,
    node: &mut UNode,
    layout: &mut ULayout,
) {
    node.width = UVal::Px(breakpoint.shell_width(window_width));
    node.height = UVal::Px(breakpoint.shell_height(window_height));
    node.padding = USides::all(breakpoint.shell_padding());

    layout.display = UDisplay::Grid;
    layout.grid_columns = breakpoint.grid_columns();
    layout.gap = 14.0;
    layout.container_ext.box_align.row_gap = Some(14.0);
    layout.container_ext.box_align.column_gap = Some(14.0);
    layout.container_ext.grid.template_columns = breakpoint.template_columns();
    layout.container_ext.grid.template_rows.clear();
    layout.container_ext.grid.auto_rows = UTrackSize::Auto;
    layout.container_ext.grid.auto_columns = UTrackSize::Auto;
}

fn apply_section_breakpoint(
    breakpoint: Breakpoint,
    section: SectionKind,
    node: &mut UNode,
    layout: &mut ULayout,
    uself: &mut USelf,
) {
    let placement = section_placement(breakpoint, section);
    if let Some(placement) = placement {
        uself.item_ext.grid = ULayoutGridItem {
            column_start: Some(placement.column_start),
            column_span: placement.column_span,
            row_start: Some(placement.row_start),
            row_span: placement.row_span,
        };
    }

    match section {
        SectionKind::Header => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = if matches!(breakpoint, Breakpoint::One) {
                UFlexDirection::Column
            } else {
                UFlexDirection::Row
            };
            layout.justify_content = if matches!(breakpoint, Breakpoint::One) {
                UJustifyContent::Start
            } else {
                UJustifyContent::SpaceBetween
            };
            layout.align_items = if matches!(breakpoint, Breakpoint::One) {
                UAlignItems::Start
            } else {
                UAlignItems::Center
            };
            layout.gap = 12.0;
            node.min_height = if matches!(breakpoint, Breakpoint::One) {
                82.0
            } else {
                56.0
            };
        }
        SectionKind::LeftNav => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = UFlexDirection::Column;
            layout.gap = 10.0;
            node.min_height = 0.0;
        }
        SectionKind::Hero => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = UFlexDirection::Column;
            layout.gap = 12.0;
            node.min_height = breakpoint.hero_chart_height() + 112.0;
        }
        SectionKind::Story => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = UFlexDirection::Column;
            layout.gap = 10.0;
            node.min_height = if matches!(breakpoint, Breakpoint::One) {
                196.0
            } else {
                0.0
            };
        }
        SectionKind::Stats => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = UFlexDirection::Column;
            layout.gap = 10.0;
            node.min_height = 0.0;
        }
        SectionKind::MiniCards => {
            if placement.is_some() {
                layout.display = UDisplay::Grid;
                layout.gap = 12.0;
                layout.grid_columns = if matches!(breakpoint, Breakpoint::One) {
                    1
                } else {
                    2
                };
                layout.container_ext.box_align.row_gap = Some(12.0);
                layout.container_ext.box_align.column_gap = Some(12.0);
                layout.container_ext.grid.template_columns =
                    if matches!(breakpoint, Breakpoint::One) {
                        vec![UTrackSize::Fr(1.0)]
                    } else {
                        vec![UTrackSize::Fr(1.0), UTrackSize::Fr(1.0)]
                    };
                node.min_height = if matches!(breakpoint, Breakpoint::One) {
                    170.0
                } else {
                    0.0
                };
            } else {
                layout.display = UDisplay::None;
            }
        }
        SectionKind::RightRail => {
            layout.display = visible_display(placement, UDisplay::Flex);
            layout.flex_direction = UFlexDirection::Column;
            layout.gap = 10.0;
            node.min_height = 0.0;
        }
    }
}

fn section_placement(breakpoint: Breakpoint, section: SectionKind) -> Option<Placement> {
    use Breakpoint::*;
    use SectionKind::*;

    match (breakpoint, section) {
        (One, Header) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 1,
            row_span: 1,
        }),
        (One, Hero) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (One, Stats) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 3,
            row_span: 1,
        }),
        (One, Story) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 4,
            row_span: 1,
        }),
        (One, MiniCards) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 5,
            row_span: 1,
        }),

        (Two, Header) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 1,
            row_span: 1,
        }),
        (Two, Hero) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Two, Stats) => Some(Placement {
            column_start: 2,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Two, Story) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 3,
            row_span: 1,
        }),
        (Two, MiniCards) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 4,
            row_span: 1,
        }),

        (Three, Header) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 1,
            row_span: 1,
        }),
        (Three, Hero) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Three, Stats) => Some(Placement {
            column_start: 2,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Three, Story) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 3,
            row_span: 1,
        }),
        (Three, RightRail) => Some(Placement {
            column_start: 2,
            column_span: 1,
            row_start: 3,
            row_span: 2,
        }),
        (Three, MiniCards) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 4,
            row_span: 1,
        }),

        (Four, Header) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 1,
            row_span: 1,
        }),
        (Four, RightRail) => Some(Placement {
            column_start: 3,
            column_span: 1,
            row_start: 1,
            row_span: 4,
        }),
        (Four, Hero) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Four, Stats) => Some(Placement {
            column_start: 2,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Four, Story) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 3,
            row_span: 1,
        }),
        (Four, MiniCards) => Some(Placement {
            column_start: 1,
            column_span: 2,
            row_start: 4,
            row_span: 1,
        }),

        (Five, LeftNav) => Some(Placement {
            column_start: 1,
            column_span: 1,
            row_start: 1,
            row_span: 4,
        }),
        (Five, Header) => Some(Placement {
            column_start: 2,
            column_span: 2,
            row_start: 1,
            row_span: 1,
        }),
        (Five, RightRail) => Some(Placement {
            column_start: 4,
            column_span: 1,
            row_start: 1,
            row_span: 4,
        }),
        (Five, Hero) => Some(Placement {
            column_start: 2,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Five, Stats) => Some(Placement {
            column_start: 3,
            column_span: 1,
            row_start: 2,
            row_span: 1,
        }),
        (Five, Story) => Some(Placement {
            column_start: 2,
            column_span: 2,
            row_start: 3,
            row_span: 1,
        }),
        (Five, MiniCards) => Some(Placement {
            column_start: 2,
            column_span: 2,
            row_start: 4,
            row_span: 1,
        }),

        _ => None,
    }
}

fn visible_display(placement: Option<Placement>, display: UDisplay) -> UDisplay {
    if placement.is_some() {
        display
    } else {
        UDisplay::None
    }
}

fn spawn_header_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::Header,
        UNode {
            padding: USides::all(12.0),
            background_color: panel_tint(),
            border_radius: UCornerRadius::all(18.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
            gap: 12.0,
            ..default()
        },
    );

    let left = spawn_container(
        commands,
        section,
        UVal::Percent(0.56),
        UFlexDirection::Column,
        4.0,
    );
    commands.spawn((
        ChildOf(left),
        label_node(),
        text("Atlas Studio", 18.0, text_dark()),
    ));
    commands.spawn((
        ChildOf(left),
        label_node(),
        text("Campaign overview · spring launch", 11.0, text_muted()),
    ));

    let right = spawn_container(commands, section, UVal::Auto, UFlexDirection::Row, 8.0);
    spawn_pill(commands, right, 94.0, 24.0, surface_card(), "Search");
    spawn_pill(commands, right, 68.0, 24.0, green_soft(), "Export");
    spawn_avatar(commands, right, 24.0, green_mid());
}

fn spawn_left_nav_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::LeftNav,
        UNode {
            padding: USides::all(14.0),
            background_color: rail_tint(),
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
            ..default()
        },
    );

    commands.spawn((
        ChildOf(section),
        label_node(),
        text("TC", 18.0, text_dark()),
    ));

    for (label, active) in [
        ("Overview", true),
        ("Campaigns", false),
        ("Analytics", false),
        ("Assets", false),
        ("Settings", false),
    ] {
        spawn_nav_item(commands, section, label, active);
    }

    let footer = commands
        .spawn((
            ChildOf(section),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: surface_card(),
                border_radius: UCornerRadius::all(14.0),
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

    commands.spawn((
        ChildOf(footer),
        label_node(),
        text("Storage 78%", 11.0, text_dark()),
    ));
    spawn_bar(commands, footer, UVal::Percent(1.0), 8.0, green_soft());
    spawn_bar(commands, footer, UVal::Percent(0.78), 8.0, green_mid());
}

fn spawn_hero_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::Hero,
        UNode {
            padding: USides::all(14.0),
            background_color: panel_tint(),
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
            ..default()
        },
    );

    spawn_section_header(
        commands,
        section,
        "Launch performance",
        "Paid social · last 14 days",
    );

    commands.spawn((
        ChildOf(section),
        HeroChart,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(Breakpoint::Five.hero_chart_height()),
            background_color: chart_tint(),
            border_radius: UCornerRadius::all(16.0),
            ..default()
        },
    ));

    let metrics = spawn_container(
        commands,
        section,
        UVal::Percent(1.0),
        UFlexDirection::Row,
        10.0,
    );
    for (label, value) in [("CTR", "4.8%"), ("CPA", "$14"), ("ROAS", "3.2x")] {
        let pill = commands
            .spawn((
                ChildOf(metrics),
                UNode {
                    min_width: 72.0,
                    padding: USides::all(8.0),
                    background_color: surface_card(),
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((ChildOf(pill), label_node(), text(label, 10.0, text_muted())));
        commands.spawn((ChildOf(pill), label_node(), text(value, 14.0, text_dark())));
    }
}

fn spawn_story_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::Story,
        UNode {
            padding: USides::all(14.0),
            background_color: panel_tint(),
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
            ..default()
        },
    );

    spawn_section_header(
        commands,
        section,
        "Recent updates",
        "Ops + creative handoff",
    );

    for (title, subtitle, status) in [
        ("Landing page v4", "Scheduled for Tuesday 09:00", "Ready"),
        ("Video subtitles", "Arabic review pending", "Review"),
        ("Influencer shortlist", "8 creators approved", "Done"),
    ] {
        spawn_update_row(commands, section, title, subtitle, status);
    }
}

fn spawn_stats_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::Stats,
        UNode {
            padding: USides::all(14.0),
            background_color: rail_tint(),
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
            ..default()
        },
    );

    spawn_section_header(commands, section, "Key metrics", "Live refresh");

    for (label, value, delta) in [
        ("Reach", "82k", "+12%"),
        ("Signups", "1.6k", "+8%"),
        ("Spend", "$9.4k", "-3%"),
    ] {
        spawn_metric_card(commands, section, label, value, delta);
    }
}

fn spawn_mini_cards_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::MiniCards,
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            display: UDisplay::Grid,
            gap: 12.0,
            grid_columns: 2,
            container_ext: ULayoutContainerExt {
                box_align: ULayoutBoxAlignContainer {
                    row_gap: Some(12.0),
                    column_gap: Some(12.0),
                    ..default()
                },
                grid: ULayoutGridContainer {
                    template_columns: vec![UTrackSize::Fr(1.0), UTrackSize::Fr(1.0)],
                    ..default()
                },
                ..default()
            },
            ..default()
        },
    );

    let tasks = spawn_card_panel(commands, section, "Tasks", "This sprint");
    spawn_bar(commands, tasks, UVal::Percent(1.0), 8.0, green_soft());
    spawn_bar(commands, tasks, UVal::Percent(0.68), 8.0, green_mid());
    commands.spawn((
        ChildOf(tasks),
        label_node(),
        text("6 of 9 tasks completed", 11.0, text_muted()),
    ));

    let budget = spawn_card_panel(commands, section, "Budget", "Current allocation");
    spawn_bar(commands, budget, UVal::Percent(1.0), 8.0, green_soft());
    spawn_bar(commands, budget, UVal::Percent(0.54), 8.0, green_mid());
    commands.spawn((
        ChildOf(budget),
        label_node(),
        text("$5.1k remaining this month", 11.0, text_muted()),
    ));
}

fn spawn_right_rail_section(commands: &mut Commands, parent: Entity) {
    let section = spawn_section(
        commands,
        parent,
        SectionKind::RightRail,
        UNode {
            padding: USides::all(14.0),
            background_color: rail_tint(),
            border_radius: UCornerRadius::all(20.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
            ..default()
        },
    );

    spawn_section_header(commands, section, "Team + schedule", "This week");

    for (name, role) in [
        ("Sara", "Design review"),
        ("Omar", "Media buying"),
        ("Lina", "Content QA"),
    ] {
        let row = commands
            .spawn((
                ChildOf(section),
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::all(8.0),
                    background_color: surface_card(),
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                    ..default()
                },
            ))
            .id();

        let left = spawn_container(
            commands,
            row,
            UVal::Percent(0.72),
            UFlexDirection::Column,
            2.0,
        );
        commands.spawn((ChildOf(left), label_node(), text(name, 12.0, text_dark())));
        commands.spawn((ChildOf(left), label_node(), text(role, 10.0, text_muted())));

        spawn_avatar(commands, row, 18.0, green_mid());
    }

    let block = commands
        .spawn((
            ChildOf(section),
            UNode {
                width: UVal::Percent(1.0),
                min_height: 110.0,
                padding: USides::all(10.0),
                background_color: surface_card(),
                border_radius: UCornerRadius::all(14.0),
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

    commands.spawn((
        ChildOf(block),
        label_node(),
        text("Upcoming", 12.0, text_dark()),
    ));
    spawn_bar(commands, block, UVal::Percent(0.72), 6.0, green_mid());
    spawn_bar(commands, block, UVal::Percent(0.84), 6.0, green_soft());
    spawn_bar(commands, block, UVal::Percent(0.62), 6.0, green_mid());
    spawn_bar(commands, block, UVal::Percent(0.9), 6.0, green_soft());
}

fn spawn_section(
    commands: &mut Commands,
    parent: Entity,
    kind: SectionKind,
    node: UNode,
    layout: ULayout,
) -> Entity {
    commands
        .spawn((ChildOf(parent), kind, USelf::default(), node, layout))
        .id()
}

fn spawn_container(
    commands: &mut Commands,
    parent: Entity,
    width: UVal,
    direction: UFlexDirection,
    gap: f32,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                width,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: direction,
                gap,
                ..default()
            },
        ))
        .id()
}

fn spawn_section_header(commands: &mut Commands, parent: Entity, title: &str, subtitle: &str) {
    let group = spawn_container(
        commands,
        parent,
        UVal::Percent(1.0),
        UFlexDirection::Column,
        2.0,
    );
    commands.spawn((ChildOf(group), label_node(), text(title, 15.0, text_dark())));
    commands.spawn((
        ChildOf(group),
        label_node(),
        text(subtitle, 10.0, text_muted()),
    ));
}

fn spawn_nav_item(commands: &mut Commands, parent: Entity, label: &str, active: bool) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(8.0),
                background_color: if active { green_soft() } else { surface_card() },
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(row), label_node(), text(label, 11.0, text_dark())));
    spawn_avatar(
        commands,
        row,
        10.0,
        if active { green_mid() } else { green_soft() },
    );
}

fn spawn_update_row(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    status: &str,
) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: surface_card(),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    let left = spawn_container(
        commands,
        row,
        UVal::Percent(0.72),
        UFlexDirection::Column,
        2.0,
    );
    commands.spawn((ChildOf(left), label_node(), text(title, 12.0, text_dark())));
    commands.spawn((
        ChildOf(left),
        label_node(),
        text(subtitle, 10.0, text_muted()),
    ));
    spawn_pill(commands, row, 58.0, 22.0, green_soft(), status);
}

fn spawn_metric_card(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    value: &str,
    delta: &str,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: surface_card(),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(card), label_node(), text(label, 10.0, text_muted())));
    commands.spawn((ChildOf(card), label_node(), text(value, 18.0, text_dark())));
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(delta, 11.0, green_strong()),
    ));
}

fn spawn_card_panel(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
) -> Entity {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                padding: USides::all(12.0),
                background_color: panel_tint(),
                border_radius: UCornerRadius::all(18.0),
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

    commands.spawn((ChildOf(card), label_node(), text(title, 14.0, text_dark())));
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(subtitle, 10.0, text_muted()),
    ));
    card
}

fn spawn_pill(
    commands: &mut Commands,
    parent: Entity,
    width: f32,
    height: f32,
    background_color: Color,
    label: &str,
) {
    let pill = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
                padding: USides::all(6.0),
                background_color,
                border_radius: UCornerRadius::all(height * 0.5),
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

    commands.spawn((ChildOf(pill), label_node(), text(label, 10.0, text_dark())));
}

fn spawn_avatar(commands: &mut Commands, parent: Entity, size: f32, color: Color) {
    commands.spawn((
        ChildOf(parent),
        UNode {
            width: UVal::Px(size),
            height: UVal::Px(size),
            background_color: color,
            border_radius: UCornerRadius::all(size * 0.5),
            ..default()
        },
    ));
}

fn spawn_bar(commands: &mut Commands, parent: Entity, width: UVal, height: f32, color: Color) {
    commands.spawn((
        ChildOf(parent),
        UNode {
            width,
            height: UVal::Px(height),
            background_color: color,
            border_radius: UCornerRadius::all(height * 0.5),
            ..default()
        },
    ));
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

fn text_dark() -> Color {
    Color::srgb(0.21, 0.23, 0.2)
}

fn text_muted() -> Color {
    Color::srgb(0.43, 0.46, 0.42)
}

fn green_strong() -> Color {
    Color::srgb(0.26, 0.64, 0.43)
}

fn panel_tint() -> Color {
    Color::srgb(0.9, 0.98, 0.92)
}

fn rail_tint() -> Color {
    Color::srgb(0.88, 0.97, 0.9)
}

fn surface_card() -> Color {
    Color::srgb(0.96, 0.995, 0.97)
}

fn chart_tint() -> Color {
    Color::srgb(0.74, 0.79, 0.76)
}

fn green_mid() -> Color {
    Color::srgb(0.5, 0.75, 0.62)
}

fn green_soft() -> Color {
    Color::srgb(0.78, 0.91, 0.82)
}
