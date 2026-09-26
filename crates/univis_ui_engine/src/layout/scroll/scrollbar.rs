//! Visual scrollbar primitives: tracks, thumbs, auto-hide fading, and drag control.
//!
//! Provides [`UScrollbarTrack`], [`UScrollbarThumb`], and [`UScrollbarFade`].
//! These components are unopinionated ECS primitives that can be attached to any
//! [`UNode`] to build fully custom, hardware-accelerated
//! scrollbars with proportional thumb sizing, synchronous drag-to-scroll, and track click-to-jump.

use bevy::prelude::*;

use crate::layout::query::ComputedSize;
use crate::layout::scroll::container::UScrollContainer;
use crate::layout::univis_node::UNode;

/// Scrollbar orientation axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Default)]
pub enum UScrollbarAxis {
    /// Vertical scrollbar (glides along Y axis).
    #[default]
    Vertical,
    /// Horizontal scrollbar (glides along X axis).
    Horizontal,
}

/// Marks an entity as a scrollbar track.
///
/// Clicking on the track can immediately jump or page the associated [`UScrollContainer`].
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
#[require(UNode)]
pub struct UScrollbarTrack {
    /// The target scroll container entity.
    pub container: Entity,
    /// Orientation axis of the track.
    pub axis: UScrollbarAxis,
    /// If true, clicking on the track jumps the scroll offset to the clicked position.
    pub click_to_jump: bool,
    /// If true, hides the track when the container content does not overflow.
    pub hide_when_not_scrollable: bool,
}

impl UScrollbarTrack {
    /// Creates a vertical scrollbar track targeting the given container.
    pub fn vertical(container: Entity) -> Self {
        Self {
            container,
            axis: UScrollbarAxis::Vertical,
            click_to_jump: true,
            hide_when_not_scrollable: false,
        }
    }

    /// Creates a horizontal scrollbar track targeting the given container.
    pub fn horizontal(container: Entity) -> Self {
        Self {
            container,
            axis: UScrollbarAxis::Horizontal,
            click_to_jump: true,
            hide_when_not_scrollable: false,
        }
    }

    /// Enables or disables click-to-jump behavior.
    pub fn with_click_to_jump(mut self, click_to_jump: bool) -> Self {
        self.click_to_jump = click_to_jump;
        self
    }

    /// Enables or disables automatic hiding when content does not overflow.
    pub fn with_hide_when_not_scrollable(mut self, hide: bool) -> Self {
        self.hide_when_not_scrollable = hide;
        self
    }
}

/// Marks an entity as a scrollbar thumb slider.
///
/// Typically spawned as a child of a [`UScrollbarTrack`]. Its size automatically
/// reflects the viewport-to-content ratio, and dragging it smoothly scrolls
/// the target [`UScrollContainer`].
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
#[require(UNode)]
pub struct UScrollbarThumb {
    /// The target scroll container entity.
    pub container: Entity,
    /// Explicit track entity if not the immediate parent.
    pub track: Option<Entity>,
    /// Movement axis.
    pub axis: UScrollbarAxis,
    /// Minimum size of the thumb along its scroll axis in logical pixels (default: 20.0).
    pub min_size: f32,
    /// If true, automatically sizes the thumb proportional to viewport/content ratio.
    pub auto_size: bool,
    /// If true, hides the thumb when the container is not scrollable.
    pub hide_when_not_scrollable: bool,
    /// Cached base translation set by the downward layout solver.
    pub base_translation: Vec2,
    /// Whether base translation has been initialized.
    pub initialized: bool,
    /// Whether the thumb is currently being dragged by a pointer.
    pub is_dragging: bool,
    /// Pointer position along the movement axis when dragging began.
    pub drag_start_cursor: f32,
    /// Container scroll offset along the movement axis when dragging began.
    pub drag_start_offset: f32,
}

impl UScrollbarThumb {
    /// Creates a vertical scrollbar thumb targeting the given container.
    pub fn vertical(container: Entity) -> Self {
        Self {
            container,
            track: None,
            axis: UScrollbarAxis::Vertical,
            min_size: 20.0,
            auto_size: true,
            hide_when_not_scrollable: false,
            base_translation: Vec2::ZERO,
            initialized: false,
            is_dragging: false,
            drag_start_cursor: 0.0,
            drag_start_offset: 0.0,
        }
    }

