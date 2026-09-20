#import bevy_pbr::{
    mesh_view_bindings,
    pbr_types,
    pbr_functions,
    forward_io::VertexOutput,
}

struct UNodeMaterial3d {
    color: vec4<f32>,
    radius: vec4<f32>,
    border_color: vec4<f32>,
    emissive: vec4<f32>,
    size: vec2<f32>,
    border_width: f32,
    softness: f32,
    metallic: f32,
    roughness: f32,
    use_texture: u32,
    shape_mode: u32, 

    grad_start: vec4<f32>,
    grad_end: vec4<f32>,
    grad_params: vec4<f32>,

    shadow_color: vec4<f32>,
    shadow_params: vec4<f32>,

    inner_glow_color: vec4<f32>,
    inner_glow_params: vec4<f32>,
}

@group(3) @binding(0) var<uniform> material: UNodeMaterial3d;
@group(3) @binding(1) var base_texture: texture_2d<f32>;
@group(3) @binding(2) var base_sampler: sampler;

fn sd_box_dynamic(p: vec2<f32>, b: vec2<f32>, r_in: vec4<f32>, mode: u32) -> f32 {
    let limit = min(b.x, b.y);
    let r_vec = min(r_in, vec4<f32>(limit));
    
    var radius: f32;
    if (p.x > 0.0) { 
        if (p.y < 0.0) { radius = r_vec.x; } else { radius = r_vec.y; }
    } else { 
        if (p.y < 0.0) { radius = r_vec.z; } else { radius = r_vec.w; }
    }

    let q = abs(p) - b + radius;
    
    if (mode == 1u) {
        let d_square = max(q.x, q.y);
        let d_diagonal = (q.x + q.y) * 0.70710678;
        return max(d_square, d_diagonal) - radius;
    } else {
        let d_round = length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0);
        return d_round - radius;
    }
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let shadow_blur = material.shadow_params.z;
    let shadow_spread = material.shadow_params.w;
    let shadow_offset = material.shadow_params.xy;
    let shadow_margin = select(0.0, shadow_blur * 2.5 + shadow_spread + max(abs(shadow_offset.x), abs(shadow_offset.y)), shadow_blur > 0.0);
    let mesh_size = material.size + vec2<f32>(shadow_margin * 2.0);

    let center_pos = (in.uv - 0.5) * mesh_size;
    let half_size = material.size * 0.5;
    
    let dist = sd_box_dynamic(center_pos, half_size, material.radius, material.shape_mode);
    let smoothing = fwidth(dist);
    let alpha = 1.0 - smoothstep(0.0, smoothing, dist);

    let body_uv = (center_pos / material.size) + 0.5;

    var current_base_color = material.color;
    if (material.grad_params.x == 1.0) {
        let angle = material.grad_params.y;
        let dir = vec2<f32>(cos(angle), sin(angle));
        let t_lin = clamp(dot(body_uv - 0.5, dir) + 0.5, 0.0, 1.0);
        current_base_color = mix(material.grad_start, material.grad_end, t_lin);
    } else if (material.grad_params.x == 2.0) {
        let center = material.grad_params.zw;
        let radius = max(material.grad_params.y, 0.001);
        let dist_rad = length(body_uv - center);
        let t_rad = clamp(dist_rad / radius, 0.0, 1.0);
        current_base_color = mix(material.grad_start, material.grad_end, t_rad);
    }

    if (material.use_texture > 0u) {
        let tex_sample = textureSample(base_texture, base_sampler, in.uv);
        current_base_color = tex_sample * current_base_color;
    }

    // Holographic inner edge glow
    if (material.inner_glow_params.y > 0.5) {
        let inner_blur = max(material.inner_glow_params.x, 0.001);
        let dist_inward = max(-dist - material.border_width, 0.0);
        let inner_factor = 1.0 - smoothstep(0.0, inner_blur, dist_inward);
        let glow_alpha = inner_factor * material.inner_glow_color.a;
        current_base_color = vec4<f32>(
            mix(current_base_color.rgb, material.inner_glow_color.rgb, glow_alpha),
            max(current_base_color.a, glow_alpha)
        );
    }

    let border_factor = smoothstep(-material.border_width - smoothing, -material.border_width, dist);
    let final_base_color = mix(current_base_color, material.border_color, border_factor);
    let border_glow = material.border_color; 
    let final_emissive = mix(material.emissive, border_glow, border_factor);

    var final_alpha = alpha;
    if (shadow_blur > 0.0) {
        let p_shadow = center_pos - shadow_offset;
        let shadow_radius = material.radius + vec4<f32>(shadow_spread);
        let half_size_shadow = half_size + vec2<f32>(shadow_spread);
        let dist_shadow = sd_box_dynamic(p_shadow, half_size_shadow, shadow_radius, material.shape_mode);
        let shadow_factor = 1.0 - smoothstep(0.0, shadow_blur, dist_shadow);
        final_alpha = max(alpha, shadow_factor * material.shadow_color.a);
    }

    if (final_alpha <= 0.0) {
        discard;
    }

    // PBR Lighting
    var pbr_input = pbr_types::pbr_input_new();
    pbr_input.material.base_color = final_base_color;
    pbr_input.material.metallic = material.metallic;
    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.emissive = final_emissive;

    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    pbr_input.is_orthographic = mesh_view_bindings::view.clip_from_view[3].w == 1.0;
    pbr_input.world_normal = pbr_functions::prepare_world_normal(in.world_normal, false, is_front);
    pbr_input.V = pbr_functions::calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.N = normalize(pbr_input.world_normal);

    var out_color = pbr_functions::apply_pbr_lighting(pbr_input);
    out_color = vec4<f32>(out_color.rgb + final_emissive.rgb, out_color.a);
    out_color.a = out_color.a * final_alpha;
    out_color = vec4<f32>(out_color.rgb * out_color.a, out_color.a);

    return out_color;
}