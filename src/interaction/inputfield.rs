use bevy::prelude::*;
use crate::prelude::*;
// =========================================================
// 1. Events & Resources (Logic Layer - Kept mostly the same)
// =========================================================

#[derive(Message)]
pub struct UTextFieldChangeEvent {
    pub entity: Entity,
    pub value: String,
}

#[derive(Message)]
pub struct UTextFieldSubmitEvent {
    pub entity: Entity,
    pub value: String,
}

#[derive(Component)]
pub struct UInputField {
    pub disabled: bool,
    pub focused: bool,
}

#[derive(Resource, Default)]
pub struct ActiveTextField(pub Option<Entity>);

#[derive(Resource)]
pub struct TextFieldCaretBlink {
    pub timer: Timer,
    pub visible: bool,
}

impl Default for TextFieldCaretBlink {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            visible: true,
        }
    }
}

// =========================================================
// 2. Components (State Layer)
// =========================================================

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub enum UInputType {
    #[default]
    Text,
    Password,
    Number,
}


// =========================================================
// 3. Systems (Logic & Input)
// =========================================================

/// Handles mouse interaction to set focus
pub fn text_field_focus_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveTextField>,
    mut query: Query<(Entity, &UInteraction, &mut UInputField)>,
) {
    let mut clicked_any = false;

    for (entity, interaction, mut field) in query.iter_mut() {
        if field.disabled { continue; }
        

        if *interaction == UInteraction::Clicked {
            active.0 = Some(entity);
            field.focused = true;
            clicked_any = true;
        } else if active.0 == Some(entity) {
            // Keep it focused
            field.focused = true;
        } else {
            field.focused = false;
        }
    }

    // Click outside to blur
    if mouse.just_pressed(MouseButton::Left) && !clicked_any {
        active.0 = None;
        for (_, _, mut field) in query.iter_mut() {
            field.focused = false;
        }
    }
}

/// Handles Keyboard Input (Logic Copied & Adapted)
pub fn text_field_input_system(
    mut active: ResMut<ActiveTextField>,
    mut keyboard_inputs: MessageReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut fields: Query<(Entity, &mut UTextInput)>,
    mut change_events: MessageWriter<UTextFieldChangeEvent>,
    mut submit_events: MessageWriter<UTextFieldSubmitEvent>,
) {
    let Some(active_entity) = active.0 else { return };
    let Ok((entity, mut field)) = fields.get_mut(active_entity) else { return };
    
    if field.disabled { return; }

    let mut changed = false;

    // 1. Handle Backspace
    if keys.just_pressed(KeyCode::Backspace) && !field.value.is_empty() {
        field.value.pop();
        changed = true;
    }

    // 2. Handle Enter
    if keys.just_pressed(KeyCode::Enter) {
        submit_events.write(UTextFieldSubmitEvent {
            entity,
            value: field.value.clone(),
        });
        // Optional: Blur on enter
        active.0 = None;
        field.focused = false;
    }

    // 3. Handle Character Input
    for ev in keyboard_inputs.read() {
        if ev.state != bevy::input::ButtonState::Pressed { continue; }
        
        // Extract text logic
        let text = ev.text.as_deref().or_else(|| match &ev.logical_key {
             bevy::input::keyboard::Key::Character(s) => Some(s.as_str()),
             _ => None
        });

        if let Some(text) = text {
            for ch in text.chars() {
                if ch.is_control() { continue; }
                
                // Max length check
                if let Some(max) = field.max_length {
                    if field.value.chars().count() >= max { continue; }
                }

                // Number filter
                if field.input_type == UInputType::Number && !ch.is_numeric() {
                    continue;
                }

                field.value.push(ch);
                changed = true;
            }
        }
    }

    if changed {
        change_events.write(UTextFieldChangeEvent {
            entity,
            value: field.value.clone(),
        });
    }
}

pub struct UnivisInputFieldPlugin;

impl Plugin for UnivisInputFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UTextFieldChangeEvent>()
           .add_message::<UTextFieldSubmitEvent>()
           .init_resource::<ActiveTextField>()
           .add_systems(Update, (
               text_field_focus_system,
               text_field_input_system));
    }
}