// 1. Vertex Output
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// 2. Material Uniform Buffer (Organized into strict 16-byte aligned blocks)
struct UNodeMaterial {
    color: vec4<f32>,        // Offset 0
    border_color: vec4<f32>, // Offset 16
    radius: vec4<f32>,       // Offset 32
    size: vec2<f32>,         // Offset 48
    border_width: f32,       // Offset 56
    border_offset: f32,      // Offset 60
    softness: f32,           // Offset 64
    shape_mode: u32,         // Offset 68
    use_texture: u32,        // Offset 72
    _pad0: f32,              // Offset 76
    
    clip_center: vec2<f32>,  // Offset 80
    clip_size: vec2<f32>,    // Offset 88
    clip_radius: vec4<f32>,  // Offset 96
    use_clip: u32,           // Offset 112
    _pad1_0: f32,            // Offset 116
    _pad1_1: f32,            // Offset 120
    _pad1_2: f32,            // Offset 124

    grad_colors: array<vec4<f32>, 8>, // Offset 128 (8 * 16 = 128 bytes)
    grad_stops: array<vec4<f32>, 2>,  // Offset 256 (2 * 16 = 32 bytes)
    grad_params: vec4<f32>,           // Offset 288: x = type (0=none, 1=linear, 2=radial), y = angle/radius, z = count, w = 0.0
    grad_center: vec4<f32>,           // Offset 304: xy = center, z = offset, w = 0.0

    shadow_color: vec4<f32>, // Offset 320
    shadow_params: vec4<f32>,// Offset 336: x = offset.x, y = offset.y, z = blur, w = spread

    inner_glow_color: vec4<f32>, // Offset 352
    inner_glow_params: vec4<f32>,// Offset 368: x = blur, y = active (1.0 or 0.0), z = spread, w = 0.0
};

@group(2) @binding(0) var<uniform> material: UNodeMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

// -----------------------------------------------------------------------------
// SDF Functions
// -----------------------------------------------------------------------------

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top   = p.y > 0.0;
    
    let r_top = select(r.z, r.x, is_right);
    let r_bot = select(r.w, r.y, is_right);
    let radius = select(r_bot, r_top, is_top);
    
    let q = abs(p) - b + radius;
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn sd_cut_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top   = p.y > 0.0;

    let r_top = select(r.z, r.x, is_right);
    let r_bot = select(r.w, r.y, is_right);
    let radius = select(r_bot, r_top, is_top);

    let q = abs(p) - b;
    let d_box = length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0);
    let d_cut = (abs(p.x) + abs(p.y) - (b.x + b.y - radius)) * 0.70710678;

    return max(d_box, d_cut);
}

// -----------------------------------------------------------------------------
// Multi-Stop Gradient Sampling
// -----------------------------------------------------------------------------

fn get_gradient_stop_pos(idx: u32) -> f32 {
    if (idx < 4u) {
        if (idx == 0u) { return material.grad_stops[0].x; }
        if (idx == 1u) { return material.grad_stops[0].y; }
        if (idx == 2u) { return material.grad_stops[0].z; }
        return material.grad_stops[0].w;
    } else {
        if (idx == 4u) { return material.grad_stops[1].x; }
        if (idx == 5u) { return material.grad_stops[1].y; }
        if (idx == 6u) { return material.grad_stops[1].z; }
        return material.grad_stops[1].w;
    }
}

fn sample_gradient(t_in: f32) -> vec4<f32> {
    let count = u32(material.grad_params.z);
    if (count == 0u) {
        return material.color;
    }
    if (count == 1u) {
        return material.grad_colors[0];
    }

    let t = clamp(t_in, 0.0, 1.0);
    let is_stepped = material.grad_params.w >= 0.5;

    // Stepped mode: discrete color bands without interpolation
    if (is_stepped) {
        var color = material.grad_colors[0];
        for (var i = 0u; i < 8u; i = i + 1u) {
            if (i >= count) {
                break;
            }
            let p = get_gradient_stop_pos(i);
            if (t >= p) {
                color = material.grad_colors[i];
            } else {
                break;
            }
        }
        return color;
    }

    let p_first = get_gradient_stop_pos(0u);
    if (t <= p_first) {
        return material.grad_colors[0];
    }
    let p_last = get_gradient_stop_pos(count - 1u);
    if (t >= p_last) {
        return material.grad_colors[count - 1u];
    }

    var color = material.grad_colors[count - 1u];
    for (var i = 0u; i < 7u; i = i + 1u) {
        if (i + 1u >= count) {
            break;
        }
        let p_curr = get_gradient_stop_pos(i);
        let p_next = get_gradient_stop_pos(i + 1u);
        if (t >= p_curr && t <= p_next) {
            let span = p_next - p_curr;
            if (span <= 0.0001) {
                color = material.grad_colors[i + 1u];
            } else {
                let factor = clamp((t - p_curr) / span, 0.0, 1.0);
                color = mix(material.grad_colors[i], material.grad_colors[i + 1u], factor);
            }
            break;
        }
    }
    return color;
}

