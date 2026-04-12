use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use univis_ui_engine::layout::geometry::UVal;
use univis_ui_engine::layout::query::ComputedSize;
use univis_ui_engine::layout::univis_node::{UPositionType, USelf};
use univis_ui_interaction::interaction::feedback::UInteraction;

/// Registers the scroll container widget.
pub struct UnivisScrollViewPlugin;

impl Plugin for UnivisScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UScrollContainer>()
            .add_systems(Update, scroll_interaction_system);
    }
}

/// Adds mouse-wheel driven scrolling to a container node.
///
/// The entity is expected to own exactly one scrollable child whose local
/// offsets are adjusted as overflow changes.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct UScrollContainer {
    pub scroll_speed: f32,
    pub vertical: bool,
    pub horizontal: bool,
    // Current internal offset applied to the scroll content.
    pub offset: Vec2,
}

impl UScrollContainer {
    /// Creates a vertical scroll container with default speed.
    pub fn new() -> Self {
        Self {
            scroll_speed: 30.0,
            vertical: true,
            horizontal: false,
            offset: Vec2::ZERO,
        }
    }
}

/// Internal system that applies wheel input to hovered scroll containers.
#[doc(hidden)]
pub fn scroll_interaction_system(
    // 1. Read wheel input.
    mut mouse_wheel: MessageReader<MouseWheel>,

    // 2. Query scroll containers.
    // Requirement: the entity must own both `UScrollContainer` and `UInteraction`.
    mut containers: Query<(
        &UInteraction, // Use interaction state instead of manual cursor math.
        &mut UScrollContainer,
        &ComputedSize,
        &Children,
    )>,

    // 3. Query the content child so its local offset can be updated.
    mut content_query: Query<(&mut USelf, &ComputedSize)>,
) {
    // Accumulate wheel motion across the frame.
    let mut scroll_delta = Vec2::ZERO;
    for ev in mouse_wheel.read() {
        scroll_delta.y += ev.y;
        scroll_delta.x += ev.x;
    }

    // Exit early when there was no wheel movement.
    if scroll_delta == Vec2::ZERO {
        return;
    }

    for (interaction, mut container, size, children) in containers.iter_mut() {
        // Use the resolved interaction state instead of recomputing hover tests here.
        if *interaction != UInteraction::Hovered {
            continue;
        }

        // The first child is treated as the scrollable content root.
        let Some(&content_entity) = children.first() else {
            continue;
        };

        let Ok((mut uself, content_size)) = content_query.get_mut(content_entity) else {
            continue;
        };

        let speed = container.scroll_speed.max(0.0);
        let overflow_y = (content_size.height - size.height).max(0.0);
        let overflow_x = (content_size.width - size.width).max(0.0);

        if container.vertical {
            container.offset.y =
                clamp_scroll_offset(container.offset.y, scroll_delta.y * speed, overflow_y);
            uself.top = UVal::Px(container.offset.y);
        }

        if container.horizontal {
            container.offset.x =
                clamp_scroll_offset(container.offset.x, scroll_delta.x * speed, overflow_x);
            uself.left = UVal::Px(container.offset.x);
            uself.position_type = UPositionType::Relative;
        }
    }
}

fn clamp_scroll_offset(current: f32, delta: f32, overflow: f32) -> f32 {
    if overflow <= 0.0 {
        0.0
    } else {
        (current + delta).clamp(-overflow, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_scroll_offset_caps_to_range() {
        assert_eq!(clamp_scroll_offset(0.0, -30.0, 120.0), -30.0);
        assert_eq!(clamp_scroll_offset(-100.0, -80.0, 120.0), -120.0);
        assert_eq!(clamp_scroll_offset(-20.0, 60.0, 120.0), 0.0);
    }

    #[test]
    fn clamp_scroll_offset_returns_zero_without_overflow() {
        assert_eq!(clamp_scroll_offset(-50.0, -20.0, 0.0), 0.0);
        assert_eq!(clamp_scroll_offset(10.0, 15.0, -5.0), 0.0);
    }
}
