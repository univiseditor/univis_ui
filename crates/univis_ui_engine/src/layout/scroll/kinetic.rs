//! Kinetic momentum drag scrolling for touch and pointer interactions.
//!
//! Provides [`UScrollKineticDrag`], enabling fluid touch-style content dragging
//! with physics momentum, flick velocity, and exponential friction decay.

use bevy::prelude::*;

use crate::layout::query::ComputedSize;
use crate::layout::scroll::container::UScrollContainer;
use crate::layout::scroll::scrollbar::UScrollbarThumb;

/// Enables kinetic momentum drag scrolling when attached to a [`UScrollContainer`].
///
/// Users can click and drag directly on the container's content area. On release,
/// the scroll container retains velocity and glides with exponential friction deceleration.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UScrollKineticDrag {
    /// Whether kinetic dragging is enabled (default: true).
    pub enabled: bool,
    /// Friction factor that decelerates momentum after pointer release (higher = stops faster, default: 8.0).
    pub friction: f32,
    /// Maximum allowed kinetic velocity in logical pixels per second (default: 5000.0).
    pub max_velocity: f32,
    /// Velocity threshold below which kinetic motion stops completely (default: 4.0).
    pub min_velocity_threshold: f32,
    /// Current kinetic velocity vector (logical pixels per second).
    pub velocity: Vec2,
    /// Whether the container is currently being dragged.
    pub is_dragging: bool,
    /// Last observed pointer position in world coordinates.
    pub last_pointer_pos: Vec2,
    /// Smoothed instantaneous velocity vector during dragging.
    pub drag_velocity: Vec2,
}

impl Default for UScrollKineticDrag {
    fn default() -> Self {
        Self {
            enabled: true,
            friction: 8.0,
            max_velocity: 5000.0,
            min_velocity_threshold: 4.0,
            velocity: Vec2::ZERO,
            is_dragging: false,
            last_pointer_pos: Vec2::ZERO,
            drag_velocity: Vec2::ZERO,
        }
    }
}

impl UScrollKineticDrag {
    /// Creates a new kinetic drag configuration with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets custom friction deceleration factor.
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction.max(0.1);
        self
    }

    /// Sets maximum allowed momentum velocity.
    pub fn with_max_velocity(mut self, max_velocity: f32) -> Self {
        self.max_velocity = max_velocity.max(100.0);
        self
    }

    /// Stops all kinetic momentum immediately.
    pub fn stop(&mut self) {
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
        self.is_dragging = false;
    }
}

