use bevy::{post_process::bloom::Bloom, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup_profile_card)
        .run();
}

fn setup_profile_card(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. كاميرا 2D عادية (لا نحتاج إضاءة)
    commands.spawn((Camera2d,Bloom::NATURAL));

    // 2. جذر الشاشة (UScreenRoot)
    // هذا المكون يملأ نافذة اللعبة تلقائياً ويعمل كحاوية رئيسية
    commands.spawn((
        UScreenRoot,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.1, 0.1, 0.12), // خلفية داكنة للشاشة كاملة
            ..default()
        },
        // نستخدم Flex لتوسيط البطاقة في منتصف الشاشة تماماً
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center, // توسيط أفقي
            align_items: UAlignItems::Center,         // توسيط عمودي
            ..default()
        }
    ))
    .with_children(|screen| {
        
        // =========================================================
        // 3. البطاقة (The Card Container)
        // =========================================================
        screen.spawn((
            UNode {
                width: UVal::Px(340.0),      // عرض ثابت للبطاقة
                height: UVal::Auto,          // الارتفاع يتمدد حسب المحتوى
                padding: USides::all(24.0),  // حشوة داخلية
                margin: USides::all(0.0),
                background_color: Color::srgb(0.18, 0.18, 0.32), // لون البطاقة (رمادي مزرق)
                border_radius: UCornerRadius::all(24.0),         // زوايا دائرية ناعمة
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column, // العناصر فوق بعضها
                align_items: UAlignItems::Center,       // توسيط المحتوى داخل البطاقة
                gap: 16.0,                              // مسافة بين العناصر
                ..default()
            },
            // إضافة ظل بسيط (محاكاة عبر حدود شفافة أو لون داكن) - اختياري
        )).with_children(|card| {

            // --- A. الصورة الشخصية (Avatar) ---
            // نستخدم دائرة بسيطة لتمثيل الصورة
            card.spawn((
                // لو أردت وضع صورة حقيقية:
                UImage {
                    width: UVal::Px(100.0),
                    height: UVal::Px(100.0),
                    radius: Some(UCornerRadius::all(50.0)),
                    texture: asset_server.load("profile.png"),
                    ..default()
                },
                UBorder {
                    color: Color::srgb(2.0, 2.0, 2.0),
                    width: 2.0,
                    offset: 7.0,
                    ..default()
                }
            ));

            // --- B. الاسم واللقب ---
            card.spawn((
                UNode { width: UVal::Auto, height: UVal::Auto, ..default() },
                ULayout { flex_direction: UFlexDirection::Column, align_items: UAlignItems::Center, gap: 4.0, ..default() }
            )).with_children(|text_container| {
                // الاسم
                text_container.spawn(UTextLabel {
                    text: "Abdellah Developer".into(),
                    font_size: 22.0,
                    color: Color::WHITE,
                    ..default()
                });
                // الوظيفة
                text_container.spawn(UTextLabel {
                    text: "@ab_ht".into(),
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.6, 0.65), // رمادي فاتح
                    ..default()
                });
            });

            // --- C. التصنيفات (Tags/Badges) ---
            card.spawn((
                UNode { width: UVal::Percent(1.0), height: UVal::Auto, ..default() },
                ULayout { 
                    display: UDisplay::Flex, 
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Center, 
                    gap: 8.0, 
                    ..default() 
                }
            )).with_children(|tags| {
                // استخدام نظام UBadge الموجود في ملف badge.txt
                spawn_badge(tags, "Rust", BadgeStyle::Warning);
                spawn_badge(tags, "Bevy", BadgeStyle::Info);
                spawn_badge(tags, "UI/UX", BadgeStyle::Success);
            });

            // --- D. الإحصائيات (Stats Row) ---
            card.spawn((
                UNode { 
                    width: UVal::Percent(1.0), 
                    height: UVal::Auto,
                    background_color: Color::BLACK.with_alpha(0.2), // خلفية خفيفة للإحصائيات
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::axes(0.0, 12.0), // حشوة رأسية فقط
                    ..default() 
                },
                ULayout { 
                    display: UDisplay::Flex, 
                    flex_direction: UFlexDirection::Row, 
                    justify_content: UJustifyContent::SpaceEvenly, // توزيع متساوي
                    ..default() 
                }
            )).with_children(|stats| {
                spawn_stat(stats, "125", "Posts");
                spawn_stat(stats, "4.2k", "Followers");
                spawn_stat(stats, "350", "Following");
            });

            // --- E. أزرار الإجراءات (Action Buttons) ---
            card.spawn((
                UNode { width: UVal::Percent(1.0), height: UVal::Auto, ..default() },
                ULayout { 
                    display: UDisplay::Flex, 
                    gap: 10.0, 
                    ..default() 
                }
            )).with_children(|actions| {
                // زر أساسي (يأخذ المساحة المتبقية بفضل FlexGrow)
                actions.spawn((
                    UButton::primary(),
                    UFlexGrow(1.0), // تمدد
                    ULayout { justify_content: UJustifyContent::Center, ..default() }
                )).with_children(|btn| {
                    btn.spawn(UTextLabel { text: "Follow".into(), font_size: 16.0, ..default() });
                });

                // زر ثانوي
                actions.spawn((
                    UButton::secondary(),
                    UFlexGrow(1.0), // تمدد
                    ULayout { justify_content: UJustifyContent::Center, ..default() }
                )).with_children(|btn| {
                    btn.spawn(UTextLabel { text: "Message".into(), font_size: 16.0, ..default() });
                });
            });

        });
    });
}

// --- دوال مساعدة لترتيب الكود ---

fn spawn_badge(parent: &mut ChildSpawnerCommands, text: &str, style: BadgeStyle) {
    parent.spawn((
        UBadge { style, size: BadgeSize::Small }, // المكون الذي يطبق التنسيق تلقائياً
        // نحتاج UNode و ULayout لأن UBadge يضيف الخصائص لهما ولا ينشئهما
        UNode::default(), 
        ULayout::default()
    )).with_children(|b| {
        b.spawn(UTextLabel {
            text: text.to_string(),
            font_size: 12.0,
            color: Color::WHITE,
            ..default()
        });
    });
}

fn spawn_stat(parent: &mut ChildSpawnerCommands, value: &str, label: &str) {
    parent.spawn((
        UNode::default(),
        ULayout { flex_direction: UFlexDirection::Column, align_items: UAlignItems::Center, ..default() }
    )).with_children(|col| {
        col.spawn(UTextLabel {
            text: value.to_string(),
            font_size: 18.0,
            color: Color::WHITE,
            ..default()
        });
        col.spawn(UTextLabel {
            text: label.to_string(),
            font_size: 12.0,
            color: Color::srgb(0.6, 0.6, 0.6),
            ..default()
        });
    });
}