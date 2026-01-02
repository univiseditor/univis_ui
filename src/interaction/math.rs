use bevy::prelude::*;

/// Signed Distance Function (SDF) for a rounded box.
///
/// Calculates the distance from point `p` to the edge of a box of size `b`
/// with variable corner radii `r`.
///
/// * `p`: Point position (local space).
/// * `b`: Half-size of the box (width/2, height/2).
/// * `r`: Corner radii (TR, BR, TL, BL).
///
/// Returns:
/// * `< 0.0`: Inside the box.
/// * `> 0.0`: Outside the box.
/// * `0.0`: On the edge.
pub fn sd_rounded_box(p: Vec2, b: Vec2, r: Vec4) -> f32 {
    // Select radius based on quadrant
    // Y+ is Up, X+ is Right
    let upper = p.y > 0.0;
    let right = p.x > 0.0;

    let radius = match (right, upper) {
        (true, true)   => r.x, // Top-Right
        (true, false)  => r.y, // Bottom-Right
        (false, true)  => r.z, // Top-Left
        (false, false) => r.w, // Bottom-Left
    };

    // Calculate distance
    let q = p.abs() - b + Vec2::splat(radius);
    
    // Standard SDF Box logic with corner radius subtraction
    q.max(Vec2::ZERO).length() + q.x.max(q.y).min(0.0) - radius
}