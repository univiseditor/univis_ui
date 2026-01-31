use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup_interaction_test)
        .run();
}

fn setup_interaction_test(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // 1. حاوية رئيسية (World Root)
    commands.spawn((
        UWorldRoot {
            size: Vec2::new(1200.0, 800.0),
            is_3d: false,
            ..default()
        },
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.05, 0.05, 0.05), // خلفية داكنة
            padding: USides::all(50.0),
            ..default()
        },
        ULayout {
            // توزيع البطاقات بجانب بعضها
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceEvenly,
            align_items: UAlignItems::Center,
            gap: 20.0,
            ..default()
        }
    ))
    .with_children(|root| {

        // =========================================================
        // الحالة 1: التجاهل (Standard Button)
        // الأب يتفاعل، والابن (النص) يتم تجاهله تماماً
        // =========================================================
        spawn_test_card(
            root,
            "1. Standard Button\n(Child Ignores)",
            "The Text is 'Pickable::IGNORE'.\nClicking text clicks the Button.",
            |card| {
                // الأب (الزر)
                card.spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(60.0),
                        background_color: Color::srgb(0.2, 0.6, 1.0), // أزرق
                        border_radius: UCornerRadius::all(10.0),
                        ..default()
                    },
                    ULayout { justify_content: UJustifyContent::Center, align_items: UAlignItems::Center, ..default() },
                    
                    UInteractionColors {
                        normal: Color::srgb(0.2, 0.6, 1.0),
                        hovered: Color::srgb(0.3, 0.7, 1.0),
                        pressed: Color::WHITE,
                    }
                )).with_children(|btn| {
                    // الابن (النص)
                    btn.spawn((
                        UTextLabel::new("Click Me"),
                        TextColor(Color::BLACK),
                        // --- السر هنا ---
                        // هذا يجعل الماوس يمر عبر النص وكأنه غير موجود
                        Pickable::IGNORE, 
                    ));
                });
            }
        );

        // =========================================================
        // الحالة 2: الحجب (Nested Blocking)
        // الابن يسرق التفاعل من الأب
        // =========================================================
        spawn_test_card(
            root,
            "2. Nested Blocking\n(Child Blocks)",
            "Parent is Red. Child is Green.\nClicking Green DOES NOT trigger Red.",
            |card| {
                // الأب (اللوحة الحمراء)
                card.spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(200.0),
                        background_color: Color::srgb(0.8, 0.2, 0.2), // أحمر
                        border_radius: UCornerRadius::all(10.0),
                        padding: USides::all(20.0),
                        ..default()
                    },
                    UInteractionColors {
                        normal: Color::srgb(0.8, 0.2, 0.2),
                        hovered: Color::srgb(0.9, 0.3, 0.3), // يضيء عند المرور
                        pressed: Color::srgb(0.6, 0.1, 0.1),
                    }
                )).with_children(|parent| {
                    // الابن (الزر الأخضر)
                    parent.spawn((
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(50.0),
                            background_color: Color::srgb(0.2, 0.8, 0.2), // أخضر
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },


                        UInteractionColors {
                            normal: Color::srgb(0.2, 0.8, 0.2),
                            hovered: Color::srgb(0.3, 0.9, 0.3),
                            pressed: Color::WHITE,
                        }
                    ));
                    
                    parent.spawn((
                        UTextLabel::new("Parent Area"),
                    ));
                });
            }
        );

        // =========================================================
        // الحالة 3: التمرير (Passthrough / Bubbling)
        // كلاهما يتفاعل في نفس الوقت
        // =========================================================
        spawn_test_card(
            root,
            "3. Passthrough\n(Both React)",
            "Child has 'should_block_lower: false'.\nHovering Child lights up BOTH.",
            |card| {
                // الأب (اللوحة الصفراء)
                card.spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(200.0),
                        background_color: Color::srgb(0.8, 0.8, 0.1), // أصفر
                        border_radius: UCornerRadius::all(10.0),
                        padding: USides::all(20.0),
                        ..default()
                    },
                    UInteractionColors {
                        normal: Color::srgb(0.8, 0.8, 0.1),
                        hovered: Color::srgb(1.0, 1.0, 0.5), // يضيء بقوة
                        pressed: Color::srgb(0.6, 0.6, 0.0),
                    }
                )).with_children(|parent| {
                    
                    // الابن (تراكب شفاف)
                    parent.spawn((
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Percent(1.0),
                            background_color: Color::srgb(1.0, 1.0, 1.0).with_alpha(0.1), // شفاف
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        // --- السر هنا ---
                        // اسمح للماوس بالمرور للأب، لكن تفاعل معي أيضاً

                        // تغيير لون الابن أيضاً لإثبات أنه تفاعل
                        UInteractionColors {
                            normal: Color::srgb(1.0, 1.0, 1.0).with_alpha(0.1),
                            hovered: Color::srgb(1.0, 1.0, 1.0).with_alpha(0.4), // يصبح أكثر بياضاً
                            pressed: Color::srgb(1.0, 1.0, 1.0).with_alpha(0.6),
                        }
                    )).with_children(|overlay| {
                         overlay.spawn((
                            UTextLabel::new("Hover Me!\n(I trigger Parent too)"),
                            // TextFont { font_size: 14.0, ..default() },
                            // TextColor(Color::BLACK),
                            USelf { align_self: UAlignSelf::Center, ..default() }
                        ));
                    });
                });
            }
        );

    });
}

// دالة مساعدة لرسم بطاقة الاختبار
fn spawn_test_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    desc: &str,
    content_fn: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent.spawn((
        UNode {
            width: UVal::Px(300.0),
            height: UVal::Px(400.0),
            background_color: Color::srgb(0.1, 0.1, 0.12),
            border_radius: UCornerRadius::all(15.0),
            padding: USides::all(15.0),
            margin: USides::all(10.0),
            ..default()
        },
        ULayout {
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 15.0,
            ..default()
        }
    )).with_children(|card| {
        // العنوان
        card.spawn((
            UTextLabel {
                text: title.to_string(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
            // TextFont { font_size: 20.0, ..default() },
            // TextColor(Color::WHITE),
            // USelf { margin: USides::bottom(10.0), ..default() }
            UNode {
                margin: USides::bottom(10.0),
                ..default()
            }
        ));

        // منطقة الاختبار (نستدعي الدالة الممررة)
        content_fn(card);

        // الوصف
        card.spawn((
            UTextLabel {
                text: desc.to_string(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
            // TextFont { font_size: 20.0, ..default() },
            // TextColor(Color::WHITE),
            // USelf { margin: USides::bottom(10.0), ..default() }
            UNode {
                margin: USides::bottom(10.0),
                ..default()
            }
        ));
    });
}