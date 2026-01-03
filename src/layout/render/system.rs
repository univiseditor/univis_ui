use bevy::{prelude::*};

use crate::prelude::*;

/// النظام المسؤول عن تحديث المرئيات (Visuals).
/// يراقب التغييرات في المكونات المنطقية (UNode, Size, Image...) ويقوم بإنشاء/تحديث
/// الـ Mesh والـ Material المناسبين (سواء 2D أو 3D).
pub fn update_shader_visuals(
    mut commands: Commands,
    
    // استعلام شامل يبحث عن أي تغيير في الخصائص المؤثرة على الشكل
    query: Query<
        (
            Entity,
            &UNode,
            &ComputedSize,
            Option<&UBorder>,
            Option<&UImage>,
            Option<&UI3d>,
            Option<&UPbr>,
        ),
        (
            Or<(
                Changed<UNode>,
                Changed<ComputedSize>,
                Changed<UBorder>,
                Changed<UImage>,
                Changed<UI3d>,
                Changed<UPbr>,
            )>,
            // شرط: يجب أن يكون للعنصر حجم محسوب (أي مر بمرحلة التخطيط)
            With<ComputedSize> 
        )
    >,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
    mut materials_3d: ResMut<Assets<UNodeMaterial3d>>,
    // mut debug_materials: ResMut<Assets<StandardMaterial>>, 
) {
    for (entity, node, size, border, image, ui3d_opt, pbr_opt) in query.iter() {
        
        // 1. تجهيز البيانات المشتركة (Geometry & Colors)
        // ----------------------------------------------------
        let size_vec = Vec2::new(size.width, size.height);
        
        // إذا كان الحجم صفراً، لا داعي لإنشاء مش (تحسين أداء)
        if size_vec.x <= 0.0 || size_vec.y <= 0.0 {
            continue;
        }

        // إعداد الصورة (Texture & Tint)
        let (tex_handle, use_tex, base_color) = if let Some(img) = image {
            (Some(img.texture.clone()), 1, LinearRgba::from(img.color))
        } else {
            (None, 0, LinearRgba::from(node.background_color))
        };

        // إعداد الحدود (Border)
        let (b_color, b_offset, b_width) = if let Some(b) = border {
            (LinearRgba::from(b.color),b.offset, b.width)
        } else {
            (LinearRgba::NONE,0.0, 0.0)
        };

        // إعداد الزوايا (Radius)
        let radius = Vec4::new(
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        );


        // info!("Entity {:?} Size: {:?}", entity, size_vec); 

        // إنشاء الـ Mesh (مستطيل قياسي)
        // Rectangle::new ينشئ Mesh مع UVs وهذا ضروري للصور ولحسابات SDF
        let mesh = meshes.add(Rectangle::new(size_vec.x, size_vec.y));

        // 2. التفريع: هل نحن في العالم 3D أم 2D؟
        // ----------------------------------------------------
        // داخل دالة update_shader_visuals ...

        if ui3d_opt.is_some() {
            // جلب خصائص PBR
            let (metallic, roughness, emissive_val) = if let Some(pbr) = pbr_opt {
                (
                    pbr.metallic, 
                    pbr.roughness, 
                    // تحويل LinearRgba إلى Vec4
                    Vec4::from(pbr.emissive.to_vec4()) 
                )
            } else {
                (0.0, 0.5, Vec4::ZERO)
            };

            // داخل دالة update_shader_visuals
        // ...
            let material = materials_3d.add(UNodeMaterial3d {
                color: Vec4::from(base_color.to_vec4()),
                size: size_vec,
                radius,
                border_color: Vec4::from(b_color.to_vec4()),
                emissive: emissive_val,
                
                border_width: b_width,
                softness: 1.0,  
                metallic,
                roughness,
                use_texture: use_tex,
                
                // تحويل الـ Enum إلى u32
                shape_mode: match node.shape_mode {
                    UShapeMode::Round => 0,
                    UShapeMode::Cut => 1,
                },

                texture: tex_handle,
            });
        // ...


            // let debug_mat = debug_materials.add(StandardMaterial {
            //     base_color: Color::Srgba(RED),
            //     unlit: true, // يضيء بنفسه لنتأكد أن الإضاءة ليست السبب
            //     cull_mode: None, // يرسم الوجهين
            //     ..default()
            // });

            // تطبيق المكونات (مع تنظيف مكونات 2D القديمة إن وجدت)
            commands.entity(entity)
                .insert((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    // ملاحظة: لا نضيف Transform هنا، لأنه موجود أصلاً من UNode
                ))
                .remove::<(Mesh2d, MeshMaterial2d<UNodeMaterial>)>();
                continue;
        } else {
            // === المسار ثنائي الأبعاد (Screen/Camera2d) ===
            
// داخل update_shader_visuals ... المسار else (2D)

            // في مسار الـ 2D داخل update_shader_visuals

// في مسار 2D داخل update_shader_visuals

            let material = materials_2d.add(UNodeMaterial {
                color: LinearRgba::from(base_color),

                radius,
                border_color: LinearRgba::from(b_color),
                
                size: size_vec,
                
                border_width: b_width,
                softness: 1.0, 
                use_texture: use_tex,
                
                shape_mode: match node.shape_mode {
                    UShapeMode::Round => 0,
                    UShapeMode::Cut => 1,
                },
                
                texture: tex_handle,
                border_offset: b_offset,
                _pad: 0.0,
            });

            // تطبيق المكونات (مع تنظيف مكونات 3D القديمة إن وجدت)
            commands.entity(entity)
                .insert((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                ))
                .remove::<(Mesh3d, MeshMaterial3d<UNodeMaterial3d>)>();
        }
    }
}