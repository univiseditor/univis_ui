use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Component)]
struct AnimatedProgress {
    speed: f32,
    phase: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_progress_bars)
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
        text("Widgets: display", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Badges, dividers, panels, and animated progress bars. This example focuses on non-text input widgets.",
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

    let left = panel(&mut commands, content, 527.0, 640.0, "Badges + dividers");
    let right = panel(&mut commands, content, 527.0, 640.0, "Progress + panels");

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "Badges size to content while dividers synchronize into node geometry.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let badge_row = commands
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

    spawn_badge(&mut commands, badge_row, "alpha", UBadge::primary().small());
    spawn_badge(&mut commands, badge_row, "stable", UBadge::success());
    spawn_badge(
        &mut commands,
        badge_row,
        "latency",
        UBadge::warning().large(),
    );
    spawn_badge(&mut commands, badge_row, "offline", UBadge::danger());
    spawn_badge(&mut commands, badge_row, "metrics", UBadge::info());

    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "Horizontal and vertical dividers can separate chunks without extra layout code.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let split_row = commands
        .spawn((
            ChildOf(left),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(150.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.12, 0.15, 0.2, 0.6),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceEvenly,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(split_row),
        label_node(),
        text("Logs", 18.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(split_row),
        UDivider::vertical()
            .with_length(UVal::Percent(1.0))
            .with_thickness(2.0),
    ));
    commands.spawn((
        ChildOf(split_row),
        label_node(),
        text("Metrics", 18.0, Color::WHITE),
    ));

    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    let card = commands
        .spawn((
            ChildOf(left),
            UPanel::card().with_gap(10.0),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(180.0),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(card),
        label_node(),
        text("Card panel", 18.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(
            "Panels are widgets too. They synchronize background, border, padding, and flex direction from a single component.",
            14.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    commands.spawn((
        ChildOf(right),
        label_node(),
        text(
            "These progress bars animate every frame to demonstrate `UProgressBar` updates.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let progress_panel = commands
        .spawn((
            ChildOf(right),
            UPanel::glass().with_gap(12.0),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(260.0),
                ..default()
            },
        ))
        .id();

    spawn_progress_row(
        &mut commands,
        progress_panel,
        "CPU",
        Color::srgb(0.2, 0.82, 0.36),
        0.7,
        0.0,
    );
    spawn_progress_row(
        &mut commands,
        progress_panel,
        "GPU",
        Color::srgb(0.22, 0.62, 0.98),
        1.1,
        1.2,
    );
    spawn_progress_row(
        &mut commands,
        progress_panel,
        "Net",
        Color::srgb(0.92, 0.58, 0.18),
        0.9,
        2.1,
    );

    commands.spawn((ChildOf(right), UDivider::horizontal().with_thickness(2.0)));

    let glass = commands
        .spawn((
            ChildOf(right),
            UPanel::glass().with_gap(10.0),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(220.0),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(glass),
        label_node(),
        text("Glass panel", 18.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(glass),
        label_node(),
        text(
            "Use glass for translucent tooling panels, overlays, and monitoring surfaces.",
            14.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));
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

fn spawn_badge(commands: &mut Commands, parent: Entity, label: &str, badge: UBadge) {
    let entity = commands.spawn((ChildOf(parent), badge)).id();
    commands.spawn((
        ChildOf(entity),
        label_node(),
        text(label, 14.0, Color::WHITE),
    ));
}

fn spawn_progress_row(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    color: Color,
    speed: f32,
    phase: f32,
) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.1, 0.13, 0.18, 0.65),
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
    commands.spawn((
        ChildOf(row),
        UProgressBar {
            value: 0.5,
            bar_color: color,
        },
        AnimatedProgress { speed, phase },
    ));
}

fn animate_progress_bars(
    time: Res<Time>,
    mut query: Query<(&AnimatedProgress, &mut UProgressBar)>,
) {
    for (anim, mut progress) in &mut query {
        let value = ((time.elapsed_secs() * anim.speed + anim.phase).sin() * 0.5) + 0.5;
        progress.value = value;
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