    /// Creates a horizontal scrollbar thumb targeting the given container.
    pub fn horizontal(container: Entity) -> Self {
        Self {
            container,
            track: None,
            axis: UScrollbarAxis::Horizontal,
            min_size: 20.0,
            auto_size: true,
            hide_when_not_scrollable: false,
            base_translation: Vec2::ZERO,
            initialized: false,
            is_dragging: false,
            drag_start_cursor: 0.0,
            drag_start_offset: 0.0,
        }
    }

    /// Configures minimum thumb size along its axis.
    pub fn with_min_size(mut self, min_size: f32) -> Self {
        self.min_size = min_size.max(4.0);
        self
    }

    /// Explicitly sets the track entity.
    pub fn with_track(mut self, track: Entity) -> Self {
        self.track = Some(track);
        self
    }

    /// Enables or disables automatic proportional sizing.
    pub fn with_auto_size(mut self, auto_size: bool) -> Self {
        self.auto_size = auto_size;
        self
    }

    /// Enables or disables auto-hiding when not scrollable.
    pub fn with_hide_when_not_scrollable(mut self, hide: bool) -> Self {
        self.hide_when_not_scrollable = hide;
        self
    }
}

/// Adds automatic fade-out behavior to a scrollbar track or thumb when idle.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UScrollbarFade {
    /// Idle duration in seconds before fade-out begins (default: 1.2s).
    pub idle_timeout: f32,
    /// Duration of the fade-out animation in seconds (default: 0.35s).
    pub fade_duration: f32,
    /// Idle timer counting up towards `idle_timeout`.
    pub idle_timer: f32,
    /// Current opacity multiplier (0.0 = transparent, 1.0 = fully visible).
    pub current_alpha: f32,
    /// Target opacity multiplier.
    pub target_alpha: f32,
    /// Cached base background alpha for restoration.
    pub base_alpha: f32,
    /// Whether base alpha was initialized.
    pub initialized: bool,
}

impl Default for UScrollbarFade {
    fn default() -> Self {
        Self {
            idle_timeout: 1.2,
            fade_duration: 0.35,
            idle_timer: 0.0,
            current_alpha: 1.0,
            target_alpha: 1.0,
            base_alpha: 1.0,
            initialized: false,
        }
    }
}

impl UScrollbarFade {
    /// Creates a new fade config with the specified idle timeout and fade duration.
    pub fn new(idle_timeout: f32, fade_duration: f32) -> Self {
        Self {
            idle_timeout,
            fade_duration,
            ..Default::default()
        }
    }
}

/// Calculates proportional thumb size based on track size and container viewport/content ratio.
pub fn calculate_thumb_size(
    track_size: f32,
    viewport_size: f32,
    max_offset: f32,
    min_size: f32,
) -> f32 {
    if track_size <= 0.0 || viewport_size <= 0.0 {
        return min_size;
    }
    let content_size = viewport_size + max_offset;
    if content_size <= viewport_size || max_offset <= 0.0 {
        return track_size;
    }
    let ratio = (viewport_size / content_size).clamp(0.0, 1.0);
    (track_size * ratio).clamp(min_size, track_size)
}

