use crate::widget::text_label::UTextLabel;
use bevy::picking::Pickable;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UDisplay, UFlexDirection, UJustifyContent, ULayout, UNode, UPositionType, USelf,
};
use univis_ui_interaction::interaction::feedback::UInteraction;

// =========================================================
// Plugin
// =========================================================

/// Registers the seek bar / slider widget.
pub struct UnivisSeekBarPlugin;

impl Plugin for UnivisSeekBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<USeekBar>()
            .add_message::<SeekBarChangedEvent>()
            .add_systems(
                Update,
                (
                    init_seekbar_visuals,
                    update_seekbar_visuals,
                    // animate_seekbar_thumb,
                    emit_seekbar_events,
                )
                    .chain(),
            )
            .add_observer(start_seekbar_interaction)
            .add_observer(drag_seekbar_interaction)
            .add_observer(release_seekbar_interaction)
            .add_observer(end_seekbar_drag);
    }
}

// =========================================================
// Components
// =========================================================

/// A value slider widget with optional range mapping and step snapping.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct USeekBar {
    /// Current normalized value in the `0.0..=1.0` range.
    pub value: f32,

    /// Previous value used internally for change detection.
    previous_value: f32,
    drag_start_value: f32, // Normalized value captured when dragging starts.

    // --- dimensions ---
    pub width: f32,
    pub track_height: f32,
    pub thumb_size: f32,

    // --- colors ---
    pub track_color: Color,
    pub fill_color: Color,
    pub thumb_color: Color,
    pub thumb_hover_color: Color,

    // --- state ---
    pub disabled: bool,
    pub is_dragging: bool,
    pub show_value: bool,

    // --- animation ---
    // pub smooth_animation: bool,
    // pub animation_speed: f32,
    // pub target_value: f32,

    // --- mapped value range ---
    pub min_value: f32,
    pub max_value: f32,
    pub step: Option<f32>, // Optional snapping step in real-value space.
}

impl Default for USeekBar {
    fn default() -> Self {
        Self {
            drag_start_value: 0.0,
            value: 0.0,
            previous_value: 0.0,
            width: 200.0,
            track_height: 6.0,
            thumb_size: 18.0,
            track_color: Color::srgb(0.3, 0.3, 0.35),
            fill_color: Color::srgb(0.2, 0.6, 1.0),
            thumb_color: Color::WHITE,
            thumb_hover_color: Color::srgb(0.9, 0.95, 1.0),
            disabled: false,
            is_dragging: false,
            show_value: false,
            // smooth_animation: true,
            // animation_speed: 15.0,
            // target_value: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            step: None,
        }
    }
}

