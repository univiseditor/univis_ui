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
    let font = Handle::<Font>::default();

    // 1. الجذر (Root)
    commands.spawn((
        UWorldRoot { size: Vec2::new(1280.0, 720.0), ..default() },
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.1, 0.11, 0.13),
            padding: USides::all(20.0),
            ..default()
        },
        // سنستخدم Flex عمودي لتقسيم الشاشة إلى صفين
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column, 
            gap: 20.0,
            ..default()
        }
    )).with_children(|root| {
        
        // --- الصف الأول (Row 1) ---
        root.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Flex(1.0), // يأخذ نصف المساحة
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row, // ترتيب أفقي
                gap: 20.0,
                // توزيع البطاقات بالتساوي
                justify_content: UJustifyContent::SpaceEvenly, 
                ..default()
            }
        )).with_children(|row| {
            // Card 1
            spawn_demo_card(row, "Center / Center", UJustifyContent::Center, UAlignItems::Center, UFlexDirection::Row, font.clone());
            // Card 2
            spawn_demo_card(row, "SpaceBetween / Center", UJustifyContent::SpaceBetween, UAlignItems::Center, UFlexDirection::Row, font.clone());
            // Card 3
            spawn_demo_card(row, "End / End", UJustifyContent::End, UAlignItems::End, UFlexDirection::Row, font.clone());
        });

        // --- الصف الثاني (Row 2) ---
        root.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Flex(1.0), // يأخذ النصف الآخر
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 20.0,
                justify_content: UJustifyContent::SpaceEvenly,
                ..default()
            }
        )).with_children(|row| {
            // Card 4
            spawn_demo_card(row, "Column: Center / Start", UJustifyContent::Center, UAlignItems::Start, UFlexDirection::Column, font.clone());
            // Card 5
            spawn_demo_card(row, "SpaceAround / Stretch", UJustifyContent::SpaceAround, UAlignItems::Stretch, UFlexDirection::Row, font.clone());
            // Card 6
            spawn_demo_card(row, "Start / Start", UJustifyContent::Start, UAlignItems::Start, UFlexDirection::Row, font.clone());
        });
    });
}

// دالة البطاقة (لم تتغير، المنطق سليم)
fn spawn_demo_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    justify: UJustifyContent,
    align: UAlignItems,
    direction: UFlexDirection,
    font: Handle<Font>
) {
    parent.spawn((
        UNode {
            // تحديد عرض ثابت للبطاقة لضمان ظهورها
            width: UVal::Px(300.0), 
            height: UVal::Percent(1.0), // تملأ ارتفاع الصف
            padding: USides::all(15.0),
            background_color: Color::srgb(0.2, 0.22, 0.25),
            border_radius: UCornerRadius::all(12.0),
            ..default()
        },
        UBorder { width: 1.0, color: Color::WHITE.with_alpha(0.1), ..default() },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
            ..default()
        }
    )).with_children(|card| {
        
        // العنوان
        card.spawn(UTextLabel {
            text: title.to_string(),
            font_size: 16.0,
            color: Color::srgb(0.6, 0.8, 1.0),
            font: font,
            ..default()
        });

        // صندوق الاختبار
        card.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Flex(1.0), // يملأ باقي البطاقة
                background_color: Color::BLACK.with_alpha(0.3),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: direction,
                justify_content: justify,
                align_items: align,
                gap: 5.0,
                ..default()
            }
        )).with_children(|container| {
            // المربعات الصغيرة
            let h1 = if align == UAlignItems::Stretch && direction == UFlexDirection::Row { UVal::Auto } else { UVal::Px(30.0) };
            let h2 = if align == UAlignItems::Stretch && direction == UFlexDirection::Row { UVal::Auto } else { UVal::Px(45.0) };
            let h3 = if align == UAlignItems::Stretch && direction == UFlexDirection::Row { UVal::Auto } else { UVal::Px(30.0) };

            container.spawn(UNode { width: UVal::Px(30.0), height: h1, background_color: Color::srgb(1.0, 0.4, 0.4), border_radius: UCornerRadius::all(4.0), ..default() });
            container.spawn(UNode { width: UVal::Px(30.0), height: h2, background_color: Color::srgb(0.4, 1.0, 0.4), border_radius: UCornerRadius::all(4.0), ..default() });
            container.spawn(UNode { width: UVal::Px(30.0), height: h3, background_color: Color::srgb(0.4, 0.6, 1.0), border_radius: UCornerRadius::all(4.0), ..default() });
        });
    });
}