use bevy::{ecs::relationship::Relationship, prelude::*};
use crate::prelude::*;

/// مكون يحفظ Handle للـ Material مع العقدة
#[derive(Component)]
pub struct MaterialHandles {
    pub material_2d: Option<Handle<UNodeMaterial>>,
    pub material_3d: Option<Handle<UNodeMaterial3d>>,
}

impl Default for MaterialHandles {
    fn default() -> Self {
        Self {
            material_2d: None,
            material_3d: None,
        }
    }
}

/// Resource لإدارة pool من المواد
#[derive(Resource, Default)]
pub struct MaterialPool {
    /// عدد المواد المُعاد استخدامها
    pub reused_count: usize,
    /// عدد المواد الجديدة المُنشأة
    pub created_count: usize,
}

impl MaterialPool {
    pub fn reset_stats(&mut self) {
        self.reused_count = 0;
        self.created_count = 0;
    }
}

/// نظام محسّن لتحديث المواد بدون تسرب
pub fn update_materials_optimized(
    mut commands: Commands,
    mut pool: ResMut<MaterialPool>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    
    // الاستعلام يشمل UI3d و UPbr
    mut query: Query<
        (
            Entity,
            &UNode,
            &ComputedSize,
            Option<&UBorder>,
            Option<&UImage>,
            Option<&UI3d>, // <--- نحتاج هذا للتمييز بين 2D و 3D
            Option<&UPbr>,
            Option<&mut MaterialHandles>,
        ),
        Or<(
            Changed<UNode>,
            Changed<ComputedSize>,
            Changed<UBorder>,
            Changed<UImage>,
            Changed<UI3d>,
            Changed<UPbr>,
            Changed<ChildOf>, // مهم للقص
        )>
    >,

    // استعلامات القص
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,

    // الموارد
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
    mut materials_3d: ResMut<Assets<UNodeMaterial3d>>,
) {
    let start = std::time::Instant::now();
    let created_before = pool.created_count;
    let reused_before = pool.reused_count;
    
    for (entity, node, size, border, image, ui3d_opt, pbr_opt, handles_opt) in query.iter_mut() {
        
        let size_vec = Vec2::new(size.width, size.height);
        if size_vec.x <= 0.0 || size_vec.y <= 0.0 { continue; }

        // --- البيانات المشتركة ---
        let (tex_handle, use_tex, base_color) = if let Some(img) = image {
            (Some(img.texture.clone()), 1, LinearRgba::from(img.color))
        } else {
            (None, 0, LinearRgba::from(node.background_color))
        };

        let (b_color, b_offset, b_width) = if let Some(b) = border {
            (LinearRgba::from(b.color), b.offset, b.width)
        } else {
            (LinearRgba::NONE, 0.0, 0.0)
        };

        let radius = Vec4::new(
            node.border_radius.top_right, node.border_radius.bottom_right,
            node.border_radius.top_left, node.border_radius.bottom_left,
        );

        let shape_mode = match node.shape_mode {
            UShapeMode::Round => 0,
            UShapeMode::Cut => 1,
        };

        // --- البحث عن القص (لـ 2D فقط حالياً) ---
        let (clip_center, clip_size, clip_radius, use_clip) = find_clipper(
            entity, &parents_query, &clipper_query
        );

        let mesh = meshes.add(Rectangle::new(size_vec.x, size_vec.y));

        // =========================================================
        // التفرع: هل نحن في وضع 3D أم 2D؟
        // =========================================================
        
        if ui3d_opt.is_some() {
            // >>>> مسار 3D <<<<
            
            let (metallic, roughness, emissive_val) = if let Some(pbr) = pbr_opt {
                (pbr.metallic, pbr.roughness, Vec4::from(pbr.emissive.to_vec4()))
            } else {
                (0.0, 0.5, Vec4::ZERO)
            };

            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_3d {
                    if let Some(existing_mat) = materials_3d.get_mut(existing_handle) {
                        // تحديث المادة 3D الموجودة
                        existing_mat.color = Vec4::from(base_color.to_vec4());
                        existing_mat.size = size_vec;
                        existing_mat.radius = radius;
                        existing_mat.border_color = Vec4::from(b_color.to_vec4());
                        existing_mat.emissive = emissive_val;
                        existing_mat.border_width = b_width;
                        existing_mat.metallic = metallic;
                        existing_mat.roughness = roughness;
                        existing_mat.use_texture = use_tex;
                        existing_mat.shape_mode = shape_mode;
                        existing_mat.texture = tex_handle.clone();
                        
                        pool.reused_count += 1;
                        existing_handle.clone()
                    } else {
                        // إعادة إنشاء 3D
                        let new_mat = materials_3d.add(create_3d_material(
                            base_color, size_vec, radius, b_color, emissive_val,
                            b_width, metallic, roughness, use_tex, shape_mode, tex_handle.clone()
                        ));
                        handles.material_3d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                    let new_mat = materials_3d.add(create_3d_material(
                        base_color, size_vec, radius, b_color, emissive_val,
                        b_width, metallic, roughness, use_tex, shape_mode, tex_handle.clone()
                    ));
                    handles.material_3d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                let new_mat = materials_3d.add(create_3d_material(
                    base_color, size_vec, radius, b_color, emissive_val,
                    b_width, metallic, roughness, use_tex, shape_mode, tex_handle.clone()
                ));
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: None,
                    material_3d: Some(new_mat.clone()),
                });
                pool.created_count += 1;
                new_mat
            };

            // تطبيق مكونات 3D وإزالة 2D
            commands.entity(entity)
                .insert((
                    Mesh3d(mesh),
                    MeshMaterial3d(material_handle),
                ))
                .remove::<(Mesh2d, MeshMaterial2d<UNodeMaterial>)>();

        } else {
            // >>>> مسار 2D (مع القص) <<<<
            
            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_2d {
                    if let Some(existing_mat) = materials_2d.get_mut(existing_handle) {
                        // تحديث المادة 2D الموجودة
                        existing_mat.color = base_color;
                        existing_mat.radius = radius;
                        existing_mat.border_color = b_color;
                        existing_mat.size = size_vec;
                        existing_mat.border_width = b_width;
                        existing_mat.border_offset = b_offset;
                        existing_mat.use_texture = use_tex;
                        existing_mat.shape_mode = shape_mode;
                        existing_mat.texture = tex_handle.clone();
                        
                        // تحديث بيانات القص
                        existing_mat.clip_center = clip_center;
                        existing_mat.clip_size = clip_size;
                        existing_mat.clip_radius = clip_radius;
                        existing_mat.use_clip = use_clip;
                        
                        pool.reused_count += 1;
                        existing_handle.clone()
                    } else {
                        // إعادة إنشاء 2D
                        let new_mat = materials_2d.add(create_2d_material(
                            base_color, radius, b_color, size_vec,
                            b_width, b_offset, use_tex, shape_mode, tex_handle.clone(),
                            clip_center, clip_size, clip_radius, use_clip
                        ));
                        handles.material_2d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                     let new_mat = materials_2d.add(create_2d_material(
                        base_color, radius, b_color, size_vec,
                        b_width, b_offset, use_tex, shape_mode, tex_handle.clone(),
                        clip_center, clip_size, clip_radius, use_clip
                    ));
                    handles.material_2d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                let new_mat = materials_2d.add(create_2d_material(
                    base_color, radius, b_color, size_vec,
                    b_width, b_offset, use_tex, shape_mode, tex_handle.clone(),
                    clip_center, clip_size, clip_radius, use_clip
                ));
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: Some(new_mat.clone()),
                    material_3d: None,
                });
                pool.created_count += 1;
                new_mat
            };

            // تطبيق مكونات 2D وإزالة 3D
            commands.entity(entity)
                .insert((
                    Mesh2d(mesh),
                    MeshMaterial2d(material_handle),
                ))
                .remove::<(Mesh3d, MeshMaterial3d<UNodeMaterial3d>)>();
        }
    }
    
    if let Some(ref mut prof) = profiler {
        prof.materials_created = pool.created_count - created_before;
        prof.materials_reused = pool.reused_count - reused_before;
        prof.material_update_time = start.elapsed().as_secs_f64() * 1000.0;
    }
}

