//! Logical UI units, spacing helpers, and box constraints used by the layout solver.

use crate::internal_prelude::*;
use bevy::prelude::*;

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

// --- 1. Basic Data Structures ---

/// Defines dimension values for width, height, or position.
#[derive(Reflect, Clone, Copy, Debug, PartialEq)]
pub enum UVal {
    /// A fixed value in logical UI units.
    Px(f32),
    /// A percentage of the parent's size (0.0 to 1.0).
    Percent(f32),
    /// Legacy intrinsic-content mode.
    ///
    /// This is kept for backward compatibility and behaves like `MaxContent`.
    Content,
    /// Sizes the element to the smallest intrinsic content width/height that still fits its contents.
    MinContent,
    /// Sizes the element to its full intrinsic content width/height.
    MaxContent,
    /// Contextual sizing.
    ///
    /// `Auto` uses intrinsic measurement as a fallback, but layout algorithms may
    /// stretch or otherwise reinterpret it when the surrounding context calls for it.
    Auto,
    /// Flex grow factor. Takes a share of the remaining space.
    Flex(f32),
}

impl Default for UVal {
    fn default() -> Self {
        Self::Px(0.0)
    }
}

impl UVal {
    /// Resolves the value against a parent/base scalar when possible.
    pub fn resolve(&self, base: f32) -> Option<f32> {
        match *self {
            UVal::Px(v) => Some(v),
            UVal::Percent(p) => Some(p * base),
            UVal::Content | UVal::MinContent | UVal::MaxContent | UVal::Auto | UVal::Flex(_) => {
                None
            }
        }
    }

    /// Resolves the value against a parent/base scalar, returning `0.0` for
    /// non-resolvable modes such as `Content`, `Auto`, and `Flex`.
    pub fn resolve_or_zero(&self, base: f32) -> f32 {
        self.resolve(base).unwrap_or(0.0)
    }

    /// Returns `true` when this value depends on intrinsic content measurement.
    pub fn uses_intrinsic_measurement(&self) -> bool {
        matches!(
            self,
            UVal::Content | UVal::MinContent | UVal::MaxContent | UVal::Auto
        )
    }
}

/// Defines spacing (Padding or Margin) for the four sides of a box.
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq)]
pub struct USides {
    /// Space on the left side.
    pub left: f32,
    /// Space on the right side.
    pub right: f32,
    /// Space on the top side.
    pub top: f32,
    /// Space on the bottom side.
    pub bottom: f32,
}

impl USides {
    /// Creates equal spacing for all sides.
    ///
    /// # Example
    /// `padding: USides::all(10.0)`
    pub fn all(val: f32) -> Self {
        Self {
            left: val,
            right: val,
            top: val,
            bottom: val,
        }
    }

    /// Creates spacing for horizontal and vertical axes separately.
    ///
    /// `row`: Applied to Left/Right.
    /// `column`: Applied to Top/Bottom.
    pub fn axes(row: f32, column: f32) -> Self {
        Self {
            left: row,
            right: row,
            top: column,
            bottom: column,
        }
    }

    /// Creates spacing for the horizontal axis (Left + Right) only.
    pub fn row(val: f32) -> Self {
        Self {
            left: val,
            right: val,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates spacing for the vertical axis (Top + Bottom) only.
    pub fn column(val: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: val,
            bottom: val,
        }
    }

    /// Creates spacing for the bottom side only.
    pub fn bottom(val: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: val,
        }
    }

    /// Creates spacing for the top side only.
    pub fn top(val: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: val,
            bottom: 0.0,
        }
    }

    /// Creates spacing for the left side only.
    pub fn left(val: f32) -> Self {
        Self {
            left: val,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates spacing for the right side only.
    pub fn right(val: f32) -> Self {
        Self {
            left: 0.0,
            right: val,
            top: 0.0,
            bottom: 0.0,
        }
    }

    // --- Helper Calculations ---

    /// Returns the sum of horizontal spacing (Left + Right).
    pub fn width_sum(&self) -> f32 {
        self.left + self.right
    }

    /// Returns the sum of vertical spacing (Top + Bottom).
    pub fn height_sum(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Defines the radius for each corner of a rounded rectangle independently.
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq)]
pub struct UCornerRadius {
    /// Radius of the top-left corner.
    pub top_left: f32,
    /// Radius of the top-right corner.
    pub top_right: f32,
    /// Radius of the bottom-right corner.
    pub bottom_right: f32,
    /// Radius of the bottom-left corner.
    pub bottom_left: f32,
}

impl UCornerRadius {
    /// Sets all corners to the same radius value.
    pub fn all(val: f32) -> Self {
        Self {
            top_left: val,
            top_right: val,
            bottom_right: val,
            bottom_left: val,
        }
    }
    /// Sets only the top corners (Top-Left, Top-Right). Useful for tabs.
    pub fn top(val: f32) -> Self {
        Self {
            top_left: val,
            top_right: val,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }

    /// Sets only the bottom corners.
    pub fn bottom(val: f32) -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: val,
            bottom_left: val,
        }
    }
}

