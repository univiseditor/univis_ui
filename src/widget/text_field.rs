use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::prelude::*;

// =========================================================
// Components
// =========================================================

/// المكون الرئيسي لحقل الإدخال
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct UTextInput {
    pub value: String,
    pub placeholder: String,
    pub input_type: UInputType,
    pub max_length: Option<usize>,
    pub cursor_position: usize, // موضع المؤشر (عدد الحروف، ليس البايت)
    pub disabled: bool,
    pub password_char: char,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub focused: bool,
}

/// علامة لكيان النص الظاهر (Value)
#[derive(Component)]
pub struct UTextInputValue;

/// علامة لكيان الـ Placeholder
#[derive(Component)]
pub struct UTextInputPlaceholder;

/// علامة لكيان المؤشر (Cursor)
#[derive(Component)]
pub struct UTextInputCursor;

/// إعدادات المؤشر (مورد عام)
#[derive(Resource, Default)]
pub struct UTextInputSettings {
    pub blink_interval: f32,
    pub cursor_width: f32,
}

impl Default for UTextInput {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::from("Enter text..."),
            input_type: UInputType::Text,
            max_length: None,
            cursor_position: 0,
            disabled: false,
            focused: false,
            password_char: '•',
            text_color: Color::WHITE,
            placeholder_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

// =========================================================
// Events (Bevy 0.17 Message System)
// =========================================================

#[derive(Message, Clone, Debug)]
pub struct UTextInputChangeEvent {
    pub entity: Entity,
    pub value: String,
}

#[derive(Message, Clone, Debug)]
pub struct UTextInputSubmitEvent {
    pub entity: Entity,
    pub value: String,
}

// =========================================================
// Builder Pattern
// =========================================================

impl UTextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        let val = value.into();
        self.cursor_position = val.chars().count();
        self.value = val;
        self
    }

    pub fn password(mut self) -> Self {
        self.input_type = UInputType::Password;
        self
    }

    pub fn number(mut self) -> Self {
        self.input_type = UInputType::Number;
        self
    }

    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// =========================================================
// Systems
// =========================================================

/// نظام إدارة التركيز (Focus)
pub fn text_input_focus_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveTextField>,
    mut query: Query<(Entity, &mut UTextInput, &UInteraction), Changed<UInteraction>>,
    // للتحقق من النقر خارج الحقل
    _all_inputs: Query<Entity, With<UTextInput>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    // معالجة النقر على الحقول
    for (entity, mut input, interaction) in query.iter_mut() {
        if *interaction == UInteraction::Clicked && !input.disabled {
            input.focused = true;
            active.0 = Some(entity);
        }
    }

    // النقر خارج الجميع = إلغاء التركيز
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else {return;};
        let Ok((camera, cam_transform)) = cameras.single() else {return;};
        
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(_world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) {
                let mut clicked_on_input = false;
                
                // التحقق إذا كان النقر داخل أي UTextInput (باستخدام ComputedSize و Transform)
                // هذا يتطلب استعلاماً إضافياً لتحديد المواقع...
                // لأجل البساطة، نستخدم UInteraction كمؤشر
                for (_, _, interaction) in query.iter() {
                    if *interaction == UInteraction::Clicked {
                        clicked_on_input = true;
                        break;
                    }
                }

                if !clicked_on_input {
                    active.0 = None;
                    for (_, mut input, _) in query.iter_mut() {
                        input.focused = false;
                    }
                }
            }
        }
    }
}

/// نظام معالجة لوحة المفاتيح (Bevy 0.17 API)
pub fn text_input_keyboard_system(
    active: Res<ActiveTextField>,
    mut keyboard: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut UTextInput)>,
    mut change_events: MessageWriter<UTextInputChangeEvent>,
    mut submit_events: MessageWriter<UTextInputSubmitEvent>,
) {
    let Some(active_entity) = active.0 else { return };
    let Ok((entity, mut input)) = query.get_mut(active_entity) else { return };
    
    if input.disabled { return; }

    let mut changed = false;
    let mut cursor_moved = false;

    // --- معالجة الأزرار الخاصة (Backspace, Delete, Arrows) ---
    
    if keys.just_pressed(KeyCode::Backspace) && input.cursor_position > 0 {
        let char_idx = input.cursor_position - 1;
        if let Some((byte_idx, _)) = input.value.char_indices().nth(char_idx) {
            input.value.remove(byte_idx);
            input.cursor_position -= 1;
            changed = true;
        }
    }

    if keys.just_pressed(KeyCode::Delete) && input.cursor_position < input.value.chars().count() {
        if let Some((byte_idx, _)) = input.value.char_indices().nth(input.cursor_position) {
            input.value.remove(byte_idx);
            changed = true;
        }
    }

    if keys.just_pressed(KeyCode::ArrowLeft) && input.cursor_position > 0 {
        input.cursor_position -= 1;
        cursor_moved = true;
    }

    if keys.just_pressed(KeyCode::ArrowRight) && input.cursor_position < input.value.chars().count() {
        input.cursor_position += 1;
        cursor_moved = true;
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        submit_events.write(UTextInputSubmitEvent {
            entity,
            value: input.value.clone(),
        });
    }

    // --- معالجة إدخال الأحرف ---
    
    for ev in keyboard.read() {
        if !ev.state.is_pressed() { continue; }
        
        if let Some(text) = ev.text.as_deref() {
            for ch in text.chars() {
                if ch.is_control() { continue; }

                // فلترة الأرقام
                if input.input_type == UInputType::Number && !ch.is_numeric() {
                    continue;
                }

                // فحص الطول الأقصى
                if let Some(max) = input.max_length {
                    if input.value.chars().count() >= max { continue; }
                }

                // إدراج الحرف في موضع المؤشر
                let byte_pos = input.value.char_indices()
                    .nth(input.cursor_position)
                    .map(|(i, _)| i)
                    .unwrap_or(input.value.len());

                input.value.insert(byte_pos, ch);
                input.cursor_position += 1;
                changed = true;
            }
        }
    }

    if changed {
        change_events.write(UTextInputChangeEvent {
            entity,
            value: input.value.clone(),
        });
    }

    // تحديث المؤشر المرئي إذا تحرك (سيتم التعامل معه في نظام آخر)
    if cursor_moved {
        // يمكن إرسال حدث تحديث المؤشر هنا إذا لزم الأمر
    }
}