// ===== Helper Functions =====
// 1. دالة البحث عن القص (مشتركة)
fn find_clipper(
    start_entity: Entity,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> (Vec2, Vec2, Vec4, u32) {
    let mut current_entity = start_entity;
    while let Ok(parent) = parents_query.get(current_entity) {
        current_entity = parent.get();
        if let Ok((transform, size, node, clip)) = clipper_query.get(current_entity) {
            if clip.enabled {
                let center = transform.translation().truncate();
                let clip_size = Vec2::new(size.width, size.height);
                let radius = Vec4::new(
                    node.border_radius.top_right, node.border_radius.bottom_right,
                    node.border_radius.top_left, node.border_radius.bottom_left,
                );
                return (center, clip_size, radius, 1);
            }
        }
    }
    (Vec2::ZERO, Vec2::ZERO, Vec4::ZERO, 0)
}

// 2. دالة إنشاء مادة 2D (محدثة مع بيانات القص)
fn create_2d_material(
    base_color: LinearRgba, radius: Vec4, b_color: LinearRgba, size_vec: Vec2,
    b_width: f32, b_offset: f32, use_tex: u32, shape_mode: u32, tex: Option<Handle<Image>>,
    // بيانات القص
    clip_center: Vec2, clip_size: Vec2, clip_radius: Vec4, use_clip: u32,
) -> UNodeMaterial {
    UNodeMaterial {
        color: base_color,
        radius,
        border_color: b_color,
        size: size_vec,
        border_width: b_width,
        border_offset: b_offset,
        softness: 1.0,
        use_texture: use_tex,
        shape_mode,
        texture: tex,
        _pad: 0.0,
        // الحقول الجديدة
        clip_center,
        clip_size,
        clip_radius,
        use_clip,
    }
}

// 3. دالة إنشاء مادة 3D (الأصلية - بدون تغييرات القص حالياً)
fn create_3d_material(
    base_color: LinearRgba, size_vec: Vec2, radius: Vec4, b_color: LinearRgba,
    emissive: Vec4, b_width: f32, metallic: f32, roughness: f32,
    use_tex: u32, shape_mode: u32, tex: Option<Handle<Image>>,
) -> UNodeMaterial3d {
    UNodeMaterial3d {
        color: Vec4::from(base_color.to_vec4()),
        size: size_vec,
        radius,
        border_color: Vec4::from(b_color.to_vec4()),
        emissive,
        border_width: b_width,
        softness: 1.0,
        metallic,
        roughness,
        use_texture: use_tex,
        shape_mode,
        texture: tex,
    }
}