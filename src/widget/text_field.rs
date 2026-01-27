use std::collections::HashSet;

use bevy::{ecs::relationship::{RelationshipSourceCollection}, prelude::*};
use crate::prelude::*;

/// المكون الرئيسي لحقل النص
/// يحتوي على الحالة والبيانات فقط (بدون منطق العرض)
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, UInteraction, ComputedSize)]
pub struct UTextField {
    pub value: String,
    pub placeholder: String,
    pub label: Option<String>,
    pub disabled: bool,
    pub error: bool,
    pub focused: bool,
    pub input_type: UInputType, // قمت بتعريفها في ملفك السابق
    pub max_length: Option<usize>,
    
    // إعدادات الألوان (Visual Config)
    pub primary_color: Color,
    pub error_color: Color,
    pub text_color: Color,
    pub cursor_color: Color,
    pub placeholder_color: Color,
}

impl Default for UTextField {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            label: None,
            disabled: false,
            error: false,
            focused: false,
            input_type: UInputType::Text,
            max_length: None,
            // ألوان افتراضية
            primary_color: Color::srgb(0.2, 0.6, 1.0),
            error_color: Color::srgb(1.0, 0.2, 0.2),
            text_color: Color::WHITE,
            cursor_color: Color::WHITE,
            placeholder_color: Color::srgb(0.6, 0.6, 0.6),
        }
    }
}

// علامات (Markers) للأبناء لسهولة الوصول إليهم في نظام العرض
#[derive(Component)]
pub struct UTextFieldInputText;

#[derive(Component)]
pub struct UTextFieldLabel;


/// نظام تحديث المؤقت للوميض (Caret Blink)
fn caret_blink_system(
    time: Res<Time>,
    mut blink: ResMut<TextFieldCaretBlink>
) {
    blink.timer.tick(time.delta());
    if blink.timer.just_finished() {
        blink.visible = !blink.visible;
    }
}

pub fn text_field_visual_event_system(
    blink: Res<TextFieldCaretBlink>,
    active_field: Res<ActiveTextField>,
    
    mut change_events: MessageReader<UTextFieldChangeEvent>,

    // 1. التعديل هنا: أضفنا Entity إلى الـ Query مباشرة
    mut field_query: Query<(Entity, Ref<UTextField>, &Children, &mut UNode, Option<&mut UBorder>)>,
    
    mut text_query: Query<(&mut UTextLabel, &mut TextColor), With<UTextFieldInputText>>,
    
    mut label_query: Query<(&mut UTextLabel, &mut TextColor), (With<UTextFieldLabel>, Without<UTextFieldInputText>)>,
) {
    // 1. تحديد الكيانات التي تحتاج تحديثاً (Dirty Set)
    let mut dirty_entities = HashSet::new();

    for ev in change_events.read() {
        dirty_entities.insert(ev.entity);
    }

    if let Some(active_entity) = active_field.0 {
        // إذا حان وقت الوميض أو تغير الحقل النشط، نعتبره "متسخاً" لتحديث المؤشر
        if blink.timer.just_finished() || active_field.is_changed() {
            dirty_entities.insert(active_entity);
        }
    }

    // 2. حلقة التحديث الرئيسية
    // التعديل: نستخدم iter_mut العادية، والـ entity يأتي كأول عنصر في الـ Tuple
    for (entity, field, children, mut node, mut border) in field_query.iter_mut() {
        
        // --- أ) تحديث الستايل (الخلفية والحدود) ---
        if field.is_changed() {
             let active_color = if field.error { field.error_color } else { field.primary_color };
             
             if let Some(b) = border.as_mut() {
                 b.color = if field.focused || field.error { active_color } else { Color::srgb(0.5, 0.5, 0.5) };
                 b.width = if field.focused { 2.0 } else { 1.0 };
             }
 
             node.background_color = if field.disabled { 
                 Color::srgb(0.1, 0.1, 0.1).with_alpha(0.5) 
             } else { 
                 Color::srgb(0.2, 0.2, 0.2) 
             };
        }

        // --- ب) تحديث النص والمؤشر ---
        let is_target_of_event = dirty_entities.contains(&entity);
        
        // نحتاج التحديث إذا: وصل حدث، أو الحقل تغير، أو الحقل نشط والوميض تغير
        if is_target_of_event || field.is_changed() {
            
            // تحضير النص للعرض
            let caret = if field.focused && blink.visible { "|" } else { "" };
            
            let display_text = if field.value.is_empty() {
                if field.focused {
                    format!("{}", caret)
                } else {
                    field.placeholder.clone()
                }
            } else {
                let val = if field.input_type == UInputType::Password {
                    "*".repeat(field.value.chars().count())
                } else {
                    field.value.clone()
                };
                format!("{}{}", val, caret)
            };

            let text_color_val = if field.value.is_empty() && !field.focused {
                field.placeholder_color
            } else {
                field.text_color
            };

            // البحث في الأبناء لتحديثهم
            for child in children.iter() {
                // تحديث النص الأساسي
                if let Ok((mut text, mut color)) = text_query.get_mut(child) {
                    if text.text != display_text {
                        text.text = display_text.clone();
                    }
                    if color.0 != text_color_val {
                        color.0 = text_color_val;
                    }
                }
                
                // تحديث العنوان (Label)
                if let Ok((mut label_text, mut label_color)) = label_query.get_mut(child) {
                    if let Some(label_str) = &field.label {
                        if label_text.text != *label_str {
                            label_text.text = label_str.clone();
                        }
                        
                        let is_active = field.focused || !field.value.is_empty();
                        let active_col = if field.error { field.error_color } else { field.primary_color };
                        let target_col = if is_active { active_col } else { field.text_color.with_alpha(0.7) };

                        if label_color.0 != target_col {
                            label_color.0 = target_col;
                        }
                    }
                }
            }
        }
    }
}


