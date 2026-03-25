use bevy::prelude::*;

/// خصائص المواد الفيزيائية (PBR) للعناصر ثلاثية الأبعاد.
/// تُطبَّق عندما يُحل الجذر إلى `UiSpace::World3d`.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct UPbr {
    /// مدى "معدنية" السطح (0.0 = بلاستيك/خشب، 1.0 = معدن).
    pub metallic: f32,

    /// مدى خشونة السطح (0.0 = مرآة لامعة، 1.0 = سطح خشن باهت).
    pub roughness: f32,

    /// لون التوهج الذاتي (Glow). مفيد جداً لشاشات الخيال العلمي.
    pub emissive: LinearRgba,
}

impl Default for UPbr {
    fn default() -> Self {
        Self {
            metallic: 0.0,
            roughness: 0.5,
            emissive: LinearRgba::BLACK, // لا يوجد توهج افتراضياً
        }
    }
}
