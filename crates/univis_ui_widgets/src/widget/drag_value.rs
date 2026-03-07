use bevy::prelude::*;
use crate::internal_prelude::*;

pub struct UnivisDragValuePlugin;

impl Plugin for UnivisDragValuePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UDragValue>()
            .add_message::<DragValueChangedEvent>()
            .add_message::<DragValueCommitEvent>()
            .add_systems(Update, (
                init_drag_value_visuals,
                handle_drag_value_interaction,
                update_drag_value_visuals,
                emit_drag_value_events,
            ).chain());
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable, UInteraction)]
pub struct UDragValue {
    pub value: f32,
    previous_value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub step: Option<f32>,
    pub pixels_per_full_range: f32,
    pub is_dragging: bool,
    pub drag_start_cursor_x: f32,
    pub drag_start_value: f32,
    pub disabled: bool,
    pub decimals: usize,
    pub text_color: Color,
    pub background: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub padding: USides,
    just_committed: bool,
}

impl Default for UDragValue {
    fn default() -> Self {
        Self {
            value: 0.0,
            previous_value: 0.0,
            min_value: 0.0,
            max_value: 1.0,
            step: None,
            pixels_per_full_range: 200.0,
            is_dragging: false,
            drag_start_cursor_x: 0.0,
            drag_start_value: 0.0,
            disabled: false,
            decimals: 2,
            text_color: Color::WHITE,
            background: Color::srgb(0.16, 0.18, 0.24),
            hover_color: Color::srgb(0.2, 0.22, 0.3),
            pressed_color: Color::srgb(0.14, 0.16, 0.22),
            padding: USides::axes(12.0, 8.0),
            just_committed: false,
        }
    }
}

impl UDragValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = clamp_value(value, self.min_value, self.max_value);
        if let Some(step) = self.step {
            self.value = snap_to_step(self.value, self.min_value, self.max_value, step);
        }
        self.previous_value = self.value;
        self
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self.value = clamp_value(self.value, self.min_value, self.max_value);
        if let Some(step) = self.step {
            self.value = snap_to_step(self.value, self.min_value, self.max_value, step);
        }
        self.previous_value = self.value;
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = if step > 0.0 { Some(step) } else { None };
        if let Some(step) = self.step {
            self.value = snap_to_step(self.value, self.min_value, self.max_value, step);
            self.previous_value = self.value;
        }
        self
    }

    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn with_sensitivity_px(mut self, pixels: f32) -> Self {
        self.pixels_per_full_range = pixels.max(1.0);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Component)]
struct DragValueLabel;

fn init_drag_value_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UDragValue, &UNode), Added<UDragValue>>,
) {
    for (entity, drag, existing_node) in query.iter() {
        let radius = if existing_node.border_radius == UCornerRadius::default() {
            UCornerRadius::all(8.0)
        } else {
            existing_node.border_radius
        };

        commands.entity(entity).insert((
            UNode {
                width: existing_node.width,
                height: existing_node.height,
                padding: drag.padding,
                margin: existing_node.margin,
                background_color: drag.background,
                border_radius: radius,
                shape_mode: existing_node.shape_mode,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UInteractionColors {
                normal: drag.background,
                hovered: drag.hover_color,
                pressed: drag.pressed_color,
            },
        ));

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                UTextLabel {
                    text: format_drag_value(drag.value, drag.decimals),
                    font_size: 16.0,
                    color: drag.text_color,
                    autosize: true,
                    ..default()
                },
                DragValueLabel,
                Pickable::IGNORE,
            ));
        });
    }
}

fn handle_drag_value_interaction(
    mut query: Query<(&UInteraction, &mut UDragValue), With<UDragValue>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
) {
    let window = if let Ok(window) = windows.single() {
        window
    } else {
        return;
    };

    let cursor_x = window.cursor_position().map(|v| v.x);
    let fine_mode = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    for (interaction, mut drag) in query.iter_mut() {
        if drag.disabled {
            if drag.is_dragging && !mouse_button.pressed(MouseButton::Left) {
                drag.is_dragging = false;
                drag.just_committed = false;
            }
            continue;
        }

        if *interaction == UInteraction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            if let Some(x) = cursor_x {
                drag.drag_start_cursor_x = x;
                drag.drag_start_value = drag.value;
                drag.is_dragging = true;
            }
        }

        if drag.is_dragging {
            if mouse_button.pressed(MouseButton::Left) {
                if let Some(x) = cursor_x {
                    let delta_px = x - drag.drag_start_cursor_x;
                    let mut delta_value = pixels_to_value_delta(
                        delta_px,
                        drag.min_value,
                        drag.max_value,
                        drag.pixels_per_full_range,
                    );

                    if fine_mode {
                        delta_value *= 0.2;
                    }

                    let mut new_value = clamp_value(
                        drag.drag_start_value + delta_value,
                        drag.min_value,
                        drag.max_value,
                    );

                    if let Some(step) = drag.step {
                        new_value = snap_to_step(new_value, drag.min_value, drag.max_value, step);
                    }

                    drag.value = new_value;
                }
            } else {
                drag.is_dragging = false;
                drag.just_committed = true;
            }
        }
    }
}

