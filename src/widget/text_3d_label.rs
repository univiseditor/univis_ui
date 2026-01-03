use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility)] 
pub struct U3DTextLabel {
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

impl Default for U3DTextLabel {
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

#[derive(Component)]
struct TextChildMarker;

pub fn init_text_label_container(
    mut commands: Commands,
    query: Query<(Entity, &U3DTextLabel), Added<U3DTextLabel>>,
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

fn fit_node_to_text_size(
    // 1. استعلام للأباء (الحاويات)
    mut parent_query: Query<(&U3DTextLabel, &mut UNode, &Children)>,
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

fn sync_text_label_props(
    label_query: Query<(&UTextLabel, &Children), Changed<U3DTextLabel>>,
    mut text_query: Query<(&mut Text, &mut TextFont, &mut TextColor, &mut TextLayout), With<TextChildMarker>>,
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

pub struct UnivisText3dPlugin;

impl Plugin for UnivisText3dPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<U3DTextLabel>()
            .add_systems(Update, (
                init_text_label_container,
                sync_text_label_props,
                
                // النظام الجديد لفرض الحجم
                fit_node_to_text_size
                    // .after(bevy::text::TextSystem::UpdateLayout) // بعد حساب حجم النص
                    .before(upward_measure_pass) // قبل بدء نظام التخطيط الخاص بك
                    // وأيضاً قبل تحديث المرئيات لضمان أن الخلفية تأخذ الحجم الصحيح في نفس الإطار
                    .before(update_shader_visuals),
            ));
    }
}