/// Reads pointer drag on scroll containers, tracks velocity, and applies kinetic momentum.
pub fn handle_kinetic_drag(
    time: Option<Res<Time>>,
    mouse_button: Option<Res<ButtonInput<MouseButton>>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    thumbs: Query<&UScrollbarThumb>,
    mut containers: Query<(
        Entity,
        &mut UScrollContainer,
        &mut UScrollKineticDrag,
        &GlobalTransform,
        &ComputedSize,
    )>,
) {
    let dt = time.map(|t| t.delta_secs().min(0.05)).unwrap_or(0.016);
    if dt <= 0.0 {
        return;
    }

    // If any thumb is currently dragging, do not initiate container kinetic drag
    let any_thumb_dragging = thumbs.iter().any(|t| t.is_dragging);

    let cursor_world_pos: Option<Vec2> = (|| {
        let window = windows.iter().next()?;
        let cursor_pos = window.cursor_position()?;
        let (camera, cam_gt) = camera_q.iter().next()?;
        camera.viewport_to_world_2d(cam_gt, cursor_pos).ok()
    })();

    let is_pressed = mouse_button
        .as_ref()
        .is_some_and(|mb| mb.pressed(MouseButton::Left));
    let just_pressed = mouse_button
        .as_ref()
        .is_some_and(|mb| mb.just_pressed(MouseButton::Left));
    let just_released = mouse_button
        .as_ref()
        .is_some_and(|mb| mb.just_released(MouseButton::Left));

    for (_e, mut container, mut kinetic, gt, computed) in containers.iter_mut() {
        if !kinetic.enabled {
            continue;
        }

        // 1. Release drag
        if (just_released || !is_pressed) && kinetic.is_dragging {
            kinetic.is_dragging = false;
            kinetic.velocity = kinetic.drag_velocity.clamp_length_max(kinetic.max_velocity);
        }

        if let Some(cursor) = cursor_world_pos {
            // 2. Drag initiation
            if just_pressed && !any_thumb_dragging {
                let container_pos = gt.translation().xy();
                let half_w = computed.width / 2.0;
                let half_h = computed.height / 2.0;

                let is_inside = (cursor.x - container_pos.x).abs() <= half_w
                    && (cursor.y - container_pos.y).abs() <= half_h;

                if is_inside {
                    kinetic.is_dragging = true;
                    kinetic.last_pointer_pos = cursor;
                    kinetic.velocity = Vec2::ZERO;
                    kinetic.drag_velocity = Vec2::ZERO;
                }
            }

            // 3. Active drag tracking
            if is_pressed && kinetic.is_dragging {
                let delta = cursor - kinetic.last_pointer_pos;
                kinetic.last_pointer_pos = cursor;

                // Dragging UP (+Y in world) pulls content UP -> reveals bottom -> increases scroll offset
                let scroll_delta_y = if container.vertical { delta.y } else { 0.0 };
                // Dragging LEFT (-X in world) pulls content LEFT -> reveals right -> increases scroll offset
                let scroll_delta_x = if container.horizontal { -delta.x } else { 0.0 };

                let target = container.scroll_offset + Vec2::new(scroll_delta_x, scroll_delta_y);
                container.jump_to(target);

                // Sample instantaneous velocity
                if dt > 0.001 {
                    let sample = Vec2::new(scroll_delta_x, scroll_delta_y) / dt;
                    let blend = (18.0 * dt).min(1.0);
                    kinetic.drag_velocity = kinetic.drag_velocity.lerp(sample, blend);
                }
            }
        }

        // 4. Kinetic momentum glide & friction decay
        if !kinetic.is_dragging && kinetic.velocity.length_squared() > 0.0 {
            container.scroll_by(kinetic.velocity * dt);

            let decay = (-kinetic.friction * dt).exp();
            kinetic.velocity *= decay;

            if kinetic.velocity.length() < kinetic.min_velocity_threshold {
                kinetic.velocity = Vec2::ZERO;
            }

            // Snap momentum when reaching vertical bounds
            if container.is_at_top() && kinetic.velocity.y < 0.0 {
                kinetic.velocity.y = 0.0;
            }
            if container.is_at_bottom() && kinetic.velocity.y > 0.0 {
                kinetic.velocity.y = 0.0;
            }

            // Snap momentum when reaching horizontal bounds
            if container.scroll_offset.x <= 0.01 && kinetic.velocity.x < 0.0 {
                kinetic.velocity.x = 0.0;
            }
            if container.max_offset.x > 0.0
                && (container.scroll_offset.x - container.max_offset.x).abs() <= 0.5
                && kinetic.velocity.x > 0.0
            {
                kinetic.velocity.x = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinetic_drag_defaults() {
        let kd = UScrollKineticDrag::default();
        assert!(kd.enabled);
        assert_eq!(kd.friction, 8.0);
        assert_eq!(kd.max_velocity, 5000.0);
        assert_eq!(kd.velocity, Vec2::ZERO);
        assert!(!kd.is_dragging);
    }

    #[test]
    fn test_kinetic_friction_decay() {
        let mut kd = UScrollKineticDrag::default().with_friction(10.0);
        kd.velocity = Vec2::new(0.0, 1000.0);

        let dt = 0.1;
        let decay = (-kd.friction * dt).exp();
        kd.velocity *= decay;

        // e^(-1) ≈ 0.367879 -> 1000 * 0.367879 ≈ 367.88
        assert!((kd.velocity.y - 367.88).abs() < 1.0);
    }

    #[test]
    fn test_kinetic_stop() {
        let mut kd = UScrollKineticDrag::default();
        kd.velocity = Vec2::new(200.0, 500.0);
        kd.drag_velocity = Vec2::new(100.0, 300.0);
        kd.is_dragging = true;

        kd.stop();
        assert_eq!(kd.velocity, Vec2::ZERO);
        assert_eq!(kd.drag_velocity, Vec2::ZERO);
        assert!(!kd.is_dragging);
    }
}
