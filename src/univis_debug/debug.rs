use bevy::prelude::*;
use crate::prelude::*;

// مكون نضعه على الكيان الذي نريد تتبع حالته
#[derive(Component)]
pub struct DebugOverlay {
    pub show_padding: bool,
    pub show_content: bool,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self { show_padding: true, show_content: true }
    }
}

pub fn draw_debug_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &ComputedSize, &UNode, Option<&DebugOverlay>)>,
) {
    for (global_transform, computed, node, debug_opt) in query.iter() {
        // إذا لم يكن هناك مكون ديباج، أو كان موجوداً، نستخدمه. 
        // هنا سأفترض أننا نريد رسم الجميع للتجربة، أو يمكن حصرها بمن يملك المكون.
        if debug_opt.is_none() { continue; } 

        let pos = global_transform.translation();
        let size = Vec2::new(computed.width, computed.height);
        
        // 1. رسم الحدود الخارجية (الحجم الكلي المحسوب) - اللون الأبيض
        gizmos.rect_2d(
            Isometry2d::from_translation(pos.truncate()), // تحويل 3D لـ 2D
            size,
            Color::WHITE.with_alpha(0.5)
        );

        // 2. رسم حدود المحتوى (بعد خصم البادينغ) - اللون الأخضر
        // هذا يساعدنا لنرى "أين مسموح للأطفال بالظهور"
        let padding_w = node.padding.left + node.padding.right;
        let padding_h = node.padding.top + node.padding.bottom;
        
        // حجم المحتوى الصافي
        let content_size = (size - Vec2::new(padding_w, padding_h)).max(Vec2::ZERO);
        
        // يجب إزاحة مركز المحتوى لأن البادينغ قد لا يكون متساوياً (مثلاً Top 20 و Bottom 0)
        // الإزاحة = (Left - Right) / 2 , (Bottom - Top) / 2 ... انتبه لمحاور Bevy
        let offset_x = (node.padding.left - node.padding.right) / 2.0;
        let offset_y = (node.padding.bottom - node.padding.top) / 2.0; // Y للأعلى في Bevy

        let content_pos = pos.truncate() + Vec2::new(offset_x, offset_y);

        gizmos.rect_2d(
            Isometry2d::from_translation(content_pos),
            content_size,
            Color::Srgba(GREEN).with_alpha(0.8)
        );
        
        // 3. رسم نقطة المركز (Pivot) - أحمر
        gizmos.circle_2d(Isometry2d::from_translation(pos.truncate()), 2.0, Color::Srgba(RED));
    }
}


pub fn input_layout_switcher(
    keyboard: Res<ButtonInput<KeyCode>>,
    // نستعلم عن الكيانات التي نحددها كـ "جذور قابلة للاختبار"
    mut query: Query<&mut ULayout, With<DebugOverlay>>,
) {
    let mut new_display = None;

    if keyboard.just_pressed(KeyCode::Digit1) { new_display = Some(UDisplay::Flex); }
    if keyboard.just_pressed(KeyCode::Digit2) { new_display = Some(UDisplay::Grid); }
    if keyboard.just_pressed(KeyCode::Digit3) { new_display = Some(UDisplay::Masonry); }
    if keyboard.just_pressed(KeyCode::Digit4) { new_display = Some(UDisplay::Radial); }
    if keyboard.just_pressed(KeyCode::Digit5) { new_display = Some(UDisplay::Stack); }

    if let Some(display) = new_display {
        for mut layout in query.iter_mut() {
            layout.display = display;
            // يمكنك هنا أيضاً تغيير grid_columns عشوائياً للتجربة
            if display == UDisplay::Grid || display == UDisplay::Masonry {
                layout.grid_columns = 3; 
            }
            println!("Switched Layout to: {:?}", display);
        }
    }
}


pub struct UnivisDebugPlugin;

impl Plugin for UnivisDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                draw_debug_gizmos,
                input_layout_switcher
            ));
    }
}

