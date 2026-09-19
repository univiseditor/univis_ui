#import bevy_pbr::{
    mesh_view_bindings,
    pbr_types,
    pbr_functions,
    forward_io::VertexOutput,
}

struct UTextLabelSdfMaterial3d {
    color: vec4<f32>,
    clip_radius: vec4<f32>,
    clip_center: vec2<f32>,
    clip_size: vec2<f32>,
    edge_softness: f32,
    use_clip: u32,
    _pad: vec2<f32>,
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
    let sdf = textureSample(text_texture, text_sampler, in.uv).a;
    let distance = sdf - 0.5;
    let edge = max(fwidth(distance) * max(material.edge_softness, 0.5), 0.001);
    var alpha = smoothstep(-edge, edge, distance);

    if (material.use_clip == 1u) {
        let clip_p = in.world_position.xy - material.clip_center;
        let clip_distance = sd_rounded_box(clip_p, material.clip_size * 0.5, material.clip_radius);
        let clip_alpha = 1.0 - smoothstep(-0.5, 0.5, clip_distance);
        alpha = alpha * clip_alpha;
    }

    if (alpha <= 0.001) {
        discard;
    }

    // PBR Setup
    var pbr_input = pbr_types::pbr_input_new();
    pbr_input.material.base_color = material.color;
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
    out_color.a = out_color.a * alpha;
    out_color = vec4<f32>(out_color.rgb * out_color.a, out_color.a);

    return out_color;
}
