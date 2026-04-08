use bevy::prelude::*;

/// Signed-distance helper for a rounded box.
///
/// This matches the shader-side corner ordering, with `Y+` up and `X+` right.
pub fn sd_rounded_box(p: Vec2, b: Vec2, r: Vec4) -> f32 {
    // r.x = Top-Right
    // r.y = Bottom-Right
    // r.z = Top-Left
    // r.w = Bottom-Left

    // 1. Pick the correct left/right pair.
    // If `x > 0`, use (Top-Right, Bottom-Right); otherwise use (Top-Left, Bottom-Left).
    let (r_top, r_bottom) = if p.x > 0.0 { (r.x, r.y) } else { (r.z, r.w) };

    // 2. Pick the top or bottom corner radius.
    // If `y > 0`, use the top radius; otherwise use the bottom radius.
    let radius = if p.y > 0.0 { r_top } else { r_bottom };

    // 3. Evaluate the same distance equation used by the shader.
    let q = p.abs() - b + Vec2::splat(radius);

    q.max(Vec2::ZERO).length() + q.x.max(q.y).min(0.0) - radius
}
