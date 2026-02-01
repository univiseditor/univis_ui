use bevy::prelude::*;

/// دالة SDF للمربع ذو الزوايا الدائرية
/// تتطابق مع منطق الـ Shader حيث:
/// Y+ = الأعلى, X+ = اليمين
pub fn sd_rounded_box(p: Vec2, b: Vec2, r: Vec4) -> f32 {
    // r.x = Top-Right
    // r.y = Bottom-Right
    // r.z = Top-Left
    // r.w = Bottom-Left

    // 1. تحديد هل نحن في اليمين أم اليسار؟
    // إذا x > 0 نختار (Top-Right, Bottom-Right) وإلا (Top-Left, Bottom-Left)
    let (r_top, r_bottom) = if p.x > 0.0 {
        (r.x, r.y)
    } else {
        (r.z, r.w)
    };

    // 2. تحديد هل نحن في الأعلى أم الأسفل؟
    // إذا y > 0 نختار r_top وإلا r_bottom
    let radius = if p.y > 0.0 { r_top } else { r_bottom };

    // 3. حساب المسافة (نفس معادلة الشيدر)
    let q = p.abs() - b + Vec2::splat(radius);
    
    q.max(Vec2::ZERO).length() + q.x.max(q.y).min(0.0) - radius
}