impl USeekBar {
    /// Creates a seek bar with default styling.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the initial normalized value.
    pub fn with_value(mut self, value: f32) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        self.value = clamped;
        self.previous_value = clamped;
        // self.target_value = clamped; ,
        self
    }

    /// Overrides the overall width, track height, and thumb size.
    pub fn with_size(mut self, width: f32, track_height: f32, thumb_size: f32) -> Self {
        self.width = width;
        self.track_height = track_height;
        self.thumb_size = thumb_size;
        self
    }

    /// Overrides the track, fill, and thumb colors.
    pub fn with_colors(mut self, track: Color, fill: Color, thumb: Color) -> Self {
        self.track_color = track;
        self.fill_color = fill;
        self.thumb_color = thumb;
        self.thumb_hover_color = thumb;
        self
    }

    /// Maps the normalized value to a custom minimum and maximum.
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }

    /// Enables snapping to a fixed step size in real-value space.
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Enables the value label.
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }

    // Disables smooth animation.
    // pub fn instant(mut self) -> Self {
    //     self.smooth_animation = false;
    //     self
    // }

    /// Returns the mapped value in the configured real range.
    pub fn real_value(&self) -> f32 {
        self.min_value + (self.value * (self.max_value - self.min_value))
    }

    /// Sets the mapped real value and converts it back into normalized space.
    pub fn set_real_value(&mut self, real_value: f32) {
        let normalized = (real_value - self.min_value) / (self.max_value - self.min_value);
        self.value = normalized.clamp(0.0, 1.0);
        // self.target_value = self.value;
    }

    // === Presets ===

    pub fn volume_style() -> Self {
        Self {
            width: 150.0,
            track_height: 4.0,
            thumb_size: 16.0,
            track_color: Color::srgb(0.25, 0.25, 0.3),
            fill_color: Color::srgb(0.0, 0.8, 0.4),
            thumb_color: Color::srgb(0.9, 0.95, 1.0),
            show_value: true,
            ..default()
        }
    }

    pub fn video_style() -> Self {
        Self {
            width: 400.0,
            track_height: 5.0,
            thumb_size: 14.0,
            track_color: Color::srgba(0.5, 0.5, 0.5, 0.5),
            fill_color: Color::srgb(0.9, 0.1, 0.2),
            thumb_color: Color::srgb(0.9, 0.1, 0.2),
            show_value: false,
            ..default()
        }
    }

    pub fn brightness_style() -> Self {
        Self {
            width: 200.0,
            track_height: 6.0,
            thumb_size: 20.0,
            track_color: Color::srgb(0.2, 0.2, 0.25),
            fill_color: Color::srgb(1.0, 0.9, 0.3),
            thumb_color: Color::srgb(1.0, 1.0, 0.5),
            show_value: true,
            min_value: 0.0,
            max_value: 100.0,
            ..default()
        }
    }

    pub fn sci_fi_style() -> Self {
        Self {
            width: 300.0,
            track_height: 8.0,
            thumb_size: 24.0,
            track_color: Color::srgb(0.05, 0.1, 0.2),
            fill_color: Color::srgb(0.0, 0.8, 1.0),
            thumb_color: Color::srgb(0.5, 1.0, 1.0),
            thumb_hover_color: Color::srgb(0.8, 1.0, 1.0),
            show_value: true,
            ..default()
        }
    }
}

/// Internal markers
#[derive(Component)]
struct SeekBarTrack;

#[derive(Component)]
struct SeekBarFill;

#[derive(Component)]
struct SeekBarThumb;

#[derive(Component)]
struct SeekBarValueLabel;

// =========================================================
// Systems
// =========================================================