/// Padding totals expressed along the current main/cross axis pair.
#[derive(Debug, Clone, Copy, Default)]
pub struct AxisPadding {
    /// Sum of padding on the main layout axis.
    pub main: f32,
    /// Sum of padding on the cross layout axis.
    pub cross: f32,
}

/// Helper for converting between world axes and main/cross layout axes.
///
/// This lets the solver share one implementation for row and column layouts.
pub struct AxisHelper {
    /// The resolved layout direction axis.
    axis: UFlexDirection,
}

impl AxisHelper {
    /// Creates a new `AxisHelper` from the given layout flex direction.
    pub fn new(axis: UFlexDirection) -> Self {
        Self { axis }
    }

    /// Returns `true` when the main axis direction is reversed.
    pub fn is_reverse(&self) -> bool {
        matches!(
            self.axis,
            UFlexDirection::RowReverse | UFlexDirection::ColumnReverse
        )
    }

    /// Returns `true` when the main axis is horizontal.
    pub fn is_row(&self) -> bool {
        matches!(self.axis, UFlexDirection::Row | UFlexDirection::RowReverse)
    }

    /// Converts a world-space size `Vec2` into a `(main, cross)` tuple.
    pub fn from_world(&self, size: Vec2) -> (f32, f32) {
        if self.is_row() {
            (size.x, size.y) // Main=Width
        } else {
            (size.y, size.x) // Main=Height
        }
    }

    /// Converts a `(main, cross)` size into a world-space `Vec2`.
    pub fn to_world(&self, main: f32, cross: f32) -> Vec2 {
        if self.is_row() {
            Vec2::new(main, cross)
        } else {
            Vec2::new(cross, main)
        }
    }

    /// Extracts box constraints oriented along the main and cross axes.
    ///
    /// Returns a tuple of `(min_main, max_main, min_cross, max_cross)`.
    pub fn extract_constraints(&self, constraints: BoxConstraints) -> (f32, f32, f32, f32) {
        if self.is_row() {
            (
                constraints.min_width,
                constraints.max_width,
                constraints.min_height,
                constraints.max_height,
            )
        } else {
            (
                constraints.min_height,
                constraints.max_height,
                constraints.min_width,
                constraints.max_width,
            )
        }
    }

    /// Extracts padding resolved for the main and cross axes.
    pub fn extract_padding(&self, padding: USides) -> AxisPadding {
        if self.is_row() {
            AxisPadding {
                main: padding.left + padding.right,
                cross: padding.top + padding.bottom,
            }
        } else {
            AxisPadding {
                main: padding.top + padding.bottom,
                cross: padding.left + padding.right,
            }
        }
    }

    /// Extracts layout margin into a tuple ordered as `(main_start, main_end, cross_start, cross_end)`.
    ///
    /// Margins follow the item even when the axis flips, so logical ordering is still enough here.
    pub fn extract_margin_sides(&self, margin: USides) -> (f32, f32, f32, f32) {
        if self.is_row() {
            (margin.left, margin.right, margin.top, margin.bottom)
        } else {
            (margin.top, margin.bottom, margin.left, margin.right)
        }
    }

    /// Extracts the main axis sizing specification from a general `SolverSpec`.
    /// Returns `(mode, val, flex)`.
    pub fn get_main_spec(&self, spec: &SolverSpec) -> (SolverSizeMode, f32, f32) {
        if self.is_row() {
            (spec.width_mode, spec.width_val, spec.width_flex)
        } else {
            (spec.height_mode, spec.height_val, spec.height_flex)
        }
    }

    /// Extracts the cross axis sizing specification from a general `SolverSpec`.
    /// Returns `(mode, val, flex)`.
    pub fn get_cross_spec(&self, spec: &SolverSpec) -> (SolverSizeMode, f32, f32) {
        if self.is_row() {
            (spec.height_mode, spec.height_val, spec.height_flex)
        } else {
            (spec.width_mode, spec.width_val, spec.width_flex)
        }
    }
}

/// Minimum and maximum size constraints passed from parent to child.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxConstraints {
    /// The minimum allowed width.
    pub min_width: f32,
    /// The maximum allowed width.
    pub max_width: f32,
    /// The minimum allowed height.
    pub min_height: f32,
    /// The maximum allowed height.
    pub max_height: f32,
}

impl BoxConstraints {
    /// Returns the maximum allowed size as Vec2.
    pub fn max_size(&self) -> Vec2 {
        Vec2::new(self.max_width, self.max_height)
    }
    /// Returns the minimum allowed size as Vec2.
    pub fn min_size(&self) -> Vec2 {
        Vec2::new(self.min_width, self.min_height)
    }

    /// Creates "tight" constraints, forcing the child to be exactly `size`.
    pub fn tight(size: Vec2) -> Self {
        Self {
            min_width: size.x,
            max_width: size.x,
            min_height: size.y,
            max_height: size.y,
        }
    }

    /// Creates "loose" constraints, allowing the child to be anywhere from 0 to `size`.
    pub fn loose(size: Vec2) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.x,
            min_height: 0.0,
            max_height: size.y,
        }
    }
}
