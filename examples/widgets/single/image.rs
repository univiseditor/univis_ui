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

    let panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UNode {
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 32.0,
                align_items: UAlignItems::End,
                ..default()
            },
        ))
        .id();

    let tex_large = images.add(test_image(200, 100, [60, 120, 200, 255]));
    let tex_square = images.add(test_image(80, 80, [200, 100, 80, 255]));

    let col1 = commands
        .spawn((
            ChildOf(panel),
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
        UImage::new(tex_large.clone())
            .with_size(UVal::Auto, UVal::Auto)
            .with_radius(UCornerRadius::all(8.0)),
    ));
    commands.spawn((ChildOf(col1), UTextLabel::new("Auto size (200x100)")));

    let col2 = commands
        .spawn((
            ChildOf(panel),
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
        UImage::new(tex_square.clone())
            .with_size(UVal::Px(120.0), UVal::Px(120.0))
            .with_radius(UCornerRadius::all(32.0)),
    ));
    commands.spawn((
        ChildOf(col2),
        UTextLabel::new("Fixed (120x120), Big Radius"),
    ));
}
