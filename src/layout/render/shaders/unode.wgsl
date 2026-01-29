// 1. تعريف VertexOutput
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>, // سنستخدم هذا لحساب القص
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// 2. هيكلية البيانات (تم تحديثها لتشمل بيانات القص)
// يجب أن تتطابق تماماً مع ترتيب الذاكرة في Rust
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
    
    // --- بيانات القص الجديدة ---
    // نحتاج لمحاذاة الذاكرة (Alignment). vec2 يبدأ عند مضاعفات 8.
    // Offset الحالي هو 76. نحتاج للقفز إلى 80.
    // (الحشو الضمني سيتم هنا تلقائياً، أو يمكننا إضافة متغير وهمي)
    
    clip_center: vec2<f32>,  // Offset 80
    clip_size: vec2<f32>,    // Offset 88
    clip_radius: vec4<f32>,  // Offset 96
    use_clip: u32,           // Offset 112
    
    // حشو نهائي لإكمال الـ 16 bytes alignment
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
// Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. حساب شكل العنصر الحالي (الابن)
    let uv_centered = in.uv - 0.5;
    let p = vec2<f32>(uv_centered.x, -uv_centered.y) * material.size;
    let half_size = material.size * 0.5;
    
    var dist_outer: f32;
    if (material.shape_mode == 1u) {
        dist_outer = sd_cut_box(p, half_size, material.radius);
    } else {
        dist_outer = sd_rounded_box(p, half_size, material.radius);
    }
    
    let aa_width = max(fwidth(dist_outer), 0.5) * max(material.softness, 0.5);
    var alpha_final = 1.0 - smoothstep(-aa_width, aa_width, dist_outer);
    
    // تحسين الأداء: إذا كان العنصر شفافاً تماماً، لا تكمل الحساب
    if (alpha_final < 0.001) { discard; }

    // ----------------------------------------------------------
    // 2. منطق القص (Clipping Logic) - الجديد
    // ----------------------------------------------------------
    if (material.use_clip == 1u) {
        // نحتاج لموقع البكسل الحالي في العالم
        // ونحوله ليكون نسبياً لمركز القص (الأب)
        // ملاحظة: y مقلوب في Bevy World Space أحياناً، لكن world_position عادة صحيح
        // إذا ظهر القص مقلوباً، جرب عكس Y هنا
        let p_clip = in.world_position.xy - material.clip_center;
        
        // حساب SDF لمنطقة القص (باعتبارها Rounded Box)
        // نستخدم half_size للقناع
        let d_clip = sd_rounded_box(p_clip, material.clip_size * 0.5, material.clip_radius);
        
        // حساب ألفا القناع:
        // إذا كانت المسافة سالبة (داخل الصندوق) -> Alpha 1
        // إذا كانت المسافة موجبة (خارج الصندوق) -> Alpha 0
        // نستخدم smoothstep صغيرة جداً للحصول على حواف ناعمة للقص
        let alpha_clip = 1.0 - smoothstep(-0.5, 0.5, d_clip);
        
        // دمج شفافية العنصر مع شفافية القناع
        alpha_final = min(alpha_final, alpha_clip);
        
        // إذا أصبح مخفياً بسبب القص، نتوقف
        if (alpha_final < 0.001) { discard; }
    }
    // ----------------------------------------------------------

    let aa_inner = aa_width * 0.5;
    let dist_border_end = dist_outer + material.border_width;
    let dist_body_start = dist_outer + material.border_width + material.border_offset;
    
    let border_mask = (1.0 - smoothstep(-aa_inner, aa_inner, dist_outer)) * 
                      smoothstep(-aa_inner, aa_inner, dist_border_end);
    
    let body_mask = 1.0 - smoothstep(-aa_inner, aa_inner, dist_body_start);
    
    var body_color = material.color;
    if (material.use_texture == 1u) {
        body_color = textureSample(texture, texture_sampler, in.uv) * material.color;
    }
    
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    final_color = mix(final_color, material.border_color, border_mask);
    final_color = mix(final_color, body_color, body_mask);
    
    // تطبيق الشفافية النهائية (بما في ذلك القص)
    final_color.a = final_color.a * alpha_final;
    
    return final_color;
}