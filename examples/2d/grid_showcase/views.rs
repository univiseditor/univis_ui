//! Showcase views and grid builders for `grid_showcase`.

use bevy::prelude::*;
use univis_ui::prelude::*;

use super::ShowcaseState;

#[derive(Component)]
pub struct LiveMetricText;

#[derive(Component)]
#[allow(dead_code)]
pub struct DynamicGridCard(usize);

pub struct GridSlotDef<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub color: Color,
    pub col_start: Option<u32>,
    pub col_span: u32,
    pub row_start: Option<u32>,
    pub row_span: u32,
}

pub struct DashCardDef<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub accent: Color,
    pub col_start: Option<u32>,
    pub col_span: u32,
    pub row_start: Option<u32>,
    pub row_span: u32,
    pub is_live_metric: bool,
}

pub fn border(color: Color, r: f32, width: f32) -> UBorder {
    UBorder {
        color,
        width,
        radius: UCornerRadius::all(r),
        offset: 0.0,
    }
}

pub fn panel(bg: Color, r: f32, pad: f32, border_col: Color) -> (UNode, UBorder) {
    (
        UNode {
            padding: USides::all(pad),
            background_color: bg,
            border_radius: UCornerRadius::all(r),
            ..default()
        },
        border(border_col, r, 1.0),
    )
}

pub fn make_grid_layout(
    cols: u32,
    col_tracks: Vec<UTrackSize>,
    row_tracks: Vec<UTrackSize>,
    gap: f32,
    auto_flow: UGridAutoFlow,
) -> ULayout {
    ULayout {
        display: UDisplay::Grid,
        grid_columns: cols,
        container_ext: ULayoutContainerExt {
            box_align: ULayoutBoxAlignContainer {
                column_gap: Some(gap),
                row_gap: Some(gap),
                ..default()
            },
            grid: ULayoutGridContainer {
                auto_flow,
                template_columns: col_tracks,
                template_rows: row_tracks,
                ..default()
            },
            ..default()
        },
        ..default()
    }
}

pub fn grid_item(
    column_start: Option<u32>,
    column_span: u32,
    row_start: Option<u32>,
    row_span: u32,
) -> USelf {
    USelf {
        item_ext: ULayoutItemExt {
            grid: ULayoutGridItem {
                column_start,
                column_span,
                row_start,
                row_span,
            },
            ..default()
        },
        ..default()
    }
}

