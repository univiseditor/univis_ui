#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct UTextLabelSdfMaterial {
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
    let fill_alpha = smoothstep(-edge, edge, distance);

#ifdef VERTEX_COLORS
    let fill_color = material.color * in.color;
#else
    let fill_color = material.color;
#endif

    var final_color: vec4<f32>;

    if (material.outline_width > 0.0 && material.outline_color.a > 0.0) {
        let outline_distance = distance + material.outline_width;
        let outline_alpha = smoothstep(-edge, edge, outline_distance);

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
        let clip_alpha = 1.0 - smoothstep(-0.5, 0.5, clip_distance);
        final_color = vec4<f32>(final_color.rgb, final_color.a * clip_alpha);
    }

    if (final_color.a <= 0.001) {
        discard;
    }

    return final_color;
}
