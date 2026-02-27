use bevy::camera::primitives::Aabb;
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility)] 
pub struct UTextLabel {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub font: Handle<Font>,
    pub justify: Justify,
    pub linebreak: LineBreak,
    /// هل يجب أن يفرض النص حجمه على UNode؟
    /// إذا كان true، سيتم تحديث width/height للـ UNode تلقائياً.
    pub autosize: bool, 
}

impl Default for UTextLabel {
    fn default() -> Self {
        Self {
            text: "Label".to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            font: Handle::default(),
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
            autosize: true, // افتراضياً، النص يتحكم بالحجم
        }
    }
}

impl UTextLabel {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..default()
        }
    }
}

#[derive(Component)]
pub struct TextChildMarker;

pub fn init_text_label_container(
    mut commands: Commands,
    query: Query<(Entity, &UTextLabel), Added<UTextLabel>>,
) {
    for (entity, label) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                // Text2d, // علامة النص في Bevy 0.17
                Text2d(label.text.clone()),
                TextFont {
                    font: label.font.clone(),
                    font_size: label.font_size,
                    ..default()
                },
                TextColor(label.color),
                TextLayout {
                    justify: label.justify,
                    linebreak: label.linebreak,
                    ..default()
                },
                // موقع النص بالنسبة للأب (الحاوية)
                // Z=0.1 لضمان ظهوره فوق خلفية UNode
                Transform::from_xyz(0.0, 0.0, 0.1), 
                TextChildMarker,
                // ملاحظة: لا نعطي الطفل UNode، هو مجرد عارض للنص
            ));
        });
    }
}

pub fn fit_node_to_text_size(
    // 1. استعلام للأباء (الحاويات)
    mut parent_query: Query<(&UTextLabel, &mut UNode, &Children)>,
    // 2. استعلام للأطفال (للحصول على حجم النص المحسوب)
    child_query: Query<&TextLayoutInfo, With<TextChildMarker>>,
) {
    for (label, mut node, children) in parent_query.iter_mut() {
        // ننفذ فقط إذا كانت خاصية التحجيم التلقائي مفعلة
        if !label.autosize { continue; }

        for &child in children {
            if let Ok(info) = child_query.get(child) {
                let text_size = info.size;

                // نتأكد أن النص تم حسابه فعلاً
                if text_size.x == 0.0 && text_size.y == 0.0 { continue; }

                // 1. حساب البادينغ الحالي من الـ UNode
                let h_pad = node.padding.width_sum();
                let v_pad = node.padding.height_sum();

                // 2. الحجم الكلي المطلوب = حجم النص + البادينغ
                let target_width = text_size.x + h_pad;
                let target_height = text_size.y + v_pad;

                // 3. تحديث UNode مباشرة (Pixel Values)
                // نستخدم هامش خطأ بسيط (Epsilon) لتجنب التحديث المستمر إذا لم يتغير شيء
                let current_w = match node.width { UVal::Px(v) => v, _ => -1.0 };
                let current_h = match node.height { UVal::Px(v) => v, _ => -1.0 };

                if (current_w - target_width).abs() > 0.1 {
                    node.width = UVal::Px(target_width);
                }
                
                if (current_h - target_height).abs() > 0.1 {
                    node.height = UVal::Px(target_height);
                }
            }
        }
    }
}

pub fn sync_text_label_props(
    label_query: Query<(&UTextLabel, &Children), Changed<UTextLabel>>,
    mut text_query: Query<(&mut Text2d, &mut TextFont, &mut TextColor, &mut TextLayout), With<TextChildMarker>>,
) {
    for (label, children) in label_query.iter() {
        for &child in children {
            if let Ok((mut text, mut font, mut color, mut _layout)) = text_query.get_mut(child) {
                if **text != label.text { **text = label.text.clone(); }
                if font.font_size != label.font_size { font.font_size = label.font_size; }
                if color.0 != label.color { color.0 = label.color; }
                // ... باقي الخصائص
            }
        }
    }
}