pub fn spawn_label(
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

// =========================================================================
// VIEW 1: RPG Equipment & Inventory Grid
// =========================================================================

pub fn spawn_inventory_view(commands: &mut Commands, parent: Entity) {
    let container = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceEvenly,
                align_items: UAlignItems::Center,
                gap: 32.0,
                ..default()
            },
        ))
        .id();

    // Left: Equipment Grid
    let (left_node, left_border) = panel(
        Color::srgba(0.07, 0.09, 0.14, 0.8),
        10.0,
        16.0,
        Color::srgba(0.25, 0.35, 0.5, 0.5),
    );
    let left_box = commands
        .spawn((
            ChildOf(container),
            left_node,
            left_border,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        commands,
        left_box,
        "CHARACTER EQUIPMENT (Explicit Grid Spans)",
        Color::srgb(0.9, 0.75, 0.3),
        13.0,
    );

    let equip_layout = make_grid_layout(
        3,
        vec![UTrackSize::Px(76.0); 3],
        vec![UTrackSize::Px(76.0); 4],
        10.0,
        UGridAutoFlow::Row,
    );
    let equip_grid = commands
        .spawn((ChildOf(left_box), UNode::default(), equip_layout))
        .id();

    let equip_items = [
        GridSlotDef {
            title: "Crown of Sol",
            subtitle: "Legendary Helm",
            color: Color::srgb(1.0, 0.8, 0.2),
            col_start: Some(2),
            col_span: 1,
            row_start: Some(1),
            row_span: 1,
        },
        GridSlotDef {
            title: "Titan Halberd",
            subtitle: "2H Polearm\n(Span 1x2)",
            color: Color::srgb(0.9, 0.3, 0.2),
            col_start: Some(1),
            col_span: 1,
            row_start: Some(2),
            row_span: 2,
        },
        GridSlotDef {
            title: "Aegis Plate",
            subtitle: "Chest Armor\n(Span 1x2)",
            color: Color::srgb(0.6, 0.3, 0.9),
            col_start: Some(2),
            col_span: 1,
            row_start: Some(2),
            row_span: 2,
        },
        GridSlotDef {
            title: "Mirror Bulwark",
            subtitle: "Tower Shield\n(Span 1x2)",
            color: Color::srgb(0.2, 0.6, 0.9),
            col_start: Some(3),
            col_span: 1,
            row_start: Some(2),
            row_span: 2,
        },
        GridSlotDef {
            title: "Ring of Power",
            subtitle: "+15% Magic",
            color: Color::srgb(0.3, 0.85, 0.6),
            col_start: Some(1),
            col_span: 1,
            row_start: Some(4),
            row_span: 1,
        },
        GridSlotDef {
            title: "Void Signet",
            subtitle: "+25 Mana",
            color: Color::srgb(0.8, 0.3, 0.8),
            col_start: Some(3),
            col_span: 1,
            row_start: Some(4),
            row_span: 1,
        },
    ];
    for item in &equip_items {
        spawn_grid_slot(commands, equip_grid, item);
    }

    // Right: Backpack & Bag Grid
    let (right_node, right_border) = panel(
        Color::srgba(0.07, 0.09, 0.14, 0.8),
        10.0,
        16.0,
        Color::srgba(0.25, 0.35, 0.5, 0.5),
    );
    let right_box = commands
        .spawn((
            ChildOf(container),
            right_node,
            right_border,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_label(
        commands,
        right_box,
        "BACKPACK INVENTORY (4x4 Matrix with 2x2 Spanning Tome)",
        Color::srgb(0.4, 0.75, 1.0),
        13.0,
    );

    let bag_layout = make_grid_layout(
        4,
        vec![UTrackSize::Px(74.0); 4],
        vec![UTrackSize::Px(74.0); 4],
        8.0,
        UGridAutoFlow::Row,
    );
    let bag_grid = commands
        .spawn((ChildOf(right_box), UNode::default(), bag_layout))
        .id();

    // 2x2 Ancient Grimoire spanning row 1-2, col 1-2
    spawn_grid_slot(
        commands,
        bag_grid,
        &GridSlotDef {
            title: "Grimoire of Eons",
            subtitle: "Ancient Spellbook\n[Span 2x2]",
            color: Color::srgb(1.0, 0.85, 0.2),
            col_start: Some(1),
            col_span: 2,
            row_start: Some(1),
            row_span: 2,
        },
    );

    let potions = [
        GridSlotDef {
            title: "Health Potion",
            subtitle: "+200 HP",
            color: Color::srgb(0.9, 0.2, 0.3),
            col_start: Some(3),
            col_span: 1,
            row_start: Some(1),
            row_span: 1,
        },
        GridSlotDef {
            title: "Mana Flask",
            subtitle: "+150 MP",
            color: Color::srgb(0.2, 0.4, 0.9),
            col_start: Some(4),
            col_span: 1,
            row_start: Some(1),
            row_span: 1,
        },
        GridSlotDef {
            title: "Stamina Brew",
            subtitle: "Energy",
            color: Color::srgb(0.2, 0.8, 0.4),
            col_start: Some(3),
            col_span: 1,
            row_start: Some(2),
            row_span: 1,
        },
        GridSlotDef {
            title: "Astral Shard",
            subtitle: "Crafting",
            color: Color::srgb(0.8, 0.4, 0.9),
            col_start: Some(4),
            col_span: 1,
            row_start: Some(2),
            row_span: 1,
        },
    ];
    for p in &potions {
        spawn_grid_slot(commands, bag_grid, p);
    }

    for c in 1..=4 {
        let name = format!("Runic Gem {c}");
        spawn_grid_slot(
            commands,
            bag_grid,
            &GridSlotDef {
                title: &name,
                subtitle: "Enchant",
                color: Color::srgb(0.4, 0.7, 0.8),
                col_start: Some(c),
                col_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
        );
        let gold_name = format!("Gold Pouch {c}");
        spawn_grid_slot(
            commands,
            bag_grid,
            &GridSlotDef {
                title: &gold_name,
                subtitle: "500 Coins",
                color: Color::srgb(0.9, 0.7, 0.1),
                col_start: Some(c),
                col_span: 1,
                row_start: Some(4),
                row_span: 1,
            },
        );
    }
}

pub fn spawn_grid_slot(commands: &mut Commands, parent: Entity, def: &GridSlotDef) {
    let name_for_inspect = format!("{} ({})", def.title, def.subtitle);
    commands
        .spawn((
            ChildOf(parent),
            grid_item(def.col_start, def.col_span, def.row_start, def.row_span),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(6.0),
                background_color: Color::srgba(0.12, 0.16, 0.22, 0.9),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(def.color.with_alpha(0.7), 8.0, 1.5),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 2.0,
                ..default()
            },
        ))
        .observe(
            move |_over: On<Pointer<Over>>, mut state: ResMut<ShowcaseState>| {
                state.selected_item = name_for_inspect.clone();
            },
        )
        .with_children(|slot| {
            slot.spawn((
                UNode::default(),
                UTextLabel {
                    text: def.title.into(),
                    color: def.color,
                    font_size: 11.0,
                    ..default()
                },
            ));
            slot.spawn((
                UNode::default(),
                UTextLabel {
                    text: def.subtitle.into(),
                    color: Color::srgb(0.65, 0.7, 0.8),
                    font_size: 9.0,
                    ..default()
                },
            ));
        });
}

// =========================================================================
// VIEW 2: Analytics & Operations Dashboard
// =========================================================================

pub fn spawn_dashboard_view(commands: &mut Commands, parent: Entity) {
    let dash_layout = make_grid_layout(
        4,
        vec![
            UTrackSize::Fr(1.2),
            UTrackSize::Fr(1.0),
            UTrackSize::Fr(1.0),
            UTrackSize::Fr(1.3),
        ],
        vec![
            UTrackSize::Px(100.0),
            UTrackSize::Px(220.0),
            UTrackSize::Px(130.0),
        ],
        16.0,
        UGridAutoFlow::Row,
    );
    let dash_grid = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(8.0),
                ..default()
            },
            dash_layout,
        ))
        .id();

    let cards = [
        DashCardDef {
            title: "SYSTEM HEALTH",
            body: "Status: OPTIMAL • Zero Oscillation",
            accent: Color::srgb(0.2, 0.85, 0.5),
            col_start: Some(1),
            col_span: 2,
            row_start: Some(1),
            row_span: 1,
            is_live_metric: false,
        },
        DashCardDef {
            title: "P99 SOLVER TIME",
            body: "0.12 ms (Pass-down safe)",
            accent: Color::srgb(0.4, 0.8, 1.0),
            col_start: Some(3),
            col_span: 1,
            row_start: Some(1),
            row_span: 1,
            is_live_metric: false,
        },
        DashCardDef {
            title: "CACHE POOL",
            body: "Frontier Dirty: 0 (Zero leaks)",
            accent: Color::srgb(0.9, 0.7, 0.2),
            col_start: Some(4),
            col_span: 1,
            row_start: Some(1),
            row_span: 1,
            is_live_metric: false,
        },
        DashCardDef {
            title: "REAL-TIME TELEMETRY",
            body: "Throughput: 142.8 kops/s | Real-time",
            accent: Color::srgb(0.3, 0.65, 1.0),
            col_start: Some(1),
            col_span: 3,
            row_start: Some(2),
            row_span: 1,
            is_live_metric: true,
        },
        DashCardDef {
            title: "AUDIT STREAM",
            body: "• CSS Grid solver ready\n• Intrinsic pass preserved\n• Budget: 1 pass",
            accent: Color::srgb(0.85, 0.45, 0.95),
            col_start: Some(4),
            col_span: 1,
            row_start: Some(2),
            row_span: 2,
            is_live_metric: false,
        },
        DashCardDef {
            title: "PIPELINE STAGES",
            body: "Root -> Hierarchy -> Measure\nSolve -> Render -> Settled",
            accent: Color::srgb(0.6, 0.7, 0.85),
            col_start: Some(1),
            col_span: 1,
            row_start: Some(3),
            row_span: 1,
            is_live_metric: false,
        },
        DashCardDef {
            title: "SDF COMPOSITOR",
            body: "Analytical Rounded Borders • Fractional Tracks",
            accent: Color::srgb(0.3, 0.85, 0.75),
            col_start: Some(2),
            col_span: 2,
            row_start: Some(3),
            row_span: 1,
            is_live_metric: false,
        },
    ];

    for card in &cards {
        spawn_dash_card(commands, dash_grid, card);
    }
}

