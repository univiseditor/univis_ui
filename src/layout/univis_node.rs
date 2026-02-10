use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisNodePlugin;

impl Plugin for UnivisNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UVal>()
            .register_type::<ULayout>()
            .register_type::<UNode>()
            .register_type::<ComputedSize>();
    }
}

#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UShapeMode {
    #[default]
    Round,
    Cut, 
}

/// The core component for any UI node.
/// Defines sizing, spacing, appearance (color/radius), and layout behavior.
#[derive(Component, Clone, Reflect)]
#[require(Transform, Visibility, ComputedSize, ULayout, IntrinsicSize)] 
pub struct UNode {
    /// Preferred width of the node.
    pub width: UVal,
    /// Preferred height of the node.
    pub height: UVal,
    
    /// Inner spacing (affects children placement).
    pub padding: USides,
    /// Outer spacing (affects placement relative to parent/siblings).
    pub margin: USides,
    
    /// Background color of the node.
    pub background_color: Color,
    /// Corner radius for rounded rectangles (independent corners).
    pub border_radius: UCornerRadius,

    pub shape_mode: UShapeMode, 
}

impl Default for UNode {
    fn default() -> Self {
        Self {
            width: UVal::Content,
            height: UVal::Content,
            padding: USides::default(),
            margin: USides::default(),
            background_color: Color::NONE,
            border_radius: UCornerRadius::default(),
            shape_mode: UShapeMode::Round,
        }
    }
}

/// Defines a visual border around a node.
#[derive(Component, Clone)]
pub struct UBorder {
    pub color: Color,
    pub width: f32,
    /// Border radius can differ from the node's radius.
    pub radius: UCornerRadius,
    /// Distance between the border and the node body.
    pub offset: f32,
}

impl Default for UBorder {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            width: 0.0,
            radius: UCornerRadius::default(),
            offset: 0.0,
        }
    }
}

/// Layout configuration component.
/// Controls how children are arranged within this node.
#[derive(Component, Debug, Clone, Reflect, Copy)]
#[reflect(Component)] // Important for Inspector
pub struct ULayout {
    /// The layout algorithm to use (Flex, Grid, Masonry...).
    pub display: UDisplay,
    /// Direction of the main axis.
    pub flex_direction: UFlexDirection,
    /// Alignment of items along the main axis.
    pub justify_content: UJustifyContent,
    /// Alignment of items along the cross axis.
    pub align_items: UAlignItems,
    /// Gap between items.
    pub gap: f32,
    
    /// Number of columns (used for Grid/Masonry layouts).
    pub grid_columns: u32, 
}

impl Default for ULayout {
    fn default() -> Self {
        Self {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Start,
            gap: 0.0,
            grid_columns: 1, // Default is one column
        }
    }
}

/// Alignment options for layout (Standard CSS-like).
#[derive(Clone, Copy, PartialEq, Debug, Default, Reflect)]
pub enum LayoutAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Defines how items are aligned on the Cross Axis.
#[derive(Clone, Copy, PartialEq, Debug, Reflect)]
pub enum UAlignItems {
    Auto,
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// The items are packed towards the start of the axis, unless the flex direction is reversed;
    /// then they are packed towards the end of the axis.
    FlexStart,
    /// The items are packed towards the end of the axis, unless the flex direction is reversed;
    /// then they are packed towards the start of the axis.
    FlexEnd,
    /// The items are packed along the center of the axis.
    Center,
    /// The items are packed such that their baselines align.
    Baseline,
    /// The items are stretched to fill the space they're given.
    Stretch,
}

/// Layout direction (Main Axis).
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UFlexDirection {
    Row,           // Left -> Right
    Column,        // Top -> Bottom
    RowReverse,    // Right -> Left (New)
    ColumnReverse, // Bottom -> Top (New)
}

/// Distribution of space along the Main Axis.
#[derive(Debug, Clone, Copy, PartialEq,Default ,Reflect)]
pub enum UJustifyContent {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    Stretch, 
    SpaceAround,
    SpaceEvenly,
}

/// Supported display/layout modes.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UDisplay {
    Flex,
    Grid,
    Stack,
    Radial,
    Masonry,
    None,
}

// /// Defines how much an item should grow relative to others to fill available space.
// /// 0.0 = Do not grow. 1.0 = Take 1 share.
// #[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
// #[reflect(Component, Default)]
// pub struct UFlexGrow(pub f32);

// impl Default for UFlexGrow {
//     fn default() -> Self {
//         Self(0.0) // Default is no growth
//     }
// }

// impl UFlexGrow {
//     /// Standard fill (shares space equally).
//     pub fn fill() -> Self { Self(1.0) }
    
//     /// Double growth factor.
//     pub fn double() -> Self { Self(2.0) }
// }

// // =========================================================
// // 3. Flex Shrink (Optional)
// // =========================================================

// /// Defines the ability of a flex item to shrink if necessary.
// #[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
// #[reflect(Component, Default)]
// pub struct UFlexShrink(pub f32);

// impl Default for UFlexShrink {
//     fn default() -> Self {
//         Self(1.0) // Shrinkable by default
//     }
// }

/// Self-control component for a child node.
/// Overrides parent settings (Alignment) or Layout flow (Positioning).
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct USelf {
    /// Self alignment overriding parent's `align_items`.
    pub align_self: UAlignSelf,
    
    /// Positioning offsets (Works for Relative and Absolute).
    pub left: UVal,    
    pub top: UVal,    
    pub bottom: UVal,    
    pub right: UVal,    
    /// Layout order (affects Z-index too).
    pub order: i32,
    pub position_type: UPositionType,
}
impl Default for USelf {
    fn default() -> Self {
        Self { 
            align_self: UAlignSelf::Auto, 
            left: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            right: UVal::Auto,
            order: 0, 
            position_type: UPositionType::Relative
        }
    }
}

impl USelf {
    pub fn get_val(&self) -> f32 {
        match &self.left {
            UVal::Px(p) => *p,
            _ => 0.0
        }
    }
}
/// Self alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UAlignSelf {
    Auto,    // Inherit from parent
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct UPosition {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

/// Determines if the element is part of the flow (Relative) or independent (Absolute).
#[derive(Reflect, Clone, Debug, Copy, PartialEq)]
pub enum UPositionType {
    Relative, // In-flow
    Absolute, // Out-of-flow
}

/// مكون يفرض القص (Masking) على جميع أبنائه.
/// يتم استخدام حدود هذا العنصر (Size + Position + Radius) كقناع.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(UNode, ComputedSize, GlobalTransform)]
pub struct UClip {
    /// هل القص مفعل؟
    pub enabled: bool,
}
impl UClip {
    pub fn enabled(enable: bool) -> Self {
        UClip { enabled: enable }
    }
}