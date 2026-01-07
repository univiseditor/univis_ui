// 1. تعريف VertexOutput يدوياً لتجنب مشاكل الاستيراد
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// 2. هيكلية البيانات (مطابقة لـ Rust Struct UNodeMaterial)
struct UNodeMaterial {
    color: vec4<f32>,        // Offset 0
    border_color: vec4<f32>, // Offset 16
    radius: vec4<f32>,       // Offset 32
    size: vec2<f32>,         // Offset 48
    border_width: f32,       // Offset 56
    border_offset: f32,      // Offset 60
    softness: f32,           // Offset 64
    
    // 0 = Round, 1 = Cut
    shape_mode: u32,         // Offset 68
    
    use_texture: u32,        // Offset 72
    // يوجد ضمنياً 4 بايت حشو هنا لإكمال الـ 80 بايت (16 alignment)
};

@group(2) @binding(0) var<uniform> material: UNodeMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

// -----------------------------------------------------------------------------
// SDF: Round Box (زوايا دائرية)
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

// -----------------------------------------------------------------------------
// SDF: Cut Box (زوايا مشطوفة / Sci-Fi)
// -----------------------------------------------------------------------------
fn sd_cut_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let is_right = p.x > 0.0;
    let is_top   = p.y > 0.0;

    let r_top = select(r.z, r.x, is_right);
    let r_bot = select(r.w, r.y, is_right);
    let radius = select(r_bot, r_top, is_top);

    // 1. مسافة المربع الحاد
    let q = abs(p) - b;
    let d_box = length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0);

    // 2. مسافة القص (Chamfer Plane) بزاوية 45 درجة
    // المعادلة: (x + y - (w + h - r)) / sqrt(2)
    // 0.70710678 هو مقلوب جذر 2
    let d_cut = (abs(p.x) + abs(p.y) - (b.x + b.y - radius)) * 0.70710678;

    // النتيجة هي التقاطع بين المربع والقص
    return max(d_box, d_cut);
}

// -----------------------------------------------------------------------------
// Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // إعداد الإحداثيات (المنتصف 0,0)
    let uv_centered = in.uv - 0.5;
    // قلب محور Y لأن حسابات SDF تفترض Y للأعلى
    let p = vec2<f32>(uv_centered.x, -uv_centered.y) * material.size;
    let half_size = material.size * 0.5;
    
    // ----------------------------------------------------------
    // 1. اختيار دالة SDF بناءً على النمط
    // ----------------------------------------------------------
    var dist_outer: f32;
    if (material.shape_mode == 1u) {
        dist_outer = sd_cut_box(p, half_size, material.radius);
    } else {
        dist_outer = sd_rounded_box(p, half_size, material.radius);
    }
    
    // ----------------------------------------------------------
    // 2. حساب الأقنعة (Masks) والنعومة (AA)
    // ----------------------------------------------------------
    
    // عرض التنعيم
    let aa_width = max(fwidth(dist_outer), 0.5) * max(material.softness, 0.5);
    let aa_inner = aa_width * 0.5;
    
    // القناع الخارجي الكلي (الشكل الكامل)
    let alpha_outer = 1.0 - smoothstep(-aa_width, aa_width, dist_outer);
    
    // إذا كنا خارج الشكل تماماً، نتوقف (تحسين للأداء)
    if (alpha_outer < 0.001) { discard; }
    
    // تحديد المسافات للطبقات الداخلية
    let dist_border_end = dist_outer + material.border_width;
    let dist_body_start = dist_outer + material.border_width + material.border_offset;
    
    // قناع البوردر: هو المنطقة المحصورة بين "الحافة الخارجية" و "نهاية البوردر"
    // المعادلة: (داخل الخارجي) * (خارج الداخلي)
    let border_mask = (1.0 - smoothstep(-aa_inner, aa_inner, dist_outer)) * 
                      smoothstep(-aa_inner, aa_inner, dist_border_end);
    
    // قناع الجسم: المنطقة التي تبدأ بعد (البوردر + الأوفست)
    let body_mask = 1.0 - smoothstep(-aa_inner, aa_inner, dist_body_start);
    
    // ----------------------------------------------------------
    // 3. تركيب الألوان (Compositing)
    // ----------------------------------------------------------
    
    // تجهيز لون الجسم (تكستشر أو لون ثابت)
    var body_color = material.color;
    if (material.use_texture == 1u) {
        body_color = textureSample(texture, texture_sampler, in.uv) * material.color;
    }
    
    // نبدأ بلون شفاف (يمثل منطقة الفجوة Gap)
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    // الطبقة 1: البوردر
    final_color = mix(final_color, material.border_color, border_mask);
    
    // الطبقة 2: الجسم (يرسم فوق البوردر والفجوة)
    final_color = mix(final_color, body_color, body_mask);
    
    // ----------------------------------------------------------
    // 4. الإخراج النهائي
    // ----------------------------------------------------------
    
    // تطبيق الشفافية الخارجية (Anti-aliasing للحواف)
    final_color.a = final_color.a * alpha_outer;
    
    // تجاهل البكسلات شبه الشفافة جداً
    if (final_color.a < 0.001) { discard; }
    
    return final_color;
}