pub fn spawn_dash_card(commands: &mut Commands, parent: Entity, def: &DashCardDef) {
    let card = commands
        .spawn((
            ChildOf(parent),
            grid_item(def.col_start, def.col_span, def.row_start, def.row_span),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(14.0),
                background_color: Color::srgba(0.09, 0.12, 0.17, 0.85),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            border(def.accent.with_alpha(0.5), 8.0, 1.2),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Start,
                ..default()
            },
        ))
        .id();

    spawn_label(commands, card, def.title, def.accent, 12.0);

    let body_entity = spawn_label(
        commands,
        card,
        def.body,
        Color::srgb(0.8, 0.85, 0.92),
        if def.is_live_metric { 14.0 } else { 11.0 },
    );

    if def.is_live_metric {
        commands.entity(body_entity).insert(LiveMetricText);
    }
}

// =========================================================================
// VIEW 3: Responsive Auto-Fit & Dynamic Flow
// =========================================================================

pub fn spawn_autoflow_view(
    commands: &mut Commands,
    parent: Entity,
    auto_flow: UGridAutoFlow,
    dynamic_item_count: usize,
) {
    let container = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Start,
                align_items: UAlignItems::Center,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    let bar = commands
        .spawn((
            ChildOf(container),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(44.0),
                padding: USides::axes(8.0, 16.0),
                background_color: Color::srgba(0.12, 0.16, 0.22, 0.8),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            border(Color::srgba(0.25, 0.35, 0.5, 0.5), 6.0, 1.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let flow_str = match auto_flow {
        UGridAutoFlow::Row => "Auto-Flow: ROW (Wraps horizontally)",
        UGridAutoFlow::Column => "Auto-Flow: COLUMN (Wraps vertically)",
    };
    spawn_label(
        commands,
        bar,
        &format!("Items: {dynamic_item_count} | {flow_str} | [Space] Add Card | [F] Toggle Flow"),
        Color::srgb(0.9, 0.95, 1.0),
        12.0,
    );

    let flow_layout = make_grid_layout(
        4,
        vec![UTrackSize::Fr(1.0); 4],
        vec![UTrackSize::Px(90.0); 3],
        12.0,
        auto_flow,
    );
    let (grid_node, grid_border) = panel(
        Color::srgba(0.06, 0.08, 0.12, 0.7),
        8.0,
        12.0,
        Color::srgba(0.2, 0.28, 0.4, 0.4),
    );
    let grid_box = commands
        .spawn((ChildOf(container), grid_node, grid_border, flow_layout))
        .id();

    let palette = [
        Color::srgb(0.2, 0.6, 0.9),
        Color::srgb(0.3, 0.8, 0.5),
        Color::srgb(0.9, 0.4, 0.3),
        Color::srgb(0.8, 0.3, 0.8),
        Color::srgb(0.9, 0.7, 0.2),
        Color::srgb(0.3, 0.8, 0.8),
    ];

    for i in 1..=dynamic_item_count {
        let col = palette[(i - 1) % palette.len()];
        commands
            .spawn((
                ChildOf(grid_box),
                DynamicGridCard(i),
                UNode {
                    width: UVal::Auto,
                    height: UVal::Auto,
                    padding: USides::all(12.0),
                    background_color: Color::srgba(0.1, 0.14, 0.2, 0.85),
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                border(col.with_alpha(0.6), 8.0, 1.2),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                    ..default()
                },
            ))
            .with_children(|card| {
                card.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: format!("Card #{i:02}"),
                        color: col,
                        font_size: 14.0,
                        ..default()
                    },
                ));
                card.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: "Auto-Placed Slot".into(),
                        color: Color::srgb(0.6, 0.7, 0.8),
                        font_size: 10.0,
                        ..default()
                    },
                ));
            });
    }
}
