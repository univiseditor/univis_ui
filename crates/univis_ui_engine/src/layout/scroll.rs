//! Scroll container and viewport management for Univis UI.
//!
//! Provides [`crate::layout::scroll::UScrollContainer`] and [`crate::layout::scroll::UScrollContent`] for smooth mouse-wheel
//! driven scrolling, automatic extent calculation, and hardware-accelerated
//! rounded SDF clipping via [`UClip`](crate::layout::univis_node::UClip).

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::layout::query::ComputedSize;
use crate::layout::univis_node::{UClip, UNode};

/// Configures a scrollable viewport container.
///
/// When attached to an entity with a [`UNode`], overflowing child content marked
/// with [`crate::layout::scroll::UScrollContent`] (or the first child node) glides smoothly using mouse
/// wheel input or programmatic target offsets.
///
/// Attaching `UScrollContainer` automatically requires [`UNode`] and [`UClip`].
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
#[require(UNode, UClip)]
pub struct UScrollContainer {
    /// Whether vertical scrolling is enabled (default: true).
    pub vertical: bool,
    /// Whether horizontal scrolling is enabled (default: false).
    pub horizontal: bool,
    /// Current rendered scroll offset in logical pixels (x = horizontal, y = vertical).
    pub scroll_offset: Vec2,
    /// Target scroll offset for smooth interpolation.
    pub target_offset: Vec2,
    /// Maximum allowed scroll offset computed from (content_size - container_size).
    pub max_offset: Vec2,
    /// Scroll sensitivity factor per mouse wheel tick (default: 40.0 pixels per notch).
    pub sensitivity: f32,
    /// Smoothing responsiveness factor (higher = faster glide, default: 18.0).
    pub smoothness: f32,
    /// Explicit hover flag for custom pointer integrations.
    pub is_hovered: bool,
}

impl Default for UScrollContainer {
    fn default() -> Self {
        Self {
            vertical: true,
            horizontal: false,
            scroll_offset: Vec2::ZERO,
            target_offset: Vec2::ZERO,
            max_offset: Vec2::ZERO,
            sensitivity: 40.0,
            smoothness: 18.0,
            is_hovered: false,
        }
    }
}

