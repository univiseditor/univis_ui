use bevy::prelude::*;
use crate::prelude::*;

// 1. المكون
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct UProgressBar {
    pub value: f32, // من 0.0 إلى 1.0
    pub bar_color: Color,
}

impl Default for UProgressBar {
    fn default() -> Self {
        Self { value: 0.5, bar_color: Color::srgb(0.2, 0.8, 0.2) }
    }
}



pub struct UnivisProgressPlugin;

impl Plugin for UnivisProgressPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_progress_bars);
    }
}

// علامة للطفل الداخلي (الشريط الملون)
#[derive(Component)]
struct ProgressBarFill;

// 2. النظام لتحديث الشكل
fn update_progress_bars(
    mut commands: Commands,
    query: Query<(Entity, &UProgressBar, Option<&Children>), Changed<UProgressBar>>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<ProgressBarFill>>,
) {
    for (entity, bar, children_opt) in query.iter() {
        // تأكد من تهيئة الحاوية (الأب)
        commands.entity(entity).insert(UNode {
            height: UVal::Px(10.0), // ارتفاع افتراضي
            width: UVal::Percent(1.0), // عرض كامل
            background_color: Color::BLACK.with_alpha(0.3),
            border_radius: UCornerRadius::all(5.0),
            padding: USides::all(0.0), // لا هوامش
            ..default()
        });

        let mut fill_found = false;
        
        // ابحث عن الطفل المسؤول عن الامتلاء
        if let Some(children) = children_opt {
            for &child in children {
                if let Ok((mut node, mut vis)) = fill_query.get_mut(child) {
                    // تحديث العرض بناءً على القيمة
                    let clamped = bar.value.clamp(0.0, 1.0);
                    node.width = UVal::Percent(clamped);
                    node.background_color = bar.bar_color;
                    
                    // إخفاء الشريط إذا كانت القيمة 0
                    *vis = if clamped > 0.001 { Visibility::Inherited } else { Visibility::Hidden };
                    
                    fill_found = true;
                    break;
                }
            }
        }

        // إذا لم يكن موجوداً، قم بإنشائه (Lazy Initialization)
        if !fill_found {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    UNode {
                        width: UVal::Percent(bar.value.clamp(0.0, 1.0)),
                        height: UVal::Percent(1.0), // ارتفاع كامل للأب
                        background_color: bar.bar_color,
                        border_radius: UCornerRadius::all(5.0),
                        ..default()
                    },
                    ProgressBarFill,
                ));
            });
        }
    }
}