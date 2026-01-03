use bevy::{color::palettes::css::*, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup_node_showcase)
        .run();
}


fn setup_node_showcase(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // سنقوم بإنشاء عقدة "Math Node" كلاسيكية
    let node_pos = Vec3::new(0.0, 0.0, 0.0);

    // 1. الحاوية الرئيسية للعقدة (The Container)
    commands.spawn((
        UWorldRoot { size: Vec2::new(220.0, 300.0), ..default() }, // الحجم المبدئي
        Transform::from_translation(node_pos),
        UNode {
            width: UVal::Px(220.0),
            height: UVal::Px(300.0),
            background_color: Color::srgb(0.12, 0.12, 0.12), // رمادي غامق جداً
            border_radius: UCornerRadius::all(12.0),
            // إضافة ظل وهمي عبر حدود داكنة (اختياري)
            ..default()
        },
        UBorder {
            width: 1.0,
            color: Color::BLACK, // حد أسود رقيق
            radius: UCornerRadius::all(12.0),
            ..default()
        },
        ULayout {
            flex_direction: UFlexDirection::Column,
            ..default() // gap 0.0 للصق الرأس بالجسم
        }
    )).with_children(|node| {

        // =========================================================
        // 2. الرأس (Header)
        // =========================================================
        node.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(32.0),
                background_color: Color::srgb(0.2, 0.6, 0.8), // أزرق (للدلالة على نوع العقدة مثلاً)
                border_radius: UCornerRadius::top(12.0), // انحناء علوي فقط
                padding: USides::column(10.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center, // توسيط النص عمودياً
                justify_content: UJustifyContent::SpaceBetween, // للنص وزر الإغلاق
                ..default()
            }
        )).with_children(|header| {
            // العنوان
            header.spawn((UTextLabel {
                text: "Mix Vector".into(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },UNode::default()));

            // زر إعدادات صغير في الرأس
            header.spawn((
                UNode { width: UVal::Px(12.0), height: UVal::Px(12.0), background_color: Color::NONE, ..default() },
                // هنا يمكن وضع أيقونة "ترس" لاحقاً
            ));
        });

        // =========================================================
        // 3. الجسم (Body) - صف يحتوي على عمودين
        // =========================================================
        node.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Content, // يتمدد حسب عدد المداخل/المخارج
                background_color: Color::NONE,
                padding: USides::all(10.0), // هوامش داخلية
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Row, // ترتيب أفقي (مداخل يسار | مخارج يمين)
                justify_content: UJustifyContent::SpaceBetween, // ادفعهم للأطراف
                ..default()
            }
        )).with_children(|body| {

            // --- العمود الأيسر (Inputs) ---
            body.spawn((
                UNode { width: UVal::Content, height: UVal::Content, background_color: Color::NONE, ..default() },
                ULayout { flex_direction: UFlexDirection::Column, gap: 12.0, ..default() }
            )).with_children(|inputs| {
                spawn_socket(inputs, "Vector A", Color::srgb(0.8, 0.3, 0.8), true); // Socket بنفسجي
                spawn_socket(inputs, "Vector B", Color::srgb(0.8, 0.3, 0.8), true);
                spawn_socket(inputs, "Factor", Color::Srgba(GRAY), true); // Socket رمادي (Float)
            });

            // body.spawn({
            //     UNode {
            //         width: UVal::Flex(1.0),
            //         height: UVal::Content,
            //         ..Default::default()
            //     }
            // });

            // --- العمود الأيمن (Outputs) ---
            body.spawn((
                UNode { width: UVal::Content, height: UVal::Content, background_color: Color::NONE, ..default() },
                ULayout { flex_direction: UFlexDirection::Column, gap: 12.0, align_items: UAlignItems::End, ..default() } // محاذاة لليمين
            )).with_children(|outputs| {
                spawn_socket(outputs, "Result", Color::srgb(0.8, 0.8, 0.3), false); // Socket أصفر
            });
        });

        // =========================================================
        // 4. المحتوى الإضافي (Extra Widget) في الأسفل
        // =========================================================
        node.spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(40.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.3), // خلفية داكنة للتحكم
                border_radius: UCornerRadius::bottom(12.0), // انحناء سفلي
                padding: USides::all(8.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            }
        )).with_children(|footer| {
            // محاكاة Checkbox
            footer.spawn(UNode {
                width: UVal::Px(16.0),
                height: UVal::Px(16.0),
                background_color: Color::srgb(0.3, 0.3, 0.3),
                border_radius: UCornerRadius::all(3.0),
                ..default()
            });
            
            // نص بجانبه
            footer.spawn((
                UNode { width: UVal::Px(5.0), height: UVal::Px(1.0), ..default() },
                ULayout::default()
            )); // Spacer
            
            footer.spawn(UTextLabel {
                text: "Clamp Result".into(),
                font_size: 12.0,
                color: Color::Srgba(GRAY),
                ..default()
            });
        });
    });
}

// --- دالة مساعدة لرسم المقابس (Sockets) ---
fn spawn_socket(parent: &mut ChildSpawnerCommands, label: &str, color: Color, is_input: bool) {
    parent.spawn((
        UNode {
            width: UVal::Content,
            height: UVal::Px(14.0), // ارتفاع السطر
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center, // محاذاة الدائرة مع النص
            gap: 8.0,
            ..default()
        }
    )).with_children(|row| {
        // شكل الدائرة (Socket Circle)
        let socket_shape = |p: &mut ChildSpawnerCommands| {
            p.spawn(UNode {
                width: UVal::Px(12.0),
                height: UVal::Px(12.0),
                background_color: color,
                border_radius: UCornerRadius::all(6.0), // دائرة كاملة
                ..default()
            });
        };

        // النص (Label)
        let socket_text = |p: &mut ChildSpawnerCommands| {
            p.spawn((
                UNode {
                    width: UVal::Content,
                    height: UVal::Content,
                    ..Default::default()
                },
                UTextLabel {
                    text: label.into(),
                    font_size: 13.0,
                    color: Color::srgb(0.8, 0.8, 0.8), // رمادي فاتح
                    ..default()
                }
            ));
        };

        if is_input {
            // المدخل: الدائرة يساراً، النص يميناً
            socket_shape(row);
            socket_text(row);
        } else {
            // المخرج: النص يساراً، الدائرة يميناً
            socket_text(row);
            socket_shape(row);
        }
    });
}