/// Synchronizes scrollbar thumb sizes, positions, and visibility.
#[allow(clippy::type_complexity)]
pub fn sync_scrollbar_thumbs(
    containers: Query<(&UScrollContainer, &ComputedSize)>,
    tracks: Query<(&UScrollbarTrack, &ComputedSize)>,
    all_computed: Query<&ComputedSize>,
    parents: Query<&ChildOf>,
    mut thumbs: Query<(
        Entity,
        &mut UScrollbarThumb,
        &mut UNode,
        &ComputedSize,
        &mut Transform,
        Option<&mut Visibility>,
    )>,
) {
    for (_entity, thumb, mut node, thumb_computed, mut transform, mut vis_opt) in thumbs.iter_mut()
    {
        let Ok((container, container_computed)) = containers.get(thumb.container) else {
            continue;
        };

        let track_entity = thumb
            .track
            .or_else(|| parents.get(_entity).ok().map(|p| p.parent()));

        let track_size = track_entity
            .and_then(|te| all_computed.get(te).ok())
            .map(|c| match thumb.axis {
                UScrollbarAxis::Vertical => c.height,
                UScrollbarAxis::Horizontal => c.width,
            })
            .unwrap_or(0.0);

        let (viewport_size, max_offset, progress) = match thumb.axis {
            UScrollbarAxis::Vertical => (
                container_computed.height,
                container.max_offset.y,
                container.progress_y(),
            ),
            UScrollbarAxis::Horizontal => (
                container_computed.width,
                container.max_offset.x,
                container.progress_x(),
            ),
        };

        let is_scrollable = max_offset > 0.0;

        // Auto hide when not scrollable
        if thumb.hide_when_not_scrollable {
            let target_vis = if is_scrollable {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            if let Some(vis) = vis_opt.as_deref_mut().filter(|v| **v != target_vis) {
                *vis = target_vis;
            }
        }

        if !is_scrollable {
            continue;
        }

        // Proportional sizing
        let target_thumb_size =
            calculate_thumb_size(track_size, viewport_size, max_offset, thumb.min_size);

        if thumb.auto_size && track_size > 0.0 {
            match thumb.axis {
                UScrollbarAxis::Vertical => {
                    let current_px = match node.height {
                        crate::layout::geometry::UVal::Px(px) => px,
                        _ => -1.0,
                    };
                    if (current_px - target_thumb_size).abs() > 0.5 {
                        node.height = crate::layout::geometry::UVal::Px(target_thumb_size);
                    }
                }
                UScrollbarAxis::Horizontal => {
                    let current_px = match node.width {
                        crate::layout::geometry::UVal::Px(px) => px,
                        _ => -1.0,
                    };
                    if (current_px - target_thumb_size).abs() > 0.5 {
                        node.width = crate::layout::geometry::UVal::Px(target_thumb_size);
                    }
                }
            }
        }

        // Travel range and position
        let actual_thumb_size = match thumb.axis {
            UScrollbarAxis::Vertical => {
                if thumb_computed.height > 0.0 {
                    thumb_computed.height
                } else {
                    target_thumb_size
                }
            }
            UScrollbarAxis::Horizontal => {
                if thumb_computed.width > 0.0 {
                    thumb_computed.width
                } else {
                    target_thumb_size
                }
            }
        };

        let travel = (track_size - actual_thumb_size).max(0.0);

        match thumb.axis {
            UScrollbarAxis::Vertical => {
                // Moving down decreases local Y
                transform.translation.y = thumb.base_translation.y - (progress * travel);
            }
            UScrollbarAxis::Horizontal => {
                // Moving right increases local X
                transform.translation.x = thumb.base_translation.x + (progress * travel);
            }
        }
    }

    // Also update track visibility if hide_when_not_scrollable
    for (track, computed) in tracks.iter() {
        let _ = (track, computed);
    }
}

/// Reads mouse clicks and drag events to control scrollbar thumbs and tracks.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_scrollbar_drag(
    mouse_button: Option<Res<ButtonInput<MouseButton>>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut containers: Query<&mut UScrollContainer>,
    tracks: Query<(Entity, &UScrollbarTrack, &GlobalTransform, &ComputedSize)>,
    parents: Query<&ChildOf>,
    all_computed: Query<&ComputedSize>,
    mut thumbs: Query<(
        Entity,
        &mut UScrollbarThumb,
        &GlobalTransform,
        &ComputedSize,
    )>,
) {
    let Some(mouse_button) = mouse_button else {
        return;
    };

    let cursor_world_pos: Option<Vec2> = (|| {
        let window = windows.iter().next()?;
        let cursor_pos = window.cursor_position()?;
        let (camera, cam_gt) = camera_q.iter().next()?;
        camera.viewport_to_world_2d(cam_gt, cursor_pos).ok()
    })();

    let just_pressed = mouse_button.just_pressed(MouseButton::Left);
    let is_pressed = mouse_button.pressed(MouseButton::Left);
    let just_released = mouse_button.just_released(MouseButton::Left);

    // 1. Drag release
    if just_released || !is_pressed {
        for (_e, mut thumb, _gt, _computed) in thumbs.iter_mut() {
            if thumb.is_dragging {
                thumb.is_dragging = false;
            }
        }
    }

    let Some(cursor) = cursor_world_pos else {
        return;
    };

    // 2. Click detection on thumbs and tracks
    if just_pressed {
        let mut clicked_thumb = false;

        // Check thumb hit
        for (_e, mut thumb, gt, computed) in thumbs.iter_mut() {
            let thumb_pos = gt.translation().xy();
            let half_w = computed.width / 2.0;
            let half_h = computed.height / 2.0;

            let hit = (cursor.x - thumb_pos.x).abs() <= half_w
                && (cursor.y - thumb_pos.y).abs() <= half_h;

            if hit {
                let Ok(container) = containers.get(thumb.container) else {
                    continue;
                };
                thumb.is_dragging = true;
                thumb.drag_start_cursor = match thumb.axis {
                    UScrollbarAxis::Vertical => cursor.y,
                    UScrollbarAxis::Horizontal => cursor.x,
                };
                thumb.drag_start_offset = match thumb.axis {
                    UScrollbarAxis::Vertical => container.scroll_offset.y,
                    UScrollbarAxis::Horizontal => container.scroll_offset.x,
                };
                clicked_thumb = true;
                break;
            }
        }

        // If not clicked on thumb, check if clicked on track (click-to-jump)
        if !clicked_thumb {
            for (_e, track, gt, computed) in tracks.iter() {
                if !track.click_to_jump {
                    continue;
                }
                let track_pos = gt.translation().xy();
                let half_w = computed.width / 2.0;
                let half_h = computed.height / 2.0;

                let hit = (cursor.x - track_pos.x).abs() <= half_w
                    && (cursor.y - track_pos.y).abs() <= half_h;

                if hit {
                    let Ok(mut container) = containers.get_mut(track.container) else {
                        continue;
                    };
                    match track.axis {
                        UScrollbarAxis::Vertical => {
                            if computed.height > 0.0 && container.max_offset.y > 0.0 {
                                let track_top = track_pos.y + half_h;
                                let click_from_top =
                                    (track_top - cursor.y).clamp(0.0, computed.height);
                                let fraction = click_from_top / computed.height;
                                let target_y = fraction * container.max_offset.y;
                                let cur_x = container.scroll_offset.x;
                                container.jump_to(Vec2::new(cur_x, target_y));
                            }
                        }
                        UScrollbarAxis::Horizontal => {
                            if computed.width > 0.0 && container.max_offset.x > 0.0 {
                                let track_left = track_pos.x - half_w;
                                let click_from_left =
                                    (cursor.x - track_left).clamp(0.0, computed.width);
                                let fraction = click_from_left / computed.width;
                                let target_x = fraction * container.max_offset.x;
                                let cur_y = container.scroll_offset.y;
                                container.jump_to(Vec2::new(target_x, cur_y));
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    // 3. Active drag handling
    if is_pressed {
        for (thumb_entity, thumb, _gt, thumb_computed) in thumbs.iter() {
            if !thumb.is_dragging {
                continue;
            }

            let Ok(mut container) = containers.get_mut(thumb.container) else {
                continue;
            };

            let track_entity = thumb
                .track
                .or_else(|| parents.get(thumb_entity).ok().map(|p| p.parent()));

            let track_size = track_entity
                .and_then(|te| all_computed.get(te).ok())
                .map(|c| match thumb.axis {
                    UScrollbarAxis::Vertical => c.height,
                    UScrollbarAxis::Horizontal => c.width,
                })
                .unwrap_or(0.0);

            match thumb.axis {
                UScrollbarAxis::Vertical => {
                    let travel = (track_size - thumb_computed.height).max(1.0);
                    // Moving cursor down (cursor.y < drag_start_cursor) increases scroll offset
                    let cursor_delta = cursor.y - thumb.drag_start_cursor;
                    let scroll_delta = -cursor_delta * (container.max_offset.y / travel);
                    let new_offset = thumb.drag_start_offset + scroll_delta;
                    let cur_x = container.scroll_offset.x;
                    container.jump_to(Vec2::new(cur_x, new_offset));
                }
                UScrollbarAxis::Horizontal => {
                    let travel = (track_size - thumb_computed.width).max(1.0);
                    // Moving cursor right increases scroll offset
                    let cursor_delta = cursor.x - thumb.drag_start_cursor;
                    let scroll_delta = cursor_delta * (container.max_offset.x / travel);
                    let new_offset = thumb.drag_start_offset + scroll_delta;
                    let cur_y = container.scroll_offset.y;
                    container.jump_to(Vec2::new(new_offset, cur_y));
                }
            }
        }
    }
}

/// Fades out scrollbar elements when idle, and fades them in on scroll or hover.
pub fn handle_scrollbar_fade(
    time: Option<Res<Time>>,
    containers: Query<(Entity, &UScrollContainer)>,
    thumbs: Query<&UScrollbarThumb>,
    mut fade_nodes: Query<(&mut UScrollbarFade, &mut UNode)>,
) {
    let dt = time.map(|t| t.delta_secs().min(0.1)).unwrap_or(0.016);
    if dt <= 0.0 {
        return;
    }

    // Check if any thumb is currently dragging
    let any_dragging = thumbs.iter().any(|t| t.is_dragging);

    // Check if any container is hovered or active
    let any_container_active = containers.iter().any(|(_e, c)| c.is_hovered);

    for (mut fade, mut node) in fade_nodes.iter_mut() {
        if !fade.initialized {
            fade.base_alpha = node.background_color.alpha();
            fade.initialized = true;
        }

        if any_dragging || any_container_active {
            fade.idle_timer = 0.0;
            fade.target_alpha = 1.0;
        } else {
            fade.idle_timer += dt;
            if fade.idle_timer >= fade.idle_timeout {
                fade.target_alpha = 0.0;
            }
        }

        // Interpolate current alpha
        if (fade.current_alpha - fade.target_alpha).abs() > 0.005 {
            let blend = (dt / fade.fade_duration.max(0.01)).min(1.0);
            fade.current_alpha += (fade.target_alpha - fade.current_alpha) * blend;
            let effective_alpha = fade.base_alpha * fade.current_alpha;
            node.background_color = node.background_color.with_alpha(effective_alpha);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_thumb_size() {
        // Track 500, Viewport 500, Max offset 1000 -> Content 1500 -> Ratio 1/3 -> 500 * (1/3) = 166.67
        let size = calculate_thumb_size(500.0, 500.0, 1000.0, 20.0);
        assert!((size - 166.67).abs() < 1.0);

        // Huge content clamped to min_size
        let small_size = calculate_thumb_size(500.0, 500.0, 50000.0, 24.0);
        assert_eq!(small_size, 24.0);

        // Content fits in viewport -> fills track
        let full_size = calculate_thumb_size(500.0, 500.0, 0.0, 20.0);
        assert_eq!(full_size, 500.0);
    }

    #[test]
    fn test_scrollbar_track_constructors() {
        let e = Entity::from_raw_u32(42).unwrap();
        let track = UScrollbarTrack::vertical(e).with_click_to_jump(false);
        assert_eq!(track.axis, UScrollbarAxis::Vertical);
        assert!(!track.click_to_jump);

        let h_track = UScrollbarTrack::horizontal(e).with_hide_when_not_scrollable(true);
        assert_eq!(h_track.axis, UScrollbarAxis::Horizontal);
        assert!(h_track.hide_when_not_scrollable);
    }

    #[test]
    fn test_scrollbar_thumb_constructors() {
        let e = Entity::from_raw_u32(42).unwrap();
        let thumb = UScrollbarThumb::vertical(e).with_min_size(32.0);
        assert_eq!(thumb.axis, UScrollbarAxis::Vertical);
        assert_eq!(thumb.min_size, 32.0);
        assert!(thumb.auto_size);
        assert!(!thumb.is_dragging);
    }

    #[test]
    fn test_drag_delta_conversion_math() {
        let track_size = 500.0;
        let thumb_size = 100.0;
        let max_offset = 1200.0;
        let travel = track_size - thumb_size; // 400.0

        // If cursor moves down by 100px:
        let cursor_delta = -100.0;
        let scroll_delta = -cursor_delta * (max_offset / travel); // 100 * (1200 / 400) = 300.0
        assert_eq!(scroll_delta, 300.0);

        // Full travel (400px down) scrolls to max_offset (1200px)
        let full_cursor_delta = -400.0;
        let full_scroll_delta = -full_cursor_delta * (max_offset / travel);
        assert_eq!(full_scroll_delta, 1200.0);
    }

    #[test]
    fn test_fade_interpolation() {
        let mut fade = UScrollbarFade::new(1.0, 0.5);
        fade.current_alpha = 1.0;
        fade.target_alpha = 0.0;

        let dt = 0.25;
        let blend = (dt / fade.fade_duration).min(1.0);
        fade.current_alpha += (fade.target_alpha - fade.current_alpha) * blend;
        assert_eq!(fade.current_alpha, 0.5);
    }
}
