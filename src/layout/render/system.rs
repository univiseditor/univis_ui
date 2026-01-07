use bevy::prelude::*;
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
    mut profiler: Option<ResMut<LayoutProfiler>>, // إضافة Profiler اختياري
    
    // العقد التي تغيرت
    mut query: Query<
        (
            Entity,
            &UNode,
            &ComputedSize,
            Option<&UBorder>,
            Option<&UImage>,
            Option<&UI3d>,
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
        )>
    >,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
    mut materials_3d: ResMut<Assets<UNodeMaterial3d>>,
) {
    let start = std::time::Instant::now();
    
    // حفظ العدادات
    let created_before = pool.created_count;
    let reused_before = pool.reused_count;
    
    for (entity, node, size, border, image, ui3d_opt, pbr_opt, handles_opt) in query.iter_mut() {
        
        let size_vec = Vec2::new(size.width, size.height);
        
        if size_vec.x <= 0.0 || size_vec.y <= 0.0 {
            continue;
        }

        // إعداد البيانات
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
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        );

        let shape_mode = match node.shape_mode {
            UShapeMode::Round => 0,
            UShapeMode::Cut => 1,
        };

        // إنشاء أو إعادة استخدام Mesh
        let mesh = meshes.add(Rectangle::new(size_vec.x, size_vec.y));

        if ui3d_opt.is_some() {
            // === وضع 3D ===
            
            let (metallic, roughness, emissive_val) = if let Some(pbr) = pbr_opt {
                (pbr.metallic, pbr.roughness, Vec4::from(pbr.emissive.to_vec4()))
            } else {
                (0.0, 0.5, Vec4::ZERO)
            };

            // محاولة إعادة استخدام المادة الموجودة
            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_3d {
                    // تحديث المادة الموجودة
                    if let Some(existing_mat) = materials_3d.get_mut(existing_handle) {
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
                        // المادة غير موجودة، إنشاء جديدة
                        let new_mat = materials_3d.add(create_3d_material(
                            base_color, size_vec, radius, b_color, emissive_val,
                            b_width, metallic, roughness, use_tex, shape_mode, tex_handle
                        ));
                        handles.material_3d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                    // لا توجد مادة سابقة، إنشاء جديدة
                    let new_mat = materials_3d.add(create_3d_material(
                        base_color, size_vec, radius, b_color, emissive_val,
                        b_width, metallic, roughness, use_tex, shape_mode, tex_handle
                    ));
                    handles.material_3d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                // لا يوجد مكون MaterialHandles، إنشاؤه
                let new_mat = materials_3d.add(create_3d_material(
                    base_color, size_vec, radius, b_color, emissive_val,
                    b_width, metallic, roughness, use_tex, shape_mode, tex_handle
                ));
                
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: None,
                    material_3d: Some(new_mat.clone()),
                });
                
                pool.created_count += 1;
                new_mat
            };

            commands.entity(entity)
                .insert((
                    Mesh3d(mesh),
                    MeshMaterial3d(material_handle),
                ))
                .remove::<(Mesh2d, MeshMaterial2d<UNodeMaterial>)>();

        } else {
            // === وضع 2D ===
            
            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_2d {
                    if let Some(existing_mat) = materials_2d.get_mut(existing_handle) {
                        existing_mat.color = base_color;
                        existing_mat.radius = radius;
                        existing_mat.border_color = b_color;
                        existing_mat.size = size_vec;
                        existing_mat.border_width = b_width;
                        existing_mat.border_offset = b_offset;
                        existing_mat.use_texture = use_tex;
                        existing_mat.shape_mode = shape_mode;
                        existing_mat.texture = tex_handle;
                        
                        pool.reused_count += 1;
                        existing_handle.clone()
                    } else {
                        let new_mat = materials_2d.add(create_2d_material(
                            base_color, radius, b_color, size_vec,
                            b_width, b_offset, use_tex, shape_mode, tex_handle
                        ));
                        handles.material_2d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                    let new_mat = materials_2d.add(create_2d_material(
                        base_color, radius, b_color, size_vec,
                        b_width, b_offset, use_tex, shape_mode, tex_handle
                    ));
                    handles.material_2d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                let new_mat = materials_2d.add(create_2d_material(
                    base_color, radius, b_color, size_vec,
                    b_width, b_offset, use_tex, shape_mode, tex_handle
                ));
                
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: Some(new_mat.clone()),
                    material_3d: None,
                });
                
                pool.created_count += 1;
                new_mat
            };

            commands.entity(entity)
                .insert((
                    Mesh2d(mesh),
                    MeshMaterial2d(material_handle),
                ))
                .remove::<(Mesh3d, MeshMaterial3d<UNodeMaterial3d>)>();
        }
    }
    
    // تحديث Profiler إن وجد
    if let Some(ref mut prof) = profiler {
        prof.materials_created = pool.created_count - created_before;
        prof.materials_reused = pool.reused_count - reused_before;
        prof.material_update_time = start.elapsed().as_secs_f64() * 1000.0;
    }
}

// ===== Helper Functions =====

fn create_3d_material(
    base_color: LinearRgba,
    size_vec: Vec2,
    radius: Vec4,
    b_color: LinearRgba,
    emissive: Vec4,
    b_width: f32,
    metallic: f32,
    roughness: f32,
    use_tex: u32,
    shape_mode: u32,
    tex: Option<Handle<Image>>,
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

fn create_2d_material(
    base_color: LinearRgba,
    radius: Vec4,
    b_color: LinearRgba,
    size_vec: Vec2,
    b_width: f32,
    b_offset: f32,
    use_tex: u32,
    shape_mode: u32,
    tex: Option<Handle<Image>>,
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
    }
}

// /// نظام لطباعة إحصائيات الـ Pool (اختياري للـ Debug)
// #[cfg(feature = "debug_pool")]
// pub fn log_pool_stats(
//     pool: Res<MaterialPool>,
//     mut timer: Local<f32>,
//     time: Res<Time>,
// ) {
//     *timer += time.delta_secs();
    
//     if *timer >= 5.0 {
//         info!(
//             "Material Pool Stats - Reused: {}, Created: {}",
//             pool.reused_count,
//             pool.created_count
//         );
//         *timer = 0.0;
//     }
// }

// /// Plugin لنظام Material Pooling
// pub struct MaterialPoolPlugin;

// impl Plugin for MaterialPoolPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .init_resource::<MaterialPool>()
//             .add_systems(Update, update_materials_optimized);
        
//         #[cfg(feature = "debug_pool")]
//         app.add_systems(Update, log_pool_stats);
//     }
// }