fn update_drag_value_visuals(
    mut query: Query<
        (
            &UDragValue,
            &UInteraction,
            &Children,
            &mut UNode,
            &mut UInteractionColors,
        ),
        Changed<UDragValue>,
    >,
    mut label_query: Query<&mut UTextLabel, With<DragValueLabel>>,
) {
    for (drag, interaction, children, mut node, mut colors) in query.iter_mut() {
        node.padding = drag.padding;

        colors.normal = drag.background;
        colors.hovered = drag.hover_color;
        colors.pressed = drag.pressed_color;

        node.background_color = match *interaction {
            UInteraction::Pressed | UInteraction::Clicked => drag.pressed_color,
            UInteraction::Hovered | UInteraction::Released => drag.hover_color,
            UInteraction::Normal => drag.background,
        };

        for &child in children {
            if let Ok(mut label) = label_query.get_mut(child) {
                label.text = format_drag_value(drag.value, drag.decimals);
                label.color = drag.text_color;
            }
        }
    }
}

fn emit_drag_value_events(
    mut changed_events: MessageWriter<DragValueChangedEvent>,
    mut commit_events: MessageWriter<DragValueCommitEvent>,
    mut query: Query<(Entity, &mut UDragValue)>,
) {
    for (entity, mut drag) in query.iter_mut() {
        let normalized = normalize_value(drag.value, drag.min_value, drag.max_value);

        if (drag.value - drag.previous_value).abs() > 0.0001 {
            changed_events.write(DragValueChangedEvent {
                entity,
                value: drag.value,
                normalized,
            });
            drag.previous_value = drag.value;
        }

        if drag.just_committed {
            commit_events.write(DragValueCommitEvent {
                entity,
                value: drag.value,
                normalized,
            });
            drag.just_committed = false;
        }
    }
}

fn sorted_range(min: f32, max: f32) -> (f32, f32) {
    if min <= max { (min, max) } else { (max, min) }
}

fn clamp_value(value: f32, min: f32, max: f32) -> f32 {
    let (lo, hi) = sorted_range(min, max);
    value.clamp(lo, hi)
}

fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
    let (lo, hi) = sorted_range(min, max);
    let span = hi - lo;
    if span.abs() <= f32::EPSILON {
        0.0
    } else {
        ((value - lo) / span).clamp(0.0, 1.0)
    }
}

fn snap_to_step(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let step = step.abs();
    if step <= f32::EPSILON {
        return clamp_value(value, min, max);
    }

    let (lo, hi) = sorted_range(min, max);
    let snapped = ((value - lo) / step).round() * step + lo;
    snapped.clamp(lo, hi)
}

fn pixels_to_value_delta(delta_px: f32, min: f32, max: f32, pixels_per_full_range: f32) -> f32 {
    let (lo, hi) = sorted_range(min, max);
    let range = hi - lo;
    let sensitivity = pixels_per_full_range.max(1.0);
    (delta_px / sensitivity) * range
}

fn format_drag_value(value: f32, decimals: usize) -> String {
    let decimals = decimals.min(10);
    format!("{:.*}", decimals, value)
}

#[derive(Message)]
pub struct DragValueChangedEvent {
    pub entity: Entity,
    pub value: f32,
    pub normalized: f32,
}

#[derive(Message)]
pub struct DragValueCommitEvent {
    pub entity: Entity,
    pub value: f32,
    pub normalized: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_value_respects_bounds() {
        assert_eq!(clamp_value(5.0, 0.0, 3.0), 3.0);
        assert_eq!(clamp_value(-1.0, 0.0, 3.0), 0.0);
        assert_eq!(clamp_value(2.0, 0.0, 3.0), 2.0);
    }

    #[test]
    fn snap_to_step_works() {
        let v = snap_to_step(2.36, 0.0, 10.0, 0.25);
        assert!((v - 2.25).abs() < 0.0001);
    }

    #[test]
    fn normalize_value_works() {
        let n = normalize_value(25.0, 0.0, 100.0);
        assert!((n - 0.25).abs() < 0.0001);
    }

    #[test]
    fn sensitivity_mapping_200px_full_range() {
        let delta = pixels_to_value_delta(200.0, 10.0, 110.0, 200.0);
        assert!((delta - 100.0).abs() < 0.0001);
    }
}
