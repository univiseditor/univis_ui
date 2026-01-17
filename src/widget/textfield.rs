use bevy::prelude::*;
use bevy::picking::pointer::PointerPress;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use crate::prelude::*;

// =========================================================
// Plugin
// =========================================================

pub struct UnivisTextFieldPlugin;

impl Plugin for UnivisTextFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UTextField>()
            .add_message::<TextFieldChangedEvent>()
            .add_message::<TextFieldSubmitEvent>()
            .add_systems(Update, (
                init_textfield_visuals,
                handle_textfield_focus,
                handle_textfield_input,
                update_textfield_visuals,
                animate_textfield_cursor,
                emit_textfield_events,
            ).chain());
    }
}

// =========================================================
// Components
// =========================================================

/// مكون TextField الرئيسي
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct UTextField {
    /// النص الحالي
    pub text: String,
    
    /// النص السابق (للكشف عن التغييرات)
    previous_text: String,
    
    /// النص الظاهر عندما يكون فارغاً
    pub placeholder: String,
    
    // --- الأبعاد ---
    pub width: f32,
    pub height: f32,
    
    // --- الألوان ---
    pub background_color: Color,
    pub background_focused_color: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub border_focused_color: Color,
    pub cursor_color: Color,
    
    // --- الخط ---
    pub font_size: f32,
    
    // --- الحالة ---
    pub focused: bool,
    pub disabled: bool,
    pub readonly: bool,
    
    // --- Cursor ---
    pub cursor_position: usize, // موضع المؤشر في النص
    pub cursor_visible: bool,
    pub cursor_blink_timer: f32,
    pub cursor_blink_speed: f32,
    
    // --- Validation ---
    pub max_length: Option<usize>,
    pub input_type: TextFieldInputType,
    
    // --- البادينغ ---
    pub padding: f32,
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum TextFieldInputType {
    Text,      // أي نص
    Number,    // أرقام فقط
    Email,     // بريد إلكتروني
    Password,  // كلمة مرور (مخفية)
}

impl Default for UTextField {
    fn default() -> Self {
        Self {
            text: String::new(),
            previous_text: String::new(),
            placeholder: "Enter text...".to_string(),
            width: 200.0,
            height: 40.0,
            background_color: Color::srgb(0.15, 0.15, 0.2),
            background_focused_color: Color::srgb(0.18, 0.18, 0.25),
            text_color: Color::srgb(0.9, 0.9, 0.95),
            placeholder_color: Color::srgb(0.5, 0.5, 0.6),
            border_color: Color::srgb(0.3, 0.3, 0.35),
            border_focused_color: Color::srgb(0.2, 0.6, 1.0),
            cursor_color: Color::srgb(0.2, 0.6, 1.0),
            font_size: 16.0,
            focused: false,
            disabled: false,
            readonly: false,
            cursor_position: 0,
            cursor_visible: true,
            cursor_blink_timer: 0.0,
            cursor_blink_speed: 0.5,
            max_length: None,
            input_type: TextFieldInputType::Text,
            padding: 12.0,
        }
    }
}

impl UTextField {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.cursor_position = text.len();
        self.text = text.clone();
        self.previous_text = text;
        self
    }
    
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
    
    pub fn input_type(mut self, input_type: TextFieldInputType) -> Self {
        self.input_type = input_type;
        self
    }
    
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    
    // === أنماط جاهزة ===
    
    pub fn search_style() -> Self {
        Self {
            placeholder: "Search...".to_string(),
            width: 250.0,
            height: 38.0,
            border_color: Color::srgba(0.4, 0.4, 0.45, 0.5),
            border_focused_color: Color::srgb(0.5, 0.5, 0.6),
            ..default()
        }
    }
    
    pub fn email_style() -> Self {
        Self {
            placeholder: "name@example.com".to_string(),
            input_type: TextFieldInputType::Email,
            width: 280.0,
            ..default()
        }
    }
    
    pub fn password_style() -> Self {
        Self {
            placeholder: "Enter password...".to_string(),
            input_type: TextFieldInputType::Password,
            width: 280.0,
            ..default()
        }
    }
    
    pub fn number_style() -> Self {
        Self {
            placeholder: "0".to_string(),
            input_type: TextFieldInputType::Number,
            width: 120.0,
            ..default()
        }
    }
}

// /// علامات داخلية
// #[derive(Component)]
// struct TextFieldContainer;

#[derive(Component)]
struct TextFieldTextLabel;

#[derive(Component)]
struct TextFieldCursor;

// #[derive(Component)]
// struct TextFieldParent(Entity);

// =========================================================
// Systems
// =========================================================

