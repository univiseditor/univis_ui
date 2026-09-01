use bevy::picking::Pickable;
use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UDisplay, UFlexDirection, ULayout, UNode, UPositionType, USelf,
};
use univis_ui_interaction::interaction::feedback::UInteraction;

// =========================================================
// Plugin
// =========================================================

/// Registers the toggle / switch widget.
pub struct UnivisTogglePlugin;

impl Plugin for UnivisTogglePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UToggle>()
            .add_message::<ToggleChangedEvent>()
            .add_systems(
                Update,
                (init_toggle_visuals, animate_toggle_knob, sync_toggle_colors).chain(),
            )
            .add_observer(update_toggle_state);
    }
}

// =========================================================
// Components
// =========================================================

/// A binary on/off switch widget.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct UToggle {
    /// Current on/off state.
    pub checked: bool,

    /// Previous state used internally to emit change events.
    previous_checked: bool,

    // --- dimensions ---
    pub width: f32,
    pub height: f32,

    // --- colors ---
    pub track_color_off: Color,
    pub track_color_on: Color,
    pub knob_color: Color,

    // --- animation ---
    pub animation_speed: f32,
    /// Current knob interpolation, where `0.0` is off and `1.0` is on.
    pub current_offset: f32,

    // --- options ---
    pub disabled: bool,
}

impl Default for UToggle {
    fn default() -> Self {
        Self {
            checked: false,
            previous_checked: false,
            width: 60.0,
            height: 30.0,
            track_color_off: Color::srgb(0.3, 0.3, 0.3),
            track_color_on: Color::srgb(0.2, 0.6, 1.0),
            knob_color: Color::WHITE,
            animation_speed: 10.0,
            current_offset: 0.0,
            disabled: false,
        }
    }
}

impl UToggle {
    /// Creates a toggle with default styling.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the initial checked state.
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.previous_checked = checked;
        self.current_offset = if checked { 1.0 } else { 0.0 };
        self
    }

    /// Overrides the track and knob colors.
    pub fn with_colors(mut self, off: Color, on: Color, knob: Color) -> Self {
        self.track_color_off = off;
        self.track_color_on = on;
        self.knob_color = knob;
        self
    }

    /// Overrides the widget size.
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Marks the toggle as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Returns an iOS-inspired preset.
    pub fn ios_style() -> Self {
        Self {
            width: 51.0,
            height: 31.0,
            track_color_off: Color::srgb(0.78, 0.78, 0.78),
            track_color_on: Color::srgb(0.2, 0.78, 0.35),
            knob_color: Color::WHITE,
            animation_speed: 12.0,
            ..default()
        }
    }

    /// Returns a Material-inspired preset.
    pub fn material_style() -> Self {
        Self {
            width: 52.0,
            height: 32.0,
            track_color_off: Color::srgb(0.6, 0.6, 0.6),
            track_color_on: Color::srgb(0.38, 0.65, 0.87),
            knob_color: Color::WHITE,
            animation_speed: 15.0,
            ..default()
        }
    }

    /// Returns a sci-fi preset with a wider track.
    pub fn sci_fi_style() -> Self {
        Self {
            width: 70.0,
            height: 35.0,
            track_color_off: Color::srgb(0.1, 0.1, 0.2),
            track_color_on: Color::srgb(0.0, 0.8, 1.0),
            knob_color: Color::srgb(0.9, 1.0, 1.0),
            animation_speed: 8.0,
            ..default()
        }
    }
}

/// Internal markers for the visual parts.
#[derive(Component)]
struct ToggleTrack;

#[derive(Component)]
struct ToggleKnob;

// =========================================================
// Systems
// =========================================================

/// Builds the toggle's visual hierarchy.
fn init_toggle_visuals(mut commands: Commands, query: Query<(Entity, &UToggle), Added<UToggle>>) {
    for (entity, toggle) in query.iter() {
        let track_color = if toggle.checked {
            toggle.track_color_on
        } else {
            toggle.track_color_off
        };

        commands
            .entity(entity)
            .insert((
                UNode {
                    width: UVal::Px(toggle.width),
                    height: UVal::Px(toggle.height),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    ..default()
                },
                UInteraction::default(),
            ))
            .with_children(|parent| {
                // Track background
                parent
                    .spawn((
                        UNode {
                            width: UVal::Px(toggle.width),
                            height: UVal::Px(toggle.height),
                            background_color: track_color,
                            border_radius: UCornerRadius::all(toggle.height / 2.0),
                            padding: USides::all(2.0),
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            ..default()
                        },
                        ToggleTrack,
                    ))
                    .with_children(|track_parent| {
                        // Sliding knob
                        let knob_size = toggle.height - 4.0;
                        let initial_x = if toggle.checked {
                            toggle.width - knob_size - 4.0
                        } else {
                            2.0
                        };

                        track_parent.spawn((
                            UNode {
                                width: UVal::Px(knob_size),
                                height: UVal::Px(knob_size),
                                background_color: toggle.knob_color,
                                border_radius: UCornerRadius::all(knob_size / 2.0),
                                ..default()
                            },
                            USelf {
                                position_type: UPositionType::Absolute,
                                left: UVal::Px(initial_x),
                                top: UVal::Px(2.0),
                                ..default()
                            },
                            ToggleKnob,
                        ));
                    });
            });
    }
}

