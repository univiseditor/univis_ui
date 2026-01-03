use bevy::{color::palettes::css::*, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        UWorldRoot { size: Vec2::new(800.0, 600.0), ..default() },
        UNode { width: UVal::Percent(1.0), height: UVal::Percent(1.0), ..default() },
        ULayout { 
            display: UDisplay::Flex, 
            justify_content: UJustifyContent::Center, 
            align_items: UAlignItems::Center,
            flex_direction: UFlexDirection::Column, // ترتيب عمودي
            gap: 20.0,
            ..default() 
        }
    )).with_children(|parent| {

        parent.spawn((
            UNode {
                width: UVal::Px(400.0),
                height: UVal::Px(400.0),
                background_color: Color::BLACK,
                ..default()
            },
            ULayout {
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            

        )).with_children(|btn| {
            btn.spawn((
                UNode {
                    width: UVal::Px(150.),
                    height: UVal::Px(150.),
                    border_radius: UCornerRadius {
                        top_left: 75.0,
                        top_right: 0.0,
                        bottom_left: 0.0,
                        bottom_right: 0.0,
                    },
                    background_color: Color::Srgba(RED),
                    ..default()
                },
                Pickable::default(),
                UInteractionColors {
                    normal: Color::Srgba(RED),
                    hovered: Color::Srgba(BLUE),
                    pressed: Color::Srgba(GREEN),
                },
            )).with_children(|ch| {
                ch.spawn(
                    UNode::default()
                );
            });
        });
    });
    
}