/// تحديث العرض المرئي (النص والمؤشر)
/// تحديث العرض المرئي (النص والمؤشر) - إصدار محدث بدون borrow conflict
pub fn text_input_visual_update(
    inputs: Query<(Entity, &UTextInput, &Children, &ComputedSize), Changed<UTextInput>>,
    mut texts: Query<(
        &mut UTextLabel, 
        Option<&UTextInputValue>, 
        Option<&UTextInputPlaceholder>
    ), Without<UTextInput>>,
    mut cursors: Query<(&mut Transform, &mut Visibility), With<UTextInputCursor>>,
) {
    for (_entity, input, children, computed_size) in inputs.iter() {
        
        // 1. تحديث النصوص (Value و Placeholder)
        for child in children.iter() {
            if let Ok((mut text, is_value, is_placeholder)) = texts.get_mut(child) {
                
                if is_value.is_some() {
                    // هذا كيان قيمة الإدخال
                    let display_text = match input.input_type {
                        UInputType::Password => {
                            input.password_char.to_string().repeat(input.value.chars().count())
                        },
                        _ => input.value.clone(),
                    };
                    text.text = display_text;
                    
                } else if is_placeholder.is_some() {
                    // هذا كيان الـ Placeholder
                    // نظهره فقط إذا كان الحقل فارغاً وغير مُركز
                    let show_placeholder = input.value.is_empty() && !input.focused;
                    text.text = if show_placeholder {
                        input.placeholder.clone()
                    } else {
                        String::new() // إخفاء Placeholder عند الكتابة
                    };
                }
            }

            // 2. تحديث المؤشر (Cursor)
            if let Ok((mut transform, mut visibility)) = cursors.get_mut(child) {
                *visibility = if input.focused && !input.disabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };

                if input.focused {
                    // حساب X بناءً على موضع المؤشر
                    // ملاحظة: هذا تقديري، يتطلب TextLayout للدقة العالية
                    let estimated_char_width = 10.0; // يجب استبداله بحساب حقيقي
                    let text_width_before_cursor = input.value[..input.cursor_position.min(input.value.len())]
                        .chars()
                        .count() as f32 * estimated_char_width;
                    
                    // توسيط النص + الإزاحة
                    let start_x = -(computed_size.width / 2.0) + 15.0; // 15px padding
                    transform.translation.x = start_x + text_width_before_cursor;
                }
            }
        }
    }
}

/// وميض المؤشر (Cursor Blink)
pub fn text_input_cursor_blink(
    time: Res<Time>,
    active: Res<ActiveTextField>,
    mut query: Query<&mut Visibility, With<UTextInputCursor>>,
) {
    // تنفيذ بسيط للوميض (يمكن تحسينه بـ Timer Resource)
    let blink_speed = 0.5; // ثانية
    let visible = ((time.elapsed_secs() / blink_speed) as i32) % 2 == 0;

    if let Some(_) = active.0 {
        for mut vis in query.iter_mut() {
            // فقط إذا كان المرئي أصلاً (للتركيز)
            if *vis != Visibility::Inherited {
                *vis = if visible { Visibility::Visible } else { Visibility::Hidden };
            }
        }
    } else {
        // إخفاء الكل عند فقدان التركيز
        for mut vis in query.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }
}

// =========================================================
// Plugin
// =========================================================

pub struct UTextInputPlugin;

impl Plugin for UTextInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UTextInputSettings>()
            .add_message::<UTextInputChangeEvent>()
            .add_message::<UTextInputSubmitEvent>()
            .add_systems(Update, (
                text_input_focus_system,
                text_input_keyboard_system.after(text_input_focus_system),
                text_input_visual_update.after(text_input_keyboard_system),
                text_input_cursor_blink,
            ).chain());
    }
}

