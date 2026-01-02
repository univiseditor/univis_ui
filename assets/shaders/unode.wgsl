#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Data structure matching the Rust 'UNodeMaterial' struct exactly.
// The layout (order and types) must be identical to the Rust definition.
struct UNodeMaterial {
    color: vec4<f32>,        // Background color
    size: vec2<f32>,         // Rectangle dimensions (Width, Height)
    radius: vec4<f32>,       // Corner radii: (TopRight, BottomRight, TopLeft, BottomLeft)
    border_color: vec4<f32>, // Border color
    border_width: f32,       // Border thickness
    softness: f32,           // Softness factor (Not used here as we rely on fwidth for auto-aa)
};

@group(2) @binding(0) var<uniform> material: UNodeMaterial;

// -----------------------------------------------------------------------------
// Signed Distance Function (SDF) for a rounded box with independent corners
// -----------------------------------------------------------------------------
fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    // Determine which corner radius to use based on the quadrant of point 'p'.
    // r.x = TopRight, r.y = BottomRight, r.z = TopLeft, r.w = BottomLeft
    
    // Select based on X axis (Right vs Left)
    var r_eff = r;
    if (p.x > 0.0) { 
        r_eff.x = r.x; // Top-Right
        r_eff.z = r.y; // Bottom-Right (Temporarily stored in z)
    } else {
        r_eff.x = r.z; // Top-Left
        r_eff.z = r.w; // Bottom-Left
    }
    
    // Select based on Y axis (Top vs Bottom)
    // Note: Assuming Y+ is Up in this mathematical context
    let select_r = select(r_eff.z, r_eff.x, p.y > 0.0);

    // Standard SDF calculation for a rounded box
    let q = abs(p) - b + select_r;
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - select_r;
}

// -----------------------------------------------------------------------------
// Main Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Coordinate Setup
    // Convert UVs (0..1) to centered local coordinates (-size/2 .. +size/2)
    let center_uv = in.uv - 0.5;
    
    // Coordinate System Adjustment:
    // Bevy UVs: Y points Down. SDF Math: Y typically points Up.
    // We flip Y here to match the Corner Radius logic (Top/Bottom) correctly.
    let p = vec2<f32>(center_uv.x, -center_uv.y) * material.size;
    let half_size = material.size * 0.5;

    // 2. Distance Calculation
    // Calculate pixel distance to the shape edge.
    // dist < 0 : Inside the shape
    // dist > 0 : Outside the shape
    // dist = 0 : Exactly on the edge
    let dist = sd_rounded_box(p, half_size, material.radius);

    // 3. Crisp Anti-aliasing
    // 'fwidth' calculates the rate of change of the distance field relative to screen pixels.
    // This ensures the anti-aliasing edge is always roughly 1 pixel wide, regardless of zoom/scale.
    let fw = fwidth(dist);
    
    // Calculate the outer shape alpha mask.
    // Values range from 0.0 (transparent) to 1.0 (opaque).
    // '0.5 - (dist / fw)' centers the transition at distance 0.
    let outer_alpha = clamp(0.5 - (dist / fw), 0.0, 1.0);

    // Optimization: Discard fully transparent pixels to save fill-rate.
    if (outer_alpha <= 0.0) {
        discard;
    }

    // 4. Border Logic
    // The border is effectively an "inner cut".
    // Inner distance = Distance + Border Width.
    let dist_inner = dist + material.border_width;
    
    // Calculate the inner body alpha mask.
    // 1.0 = Inside the main body (away from border)
    // 0.0 = Inside the border area
    let inner_body_alpha = clamp(0.5 - (dist_inner / fw), 0.0, 1.0);

    // 5. Color Mixing
    // Mix between Border Color and Main Background Color based on the inner body mask.
    let mixed_color = mix(material.border_color, material.color, inner_body_alpha);

    // 6. Final Output
    // Multiply the resulting color's alpha by the outer shape's anti-aliasing alpha.
    return vec4<f32>(mixed_color.rgb, mixed_color.a * outer_alpha);
}