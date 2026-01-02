use bevy::prelude::*;
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
                // border_radius: UCornerRadius::all(50.0),
                ..default()
            },
            ULayout {
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            // هام جداً: تفعيل التقاط الماوس
            

        )).with_children(|btn| {
            btn.spawn((
                UNode {
                    width: UVal::Px(150.),
                    height: UVal::Px(150.),
                    border_radius: UCornerRadius::all(70.),
                    background_color: Color::Srgba(RED),
                    ..default()
                },
                Pickable::default(),
            // ألوان التفاعل التلقائية (Hover/Press)
                UInteractionColors {
                    normal: Color::Srgba(RED),
                    hovered: Color::Srgba(BLUE),
                    pressed: Color::Srgba(GREEN),
                },
            ));
        });
        // --- استخدام Observer (الطريقة الحديثة في Bevy) --
    });
}