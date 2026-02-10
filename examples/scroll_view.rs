use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Update, scroll_interaction_system)
        .add_systems(Startup, setup_scroll_list)
        .run();
}

fn setup_scroll_list(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 1. الحاوية الخارجية (التي تقص)
    commands
        .spawn((
            UWorldRoot::default(),
            // تفعيل القص
            UClip { enabled: true },
            // تفعيل منطق التمرير
            UScrollContainer::new(),
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(400.0),
                background_color: Color::BLACK,
                border_radius: UCornerRadius::all(15.0),
                // مهم: Overflow Hidden منطقياً
                ..default()
            },
            // التفاعل ضروري لاكتشاف الـ Hover
            UInteraction::default(),
            // موقع الحاوية في الشاشة
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(100.0),
                top: UVal::Px(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // 2. المحتوى الداخلي (القائمة الطويلة)
            parent
                .spawn((
                    // هذا العنصر هو الذي سيتحرك (يتم تغيير Top الخاص به)
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Content, // ارتفاعه حسب محتواه
                        background_color: Color::srgb(0.1, 0.1, 0.1),
                        padding: USides::all(10.0),
                        ..default()
                    },
                    ULayout {
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                        ..default()
                    },
                    // يجب أن يكون Relative ليتحرك بالنسبة للأب
                ))
                .with_children(|list| {
                    // إضافة 20 عنصر
                    for i in 0..20 {
                        list.spawn((UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(50.0),
                            background_color: Color::srgb(0.2, 0.2, 0.25),
                            border_radius: UCornerRadius::all(5.0),
                            padding: USides::all(10.0),
                            ..default()
                        },))
                            .with_children(|item| {
                                item.spawn((
                                    UTextLabel::new(format!("Item #{}", i + 1).as_str()),
                                    
                                ));
                            });
                    }
                });
        });
}