impl UScrollContainer {
    /// Creates a vertical scroll container.
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            horizontal: false,
            ..Default::default()
        }
    }

    /// Creates a horizontal scroll container.
    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            horizontal: true,
            ..Default::default()
        }
    }

    /// Creates a bidirectional scroll container (both vertical and horizontal).
    pub fn both() -> Self {
        Self {
            vertical: true,
            horizontal: true,
            ..Default::default()
        }
    }

    /// Sets custom scroll sensitivity (pixels per wheel notch).
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Sets custom smoothing factor (higher = snappier glide).
    pub fn with_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    /// Sets the target scroll offset immediately, clamped to `max_offset`.
    pub fn scroll_to(&mut self, target: Vec2) {
        let x = if self.horizontal {
            target.x.clamp(0.0, self.max_offset.x)
        } else {
            0.0
        };
        let y = if self.vertical {
            target.y.clamp(0.0, self.max_offset.y)
        } else {
            0.0
        };
        self.target_offset = Vec2::new(x, y);
    }

    /// Adjusts the target scroll offset by a relative delta.
    pub fn scroll_by(&mut self, delta: Vec2) {
        self.scroll_to(self.target_offset + delta);
    }

    /// Instantly snaps to the target offset without smooth interpolation.
    pub fn jump_to(&mut self, target: Vec2) {
        self.scroll_to(target);
        self.scroll_offset = self.target_offset;
    }

    /// Returns true if vertical scrolling is at the top (offset == 0.0).
    pub fn is_at_top(&self) -> bool {
        self.scroll_offset.y <= 0.01
    }

    /// Returns true if vertical scrolling reached the bottom extent.
    pub fn is_at_bottom(&self) -> bool {
        self.max_offset.y > 0.0 && (self.scroll_offset.y - self.max_offset.y).abs() <= 0.5
    }

    /// Returns normalized vertical scroll progress in 0.0..=1.0.
    pub fn progress_y(&self) -> f32 {
        if self.max_offset.y > 0.0 {
            (self.scroll_offset.y / self.max_offset.y).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Marks an entity as the scrollable content inside a [`crate::layout::scroll::UScrollContainer`].
///
/// When attached to a child of a `UScrollContainer`, its layout position is preserved
/// as `base_translation`, and its rendered transform is smoothly offset by the container's
/// scroll offset.
#[derive(Component, Clone, Copy, Debug, Default, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UScrollContent {
    /// Cached base translation established by the downward layout solver.
    pub base_translation: Vec2,
    /// Whether the base translation has been initialized.
    pub initialized: bool,
}

impl UScrollContent {
    /// Creates a new `UScrollContent` marker.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Automatically ensures [`UClip`] is enabled when [`crate::layout::scroll::UScrollContainer`] is added.
pub fn init_scroll_containers(
    mut commands: Commands,
    query: Query<(Entity, Option<&UClip>), Added<UScrollContainer>>,
) {
    for (entity, clip) in query.iter() {
        if clip.map_or(true, |c| !c.enabled) {
            commands.entity(entity).insert(UClip { enabled: true });
        }
    }
}

/// Automatically attaches [`crate::layout::scroll::UScrollContent`] to the first child of a [`crate::layout::scroll::UScrollContainer`] if missing.
pub fn auto_init_scroll_content(
    mut commands: Commands,
    containers: Query<(Entity, &Children), With<UScrollContainer>>,
    all_content: Query<(), With<UScrollContent>>,
    unode_query: Query<(), With<UNode>>,
) {
    for (_container_entity, children) in containers.iter() {
        let has_content_child = children.iter().any(|c| all_content.contains(c));
        if !has_content_child {
            if let Some(&first_child) = children.first() {
                if unode_query.contains(first_child) {
                    commands
                        .entity(first_child)
                        .insert(UScrollContent::default());
                }
            }
        }
    }
}

/// Synchronizes container maximum scroll extents from container and content computed sizes.
pub fn sync_scroll_extents(
    mut containers: Query<(Entity, &mut UScrollContainer, &ComputedSize)>,
    content_query: Query<(&UScrollContent, &ComputedSize, &ChildOf)>,
) {
    for (_content, content_size, parent) in content_query.iter() {
        if let Ok((_entity, mut container, container_size)) = containers.get_mut(parent.parent()) {
            let overflow_y = (content_size.height - container_size.height).max(0.0);
            let overflow_x = (content_size.width - container_size.width).max(0.0);
            container.max_offset = Vec2::new(overflow_x, overflow_y);

            // Clamp offsets when extents change
            container.target_offset = container
                .target_offset
                .clamp(Vec2::ZERO, container.max_offset);
            container.scroll_offset = container
                .scroll_offset
                .clamp(Vec2::ZERO, container.max_offset);
        }
    }
}

/// Reads mouse wheel input and updates target scroll offsets on hovered scroll containers.
pub fn handle_mouse_wheel_scroll(
    mouse_wheel: Option<MessageReader<MouseWheel>>,
    mut containers: Query<(
        Entity,
        &mut UScrollContainer,
        &GlobalTransform,
        &ComputedSize,
    )>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let Some(mut mouse_wheel) = mouse_wheel else {
        return;
    };

    if mouse_wheel.is_empty() {
        return;
    }

    let cursor_world_pos: Option<Vec2> = (|| {
        let window = windows.iter().next()?;
        let cursor_pos = window.cursor_position()?;
        let (camera, cam_gt) = camera_q.iter().next()?;
        camera.viewport_to_world_2d(cam_gt, cursor_pos).ok()
    })();

    for ev in mouse_wheel.read() {
        for (_entity, mut container, global_transform, computed) in containers.iter_mut() {
            let is_inside = if let Some(cursor_pos) = cursor_world_pos {
                let container_pos = global_transform.translation().xy();
                let half_w = computed.width / 2.0;
                let half_h = computed.height / 2.0;
                (cursor_pos.x - container_pos.x).abs() <= half_w
                    && (cursor_pos.y - container_pos.y).abs() <= half_h
            } else {
                false
            };

            if !container.is_hovered && !is_inside {
                continue;
            }

            let delta = match ev.unit {
                MouseScrollUnit::Line => Vec2::new(ev.x, ev.y) * container.sensitivity,
                MouseScrollUnit::Pixel => Vec2::new(ev.x, ev.y),
            };

            if container.vertical {
                // Scrolling wheel down (ev.y < 0) reveals lower content: target offset increases
                container.target_offset.y =
                    (container.target_offset.y - delta.y).clamp(0.0, container.max_offset.y);
            }

            if container.horizontal {
                container.target_offset.x =
                    (container.target_offset.x - delta.x).clamp(0.0, container.max_offset.x);
            }
        }
    }
}

/// Smoothly interpolates scroll offset towards target offset and updates the content transform.
pub fn apply_scroll_transitions(
    time: Option<Res<Time>>,
    mut containers: Query<&mut UScrollContainer>,
    mut content_query: Query<(&UScrollContent, &mut Transform, &ChildOf)>,
) {
    let dt = time.map(|t| t.delta_secs().min(0.05)).unwrap_or(0.016);
    if dt <= 0.0 {
        return;
    }

    for (content, mut transform, parent) in content_query.iter_mut() {
        let Ok(mut container) = containers.get_mut(parent.parent()) else {
            continue;
        };

        if !content.initialized {
            continue;
        }

        // Smooth glide towards target offset
        let diff = container.target_offset - container.scroll_offset;
        if diff.length_squared() < 0.0001 {
            container.scroll_offset = container.target_offset;
        } else {
            let blend = 1.0 - (-container.smoothness * dt).exp();
            container.scroll_offset += diff * blend;
        }

        let effective_x = if container.horizontal {
            container.scroll_offset.x
        } else {
            0.0
        };
        let effective_y = if container.vertical {
            container.scroll_offset.y
        } else {
            0.0
        };

        transform.translation.x = content.base_translation.x - effective_x;
        transform.translation.y = content.base_translation.y + effective_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_container_constructors() {
        let v = UScrollContainer::vertical();
        assert!(v.vertical);
        assert!(!v.horizontal);
        assert_eq!(v.sensitivity, 40.0);

        let h = UScrollContainer::horizontal();
        assert!(!h.vertical);
        assert!(h.horizontal);

        let b = UScrollContainer::both();
        assert!(b.vertical);
        assert!(b.horizontal);
    }

    #[test]
    fn test_scroll_to_and_by_clamps() {
        let mut sc = UScrollContainer::vertical();
        sc.max_offset = Vec2::new(100.0, 500.0);

        sc.scroll_to(Vec2::new(50.0, 200.0));
        assert_eq!(sc.target_offset, Vec2::new(0.0, 200.0)); // horizontal disabled

        sc.scroll_by(Vec2::new(0.0, 400.0));
        assert_eq!(sc.target_offset.y, 500.0); // clamped to max

        sc.scroll_by(Vec2::new(0.0, -600.0));
        assert_eq!(sc.target_offset.y, 0.0); // clamped to 0
    }

    #[test]
    fn test_scroll_progress() {
        let mut sc = UScrollContainer::vertical();
        sc.max_offset.y = 200.0;
        assert!(sc.is_at_top());
        assert_eq!(sc.progress_y(), 0.0);

        sc.scroll_offset.y = 100.0;
        assert_eq!(sc.progress_y(), 0.5);

        sc.scroll_offset.y = 200.0;
        assert!(sc.is_at_bottom());
        assert_eq!(sc.progress_y(), 1.0);
    }

    #[test]
    fn test_scroll_smooth_gliding() {
        let mut sc = UScrollContainer::vertical().with_smoothness(20.0);
        sc.max_offset.y = 400.0;
        sc.scroll_to(Vec2::new(0.0, 200.0));

        let dt = 1.0 / 60.0;
        for _ in 0..30 {
            let diff = sc.target_offset - sc.scroll_offset;
            let blend = 1.0 - (-sc.smoothness * dt).exp();
            sc.scroll_offset += diff * blend;
        }

        assert!(
            (sc.scroll_offset.y - 200.0).abs() < 1.0,
            "Scroll offset failed to glide to target: offset = {}",
            sc.scroll_offset.y
        );
    }
}