/// إنشاء الهيكل البصري
fn init_textfield_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UTextField), Added<UTextField>>,
) {
    for (entity, textfield) in query.iter() {
        
        let border_color = if textfield.focused {
            textfield.border_focused_color
        } else {
            textfield.border_color
        };
        
        let bg_color = if textfield.focused {
            textfield.background_focused_color
        } else {
            textfield.background_color
        };
        
        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(textfield.width),
                height: UVal::Px(textfield.height),
                background_color: bg_color,
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(textfield.padding),
                ..default()
            },
            UBorder {
                color: border_color,
                width: 2.0,
                radius: UCornerRadius::all(8.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                ..default()
            },
        ));
        
        // إضافة Observer
        commands.entity(entity).observe(on_textfield_click);
        
        commands.entity(entity).with_children(|parent| {
            
            // النص أو Placeholder
            let display_text = if textfield.text.is_empty() {
                &textfield.placeholder
            } else if textfield.input_type == TextFieldInputType::Password {
                &"•".repeat(textfield.text.len())
            } else {
                &textfield.text
            };
            
            let text_color = if textfield.text.is_empty() {
                textfield.placeholder_color
            } else {
                textfield.text_color
            };
            
            parent.spawn((
                UTextLabel {
                    text: display_text.to_string(),
                    font_size: textfield.font_size,
                    color: text_color,
                    autosize: false,
                    ..default()
                },
                TextFieldTextLabel,
            ));
            
            // Cursor (المؤشر الوامض)
            if textfield.focused {
                parent.spawn((
                    UNode {
                        width: UVal::Px(2.0),
                        height: UVal::Px(textfield.font_size * 1.2),
                        background_color: textfield.cursor_color,
                        border_radius: UCornerRadius::all(1.0),
                        margin: USides::left(2.0),
                        ..default()
                    },
                    TextFieldCursor,
                ));
            }
        });
    }
}

/// Observer للنقر على TextField
fn on_textfield_click(
    trigger: On<Pointer<PointerPress>>,
    mut textfield_query: Query<&mut UTextField>,
) {
    let entity = trigger.entity.entity();
    let Ok(mut textfield) = textfield_query.get_mut(entity) else { return };
    
    if !textfield.disabled && !textfield.readonly {
        textfield.focused = true;
        textfield.cursor_visible = true;
        textfield.cursor_blink_timer = 0.0;
    }
}

/// معالجة Focus/Blur
fn handle_textfield_focus(
    mut textfield_query: Query<(Entity, &mut UTextField)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    global_transforms: Query<&GlobalTransform>,
    computed_sizes: Query<&ComputedSize>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    
    for (entity, mut textfield) in textfield_query.iter_mut() {
        
        // التحقق من النقر خارج TextField
        if let (Ok(transform), Ok(size)) = (
            global_transforms.get(entity),
            computed_sizes.get(entity)
        ) {
            let pos = transform.translation().truncate();
            let half_size = Vec2::new(size.width, size.height) / 2.0;
            
            let min = pos - half_size;
            let max = pos + half_size;
            
            let is_inside = cursor_pos.x >= min.x && cursor_pos.x <= max.x
                && cursor_pos.y >= min.y && cursor_pos.y <= max.y;
            
            if !is_inside && textfield.focused {
                textfield.focused = false;
            }
        }
    }
}

/// معالجة إدخال لوحة المفاتيح
fn handle_textfield_input(
    mut textfield_query: Query<&mut UTextField>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    _keyboard: Res<ButtonInput<KeyCode>>,
) {
    // البحث عن TextField المركز عليه
    let mut focused_textfield: Option<Mut<UTextField>> = None;
    for textfield in textfield_query.iter_mut() {
        if textfield.focused && !textfield.disabled && !textfield.readonly {
            focused_textfield = Some(textfield);
            break;
        }
    }
    
    let Some(mut textfield) = focused_textfield else { return };
    
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        
        match &event.logical_key {
            // Enter - Submit
            Key::Enter => {
                // سيتم إطلاق Event في نظام آخر
            }
            
            // Backspace - حذف
            Key::Backspace => {
                if textfield.cursor_position > 0 && !textfield.text.is_empty() {
                    let num = textfield.cursor_position;
                    textfield.text.remove(num - 1);
                    textfield.cursor_position -= 1;
                }
            }
            
            // Delete
            Key::Delete => {
                if textfield.cursor_position < textfield.text.len() {
                    let num = textfield.cursor_position;
                    textfield.text.remove(num);
                }
            }
            
            // Arrow Left
            Key::ArrowLeft => {
                if textfield.cursor_position > 0 {
                    textfield.cursor_position -= 1;
                }
            }
            
            // Arrow Right
            Key::ArrowRight => {
                if textfield.cursor_position < textfield.text.len() {
                    textfield.cursor_position += 1;
                }
            }
            
            // Home
            Key::Home => {
                textfield.cursor_position = 0;
            }
            
            // End
            Key::End => {
                textfield.cursor_position = textfield.text.len();
            }
            
            // Character input
            Key::Character(char_str) => {
                // التحقق من Max Length
                if let Some(max) = textfield.max_length {
                    if textfield.text.len() >= max {
                        continue;
                    }
                }
                
                // Validation حسب النوع
                let valid = match textfield.input_type {
                    TextFieldInputType::Number => {
                        char_str.chars().all(|c| c.is_numeric() || c == '.' || c == '-')
                    }
                    TextFieldInputType::Email => {
                        char_str.chars().all(|c| {
                            c.is_alphanumeric() || c == '@' || c == '.' || c == '_' || c == '-'
                        })
                    }
                    _ => true,
                };
                
                if valid {
                    // إدراج النص عند موضع Cursor
                    let num = textfield.cursor_position;
                    textfield.text.insert_str(num, char_str);
                    textfield.cursor_position += char_str.len();
                }
            }
            
            _ => {}
        }
        
        // إعادة تعيين Cursor blink
        textfield.cursor_visible = true;
        textfield.cursor_blink_timer = 0.0;
    }
}

