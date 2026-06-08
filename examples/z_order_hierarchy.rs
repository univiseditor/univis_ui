use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Component)]
struct PriorityBranch;

#[derive(Component)]
struct PriorityBranchTitle;

#[derive(Component)]
struct PriorityStatusLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_priority_branch_order)
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

    let info_panel = commands
        .spawn((
            ChildOf(root),
            panel_node(470.0, 172.0, Color::srgba(0.08, 0.1, 0.14, 0.96)),
            panel_border(Color::srgba(0.7, 0.82, 1.0, 0.28), 18.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
            absolute(28.0, 28.0, 0),
        ))
        .id();

    commands.spawn((
        ChildOf(info_panel),
        label_node(Color::NONE),
        text("z-order hierarchy demo", 28.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(info_panel),
        label_node(Color::NONE),
        text(
            "Branch A child is deeper, but it stays below Branch B.",
            16.0,
            Color::srgb(0.83, 0.88, 0.94),
        ),
    ));
    commands.spawn((
        ChildOf(info_panel),
        label_node(Color::NONE),
        text(
            "Press Space to toggle the priority branch order.",
            16.0,
            Color::srgb(0.83, 0.88, 0.94),
        ),
    ));
    commands.spawn((
        ChildOf(info_panel),
        PriorityStatusLabel,
        label_node(Color::srgba(0.49, 0.19, 0.15, 0.78)),
        text(
            "Current priority order: 1",
            16.0,
            Color::srgb(1.0, 0.93, 0.9),
        ),
    ));

    let canvas = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(980.0),
                height: UVal::Px(560.0),
                background_color: Color::srgba(0.09, 0.12, 0.16, 0.98),
                border_radius: UCornerRadius::all(26.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.73, 0.79, 0.89, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(26.0),
                offset: 0.0,
            },
            ULayout::default(),
        ))
        .id();

    commands.spawn((
        ChildOf(canvas),
        absolute(24.0, 18.0, 0),
        label_node(Color::NONE),
        text(
            "Expected stack: Branch A < A child < Branch B subtree < Priority subtree",
            18.0,
            Color::srgb(0.76, 0.82, 0.9),
        ),
    ));

    let branch_a = commands
        .spawn((
            ChildOf(canvas),
            panel_node(340.0, 220.0, Color::srgba(0.19, 0.33, 0.78, 0.92)),
            panel_border(Color::srgba(0.72, 0.84, 1.0, 0.55), 22.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
            absolute(90.0, 120.0, 0),
        ))
        .id();

    commands.spawn((
        ChildOf(branch_a),
        label_node(Color::NONE),
        text("Branch A", 24.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(branch_a),
        label_node(Color::NONE),
        text(
            "first sibling in the canvas",
            16.0,
            Color::srgb(0.9, 0.95, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(branch_a),
        label_node(Color::NONE),
        text(
            "its subtree starts below later siblings",
            16.0,
            Color::srgb(0.9, 0.95, 1.0),
        ),
    ));

    let branch_a_child = commands
        .spawn((
            ChildOf(branch_a),
            panel_node(230.0, 142.0, Color::srgba(0.09, 0.78, 0.84, 0.93)),
            panel_border(Color::srgba(0.76, 1.0, 1.0, 0.55), 20.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
            absolute(168.0, 110.0, 0),
        ))
        .id();

    commands.spawn((
        ChildOf(branch_a_child),
        label_node(Color::NONE),
        text("A child", 22.0, Color::srgb(0.03, 0.12, 0.16)),
    ));
    commands.spawn((
        ChildOf(branch_a_child),
        label_node(Color::NONE),
        text("deeper than Branch A", 16.0, Color::srgb(0.05, 0.16, 0.2)),
    ));
    commands.spawn((
        ChildOf(branch_a_child),
        label_node(Color::NONE),
        text(
            "but still below Branch B",
            16.0,
            Color::srgb(0.05, 0.16, 0.2),
        ),
    ));

    let branch_b = commands
        .spawn((
            ChildOf(canvas),
            panel_node(318.0, 220.0, Color::srgba(0.89, 0.56, 0.18, 0.94)),
            panel_border(Color::srgba(1.0, 0.89, 0.64, 0.58), 22.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
            absolute(470.0, 156.0, 0),
        ))
        .id();

    commands.spawn((
        ChildOf(branch_b),
        label_node(Color::NONE),
        text("Branch B", 24.0, Color::srgb(0.17, 0.1, 0.05)),
    ));
    commands.spawn((
        ChildOf(branch_b),
        label_node(Color::NONE),
        text("later sibling branch", 16.0, Color::srgb(0.23, 0.14, 0.07)),
    ));
    commands.spawn((
        ChildOf(branch_b),
        label_node(Color::NONE),
        text(
            "it paints above the full Branch A subtree",
            16.0,
            Color::srgb(0.23, 0.14, 0.07),
        ),
    ));

    let priority_branch = commands
        .spawn((
            ChildOf(canvas),
            PriorityBranch,
            panel_node(372.0, 148.0, Color::srgba(0.78, 0.24, 0.36, 0.95)),
            panel_border(Color::srgba(1.0, 0.77, 0.83, 0.58), 22.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
            absolute(248.0, 344.0, 1),
        ))
        .id();

    commands.spawn((
        ChildOf(priority_branch),
        PriorityBranchTitle,
        label_node(Color::NONE),
        text(
            "Priority branch (order = 1)",
            24.0,
            Color::srgb(1.0, 0.95, 0.96),
        ),
    ));
    commands.spawn((
        ChildOf(priority_branch),
        label_node(Color::NONE),
        text(
            "its full subtree rises above both branches",
            16.0,
            Color::srgb(1.0, 0.88, 0.9),
        ),
    ));

    let priority_child = commands
        .spawn((
            ChildOf(priority_branch),
            panel_node(160.0, 68.0, Color::srgba(1.0, 0.71, 0.79, 0.95)),
            panel_border(Color::srgba(1.0, 0.9, 0.93, 0.5), 18.0),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            absolute(188.0, 40.0, 0),
        ))
        .id();

    commands.spawn((
        ChildOf(priority_child),
        label_node(Color::NONE),
        text("priority child", 16.0, Color::srgb(0.26, 0.04, 0.09)),
    ));
}

fn toggle_priority_branch_order(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut priority_branch_query: Query<&mut UZIndex, With<PriorityBranch>>,
    mut label_query: Query<
        (
            &mut UTextLabel,
            Option<&PriorityBranchTitle>,
            Option<&PriorityStatusLabel>,
        ),
        Or<(With<PriorityBranchTitle>, With<PriorityStatusLabel>)>,
    >,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let mut next_order = None;
    for mut z_index in &mut priority_branch_query {
        let new_order = match *z_index {
            UZIndex::Local(order) if order > 0 => -1,
            _ => 1,
        };
        *z_index = UZIndex::Local(new_order);
        next_order = Some(new_order);
    }

    let Some(next_order) = next_order else {
        return;
    };

    for (mut label, is_title, is_status) in &mut label_query {
        if is_title.is_some() {
            label.text = format!("Priority branch (order = {next_order})");
        }
        if is_status.is_some() {
            label.text = format!("Current priority order: {next_order}");
        }
    }
}

fn absolute(left: f32, top: f32, order: i32) -> (USelf, UZIndex) {
    (
        USelf {
            left: UVal::Px(left),
            top: UVal::Px(top),
            position_type: UPositionType::Absolute,
            ..default()
        },
        UZIndex::Local(order),
    )
}

fn panel_node(width: f32, height: f32, background_color: Color) -> UNode {
    UNode {
        width: UVal::Px(width),
        height: UVal::Px(height),
        padding: USides::all(16.0),
        background_color,
        border_radius: UCornerRadius::all(22.0),
        ..default()
    }
}

fn panel_border(color: Color, radius: f32) -> UBorder {
    UBorder {
        color,
        width: 1.5,
        radius: UCornerRadius::all(radius),
        offset: 0.0,
    }
}

fn label_node(background_color: Color) -> UNode {
    UNode {
        background_color,
        padding: USides::axes(10.0, 6.0),
        border_radius: UCornerRadius::all(12.0),
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
