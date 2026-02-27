use bevy::prelude::*;
use crate::prelude::*;
use bevy::input::mouse::MouseWheel;

pub struct UnivisScrollViewPlugin;

impl Plugin for UnivisScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UScrollContainer>()
            .add_systems(Update, scroll_interaction_system);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct UScrollContainer {
    pub scroll_speed: f32,
    pub vertical: bool,
    pub horizontal: bool,
    // الحالة الحالية (للاستخدام الداخلي)
    pub offset: Vec2, 
}

impl UScrollContainer {
    pub fn new() -> Self {
        Self {
            scroll_speed: 30.0,
            vertical: true,
            horizontal: false,
            offset: Vec2::ZERO,
        }
    }
}

pub fn scroll_interaction_system(
    // 1. قراءة عجلة الماوس
    mut mouse_wheel: MessageReader<MouseWheel>,
    
    // 2. الاستعلام عن الحاويات
    // الشرط: يجب أن يكون لديها UScrollContainer و UInteraction
    mut containers: Query<(
        &UInteraction,      // نستخدم هذا بدلاً من حسابات الماوس اليدوية
        &mut UScrollContainer, 
        &ComputedSize, 
        &Children
    )>,
    
    // 3. الاستعلام عن المحتوى (لتحديث موقعه)
    mut content_query: Query<(&mut USelf, &ComputedSize)>,
) {
    // جمع حركة العجلة
    let mut scroll_delta = Vec2::ZERO;
    for ev in mouse_wheel.read() {
        scroll_delta.y += ev.y;
        scroll_delta.x += ev.x;
    }

    // إذا لم يحرك المستخدم العجلة، لا نفعل شيئاً
    if scroll_delta == Vec2::ZERO {
        return;
    }

    for (interaction, mut container, size, children) in containers.iter_mut() {
        
        // === هنا السحر: استخدام UInteraction ===
        // بدلاً من الحسابات المعقدة، نسأل فقط: هل الماوس فوق الحاوية؟
        if *interaction != UInteraction::Hovered {
            continue;
        }
            
        // العثور على المحتوى (أول ابن)
        let Some(&content_entity) = children.first() else {
            continue;
        };

        let Ok((mut uself, content_size)) = content_query.get_mut(content_entity) else {
            continue;
        };

        let speed = container.scroll_speed.max(0.0);
        let overflow_y = (content_size.height - size.height).max(0.0);
        let overflow_x = (content_size.width - size.width).max(0.0);

        if container.vertical {
            container.offset.y = clamp_scroll_offset(
                container.offset.y,
                scroll_delta.y * speed,
                overflow_y,
            );
            uself.top = UVal::Px(container.offset.y);
        }

        if container.horizontal {
            container.offset.x = clamp_scroll_offset(
                container.offset.x,
                scroll_delta.x * speed,
                overflow_x,
            );
            uself.left = UVal::Px(container.offset.x);
            uself.position_type = UPositionType::Relative;
        }
    }
}

fn clamp_scroll_offset(current: f32, delta: f32, overflow: f32) -> f32 {
    if overflow <= 0.0 {
        0.0
    } else {
        (current + delta).clamp(-overflow, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_scroll_offset_caps_to_range() {
        assert_eq!(clamp_scroll_offset(0.0, -30.0, 120.0), -30.0);
        assert_eq!(clamp_scroll_offset(-100.0, -80.0, 120.0), -120.0);
        assert_eq!(clamp_scroll_offset(-20.0, 60.0, 120.0), 0.0);
    }

    #[test]
    fn clamp_scroll_offset_returns_zero_without_overflow() {
        assert_eq!(clamp_scroll_offset(-50.0, -20.0, 0.0), 0.0);
        assert_eq!(clamp_scroll_offset(10.0, 15.0, -5.0), 0.0);
    }
}
