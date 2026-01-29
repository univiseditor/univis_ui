use bevy::prelude::*;
use crate::prelude::*;


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

use bevy::input::mouse::MouseWheel;

pub fn scroll_interaction_system(
    // 1. قراءة عجلة الماوس
    mut mouse_wheel: MessageReader<MouseWheel>,
    
    // 2. الاستعلام عن الحاويات
    // الشرط: يجب أن يكون لديها UScrollContainer و UInteraction
    containers: Query<(
        &UInteraction,      // نستخدم هذا بدلاً من حسابات الماوس اليدوية
        &UScrollContainer, 
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

    for (interaction, container, size, children) in containers.iter() {
        
        // === هنا السحر: استخدام UInteraction ===
        // بدلاً من الحسابات المعقدة، نسأل فقط: هل الماوس فوق الحاوية؟
        if *interaction == UInteraction::Hovered {
            
            // العثور على المحتوى (أول ابن)
            if let Some(&content_entity) = children.first() {
                if let Ok((mut uself, content_size)) = content_query.get_mut(content_entity) {
                    
                    let mut current_top = match uself.top { UVal::Px(v) => v, _ => 0.0 };
                    
                    if container.vertical {
                        // تحريك للأعلى/الأسفل
                        current_top += scroll_delta.y * container.scroll_speed;
                        
                        // === منطق الحدود (Clamping) ===
                        // الفائض = (طول المحتوى - طول النافذة)
                        let overflow = (content_size.height - size.height).max(0.0);
                        
                        // الحدود: من (-الفائض) إلى (0)
                        let min_top = -overflow;
                        let max_top = 0.0;

                        current_top = current_top.clamp(min_top, max_top);
                        
                        uself.top = UVal::Px(current_top);
                    }
                }
            }
        }
    }
}