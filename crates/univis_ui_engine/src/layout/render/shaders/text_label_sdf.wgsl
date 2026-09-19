struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct UTextLabelSdfMaterial {
    color: vec4<f32>,
    clip_radius: vec4<f32>,
    clip_center: vec2<f32>,
    clip_size: vec2<f32>,
    edge_softness: f32,
    use_clip: u32,
    _pad: vec2<f32>,
};

@group(2) @binding(0) var<uniform> material: UTextLabelSdfMaterial;
@group(2) @binding(1) var text_texture: texture_2d<f32>;
@group(2) @binding(2) var text_sampler: sampler;

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
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
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

    return vec4<f32>(material.color.rgb, material.color.a * alpha);
}
