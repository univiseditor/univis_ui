#import bevy_pbr::{
    mesh_view_bindings,
    pbr_types,
    pbr_functions,
    forward_io::VertexOutput,
}

struct UTextLabelSdfMaterial3d {
    color: vec4<f32>,
    outline_color: vec4<f32>,
    clip_radius: vec4<f32>,
    clip_center: vec2<f32>,
    clip_size: vec2<f32>,
    edge_softness: f32,
    outline_width: f32,
    use_clip: u32,
    _pad: f32,
};

@group(3) @binding(0) var<uniform> material: UTextLabelSdfMaterial3d;
@group(3) @binding(1) var text_texture: texture_2d<f32>;
@group(3) @binding(2) var text_sampler: sampler;

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top = p.y > 0.0;

    let r_top = select(r.z, r.x, is_right);
    let r_bottom = select(r.w, r.y, is_right);
    let radius = select(r_bottom, r_top, is_top);

    let q = abs(p) - b + radius;
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(text_texture));
    let uv_dx = dpdx(in.uv);
    let uv_dy = dpdy(in.uv);

    // Isotropic screen-space derivative in atlas texels per screen pixel
    let d_tex_x = uv_dx * tex_size;
    let d_tex_y = uv_dy * tex_size;
    let texels_per_px = length(vec2<f32>(length(d_tex_x), length(d_tex_y))) * 0.70710678;
    // Spread is 12 texels, distance is normalized in [-0.5, 0.5] over 24 texels
    let ddist = max(texels_per_px / 24.0, 0.0001);

    // Filter edge softness
    let edge = max(ddist * 0.5 * max(material.edge_softness, 0.1), 0.0001);

    // Rotated Grid Super-Sampling (RGSS 4-tap) offsets in screen pixels:
    // (-1/8, -3/8), (3/8, -1/8), (-3/8, 1/8), (1/8, 3/8)
    let o0 = uv_dx * -0.125 + uv_dy * -0.375;
    let o1 = uv_dx *  0.375 + uv_dy * -0.125;
    let o2 = uv_dx * -0.375 + uv_dy *  0.125;
    let o3 = uv_dx *  0.125 + uv_dy *  0.375;

    let s0 = textureSampleLevel(text_texture, text_sampler, in.uv + o0, 0.0).a;
    let s1 = textureSampleLevel(text_texture, text_sampler, in.uv + o1, 0.0).a;
    let s2 = textureSampleLevel(text_texture, text_sampler, in.uv + o2, 0.0).a;
    let s3 = textureSampleLevel(text_texture, text_sampler, in.uv + o3, 0.0).a;

    let dist0 = s0 - 0.5;
    let dist1 = s1 - 0.5;
    let dist2 = s2 - 0.5;
    let dist3 = s3 - 0.5;

    let a0 = smoothstep(-edge, edge, dist0);
    let a1 = smoothstep(-edge, edge, dist1);
    let a2 = smoothstep(-edge, edge, dist2);
    let a3 = smoothstep(-edge, edge, dist3);
    let fill_alpha = (a0 + a1 + a2 + a3) * 0.25;

#ifdef VERTEX_COLORS
    let fill_color = material.color * in.color;
#else
    let fill_color = material.color;
#endif

    var final_color: vec4<f32>;

    if (material.outline_width > 0.0 && material.outline_color.a > 0.0) {
        let out_a0 = smoothstep(-edge, edge, dist0 + material.outline_width);
        let out_a1 = smoothstep(-edge, edge, dist1 + material.outline_width);
        let out_a2 = smoothstep(-edge, edge, dist2 + material.outline_width);
        let out_a3 = smoothstep(-edge, edge, dist3 + material.outline_width);
        let outline_alpha = (out_a0 + out_a1 + out_a2 + out_a3) * 0.25;

        let outline_blend = outline_alpha * (1.0 - fill_alpha);
        let alpha = fill_alpha * fill_color.a + outline_blend * material.outline_color.a;
        let rgb = (fill_color.rgb * (fill_alpha * fill_color.a) + material.outline_color.rgb * (outline_blend * material.outline_color.a)) / max(alpha, 0.0001);
        final_color = vec4<f32>(rgb, alpha);
    } else {
        final_color = vec4<f32>(fill_color.rgb, fill_color.a * fill_alpha);
    }

    if (material.use_clip == 1u) {
        let clip_p = in.world_position.xy - material.clip_center;
        let clip_distance = sd_rounded_box(clip_p, material.clip_size * 0.5, material.clip_radius);
        let clip_ddist = length(vec2<f32>(dpdx(clip_distance), dpdy(clip_distance)));
        let clip_edge = max(clip_ddist * 0.5, 0.0001);
        let clip_alpha = 1.0 - smoothstep(-clip_edge, clip_edge, clip_distance);
        final_color = vec4<f32>(final_color.rgb, final_color.a * clip_alpha);
    }

    if (final_color.a <= 0.001) {
        discard;
    }

    // PBR Setup
    var pbr_input = pbr_types::pbr_input_new();
    pbr_input.material.base_color = vec4<f32>(final_color.rgb, 1.0);
    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = 0.8;
    // We could add emissive later if we want glowing text
    
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    pbr_input.is_orthographic = mesh_view_bindings::view.clip_from_view[3].w == 1.0;
    pbr_input.world_normal = pbr_functions::prepare_world_normal(in.world_normal, false, is_front);
    pbr_input.V = pbr_functions::calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.N = normalize(pbr_input.world_normal);

    // Apply lighting
    var out_color = pbr_functions::apply_pbr_lighting(pbr_input);

    // Apply alpha
    out_color.a = out_color.a * final_color.a;
    out_color = vec4<f32>(out_color.rgb * out_color.a, out_color.a);

    return out_color;
}
