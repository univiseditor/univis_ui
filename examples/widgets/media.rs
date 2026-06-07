use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn test_image(width: u32, height: u32, color: [u8; 4]) -> Image {
    Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &color,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    )
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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
        text("Widgets: media", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Images and icon buttons. Images can adapt to their native texture sizes or be constrained.",
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

    let left = panel(&mut commands, content, 527.0, 640.0, "Icon Buttons");
    let right = panel(&mut commands, content, 527.0, 640.0, "Images");

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "UIconButton renders a button with an icon from the loaded font theme.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let btn_row1 = commands
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

    commands.spawn((ChildOf(btn_row1), UIconButton::primary(Icon::PLAY)));
    commands.spawn((ChildOf(btn_row1), UIconButton::secondary(Icon::PAUSE)));
    commands.spawn((ChildOf(btn_row1), UIconButton::success(Icon::CHECK)));
    commands.spawn((ChildOf(btn_row1), UIconButton::danger(Icon::X)));
    commands.spawn((ChildOf(btn_row1), UIconButton::primary(Icon::HOUSE)));

    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "You can customize icon size, padding, and colors independently.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let btn_row2 = commands
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
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Custom large icon button
    commands.spawn((
        ChildOf(btn_row2),
        UIconButton {
            icon: Icon::SETTINGS,
            icon_size: 32.0,
            padding: USides::all(20.0),
            background: Color::srgba(0.2, 0.2, 0.2, 0.8),
            border_radius: UCornerRadius::all(16.0),
            ..default()
        },
    ));

    // Custom colored icon button
    commands.spawn((
        ChildOf(btn_row2),
        UIconButton {
            icon: Icon::HEART,
            icon_size: 24.0,
            icon_color: Color::srgb(1.0, 0.3, 0.4),
            padding: USides::axes(24.0, 16.0),
            background: Color::srgba(0.1, 0.1, 0.1, 0.9),
            border_radius: UCornerRadius::all(32.0), // Pill shape
            ..default()
        },
    ));

    // --- Images Panel ---

    commands.spawn((
        ChildOf(right),
        label_node(),
        text(
            "UImage binds a texture to the layout engine. It can use native texture dimensions or be forced into specific sizes.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    // Create some placeholder textures
    let tex_landscape = images.add(test_image(200, 100, [60, 120, 200, 255]));
    let tex_portrait = images.add(test_image(80, 160, [200, 100, 80, 255]));

    let img_row = commands
        .spawn((
            ChildOf(right),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                align_items: UAlignItems::End,
                ..default()
            },
        ))
        .id();

    // Native size image
    let col1 = commands
        .spawn((
            ChildOf(img_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(col1),
        UImage::new(tex_landscape.clone())
            .with_size(UVal::Auto, UVal::Auto)
            .with_radius(UCornerRadius::all(8.0)),
    ));
    commands.spawn((
        ChildOf(col1),
        label_node(),
        text("Native Size (Auto)", 13.0, Color::srgb(0.6, 0.6, 0.6)),
    ));

    // Fixed size image
    let col2 = commands
        .spawn((
            ChildOf(img_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(col2),
        UImage::new(tex_portrait.clone())
            .with_size(UVal::Px(120.0), UVal::Px(120.0))
            .with_radius(UCornerRadius::all(16.0)),
    ));
    commands.spawn((
        ChildOf(col2),
        label_node(),
        text("Fixed (120x120)", 13.0, Color::srgb(0.6, 0.6, 0.6)),
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
