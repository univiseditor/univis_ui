//! Physics-based layout transitions for Univis UI nodes.
//!
//! Provides [`UTransition`](crate::layout::transition::UTransition) to configure smooth, spring-driven or easing
//! transitions when node positions reflow during layout updates (such as
//! window resizing or responsive grid reorganization).

use bevy::prelude::*;

/// Configures smooth spring transitions for node layout movements.
///
/// When attached to an entity with a [`crate::layout::univis_node::UNode`],
/// any changes to the node's layout-solved translation glide smoothly into
/// position instead of snapping instantly in one frame.
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UTransition {
    /// Spring stiffness / angular frequency response (higher = snappier, default 180.0).
    pub stiffness: f32,
    /// Damping ratio (1.0 = critically damped / no overshoot, < 1.0 = bouncy).
    pub damping: f32,
}

impl Default for UTransition {
    fn default() -> Self {
        Self {
            stiffness: 180.0,
            damping: 1.0,
        }
    }
}

impl UTransition {
    /// Critically damped spring transition (no overshoot).
    pub fn spring(stiffness: f32) -> Self {
        Self {
            stiffness,
            damping: 1.0,
        }
    }

    /// Spring with a custom damping ratio (e.g. 0.7 for slight playful bounce, 1.0 for smooth).
    pub fn bouncy(stiffness: f32, damping: f32) -> Self {
        Self { stiffness, damping }
    }

    /// Very snappy, responsive layout glide.
    pub fn snappy() -> Self {
        Self {
            stiffness: 260.0,
            damping: 1.0,
        }
    }

    /// Gentle, fluid layout glide.
    pub fn smooth() -> Self {
        Self {
            stiffness: 110.0,
            damping: 1.0,
        }
    }
}

/// Runtime animation state tracking the spring velocity and target layout position.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UTransitionState {
    /// Target translation computed by the downward layout solver.
    pub target_translation: Vec2,
    /// Current velocity vector of the spring.
    pub velocity: Vec2,
    /// Whether the initial position has been established.
    pub initialized: bool,
}

/// Automatically inserts [`UTransitionState`] when [`UTransition`] is added to an entity.
pub fn init_transition_states(
    mut commands: Commands,
    query: Query<Entity, (With<UTransition>, Without<UTransitionState>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(UTransitionState::default());
    }
}

/// Advances the critically damped spring simulation for active layout transitions.
pub fn apply_layout_transitions(
    time: Option<Res<Time>>,
    mut query: Query<(&UTransition, &mut UTransitionState, &mut Transform)>,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }

    for (transition, mut state, mut transform) in query.iter_mut() {
        if !state.initialized {
            continue;
        }

        let current = transform.translation.xy();
        let target = state.target_translation;
        let diff = target - current;

        // Sleep when settled within sub-pixel threshold
        if diff.length_squared() < 0.0001 && state.velocity.length_squared() < 0.0001 {
            if transform.translation.x != target.x || transform.translation.y != target.y {
                transform.translation.x = target.x;
                transform.translation.y = target.y;
            }
            state.velocity = Vec2::ZERO;
            continue;
        }

        let omega = transition.stiffness.sqrt();
        let spring_force = diff * transition.stiffness;
        let damping_force = state.velocity * (2.0 * transition.damping * omega);
        let accel = spring_force - damping_force;

        state.velocity += accel * dt;
        let next_pos = current + state.velocity * dt;

        transform.translation.x = next_pos.x;
        transform.translation.y = next_pos.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_constructors() {
        let def = UTransition::default();
        assert_eq!(def.stiffness, 180.0);
        assert_eq!(def.damping, 1.0);

        let snappy = UTransition::snappy();
        assert_eq!(snappy.stiffness, 260.0);
        assert_eq!(snappy.damping, 1.0);

        let bouncy = UTransition::bouncy(200.0, 0.7);
        assert_eq!(bouncy.stiffness, 200.0);
        assert_eq!(bouncy.damping, 0.7);
    }

    #[test]
    fn test_spring_converges_to_target() {
        let transition = UTransition::default();
        let mut state = UTransitionState {
            target_translation: Vec2::new(100.0, 50.0),
            velocity: Vec2::ZERO,
            initialized: true,
        };
        let mut pos = Vec2::ZERO;

        let dt = 1.0 / 60.0;
        for _ in 0..120 {
            let diff = state.target_translation - pos;
            let omega = transition.stiffness.sqrt();
            let spring_force = diff * transition.stiffness;
            let damping_force = state.velocity * (2.0 * transition.damping * omega);
            let accel = spring_force - damping_force;

            state.velocity += accel * dt;
            pos += state.velocity * dt;
        }

        assert!(
            (pos.x - 100.0).abs() < 0.05,
            "Spring x failed to converge: pos.x = {}",
            pos.x
        );
        assert!(
            (pos.y - 50.0).abs() < 0.05,
            "Spring y failed to converge: pos.y = {}",
            pos.y
        );
    }
}
