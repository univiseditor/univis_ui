use bevy::{input::{ButtonState, keyboard::*}, prelude::*};
use crate::prelude::*;

pub struct UnivisTextFieldPlugin;

impl Plugin for UnivisTextFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UTextField>()
            .add_message::<TextFieldChangedEvent>()
            .add_message::<TextFieldSubmitEvent>()
            .add_systems(Update, (
                init_textfield_visuals,
                handle_global_unfocus,  // ✅ نظام جديد
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

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct UTextField {
    pub text: String,
    previous_text: String,
    pub placeholder: String,
    pub width: f32,
    pub height: f32,
    pub background_color: Color,
    pub background_focused_color: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub border_focused_color: Color,
    pub cursor_color: Color,
    pub font_size: f32,
    pub focused: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub cursor_position: usize,
    pub cursor_visible: bool,
    pub cursor_blink_timer: f32,
    pub cursor_blink_speed: f32,
    pub max_length: Option<usize>,
    pub input_type: TextFieldInputType,
    pub padding: f32,
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum TextFieldInputType {
    Text,
    Number,
    Email,
    Password,
}

impl Default for UTextField {
    fn default() -> Self {
        Self {
            text: String::new(),
            previous_text: String::new(),
            placeholder: "Enter text...".to_string(),
            width: 300.0,
            height: 50.0,
            background_color: Color::srgb(0.15, 0.15, 0.2),
            background_focused_color: Color::srgb(0.2, 0.2, 0.3),
            text_color: Color::WHITE,
            placeholder_color: Color::srgb(0.5, 0.5, 0.6),
            border_color: Color::srgb(0.3, 0.3, 0.35),
            border_focused_color: Color::srgb(0.3, 0.7, 1.0),
            cursor_color: Color::srgb(0.3, 0.7, 1.0),
            font_size: 18.0,
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
}

#[derive(Component)]
struct TextFieldTextLabel;

#[derive(Component)]
struct TextFieldCursor;

// =========================================================
// Systems
// =========================================================

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
            UInteraction::default(),
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
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ));
        
        commands.entity(entity).observe(on_textfield_click);
        
        commands.entity(entity).with_children(|parent| {
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
            
            if textfield.focused {
                parent.spawn((
                    UNode {
                        width: UVal::Px(2.0),
                        height: UVal::Px(textfield.font_size * 1.2),
                        background_color: textfield.cursor_color,
                        border_radius: UCornerRadius::all(1.0),
                        margin: USides::left(4.0),
                        ..default()
                    },
                    TextFieldCursor,
                ));
            }
        });
    }
}

// ✅ Observer للنقر
fn on_textfield_click(
    trigger: On<Pointer<Press>>,
    mut textfield_query: Query<(Entity, &mut UTextField)>,
) {
    // إلغاء التركيز من جميع الحقول
    for (entity, mut textfield) in textfield_query.iter_mut() {
        if entity == trigger.entity.entity() {
            if !textfield.disabled && !textfield.readonly {
                textfield.focused = true;
                textfield.cursor_visible = true;
                textfield.cursor_blink_timer = 0.0;
            }
        } else {
            textfield.focused = false;
        }
    }
}

// ✅ نظام جديد: إلغاء التركيز عند النقر خارج TextField
fn handle_global_unfocus(
    mut textfield_query: Query<&mut UTextField>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<&UInteraction>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // التحقق: هل تم النقر على أي TextField؟
    let clicked_on_textfield = interaction_query.iter().any(|interaction| {
        matches!(interaction, UInteraction::Pressed)
    });

    // إذا لم يتم النقر على أي TextField، ألغِ التركيز
    if !clicked_on_textfield {
        for mut textfield in textfield_query.iter_mut() {
            textfield.focused = false;
        }
    }
}

fn handle_textfield_input(
    mut textfield_query: Query<&mut UTextField>,
    mut keyboard_events: MessageReader<KeyboardInput>,
) {
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
            Key::Enter => {}
            
            Key::Backspace => {
                let textfi = textfield.cursor_position;
                if textfield.cursor_position > 0 && !textfield.text.is_empty() {
                    textfield.text.remove(textfi - 1);
                    textfield.cursor_position -= 1;
                }
            }
            
            Key::Delete => {
                let textfi = textfield.cursor_position;
                if textfield.cursor_position < textfield.text.len() {
                    textfield.text.remove(textfi);
                }
            }
            
            Key::ArrowLeft => {
                if textfield.cursor_position > 0 {
                    textfield.cursor_position -= 1;
                }
            }
            
            Key::ArrowRight => {
                if textfield.cursor_position < textfield.text.len() {
                    textfield.cursor_position += 1;
                }
            }
            
            Key::Home => {
                textfield.cursor_position = 0;
            }
            
            Key::End => {
                textfield.cursor_position = textfield.text.len();
            }
            
            Key::Character(char_str) => {
                if let Some(max) = textfield.max_length {
                    if textfield.text.len() >= max {
                        continue;
                    }
                }
                
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
                    let textfi = textfield.cursor_position;
                    textfield.text.insert_str(textfi, char_str);
                    textfield.cursor_position += char_str.len();
                }
            }
            
            _ => {}
        }
        
        textfield.cursor_visible = true;
        textfield.cursor_blink_timer = 0.0;
    }
}

fn update_textfield_visuals(
    textfield_query: Query<(Entity, &UTextField, &Children), Changed<UTextField>>,
    mut node_query: Query<&mut UNode>,
    mut border_query: Query<&mut UBorder>,
    mut text_query: Query<&mut UTextLabel, With<TextFieldTextLabel>>,
    cursor_query: Query<Entity, With<TextFieldCursor>>,
    mut commands: Commands,
) {
    for (entity, textfield, children) in textfield_query.iter() {
        
        // تحديث الـ Node الرئيسي
        if let Ok(mut node) = node_query.get_mut(entity) {
            node.background_color = if textfield.focused {
                textfield.background_focused_color
            } else {
                textfield.background_color
            };
        }
        
        // تحديث Border
        if let Ok(mut border) = border_query.get_mut(entity) {
            border.color = if textfield.focused {
                textfield.border_focused_color
            } else {
                textfield.border_color
            };
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
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    UNode {
                        width: UVal::Px(2.0),
                        height: UVal::Px(textfield.font_size * 1.2),
                        background_color: textfield.cursor_color,
                        border_radius: UCornerRadius::all(1.0),
                        margin: USides::left(4.0),
                        ..default()
                    },
                    TextFieldCursor,
                ));
            });
        } else if !textfield.focused && has_cursor {
            for child in children.iter() {
                if cursor_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

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

fn emit_textfield_events(
    mut changed_events: MessageWriter<TextFieldChangedEvent>,
    mut submit_events: MessageWriter<TextFieldSubmitEvent>,
    mut textfield_query: Query<(Entity, &mut UTextField)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (entity, mut textfield) in textfield_query.iter_mut() {
        if textfield.text != textfield.previous_text {
            changed_events.write(TextFieldChangedEvent {
                entity,
                text: textfield.text.clone(),
            });
            
            textfield.previous_text = textfield.text.clone();
        }
        
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