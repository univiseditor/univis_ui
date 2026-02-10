use bevy::prelude::*;
use univis_ui::prelude::*; // استيراد المكتبة

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin) // 1. تفعيل الـ Plugin
        .add_systems(Startup, setup)
        .add_systems(Update, change_text_label)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // 2. إنشاء "جذر" للواجهة (Canvas)
    // هذا الكيان يحدد حجم مساحة العمل
    commands.spawn((
        UScreenRoot,
        // 3. إضافة خصائص الشكل (UNode)
        UNode {
            width: UVal::Percent(1.0),  // عرض كامل
            height: UVal::Percent(1.0), // ارتفاع كامل
            background_color: Color::srgb(0.1, 0.1, 0.15),
            ..default()
        },
        // 4. إضافة خصائص التخطيط (ULayout)
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center, // توسيط المحتوى
            align_items: UAlignItems::Center,
            ..default()
        }
    ))
    .with_children(|parent| {
        // 5. إضافة عنصر ابن (زر/بطاقة)
        parent.spawn((
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(100.0),
                background_color: Color::srgb(0.2, 0.6, 1.0),
                border_radius: UCornerRadius::all(20.0), // زوايا دائرية
                padding: USides::all(10.0),
                ..default()
            },
            ULayout {
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
            // إضافة حدود متوهجة
            UBorder {
                width: 4.0,
                color: Color::WHITE,
                ..default()
            }
        )).with_children(|card| {
            // 6. إضافة نص
            card.spawn((
                UTextLabel {
                text: "0.0".into(),
                font_size: 32.0,
                color: Color::WHITE,
                autosize: true, // النص يضبط حجم الحاوية تلقائياً
                ..default()
            },
            ));
        });
    });
}

fn change_text_label(
    mut query: Query<&mut UTextLabel>
) {
    let mut n = 1.0; 
    for mut text in query.iter_mut() {
        if text.text.parse::<f32>().is_ok() {
            n = n + text.text.parse::<f32>().unwrap();
        } else {
            n = 1.0;
        }
        
        text.text = format!("{}",n);
        println!("{}",n);
    }
}