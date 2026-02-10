use bevy::prelude::*;
use crate::prelude::*;

// --- ComputedSize ---

/// Component that holds the final calculated size and position of a node.
///
/// This is the result of the layout solver. It is updated automatically during
/// the `downward_solve_pass`.
#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct ComputedSize {
    /// The calculated width of the node in logical pixels.
    pub width: f32,
    /// The calculated height of the node in logical pixels.
    pub height: f32,
    /// The local position of the node relative to its parent's center.
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
    /// A fixed value in pixels.
    Px(f32),
    /// A percentage of the parent's size (0.0 to 1.0).
    Percent(f32),
    /// Sizes the element based on its content/children.
    Content,
    /// Automatic sizing (fills remaining space or adapts to context).
    Auto,
    /// Flex grow factor. Takes a share of the remaining space.
    Flex(f32)
}

impl Default for UVal {
    fn default() -> Self { Self::Px(0.0) }
}

// impl UVal {
//     fn get_val(&self) -> f32 {
//         match &self {
//             UVal::Flex(f)
//         }
//     }
// }

/// Defines spacing (Padding or Margin) for the four sides of a box.
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq)]
pub struct USides {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl USides {
    /// Creates equal spacing for all sides.
    ///
    /// # Example
    /// `padding: USides::all(10.0)`
    pub fn all(val: f32) -> Self {
        Self { left: val, right: val, top: val, bottom: val }
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
            bottom: column 
        }
    }

    /// Creates spacing for the horizontal axis (Left + Right) only.
    pub fn row(val: f32) -> Self {
        Self { 
            left: val, 
            right: val, 
            top: 0.0, 
            bottom: 0.0 
        }
    }

    /// Creates spacing for the vertical axis (Top + Bottom) only.
    pub fn column(val: f32) -> Self {
        Self { 
            left: 0.0, 
            right: 0.0, 
            top: val, 
            bottom: val 
        }
    }

    /// Creates spacing for the bottom side only.
    pub fn bottom(val: f32) -> Self {
        Self { 
            left: 0.0, 
            right: 0.0, 
            top: 0.0, 
            bottom: val 
        }
    }

    /// Creates spacing for the top side only.
    pub fn top(val: f32) -> Self {
        Self { 
            left: 0.0, 
            right: 0.0, 
            top: val, 
            bottom: 0.0 
        }
    }

    /// Creates spacing for the left side only.
    pub fn left(val: f32) -> Self {
        Self { 
            left: val, 
            right: 0.0, 
            top: 0.0, 
            bottom: 0.0 
        }
    }

    /// Creates spacing for the right side only.
    pub fn right(val: f32) -> Self {
        Self { 
            left: 0.0, 
            right: val, 
            top: 0.0, 
            bottom: 0.0 
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
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl UCornerRadius {
    /// Sets all corners to the same radius value.
    pub fn all(val: f32) -> Self {
        Self { top_left: val, top_right: val, bottom_right: val, bottom_left: val }
    }
    /// Sets only the top corners (Top-Left, Top-Right). Useful for tabs.
    pub fn top(val: f32) -> Self {
        Self { top_left: val, top_right: val, bottom_right: 0.0, bottom_left: 0.0 }
    }

    /// Sets only the bottom corners.
    pub fn bottom(val: f32) -> Self {
        Self { top_left: 0.0, top_right: 0.0, bottom_right: val, bottom_left: val }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AxisPadding {
    pub main: f32,  // Sum of padding on the main axis
    pub cross: f32, // Sum of padding on the cross axis
}


/// A helper struct to abstract Main/Cross axis logic.
/// Allows writing a single algorithm for both Row and Column directions.
pub struct AxisHelper {
    axis: UFlexDirection,
}

impl AxisHelper {
    pub fn new(axis: UFlexDirection) -> Self {
        Self { axis }
    }

    /// هل الاتجاه معكوس؟
    pub fn is_reverse(&self) -> bool {
        matches!(self.axis, UFlexDirection::RowReverse | UFlexDirection::ColumnReverse)
    }

    /// هل هو صف (أفقي)؟
    pub fn is_row(&self) -> bool {
        matches!(self.axis, UFlexDirection::Row | UFlexDirection::RowReverse)
    }

    pub fn from_world(&self, size: Vec2) -> (f32, f32) {
        if self.is_row() {
            (size.x, size.y) // Main=Width
        } else {
            (size.y, size.x) // Main=Height
        }
    }

    pub fn to_world(&self, main: f32, cross: f32) -> Vec2 {
        if self.is_row() {
            Vec2::new(main, cross)
        } else {
            Vec2::new(cross, main)
        }
    }

    pub fn extract_constraints(&self, constraints: BoxConstraints) -> (f32, f32, f32, f32) {
        if self.is_row() {
            (constraints.min_width, constraints.max_width, constraints.min_height, constraints.max_height)
        } else {
            (constraints.min_height, constraints.max_height, constraints.min_width, constraints.max_width)
        }
    }

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

    // نحتاج أيضاً لقلب الهوامش إذا كان الاتجاه معكوساً، لكن في Flexbox
    // الهوامش تتبع العنصر، لذا الترتيب المنطقي يكفي.
    pub fn extract_margin_sides(&self, margin: USides) -> (f32, f32, f32, f32) {
        if self.is_row() {
            (margin.left, margin.right, margin.top, margin.bottom)
        } else {
            (margin.top, margin.bottom, margin.left, margin.right)
        }
    }
    
    // ... باقي الدوال (get_main_spec, get_cross_spec) تستخدم نفس منطق is_row()
    pub fn get_main_spec(&self, spec: &SolverSpec) -> (SolverSizeMode, f32, f32) {
        if self.is_row() {
            (spec.width_mode, spec.width_val, spec.width_flex)
        } else {
            (spec.height_mode, spec.height_val, spec.height_flex)
        }
    }

    pub fn get_cross_spec(&self, spec: &SolverSpec) -> (SolverSizeMode, f32, f32) {
        if self.is_row() {
            (spec.height_mode, spec.height_val, spec.height_flex)
        } else {
            (spec.width_mode, spec.width_val, spec.width_flex)
        }
    }
}

/// Layout constraints passed down from parent to child.
/// Defines the minimum and maximum allowed size for a child node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
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