// -----------------------------------------------------------------------------
// Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let shadow_blur = material.shadow_params.z;
    let shadow_spread = material.shadow_params.w;
    let shadow_offset = material.shadow_params.xy;
    let shadow_margin = select(0.0, shadow_blur * 2.5 + shadow_spread + max(abs(shadow_offset.x), abs(shadow_offset.y)), shadow_blur > 0.0);
    let mesh_size = material.size + vec2<f32>(shadow_margin * 2.0);

    let uv_centered = in.uv - 0.5;
    let p = vec2<f32>(uv_centered.x, -uv_centered.y) * mesh_size;
    let half_size = material.size * 0.5;
    
    var dist_outer: f32;
    if (material.shape_mode == 1u) {
        dist_outer = sd_cut_box(p, half_size, material.radius);
    } else {
        dist_outer = sd_rounded_box(p, half_size, material.radius);
    }
    
    let softness = max(material.softness, 0.0001);
    let aa_width = max(fwidth(dist_outer), softness);
    let aa_inner = aa_width * 0.5;

    // Optional clipping against parent container
    var alpha_clip = 1.0;
    if (material.use_clip == 1u) {
        let p_clip = in.world_position.xy - material.clip_center;
        let d_clip = sd_rounded_box(p_clip, material.clip_size * 0.5, material.clip_radius);
        alpha_clip = 1.0 - smoothstep(-softness, softness, d_clip);
        if (alpha_clip < 0.001) {
            discard;
        }
    }

    let dist_border_end = dist_outer + material.border_width;
    let dist_body_start = dist_outer + material.border_width + material.border_offset;
    
    let border_mask = (1.0 - smoothstep(-aa_inner, aa_inner, dist_outer)) * 
                      smoothstep(-aa_inner, aa_inner, dist_border_end);
    
    let body_mask = 1.0 - smoothstep(-aa_inner, aa_inner, dist_body_start);

    // Body UV relative to the base element bounds
    let body_uv = (p / material.size) * vec2<f32>(1.0, -1.0) + 0.5;

    // Multi-stop gradient fill: color, linear gradient, or radial gradient
    var body_color = material.color;
    let offset = material.grad_center.z;
    if (material.grad_params.x == 1.0) {
        // Multi-stop linear gradient
        let angle = material.grad_params.y;
        let dir = vec2<f32>(cos(angle), sin(angle));
        let t_lin = dot(body_uv - 0.5, dir) + 0.5 - offset;
        body_color = sample_gradient(t_lin);
    } else if (material.grad_params.x == 2.0) {
        // Multi-stop radial gradient
        let center = material.grad_center.xy;
        let radius = max(material.grad_params.y, 0.001);
        let dist_rad = length(body_uv - center);
        let t_rad = (dist_rad / radius) - offset;
        body_color = sample_gradient(t_rad);
    }

    if (material.use_texture == 1u) {
        body_color = textureSample(texture, texture_sampler, body_uv) * body_color;
    }

    // Holographic inner edge glow along chamfers and rounded edges
    if (material.inner_glow_params.y > 0.5) {
        let inner_blur = max(material.inner_glow_params.x, 0.001);
        let dist_inward = max(-dist_body_start, 0.0);
        let inner_factor = 1.0 - smoothstep(0.0, inner_blur, dist_inward);
        let glow_alpha = inner_factor * material.inner_glow_color.a;
        body_color = vec4<f32>(
            mix(body_color.rgb, material.inner_glow_color.rgb, glow_alpha),
            max(body_color.a, glow_alpha)
        );
    }

    // Base compositing: Outer Shadow -> Border -> Body
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    if (shadow_blur > 0.0) {
        let p_shadow = p - vec2<f32>(shadow_offset.x, -shadow_offset.y);
        let shadow_radius = material.radius + vec4<f32>(shadow_spread);
        let half_size_shadow = half_size + vec2<f32>(shadow_spread);
        var dist_shadow: f32;
        if (material.shape_mode == 1u) {
            dist_shadow = sd_cut_box(p_shadow, half_size_shadow, shadow_radius);
        } else {
            dist_shadow = sd_rounded_box(p_shadow, half_size_shadow, shadow_radius);
        }
        let shadow_factor = 1.0 - smoothstep(-aa_inner, shadow_blur, dist_shadow);
        let shadow_alpha = shadow_factor * material.shadow_color.a;
        final_color = vec4<f32>(material.shadow_color.rgb, shadow_alpha);
    }

    final_color = mix(final_color, material.border_color, border_mask);
    final_color = mix(final_color, body_color, body_mask);
    final_color.a = final_color.a * alpha_clip;

    if (final_color.a < 0.001) {
        discard;
    }

    return final_color;
}