pub struct UnivisTextFieldVisualPlugin;

impl Plugin for UnivisTextFieldVisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TextFieldCaretBlink>()
            .add_systems(Update, (
            caret_blink_system,
            text_field_focus_system
        ));
    }
}


pub struct UTextFieldBuilder {
    field: UTextField,
    width: UVal,
}

impl UTextFieldBuilder {
    pub fn new() -> Self {
        Self {
            field: UTextField::default(),
            width: UVal::Px(250.0), // عرض افتراضي
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.field.label = Some(label.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.field.placeholder = placeholder.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.field.value = value.into();
        self
    }

    pub fn password(mut self) -> Self {
        self.field.input_type = UInputType::Password;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = UVal::Px(width);
        self
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands) -> Entity {
        parent.spawn((
            self.field,
            // تنسيق الحاوية الخارجية (SDF Box)
            UNode {
                width: self.width,
                height: UVal::Px(50.0), // ارتفاع قياسي
                padding: USides::all(10.0),
                border_radius: UCornerRadius::all(6.0),
                background_color: Color::srgb(0.2, 0.2, 0.2),
                ..default()
            },
            UBorder {
                color: Color::WHITE,
                width: 1.0,
                radius: UCornerRadius::all(6.0),
                ..default()
            },
            ULayout {
                flex_direction: UFlexDirection::Column, // العنوان فوق، النص تحت
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Stretch,
                ..default()
            },
            UInteraction::default(), // لتفعيل النقر والتركيز
        )).with_children(|container| {
            
            // 1. إنشاء العنوان (Label) - اختياري
            if !container.target_entity().is_empty() { 
                 container.spawn((
                    UTextFieldLabel,
                    UTextLabel::new("Label"), // سيتم تحديثه بواسطة Visual System
                    // TextFont { font_size: 10.0, ..default() },
                    // TextColor(Color::WHITE),
                    UNode {
                        margin: USides::bottom(2.0),
                        height: UVal::Content,
                        ..default()
                    }
                ));
            }

            // 2. إنشاء نص الإدخال (Input Text)
            container.spawn((
                UTextFieldInputText,
                UTextLabel::new(""),
                // TextFont { font_size: 14.0, ..default() },
                // TextColor(Color::WHITE),
                UNode {
                    height: UVal::Content,
                    ..default()
                }
            ));
        }).id()
    }
}