pub fn sync_text_clip_visibility(
    mut text_query: Query<
        (
            Entity,
            &GlobalTransform,
            Option<&Aabb>,
            Option<&TextLayoutInfo>,
            &mut Visibility,
        ),
        With<TextChildMarker>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UClip)>,
) {
    for (entity, global_transform, aabb, layout_info, mut visibility) in text_query.iter_mut() {
        let Some(world_quad) = text_world_quad(global_transform, aabb, layout_info) else {
            continue;
        };

        let visible_in_clips =
            is_quad_fully_inside_active_clippers(entity, &world_quad, &parents_query, &clipper_query);

        *visibility = if visible_in_clips {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn text_world_quad(
    global_transform: &GlobalTransform,
    aabb: Option<&Aabb>,
    layout_info: Option<&TextLayoutInfo>,
) -> Option<[Vec2; 4]> {
    if let Some(aabb) = aabb {
        return Some(local_quad_to_world(
            global_transform,
            Vec2::new(aabb.center.x, aabb.center.y),
            Vec2::new(aabb.half_extents.x, aabb.half_extents.y),
        ));
    }

    let layout_info = layout_info?;
    if layout_info.size.x <= 0.0 || layout_info.size.y <= 0.0 {
        return None;
    }

    Some(local_quad_to_world(
        global_transform,
        Vec2::ZERO,
        layout_info.size * 0.5,
    ))
}

fn local_quad_to_world(
    global_transform: &GlobalTransform,
    local_center: Vec2,
    local_half_extents: Vec2,
) -> [Vec2; 4] {
    let transform = global_transform.to_matrix();
    let local_points = [
        Vec2::new(
            local_center.x - local_half_extents.x,
            local_center.y - local_half_extents.y,
        ),
        Vec2::new(
            local_center.x - local_half_extents.x,
            local_center.y + local_half_extents.y,
        ),
        Vec2::new(
            local_center.x + local_half_extents.x,
            local_center.y - local_half_extents.y,
        ),
        Vec2::new(
            local_center.x + local_half_extents.x,
            local_center.y + local_half_extents.y,
        ),
    ];

    [
        transform
            .transform_point3(local_points[0].extend(0.0))
            .truncate(),
        transform
            .transform_point3(local_points[1].extend(0.0))
            .truncate(),
        transform
            .transform_point3(local_points[2].extend(0.0))
            .truncate(),
        transform
            .transform_point3(local_points[3].extend(0.0))
            .truncate(),
    ]
}

fn is_quad_fully_inside_active_clippers(
    entity: Entity,
    world_quad: &[Vec2; 4],
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UClip)>,
) -> bool {
    let mut current = entity;

    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();

        let Ok((clip_transform, clip_size, clip)) = clipper_query.get(current) else {
            continue;
        };

        if !clip.enabled {
            continue;
        }

        let half_size = Vec2::new(clip_size.width, clip_size.height) * 0.5;
        if !is_quad_inside_oriented_rect(world_quad, clip_transform, half_size) {
            return false;
        }
    }

    true
}

fn is_quad_inside_oriented_rect(
    world_quad: &[Vec2; 4],
    rect_transform: &GlobalTransform,
    rect_half_size: Vec2,
) -> bool {
    let inv = rect_transform.to_matrix().inverse();

    world_quad.iter().all(|world_point| {
        let local = inv.transform_point3(world_point.extend(0.0)).truncate();
        local.x >= -rect_half_size.x
            && local.x <= rect_half_size.x
            && local.y >= -rect_half_size.y
            && local.y <= rect_half_size.y
    })
}

pub struct UnivisTextPlugin;

impl Plugin for UnivisTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UTextLabel>()
            .add_systems(Update, (
                init_text_label_container,
                sync_text_label_props,
                
                // النظام الجديد لفرض الحجم
                fit_node_to_text_size
                    // .after(bevy::text::TextSystem::UpdateLayout) // بعد حساب حجم النص
                    .before(upward_measure_pass_cached) // قبل بدء نظام التخطيط الخاص بك
                    // وأيضاً قبل تحديث المرئيات لضمان أن الخلفية تأخذ الحجم الصحيح في نفس الإطار
                    .before(update_materials_optimized),
                sync_text_clip_visibility,
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_inside_oriented_rect_returns_true() {
        let rect_transform = GlobalTransform::from(Transform::default());
        let rect_half_size = Vec2::new(50.0, 30.0);
        let quad = [
            Vec2::new(-20.0, -10.0),
            Vec2::new(-20.0, 10.0),
            Vec2::new(20.0, -10.0),
            Vec2::new(20.0, 10.0),
        ];

        assert!(is_quad_inside_oriented_rect(
            &quad,
            &rect_transform,
            rect_half_size,
        ));
    }

    #[test]
    fn quad_outside_oriented_rect_returns_false() {
        let rect_transform = GlobalTransform::from(Transform::default());
        let rect_half_size = Vec2::new(50.0, 30.0);
        let quad = [
            Vec2::new(-20.0, -10.0),
            Vec2::new(-20.0, 10.0),
            Vec2::new(60.0, -10.0),
            Vec2::new(60.0, 10.0),
        ];

        assert!(!is_quad_inside_oriented_rect(
            &quad,
            &rect_transform,
            rect_half_size,
        ));
    }
}