/// Builds the seek bar visual hierarchy.
fn init_seekbar_visuals(
    mut commands: Commands,
    query: Query<(Entity, &USeekBar), Added<USeekBar>>,
) {
    for (entity, seekbar) in query.iter() {
        commands
            .entity(entity)
            .insert((
                UNode {
                    width: UVal::Px(seekbar.width),
                    height: UVal::Px(seekbar.thumb_size + 10.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    gap: 5.0,
                    ..default()
                },
                UInteraction::default(),
            ))
            .with_children(|parent| {
                // Container that owns the track and thumb.
                parent
                    .spawn((
                        UNode {
                            width: UVal::Px(seekbar.width),
                            height: UVal::Px(seekbar.thumb_size),
                            background_color: Color::NONE,
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            align_items: UAlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|track_container| {
                        // Track background
                        track_container
                            .spawn((
                                UNode {
                                    width: UVal::Px(seekbar.width),
                                    height: UVal::Px(seekbar.track_height),
                                    background_color: seekbar.track_color,
                                    border_radius: UCornerRadius::all(seekbar.track_height / 2.0),
                                    ..default()
                                },
                                ULayout {
                                    display: UDisplay::Flex,
                                    ..default()
                                },
                                SeekBarTrack,
                            ))
                            .with_children(|track_parent| {
                                // Filled portion
                                let fill_width = seekbar.width * seekbar.value;
                                track_parent.spawn((
                                    UNode {
                                        width: UVal::Px(fill_width),
                                        height: UVal::Px(seekbar.track_height),
                                        background_color: seekbar.fill_color,
                                        border_radius: UCornerRadius::all(
                                            seekbar.track_height / 2.0,
                                        ),
                                        ..default()
                                    },
                                    SeekBarFill,
                                ));

                                // Draggable thumb
                                let thumb_x = (seekbar.width - seekbar.thumb_size) * seekbar.value;
                                track_parent.spawn((
                                    UNode {
                                        width: UVal::Px(seekbar.thumb_size),
                                        height: UVal::Px(seekbar.thumb_size),
                                        background_color: seekbar.thumb_color,
                                        border_radius: UCornerRadius::all(seekbar.thumb_size / 2.0),
                                        ..default()
                                    },
                                    USelf {
                                        position_type: UPositionType::Absolute,
                                        left: UVal::Px(thumb_x),
                                        top: UVal::Px(
                                            (seekbar.track_height - seekbar.thumb_size) / 2.0,
                                        ),
                                        ..default()
                                    },
                                    SeekBarThumb,
                                    UInteraction::default(),
                                ));
                            });
                    });

                // Optional value label
                if seekbar.show_value {
                    parent.spawn((
                        UTextLabel {
                            text: format!("{:.0}", seekbar.real_value()),
                            font_size: 12.0,
                            color: Color::srgb(0.7, 0.7, 0.8),
                            ..default()
                        },
                        SeekBarValueLabel,
                    ));
                }
            });
    }
}

fn start_seekbar_interaction(
    mut press: On<Pointer<Press>>,
    mut seekbar_query: Query<(&mut USeekBar, &GlobalTransform)>,
) {
    let entity = press.entity.entity();
    let Ok((mut seekbar, transform)) = seekbar_query.get_mut(entity) else {
        return;
    };
    if seekbar.disabled || press.button != PointerButton::Primary {
        return;
    }

    press.propagate(false);

    if let Some(hit_world) = press.hit.position {
        let local_x = transform
            .to_matrix()
            .inverse()
            .transform_point3(hit_world)
            .x;
        seekbar.value = normalized_value_from_local_x(local_x, seekbar.width, seekbar.thumb_size);
    }

    seekbar.drag_start_value = seekbar.value;
    seekbar.is_dragging = true;
}

fn drag_seekbar_interaction(mut drag: On<Pointer<Drag>>, mut seekbar_query: Query<&mut USeekBar>) {
    let entity = drag.entity.entity();
    let Ok(mut seekbar) = seekbar_query.get_mut(entity) else {
        return;
    };
    if seekbar.disabled || !seekbar.is_dragging || drag.button != PointerButton::Primary {
        return;
    }

    drag.propagate(false);

    let usable_width = (seekbar.width - seekbar.thumb_size).max(1.0);
    let delta_ratio = drag.distance.x / usable_width;
    seekbar.value = (seekbar.drag_start_value + delta_ratio).clamp(0.0, 1.0);
}

fn release_seekbar_interaction(
    mut release: On<Pointer<Release>>,
    mut seekbar_query: Query<&mut USeekBar>,
) {
    let entity = release.entity.entity();
    let Ok(mut seekbar) = seekbar_query.get_mut(entity) else {
        return;
    };
    if release.button != PointerButton::Primary {
        return;
    }

    release.propagate(false);
    seekbar.is_dragging = false;
    seekbar.drag_start_value = seekbar.value;
}

fn end_seekbar_drag(mut drag_end: On<Pointer<DragEnd>>, mut seekbar_query: Query<&mut USeekBar>) {
    let entity = drag_end.entity.entity();
    let Ok(mut seekbar) = seekbar_query.get_mut(entity) else {
        return;
    };
    if drag_end.button != PointerButton::Primary {
        return;
    }

    drag_end.propagate(false);
    seekbar.is_dragging = false;
    seekbar.drag_start_value = seekbar.value;
}

/// Updates the seek bar visuals after state changes.
fn update_seekbar_visuals(
    // Find changed seek bars.
    seekbar_query: Query<(&USeekBar, &Children), Changed<USeekBar>>,

    // Traverse nested children.
    children_query: Query<&Children>,

    // Identify the track node.
    track_marker: Query<(), With<SeekBarTrack>>,

    // Update the visual parts.
    mut fill_query: Query<&mut UNode, With<SeekBarFill>>,
    mut thumb_query: Query<&mut USelf, With<SeekBarThumb>>,
    mut label_query: Query<&mut UTextLabel, With<SeekBarValueLabel>>,
) {
    for (seekbar, children) in seekbar_query.iter() {
        // Walk direct children: the container and optional label.
        for child in children.iter() {
            // 1. Update the direct label child when present.
            if let Ok(mut label) = label_query.get_mut(child) {
                label.text = format!("{:.0}", seekbar.real_value());
                continue; // Move on to the next child.
            }

            // 2. Otherwise this child is the container. Walk into it.
            if let Ok(container_children) = children_query.get(child) {
                for item in container_children.iter() {
                    // 3. Find the track node.
                    if track_marker.get(item).is_ok() {
                        // Once we have the track, refresh its fill and thumb children.
                        if let Ok(track_children) = children_query.get(item) {
                            for track_item in track_children.iter() {
                                // Update the fill width.
                                if let Ok(mut fill_node) = fill_query.get_mut(track_item) {
                                    let fill_width = seekbar.width * seekbar.value;
                                    fill_node.width = UVal::Px(fill_width);
                                }

                                // Update the thumb position.
                                if let Ok(mut thumb_uself) = thumb_query.get_mut(track_item) {
                                    let thumb_x =
                                        (seekbar.width - seekbar.thumb_size) * seekbar.value;
                                    thumb_uself.left = UVal::Px(thumb_x);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Smooth thumb animation placeholder.
// fn animate_seekbar_thumb(
//     time: Res<Time>,
//     mut query: Query<&mut USeekBar>,
// ) {
//     for mut seekbar in query.iter_mut() {
//         if !seekbar.smooth_animation {
//             continue;
//         }

//         let diff =seekbar.value - seekbar.target_value;

//         if diff.abs() < 0.001 {
//             seekbar.target_value = seekbar.value;
//             continue;
//         }

//         let delta = time.delta_secs() * seekbar.animation_speed;
//         seekbar.target_value += diff * delta;
//     }
// }

/// Emits seek-bar change events.
fn emit_seekbar_events(
    mut events: MessageWriter<SeekBarChangedEvent>,
    mut query: Query<(Entity, &mut USeekBar)>,
) {
    for (entity, mut seekbar) in query.iter_mut() {
        if (seekbar.value - seekbar.previous_value).abs() > 0.001 {
            events.write(SeekBarChangedEvent {
                entity,
                value: seekbar.value,
                real_value: seekbar.real_value(),
            });

            seekbar.previous_value = seekbar.value;
        }
    }
}

// =========================================================
// Event
// =========================================================

#[derive(Message)]
pub struct SeekBarChangedEvent {
    pub entity: Entity,
    pub value: f32,      // 0.0 - 1.0
    pub real_value: f32, // Value mapped into the configured real range.
}

fn normalized_value_from_local_x(local_x: f32, width: f32, thumb_size: f32) -> f32 {
    let usable_width = (width - thumb_size).max(1.0);
    let x_from_left = local_x + (width * 0.5);
    let adjusted_x = x_from_left - (thumb_size * 0.5);
    (adjusted_x / usable_width).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::normalized_value_from_local_x;

    #[test]
    fn normalized_value_clamps_at_track_edges() {
        assert_eq!(normalized_value_from_local_x(-100.0, 200.0, 20.0), 0.0);
        assert_eq!(normalized_value_from_local_x(100.0, 200.0, 20.0), 1.0);
    }

    #[test]
    fn normalized_value_maps_track_center_to_midpoint() {
        let midpoint = normalized_value_from_local_x(0.0, 200.0, 20.0);
        assert!((midpoint - 0.5).abs() < 0.001);
    }
}
