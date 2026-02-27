use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UnivisUiPlugin,LayoutProfilingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, add_node)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(UWorldRoot::default())
            .with_children(|btn| {
                btn.spawn((UButton::default(),UInteraction::default()));
            });

}


fn add_node(
    mut commands: Commands,
    query: Query<(Entity, &UInteraction, &UButton)>,
) {
    for (_,click,_) in query.iter() {
        if *click == UInteraction::Pressed {
            spawn_node(&mut commands, "title", 1, 1, Vec2::ZERO);
        }
    }
}



pub fn spawn_node(
    commands: &mut Commands, // <-- التغيير هنا: نستخدم ChildBuilder بدلاً من Commands
    title: &str,
    inputs: usize,
    outputs: usize,
    pos: Vec2
) -> Entity {
    let node_width = 250.0;
    
    // إنشاء العقدة كـ ابن مباشر
    commands.spawn((
                UWorldRoot {
                    size: Vec2::ZERO,
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 1.),
                UInteraction::default(), // لتفعيل التفاعل.
                UBorder {
                    color: Color::WHITE,
                    width: 1.0,
                    ..default()
                },
                UNode {
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(3.0),
                    shape_mode: UShapeMode::Round,
                    ..default()
                }
            )).with_children(|parent| {
                
                parent.spawn((
                    UNode {
                        width: UVal::Px(node_width),
                        height: UVal::Content,
                        shape_mode: UShapeMode::Round,
                        background_color: Color::srgb(0.15, 0.15, 0.18),
                        border_radius: UCornerRadius::all(8.0),
                        ..default()
                    },
                    UBorder {
                        color: Color::srgb(0.3, 0.3, 0.35),
                        width: 1.0,
                        radius: UCornerRadius::all(8.0),
                        ..default()
                    },
                    ULayout {flex_direction: UFlexDirection::Column,..default()},
                    
                )).with_children(|node_parent| {
            
            // A. شريط العنوان (Header)
            node_parent.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(30.0),
                    shape_mode: UShapeMode::Round,
                    background_color: Color::srgb(0.25, 0.25, 0.3),
                    padding: USides::axes(10.0, 5.0),
                    border_radius: UCornerRadius::top(8.0), 
                    ..default()
                },
                ULayout {
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                }
            )).with_children(|header| {
                // العنوان
                header.spawn((
                    UTextLabel {
                        text: title.into(),
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));

                // زر إعدادات صغير في الرأس
                header.spawn((
                    UIconButton {
                        icon: Icon::MOVE,
                        background: Color::NONE,
                        hover_color: Color::NONE,
                        pressed_color: Color::NONE,
                        ..default()
                    },
                    // UInteraction::default()
                    // هنا يمكن وضع أيقونة "ترس" لاحقاً
                ));
            });

            // B. الجسم (Body)
            node_parent.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::all(10.0),
                    ..default()
                },
                
                ULayout {
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                }
            )).with_children(|body| {
                
                // المدخلات (يسار)
                body.spawn((
                    UNode { width: UVal::Auto, ..default() },
                    ULayout { flex_direction: UFlexDirection::Column, gap: 10.0, ..default() }
                )).with_children(|col| {
                    for i in 0..inputs {
                        spawn_port(col, true, format!("In {}", i));
                    }
                });

                // المخرجات (يمين)
                body.spawn((
                    UNode { width: UVal::Auto, ..default() },
                    ULayout { flex_direction: UFlexDirection::Column, gap: 10.0, align_items: UAlignItems::End, ..default() }
                )).with_children(|col| {
                    for i in 0..outputs {
                        spawn_port(col, false, format!("Out {}", i));
                    }
                });
            });
        });
    }).id()
}

fn spawn_port(parent: &mut ChildSpawnerCommands, is_input: bool, label: String) {
    parent.spawn((
        UNode {
            margin: USides::bottom(2.0),
            ..default()
        },
        ULayout {
            flex_direction: if is_input { UFlexDirection::Row } else { UFlexDirection::RowReverse },
            align_items: UAlignItems::Center,
            gap: 8.0,
            ..default()
        }
    )).with_children(|row| {
        row.spawn((
            UNode {
                width: UVal::Px(12.0),
                height: UVal::Px(12.0),
                background_color: if is_input { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgb(0.2, 0.6, 1.0) },
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UInteraction::default(),
        ));     

        row.spawn((
            UTextLabel {
                text: label,
                color: Color::srgb(0.8, 0.8, 0.8),
                font_size: 11.0,
                ..default()
            },
        ));
    });
}