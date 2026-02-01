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
}

@group(3) @binding(0) var<uniform> material: UNodeMaterial3d;
@group(3) @binding(1) var base_texture: texture_2d<f32>;
@group(3) @binding(2) var base_sampler: sampler;

// --- دالة حساب الشكل (SDF) المعدلة ---
fn sd_box_dynamic(p: vec2<f32>, b: vec2<f32>, r_in: vec4<f32>, mode: u32) -> f32 {
    let limit = min(b.x, b.y);
    let r_vec = min(r_in, vec4<f32>(limit));
    
    var radius: f32;
    // تحديد نصف القطر حسب الربع (Quadrant)
    if (p.x > 0.0) { 
        if (p.y < 0.0) { radius = r_vec.x; } else { radius = r_vec.y; }
    } else { 
        if (p.y < 0.0) { radius = r_vec.z; } else { radius = r_vec.w; }
    }

    // q: الإحداثيات بالنسبة للزاوية الداخلية
    let q = abs(p) - b + radius;
    
    if (mode == 1u) {
        // --- Cut Mode (القص/Chamfer) ---
        // في هذا النمط، المسافة هي تقاطع بين "المربع" و"الخط المائل".
        // نستخدم دالة max للتقاطع.
        
        // 1. مسافة المربع (Straight Edges)
        let d_square = max(q.x, q.y);
        
        // 2. مسافة الخط المائل (Diagonal)
        // (x + y) / sqrt(2)
        let d_diagonal = (q.x + q.y) * 0.70710678;
        
        // النتيجة: نأخذ الأبعد بينهما (لأننا في SDF، الأكبر يعني الخروج للخارج)
        // ثم نطرح نصف القطر لتحديد سطح الشكل
        return max(d_square, d_diagonal) - radius;
        
    } else {
        // --- Round Mode (التدوير) ---
        // المسافة الإقليدية التقليدية (دائرة عند الزوايا)
        let d_round = length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0);
        return d_round - radius;
    }
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    
    // 1. حساب المسافة (SDF)
    let center_pos = (in.uv - 0.5) * material.size;
    let half_size = material.size * 0.5;
    
    // حساب المسافة باستخدام الدالة الجديدة
    let dist = sd_box_dynamic(center_pos, half_size, material.radius, material.shape_mode);
    
    // 2. القناع والنعومة (Anti-aliasing)
    let smoothing = fwidth(dist);
    let alpha = 1.0 - smoothstep(0.0, smoothing, dist);

    if (alpha <= 0.0) {
        discard;
    }

    // 3. حساب الحدود
    // نستخدم المسافة المباشرة لضمان سمك ثابت حتى مع القص
    let border_factor = smoothstep(-material.border_width - smoothing, -material.border_width, dist);

    // 4. الألوان
    var current_base_color = material.color;
    if (material.use_texture > 0u) {
        let tex_sample = textureSample(base_texture, base_sampler, in.uv);
        current_base_color = tex_sample * material.color;
    }
    
    let final_base_color = mix(current_base_color, material.border_color, border_factor);
    
    // التوهج: الحدود تتوهج
    let border_glow = material.border_color; 
    let final_emissive = mix(material.emissive, border_glow, border_factor);

    // 5. PBR
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

    // 6. الإضاءة
    var out_color = pbr_functions::apply_pbr_lighting(pbr_input);

    // إضافة التوهج يدوياً (للقيم العالية)
    out_color = vec4<f32>(out_color.rgb + final_emissive.rgb, out_color.a);

    // تطبيق الشفافية
    out_color.a = out_color.a * alpha;
    out_color = vec4<f32>(out_color.rgb * out_color.a, out_color.a);

    return out_color;
}