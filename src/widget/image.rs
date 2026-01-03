use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility)] 
pub struct UImage {
    pub texture: Handle<Image>,
    pub color: Color,
    
    // --- الحقول الجديدة ---
    /// العرض المطلوب. Auto تعني "استخدم حجم الصورة الأصلي".
    pub width: UVal,
    /// الارتفاع المطلوب. Auto تعني "استخدم حجم الصورة الأصلي".
    pub height: UVal,
    /// نصف القطر. إذا تم تحديده، سيفرض نفسه على UNode.
    pub radius: Option<UCornerRadius>,
}

impl Default for UImage {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            color: Color::WHITE,
            // الافتراضي: الحجم التلقائي (حجم الصورة)
            width: UVal::Auto, 
            height: UVal::Auto,
            radius: None,
        }
    }
}

impl UImage {
    /// إنشاء صورة سريعة
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            texture,
            ..default()
        }
    }

    /// تحديد حجم مخصص
    pub fn with_size(mut self, width: UVal, height: UVal) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// تحديد زوايا دائرية
    pub fn with_radius(mut self, radius: UCornerRadius) -> Self {
        self.radius = Some(radius);
        self
    }
}

pub fn sync_image_geometry(
    // نراقب تغيرات UImage أو UNode
    mut query: Query<(&UImage, &mut UNode), Or<(Changed<UImage>, Changed<UNode>)>>,
    // نحتاج الوصول للأصول لمعرفة حجم الصورة الحقيقي
    images: Res<Assets<Image>>, 
) {
    for (ui_image, mut node) in query.iter_mut() {
        
        // 1. مزامنة نصف القطر (إذا وجد)
        if let Some(r) = ui_image.radius {
            if node.border_radius != r {
                node.border_radius = r;
            }
        }

        // 2. منطق حساب الحجم
        // هل نحتاج للبحث عن الحجم الأصلي للصورة؟
        let needs_native_size = ui_image.width == UVal::Auto || ui_image.height == UVal::Auto;
        
        let mut native_size = Vec2::ZERO;
        if needs_native_size {
            if let Some(img) = images.get(&ui_image.texture) {
                let size = img.size_f32(); // دالة في Bevy ترجع UVec2 كـ Vec2
                native_size = size;
            }
        }

        // 3. تطبيق العرض (Width)
        let target_width = match ui_image.width {
            UVal::Auto => if native_size.x > 0.0 { UVal::Px(native_size.x) } else { UVal::Auto },
            other => other,
        };

        if node.width != target_width {
            node.width = target_width;
        }

        // 4. تطبيق الارتفاع (Height)
        let target_height = match ui_image.height {
            UVal::Auto => if native_size.y > 0.0 { UVal::Px(native_size.y) } else { UVal::Auto },
            other => other,
        };

        if node.height != target_height {
            node.height = target_height;
        }
    }
}