/// تحديث المظهر
fn update_textfield_visuals(
    textfield_query: Query<(&UTextField, &Children), Changed<UTextField>>,
    mut node_query: Query<&mut UNode>,
    mut _border_query: Query<&mut UBorder>,
    mut text_query: Query<&mut UTextLabel, With<TextFieldTextLabel>>,
    cursor_query: Query<Entity, With<TextFieldCursor>>,
    mut commands: Commands,
) {
    for (textfield, children) in textfield_query.iter() {
        
        // تحديث الخلفية والحدود
        for child in children.iter() {
            // تحديث Node
            if let Ok(mut node) = node_query.get_mut(child) {
                node.background_color = if textfield.focused {
                    textfield.background_focused_color
                } else {
                    textfield.background_color
                };
            }
        }
        
        // تحديث Border من الأب مباشرة
        if let Ok(_parent_children) = textfield_query.single() {
            // ... (سيتم معالجتها في تحديث لاحق)
        }
        
        // تحديث النص
        for child in children.iter() {
            if let Ok(mut text_label) = text_query.get_mut(child) {
                let display_text = if textfield.text.is_empty() {
                    &textfield.placeholder
                } else if textfield.input_type == TextFieldInputType::Password {
                    &"•".repeat(textfield.text.len())
                } else {
                    &textfield.text
                };
                
                text_label.text = display_text.to_string();
                text_label.color = if textfield.text.is_empty() {
                    textfield.placeholder_color
                } else {
                    textfield.text_color
                };
            }
        }
        
        // إدارة Cursor
        let has_cursor = children.iter().any(|c| cursor_query.get(c).is_ok());
        
        if textfield.focused && !has_cursor {
            // إضافة Cursor
            commands.entity(*children.first().unwrap()).with_children(|parent| {
                parent.spawn((
                    UNode {
                        width: UVal::Px(2.0),
                        height: UVal::Px(textfield.font_size * 1.2),
                        background_color: textfield.cursor_color,
                        border_radius: UCornerRadius::all(1.0),
                        margin: USides::left(2.0),
                        ..default()
                    },
                    TextFieldCursor,
                ));
            });
        } else if !textfield.focused && has_cursor {
            // حذف Cursor
            for child in children.iter() {
                if cursor_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

/// تحريك Cursor (الوميض)
fn animate_textfield_cursor(
    time: Res<Time>,
    mut textfield_query: Query<(&mut UTextField, &Children)>,
    mut cursor_query: Query<&mut Visibility, With<TextFieldCursor>>,
) {
    for (mut textfield, children) in textfield_query.iter_mut() {
        if !textfield.focused {
            continue;
        }
        
        textfield.cursor_blink_timer += time.delta_secs();
        
        if textfield.cursor_blink_timer >= textfield.cursor_blink_speed {
            textfield.cursor_blink_timer = 0.0;
            textfield.cursor_visible = !textfield.cursor_visible;
            
            // تطبيق على Cursor
            for child in children.iter() {
                if let Ok(mut visibility) = cursor_query.get_mut(child) {
                    *visibility = if textfield.cursor_visible {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
            }
        }
    }
}

/// إطلاق الأحداث
fn emit_textfield_events(
    mut changed_events: MessageWriter<TextFieldChangedEvent>,
    mut submit_events: MessageWriter<TextFieldSubmitEvent>,
    mut textfield_query: Query<(Entity, &mut UTextField)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (entity, mut textfield) in textfield_query.iter_mut() {
        
        // Changed Event
        if textfield.text != textfield.previous_text {
            changed_events.write(TextFieldChangedEvent {
                entity,
                text: textfield.text.clone(),
            });
            
            textfield.previous_text = textfield.text.clone();
        }
        
        // Submit Event (عند الضغط على Enter)
        if textfield.focused && keyboard.just_pressed(KeyCode::Enter) {
            submit_events.write(TextFieldSubmitEvent {
                entity,
                text: textfield.text.clone(),
            });
        }
    }
}

// =========================================================
// Events
// =========================================================

#[derive(Message)]
pub struct TextFieldChangedEvent {
    pub entity: Entity,
    pub text: String,
}

#[derive(Message)]
pub struct TextFieldSubmitEvent {
    pub entity: Entity,
    pub text: String,
}