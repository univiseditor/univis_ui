#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    
    // Per-instance attributes (StepMode::Instance)
    @location(0) pos_size: vec4<f32>,       // xy = world_pos, zw = size
    @location(1) color: vec4<f32>,          // background color
    @location(2) border_color: vec4<f32>,   // border color
    @location(3) radius: vec4<f32>,         // (tr, br, tl, bl)
    @location(4) border_params: vec4<f32>,  // x = border_width, y = border_offset, z = softness, w = shape_mode (0=round, 1=cut)
    @location(5) clip_rect: vec4<f32>,      // xy = clip_center, zw = clip_size
    @location(6) clip_radius: vec4<f32>,    // clip corner radii
    @location(7) shadow_color: vec4<f32>,   // shadow color
    @location(8) shadow_params: vec4<f32>,  // xy = shadow_offset, z = blur, w = spread
    @location(9) flags: vec4<f32>,          // x = use_clip (1.0 or 0.0)
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) world_pos: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) border_color: vec4<f32>,
    @location(5) radius: vec4<f32>,
    @location(6) border_params: vec4<f32>,
    @location(7) clip_rect: vec4<f32>,
    @location(8) clip_radius: vec4<f32>,
    @location(9) shadow_color: vec4<f32>,
    @location(10) shadow_params: vec4<f32>,
    @location(11) base_size: vec2<f32>,
    @location(12) use_clip: f32,
};

// -----------------------------------------------------------------------------
// Procedural Quad Vertex Shader (6 indices)
// -----------------------------------------------------------------------------
@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let shadow_blur = in.shadow_params.z;
    let shadow_spread = in.shadow_params.w;
    let shadow_offset = in.shadow_params.xy;
    let shadow_margin = select(0.0, shadow_blur * 2.5 + shadow_spread + max(abs(shadow_offset.x), abs(shadow_offset.y)), shadow_blur > 0.0);

    let base_size = in.pos_size.zw;
    let mesh_size = base_size + vec2<f32>(shadow_margin * 2.0);

    // Quad triangle positions centered around (0,0)
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5),
        vec2<f32>( 0.5,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5,  0.5),
        vec2<f32>(-0.5,  0.5),
    );

    let unit_pos = uvs[in.vertex_index];
    let local_pixel = unit_pos * mesh_size;
    let world_xy = in.pos_size.xy + local_pixel;
    let world_z = in.flags.y;

    out.position = view.clip_from_world * vec4<f32>(world_xy, world_z, 1.0);
    out.uv = unit_pos + vec2<f32>(0.5, 0.5);
    out.local_pos = vec2<f32>(unit_pos.x, -unit_pos.y) * mesh_size;
    out.world_pos = world_xy;

    out.color = in.color;
    out.border_color = in.border_color;
    out.radius = in.radius;
    out.border_params = in.border_params;
    out.clip_rect = in.clip_rect;
    out.clip_radius = in.clip_radius;
    out.shadow_color = in.shadow_color;
    out.shadow_params = in.shadow_params;
    out.base_size = base_size;
    out.use_clip = in.flags.x;

    return out;
}

// -----------------------------------------------------------------------------
// SDF Primitives
// -----------------------------------------------------------------------------
fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top   = p.y > 0.0;
    
    let r_top = select(r.z, r.x, is_right);
    let r_bot = select(r.w, r.y, is_right);
    let radius = select(r_bot, r_top, is_top);
    
    let q = abs(p) - b + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn sd_cut_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top   = p.y > 0.0;

    let r_top = select(r.z, r.x, is_right);
    let r_bot = select(r.w, r.y, is_right);
    let radius = select(r_bot, r_top, is_top);

    let q = abs(p) - b;
    let d_box = length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0);
    let d_cut = (abs(p.x) + abs(p.y) - (b.x + b.y - radius)) * 0.70710678;

    return max(d_box, d_cut);
}

// -----------------------------------------------------------------------------
// Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let half_size = in.base_size * 0.5;
    let p = in.local_pos;

    let shape_mode = u32(in.border_params.w);
    var dist_outer: f32;
    if (shape_mode == 1u) {
        dist_outer = sd_cut_box(p, half_size, in.radius);
    } else {
        dist_outer = sd_rounded_box(p, half_size, in.radius);
    }

    let softness = max(in.border_params.z, 0.0001);
    let aa_width = max(fwidth(dist_outer), softness);
    let aa_inner = aa_width * 0.5;

    // Hardware clipping
    var alpha_clip = 1.0;
    if (in.use_clip > 0.5) {
        let p_clip = in.world_pos - in.clip_rect.xy;
        let d_clip = sd_rounded_box(p_clip, in.clip_rect.zw * 0.5, in.clip_radius);
        alpha_clip = 1.0 - smoothstep(-softness, softness, d_clip);
        if (alpha_clip < 0.001) {
            discard;
        }
    }

    let border_width = in.border_params.x;
    let border_offset = in.border_params.y;
    let dist_border_end = dist_outer + border_width;
    let dist_body_start = dist_outer + border_width + border_offset;

    let border_mask = (1.0 - smoothstep(-aa_inner, aa_inner, dist_outer)) *
                      smoothstep(-aa_inner, aa_inner, dist_border_end);

    let body_mask = 1.0 - smoothstep(-aa_inner, aa_inner, dist_body_start);

    // Outer shadow
    let shadow_blur = in.shadow_params.z;
    let shadow_spread = in.shadow_params.w;
    let shadow_offset = in.shadow_params.xy;
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    if (shadow_blur > 0.0) {
        let p_shadow = p - vec2<f32>(shadow_offset.x, -shadow_offset.y);
        let shadow_radius = in.radius + vec4<f32>(shadow_spread);
        let half_size_shadow = half_size + vec2<f32>(shadow_spread);
        var dist_shadow: f32;
        if (shape_mode == 1u) {
            dist_shadow = sd_cut_box(p_shadow, half_size_shadow, shadow_radius);
        } else {
            dist_shadow = sd_rounded_box(p_shadow, half_size_shadow, shadow_radius);
        }
        let shadow_factor = 1.0 - smoothstep(-aa_inner, shadow_blur, dist_shadow);
        let shadow_alpha = shadow_factor * in.shadow_color.a;
        final_color = vec4<f32>(in.shadow_color.rgb, shadow_alpha);
    }

    final_color = mix(final_color, in.border_color, border_mask);
    final_color = mix(final_color, in.color, body_mask);
    final_color.a = final_color.a * alpha_clip;

    if (final_color.a < 0.001) {
        discard;
    }

    return final_color;
}