/// Flips the toggle state on click.
fn update_toggle_state(
    events: On<Pointer<Click>>,
    mut toggle_query: Query<(&mut UToggle, &UInteraction)>,
) {
    if let Ok((mut toggle, _inter)) = toggle_query.get_mut(events.entity.entity()) {
        if toggle.disabled {
            return;
        }
        // Toggle immediately on click.
        toggle.previous_checked = toggle.checked;
        toggle.checked = !toggle.checked;
    }
}

/// Animates the knob toward the target state.
fn animate_toggle_knob(
    time: Res<Time>,
    mut toggle_query: Query<(&mut UToggle, &Children)>,
    track_query: Query<&Children, With<ToggleTrack>>,
    mut knob_query: Query<&mut USelf, With<ToggleKnob>>,
) {
    for (mut toggle, children) in toggle_query.iter_mut() {
        // Final target position
        let target_offset = if toggle.checked { 1.0 } else { 0.0 };

        // Remaining distance
        let diff = target_offset - toggle.current_offset;

        // Snap once the remaining distance is tiny.
        // Only write when the value actually changes — an unconditional
        // assignment marks the UToggle component changed every frame, which
        // re-triggers layout/render invalidation each frame (visible as
        // whole-panel flicker with the incremental renderer).
        if diff.abs() < 0.01 {
            if toggle.current_offset != target_offset {
                toggle.current_offset = target_offset;
            }
            continue;
        }

        // Smooth interpolation
        let delta = time.delta_secs() * toggle.animation_speed;
        let next = toggle.current_offset + diff * delta;
        if next != toggle.current_offset {
            toggle.current_offset = next;
        }

        // Apply the resolved offset to the knob.
        let track_entity = children
            .iter()
            .find(|&child| track_query.get(child).is_ok());

        if let Some(track) = track_entity {
            if let Ok(track_children) = track_query.get(track) {
                for knob_entity in track_children.iter() {
                    if let Ok(mut uself) = knob_query.get_mut(knob_entity) {
                        let knob_size = toggle.height - 4.0;
                        let max_offset = toggle.width - knob_size - 4.0;
                        let new_x = 2.0 + (toggle.current_offset * (max_offset - 2.0));

                        uself.left = UVal::Px(new_x);
                    }
                }
            }
        }
    }
}

/// Syncs the track color with the current state.
fn sync_toggle_colors(
    toggle_query: Query<(&UToggle, &Children), Changed<UToggle>>,
    mut track_query: Query<&mut UNode, With<ToggleTrack>>,
) {
    for (toggle, children) in toggle_query.iter() {
        let target_color = if toggle.checked {
            toggle.track_color_on
        } else {
            toggle.track_color_off
        };

        // Update the track color directly for now.
        for child in children.iter() {
            if let Ok(mut node) = track_query.get_mut(child) {
                // A lerped color transition could be added later if needed.
                node.background_color = target_color;
            }
        }
    }
}

// =========================================================
// Event emitted for external observers
// =========================================================

/// Message emitted when a toggle changes state.
#[derive(Message)]
pub struct ToggleChangedEvent {
    pub entity: Entity,
    pub checked: bool,
}

/// Internal system that emits [`ToggleChangedEvent`].
#[doc(hidden)]
pub fn emit_toggle_events(
    mut events: MessageWriter<ToggleChangedEvent>,
    mut query: Query<(Entity, &mut UToggle), Changed<UToggle>>,
) {
    for (entity, mut toggle) in query.iter_mut() {
        if toggle.checked != toggle.previous_checked {
            events.write(ToggleChangedEvent {
                entity,
                checked: toggle.checked,
            });
            // Sync the previous state so externally-mutated `checked` (game
            // code driving widgets programmatically) does not re-emit the
            // event every frame — matches select/seekbar emit behavior.
            toggle.previous_checked = toggle.checked;
        }
    }
}
