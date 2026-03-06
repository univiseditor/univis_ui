use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisCheckboxPlugin;

impl Plugin for UnivisCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UCheckbox>()
            .add_systems(Update, (
                init_checkbox,      // بناء الشكل عند الإنشاء
                // update_checkbox_visuals // تحديث الألوان والظهور
            ))
            .add_observer(toggle_checkbox_handler);
    }
}

// =========================================================
// 1. المكونات (Components)
// =========================================================

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UCheckbox {
    pub checked: bool,
    pub label: Option<String>,
    
    // --- Style Config ---
    pub size: f32,
    pub checked_color: Color,
    pub unchecked_color: Color,
    pub border_color: Color,
}

impl Default for UCheckbox {
    fn default() -> Self {
        Self {
            checked: false,
            label: None,
            size: 24.0,
            checked_color: Color::srgb(0.2, 0.5, 0.9), // أزرق
            unchecked_color: Color::srgb(0.2, 0.2, 0.2), // رمادي غامق
            border_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

// =========================================================
// 2. الأنظمة (Systems)
// =========================================================

/// 1. بناء هيكل الـ Checkbox عند إضافته لأول مرة
fn init_checkbox(
    mut commands: Commands,
    query: Query<(Entity, &UCheckbox), Added<UCheckbox>>,
    _asset_server: Res<AssetServer>, // لتحميل خط افتراضي
) {
    for (entity, checkbox) in query.iter() {
        
        // إعداد الحاوية الرئيسية (Row Layout)
        commands.entity(entity).insert((
            UNode {
                // الحجم يتحدد بالمحتوى (المربع + النص)
                width: UVal::Content,
                height: UVal::Content,
                padding: USides::all(4.0), // مساحة للنقر
                background_color: Color::NONE, // خلفية شفافة للحاوية الكلية
                ..default()
            }, 
            ULayout {
                align_items: UAlignItems::Center, // محاذاة النص مع المربع
                justify_content: UJustifyContent::Center,
                gap: 8.0, // مسافة بين المربع والنص
                ..default()
            },
            
        ));

        commands.entity(entity).with_children(|parent| {

            let color;
            let border_color;
            if checkbox.checked {
               color =  checkbox.checked_color;
               border_color = checkbox.border_color;
            } else {
                border_color = checkbox.border_color;
                color = checkbox.unchecked_color;
                }

            // أ) المربع نفسه (The Box)
            parent.spawn((
                UNode {
                    width: UVal::Px(checkbox.size),
                    height: UVal::Px(checkbox.size),
                    border_radius: UCornerRadius::all(checkbox.size * 0.25), // زوايا دائرية قليلاً
                    background_color: color,
                    ..default()
                },
                
                UBorder {
                    width: 2.0,
                    color: border_color,
                    offset: 4.0,
                    radius: UCornerRadius::all(checkbox.size * 0.25),
                },
                // نحتاج لمعرفة هذا المربع لتغيير لونه لاحقاً
            ));

            // ج) النص (Label) - إذا وجد
            if let Some(text) = &checkbox.label {
                parent.spawn((
                    UTextLabel {
                        text: text.clone(),
                        font_size: checkbox.size * 0.75,
                        color: Color::WHITE,
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            }
        });
    }
}

/// 2. منطق التغيير (Logic Handler)
fn toggle_checkbox_handler(
    trigger: On<Pointer<Click>>,
    mut box_query: Query<(&mut UNode, &mut UBorder)>,
    mut parent_query: Query<(&mut UCheckbox, &Children)>,
) {
    let entity = trigger.entity.entity();
    if let Ok((mut checkbox, child)) = parent_query.get_mut(entity) {
        checkbox.checked = !checkbox.checked;
        
        for &child in child {
            
            if let Ok((mut node,mut border)) = box_query.get_mut(child) {
                if checkbox.checked {
                    node.background_color = checkbox.checked_color;
                    border.color = checkbox.checked_color; // إخفاء الحدود عند التحديد (ستايل حديث)
                } else {
                    node.background_color = checkbox.unchecked_color;
                    border.color = checkbox.border_color;
                }
            }
        }
    }
}

impl UCheckbox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            ..default()
        }
    }
    
    pub fn checked(mut self, state: bool) -> Self {
        self.checked = state;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.checked_color = color;
        self
    }
}