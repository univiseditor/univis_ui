//! Logical UI units, spacing helpers, and box constraints used by the layout solver.

use bevy::prelude::*;

pub use univis_layout::geometry::*;

// --- ComputedSize ---

/// Component that holds the final calculated size and position of a node.
///
/// This is the result of the layout solver. It is updated automatically during
/// the `downward_solve_pass`.
#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct ComputedSize {
    /// The calculated width of the node in logical UI units.
    pub width: f32,
    /// The calculated height of the node in logical UI units.
    pub height: f32,
    /// The local position of the node relative to its parent's center in logical UI units.
    pub local_pos: Vec2,
}

impl ComputedSize {
    /// Returns the calculated dimensions as a `Vec2`.
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}
