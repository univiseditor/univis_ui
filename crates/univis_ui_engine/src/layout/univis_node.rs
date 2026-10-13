//! Core node, layout, and local positioning components.

use crate::internal_prelude::*;
use bevy::prelude::*;

/// Registers the node and layout types used by the engine layout solver.
pub struct UnivisNodePlugin;

impl Plugin for UnivisNodePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UVal>()
            .register_type::<UValLength>()
            .register_type::<ULayout>()
            .register_type::<UNode>()
            .register_type::<ComputedSize>()
            .register_type::<ULayoutContainerExt>()
            .register_type::<ULayoutBoxAlignContainer>()
            .register_type::<ULayoutFlexContainer>()
            .register_type::<ULayoutGridContainer>()
            .register_type::<ULayoutItemExt>()
            .register_type::<ULayoutBoxAlignSelf>()
            .register_type::<ULayoutFlexItem>()
            .register_type::<ULayoutGridItem>()
            .register_type::<UAlignSelfExt>()
            .register_type::<UAlignItemsExt>()
            .register_type::<UContentAlignExt>()
            .register_type::<UOverflowPosition>()
            .register_type::<UFlexWrap>()
            .register_type::<UTrackSize>()
            .register_type::<UTrackBound>()
            .register_type::<UTrackRepeat>()
            .register_type::<UGridAutoFlow>()
            .register_type::<UZIndex>();
    }
}

/// Determines the shape and corner treatment of a node.
#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UShapeMode {
    /// Rounded corners.
    #[default]
    Round,
    /// Chamfered corners (angled cuts instead of curves).
    Cut,
}

/// The core component for any UI node.
///
/// `UNode` carries the box-model information for a visual UI entity: preferred
/// size, padding, margin, background color, and border radius.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_engine::prelude::*;
///
/// fn spawn_panel(commands: &mut Commands) {
///     commands.spawn(UNode {
///         width: UVal::Px(320.0),
///         height: UVal::Px(120.0),
///         padding: USides::all(16.0),
///         background_color: Color::srgb(0.12, 0.15, 0.2),
///         border_radius: UCornerRadius::all(16.0),
///         ..default()
///     });
/// }
/// ```
#[derive(Component, Clone, PartialEq, Reflect)]
#[require(Transform, Visibility, ComputedSize, ULayout, IntrinsicSize)]
pub struct UNode {
    /// Preferred width of the node.
    pub width: UVal,
    /// Preferred height of the node.
    pub height: UVal,
    /// Minimum width constraint applied after preferred or intrinsic sizing.
    pub min_width: f32,
    /// Maximum width constraint applied after preferred or intrinsic sizing.
    pub max_width: f32,
    /// Minimum height constraint applied after preferred or intrinsic sizing.
    pub min_height: f32,
    /// Maximum height constraint applied after preferred or intrinsic sizing.
    pub max_height: f32,
    /// Preferred aspect ratio (width / height) conforming to CSS aspect-ratio.
    ///
    /// When set, automatic or flexible sizing on one axis is calculated from the other axis.
    pub aspect_ratio: Option<f32>,

    /// Inner spacing (affects children placement).
    pub padding: USides,
    /// Outer spacing (affects placement relative to parent/siblings).
    pub margin: USides,

    /// Background color of the node.
    pub background_color: Color,
    /// Corner radius for rounded rectangles (independent corners).
    pub border_radius: UCornerRadius,
    /// The shape mode (e.g. rounded or cut corners).
    pub shape_mode: UShapeMode,
}

impl Default for UNode {
    fn default() -> Self {
        Self {
            width: UVal::Auto,
            height: UVal::Auto,
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
            aspect_ratio: None,
            padding: USides::default(),
            margin: USides::default(),
            background_color: Color::NONE,
            border_radius: UCornerRadius::default(),
            shape_mode: UShapeMode::Round,
        }
    }
}

impl UNode {
    /// Returns the sanitized aspect ratio (width / height) if positive and finite.
    pub fn sanitized_aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio.filter(|&r| r > 0.0 && r.is_finite())
    }

    /// Returns sanitized width bounds as `(min, max)`.
    pub fn width_bounds(&self) -> (f32, f32) {
        sanitize_bounds(self.min_width, self.max_width)
    }

    /// Returns sanitized height bounds as `(min, max)`.
    pub fn height_bounds(&self) -> (f32, f32) {
        sanitize_bounds(self.min_height, self.max_height)
    }

    /// Clamps a width against this node's min/max width bounds.
    pub fn clamp_width(&self, width: f32) -> f32 {
        let (min, max) = self.width_bounds();
        width.clamp(min, max)
    }

    /// Clamps a height against this node's min/max height bounds.
    pub fn clamp_height(&self, height: f32) -> f32 {
        let (min, max) = self.height_bounds();
        height.clamp(min, max)
    }

    /// Creates a card container with background color, corner radius, and uniform padding.
    pub fn card(bg: Color, radius: f32, padding: f32) -> Self {
        Self {
            background_color: bg,
            border_radius: UCornerRadius::all(radius),
            padding: USides::all(padding),
            ..Default::default()
        }
    }

    /// Creates a card container with background color, corner radius, and independent horizontal/vertical padding.
    pub fn card_axes(bg: Color, radius: f32, x_pad: f32, y_pad: f32) -> Self {
        Self {
            background_color: bg,
            border_radius: UCornerRadius::all(radius),
            padding: USides::axes(y_pad, x_pad),
            ..Default::default()
        }
    }

    /// Creates a node with explicit pixel width and height.
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: UVal::Px(width),
            height: UVal::Px(height),
            ..Default::default()
        }
    }
}

fn sanitize_bounds(min: f32, max: f32) -> (f32, f32) {
    let min = min.max(0.0);
    let max = if max.is_finite() {
        max.max(0.0)
    } else {
        f32::INFINITY
    };

    if max < min { (min, min) } else { (min, max) }
}

/// Defines a border rendered around a [`UNode`].
#[derive(Component, Clone)]
pub struct UBorder {
    /// The color of the border.
    pub color: Color,
    /// The width (thickness) of the border.
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

impl UBorder {
    /// Creates a solid border with the given color, width, and uniform corner radius.
    pub fn solid(color: Color, width: f32, radius: f32) -> Self {
        Self {
            color,
            width,
            radius: UCornerRadius::all(radius),
            offset: 0.0,
        }
    }
}

/// Layout configuration for a [`UNode`].
///
/// This controls the container algorithm and the most common alignment rules
/// used to place direct children.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)] // Important for Inspector
pub struct ULayout {
    /// The layout algorithm to use (Flex, Grid, Stack, Radial, Masonry...).
    pub display: UDisplay,
    /// Direction of the main axis for Flex layout.
    pub flex_direction: UFlexDirection,
    /// Alignment of items along the main axis.
    pub justify_content: UJustifyContent,
    /// Alignment of items along the cross axis.
    pub align_items: UAlignItems,
    /// Uniform gap between items.
    pub gap: f32,
    /// Overrides gap between rows (for Grid / wrapped Flex).
    pub row_gap: Option<f32>,
    /// Overrides gap between columns (for Grid / wrapped Flex).
    pub column_gap: Option<f32>,

    /// Number of columns (used for Grid/Masonry layouts).
    pub grid_columns: u32,
    /// Explicit track sizes for grid columns.
    pub grid_template_columns: Vec<UTrackSize>,
    /// Explicit track sizes for grid rows.
    pub grid_template_rows: Vec<UTrackSize>,
    /// Auto placement flow direction for grid items.
    pub grid_auto_flow: UGridAutoFlow,
    /// Default track sizing for implicitly created grid rows.
    pub grid_auto_rows: UTrackSize,
    /// Default track sizing for implicitly created grid columns.
    pub grid_auto_columns: UTrackSize,
    /// Wrap behavior for flex items.
    pub flex_wrap: UFlexWrap,
    /// Overrides justify-items alignment inside grid cells.
    pub justify_items: Option<UAlignItemsExt>,

    /// Advanced container-only layout controls (preserved for backward compatibility).
    #[deprecated(
        since = "0.3.2",
        note = "Use flat fields directly on `ULayout` (e.g. `grid_template_columns`, `row_gap`, `column_gap`, `flex_wrap`) instead of nested `container_ext`."
    )]
    pub container_ext: ULayoutContainerExt,
}

impl Default for ULayout {
    fn default() -> Self {
        Self {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Start,
            gap: 0.0,
            row_gap: None,
            column_gap: None,
            grid_columns: 1, // Default is one column
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_auto_flow: UGridAutoFlow::Row,
            grid_auto_rows: UTrackSize::Auto,
            grid_auto_columns: UTrackSize::Auto,
            flex_wrap: UFlexWrap::NoWrap,
            justify_items: None,
            #[allow(deprecated)]
            container_ext: ULayoutContainerExt::default(),
        }
    }
}

impl ULayout {
    /// Creates a grid layout with the given column count and gap.
    pub fn grid(columns: u32, gap: f32) -> Self {
        Self {
            display: UDisplay::Grid,
            grid_columns: columns,
            gap,
            align_items: UAlignItems::Stretch,
            justify_items: Some(UAlignItemsExt::Stretch),
            ..Default::default()
        }
    }

    /// Creates a flex row layout with the given gap.
    pub fn flex_row(gap: f32) -> Self {
        Self {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap,
            ..Default::default()
        }
    }

    /// Creates a flex column layout with the given gap.
    pub fn flex_col(gap: f32) -> Self {
        Self {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap,
            ..Default::default()
        }
    }

    /// Creates a layer stack layout where children overlap in the same container.
    pub fn stack() -> Self {
        Self {
            display: UDisplay::Stack,
            ..Default::default()
        }
    }
}

/// Alignment options for layout (Standard CSS-like).
#[derive(Clone, Copy, PartialEq, Debug, Default, Reflect)]
pub enum LayoutAlign {
    /// Align to the start of the layout axis.
    #[default]
    Start,
    /// Align to the center of the layout axis.
    Center,
    /// Align to the end of the layout axis.
    End,
}

pub use univis_layout::types::{
    UAlignItems, UAlignItemsExt, UAlignSelfExt, UContentAlignExt, UDisplay, UFlexDirection,
    UFlexWrap, UGridAutoFlow, UJustifyContent, UOverflowPosition, UTrackBound, UTrackRepeat,
    UTrackSize,
};

/// Advanced container-only controls nested under [`ULayout`].
#[derive(Debug, Clone, PartialEq, Reflect, Default)]
pub struct ULayoutContainerExt {
    /// Extended alignment options for containers.
    pub box_align: ULayoutBoxAlignContainer,
    /// Extended flex container options.
    pub flex: ULayoutFlexContainer,
    /// Extended grid container options.
    pub grid: ULayoutGridContainer,
}

/// Extended container-level alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub struct ULayoutBoxAlignContainer {
    /// Overrides justify items alignment for the container.
    pub justify_items: Option<UAlignItemsExt>,
    /// Overrides content alignment for the container.
    pub align_content: Option<UContentAlignExt>,
    /// Specifies the gap between rows.
    pub row_gap: Option<f32>,
    /// Specifies the gap between columns.
    pub column_gap: Option<f32>,
}

/// Extended flex container options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct ULayoutFlexContainer {
    /// Wrap behavior for flex items.
    pub wrap: UFlexWrap,
    /// Alignment of content lines across the cross axis.
    pub align_content: Option<UContentAlignExt>,
}

impl Default for ULayoutFlexContainer {
    fn default() -> Self {
        Self {
            wrap: UFlexWrap::NoWrap,
            align_content: None,
        }
    }
}

/// Extended grid container options.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ULayoutGridContainer {
    /// Defines the columns of the grid.
    pub template_columns: Vec<UTrackSize>,
    /// Defines the rows of the grid.
    pub template_rows: Vec<UTrackSize>,
    /// Defines the auto-placement flow.
    pub auto_flow: UGridAutoFlow,
    /// Defines the size of implicit rows.
    pub auto_rows: UTrackSize,
    /// Defines the size of implicit columns.
    pub auto_columns: UTrackSize,
}

impl Default for ULayoutGridContainer {
    fn default() -> Self {
        Self {
            template_columns: Vec::new(),
            template_rows: Vec::new(),
            auto_flow: UGridAutoFlow::Row,
            auto_rows: UTrackSize::Auto,
            auto_columns: UTrackSize::Auto,
        }
    }
}

/// Advanced item-only controls nested under [`USelf`].
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub struct ULayoutItemExt {
    /// Extended child-level alignment overrides.
    pub box_align: ULayoutBoxAlignSelf,
    /// Extended flex item overrides.
    pub flex: ULayoutFlexItem,
    /// Extended grid item placement overrides.
    pub grid: ULayoutGridItem,
}

/// Extended child-level alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct ULayoutBoxAlignSelf {
    /// Overrides justify self alignment.
    pub justify_self: Option<UAlignSelfExt>,
    /// Overrides align self alignment.
    pub align_self: Option<UAlignSelfExt>,
    /// Overrides justify overflow behavior.
    pub justify_overflow: UOverflowPosition,
    /// Overrides align overflow behavior.
    pub align_overflow: UOverflowPosition,
}

impl Default for ULayoutBoxAlignSelf {
    fn default() -> Self {
        Self {
            justify_self: None,
            align_self: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
        }
    }
}

/// Extended flex item options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub struct ULayoutFlexItem {
    /// The flex grow factor.
    pub flex_grow: Option<f32>,
    /// The flex shrink factor.
    pub flex_shrink: Option<f32>,
    /// The initial main size of the flex item.
    pub flex_basis: Option<UVal>,
}

/// Extended grid item placement options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct ULayoutGridItem {
    /// The starting column position.
    pub column_start: Option<u32>,
    /// The number of columns to span.
    pub column_span: u32,
    /// The starting row position.
    pub row_start: Option<u32>,
    /// The number of rows to span.
    pub row_span: u32,
}

impl Default for ULayoutGridItem {
    fn default() -> Self {
        Self {
            column_start: None,
            column_span: 1,
            row_start: None,
            row_span: 1,
        }
    }
}

/// Child-local layout overrides and positioning rules.
///
/// `USelf` only affects layout and local stacking inside the same root capsule.
/// Its [`USelf::order`] field is not a global `z-index`.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct USelf {
    /// Self alignment overriding parent's `align_items`.
    pub align_self: UAlignSelf,

    /// Starting column position for grid layouts (1-indexed).
    pub column_start: Option<u32>,
    /// Number of columns to span (minimum 1).
    pub column_span: u32,
    /// Starting row position for grid layouts (1-indexed).
    pub row_start: Option<u32>,
    /// Number of rows to span (minimum 1).
    pub row_span: u32,

    /// The flex grow factor.
    pub flex_grow: Option<f32>,
    /// The flex shrink factor.
    pub flex_shrink: Option<f32>,
    /// The initial main size of the flex item.
    pub flex_basis: Option<UVal>,
    /// Overrides horizontal alignment of this item in a grid cell.
    pub justify_self: Option<UAlignSelfExt>,

    /// Positioning offsets (Works for Relative and Absolute).
    /// Left positioning offset.
    pub left: UVal,
    /// Top positioning offset.
    pub top: UVal,
    /// Bottom positioning offset.
    pub bottom: UVal,
    /// Right positioning offset.
    pub right: UVal,
    /// Layout order for siblings and local stacking inside the same root capsule.
    #[deprecated(
        since = "0.2.0-alpha.3",
        note = "Use `UZIndex` component instead for sorting and stacking contexts."
    )]
    pub order: i32,
    /// The position type (e.g. Relative or Absolute).
    pub position_type: UPositionType,
    /// Advanced item-only layout controls (preserved for backward compatibility).
    #[deprecated(
        since = "0.3.2",
        note = "Use flat fields directly on `USelf` (e.g. `column_span`, `row_span`, `flex_grow`, `flex_shrink`) instead of nested `item_ext`."
    )]
    pub item_ext: ULayoutItemExt,
}

impl Default for USelf {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            align_self: UAlignSelf::Auto,
            column_start: None,
            column_span: 1,
            row_start: None,
            row_span: 1,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            justify_self: None,
            left: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            right: UVal::Auto,
            order: 0,
            position_type: UPositionType::Relative,
            item_ext: ULayoutItemExt::default(),
        }
    }
}

impl USelf {
    /// Returns the raw pixel value of the left offset, or 0.0 if not pixel-based.
    pub fn get_val(&self) -> f32 {
        match &self.left {
            UVal::Px(p) => *p,
            _ => 0.0,
        }
    }

    /// Creates a grid item specifying column span and row span.
    pub fn grid_span(col_span: u32, row_span: u32) -> Self {
        Self {
            column_span: col_span.max(1),
            row_span: row_span.max(1),
            ..Default::default()
        }
    }

    /// Creates a grid item placed at a specific cell with optional spans.
    pub fn grid_cell(col: u32, row: u32, col_span: u32, row_span: u32) -> Self {
        Self {
            column_start: Some(col),
            column_span: col_span.max(1),
            row_start: Some(row),
            row_span: row_span.max(1),
            ..Default::default()
        }
    }

    /// Positions the element at the top-left of the container/HUD with pixel offset.
    pub fn anchor_top_left(offset: Vec2) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            left: UVal::Px(offset.x),
            top: UVal::Px(offset.y),
            ..Default::default()
        }
    }

    /// Positions the element at the top-right of the container/HUD with pixel offset.
    pub fn anchor_top_right(offset: Vec2) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            right: UVal::Px(offset.x),
            top: UVal::Px(offset.y),
            ..Default::default()
        }
    }

    /// Positions the element at the bottom-left of the container/HUD with pixel offset.
    pub fn anchor_bottom_left(offset: Vec2) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            left: UVal::Px(offset.x),
            bottom: UVal::Px(offset.y),
            ..Default::default()
        }
    }

    /// Positions the element at the bottom-right of the container/HUD with pixel offset.
    pub fn anchor_bottom_right(offset: Vec2) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            right: UVal::Px(offset.x),
            bottom: UVal::Px(offset.y),
            ..Default::default()
        }
    }

    /// Positions the element centered horizontally at the top edge with vertical offset.
    pub fn anchor_top_center(offset_y: f32) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            top: UVal::Px(offset_y),
            justify_self: Some(UAlignSelfExt::Center),
            ..Default::default()
        }
    }

    /// Positions the element centered horizontally at the bottom edge (e.g. action hotbar).
    pub fn anchor_bottom_center(offset_y: f32) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            bottom: UVal::Px(offset_y),
            justify_self: Some(UAlignSelfExt::Center),
            ..Default::default()
        }
    }

    /// Positions the element centered vertically at the left edge with horizontal offset.
    pub fn anchor_center_left(offset_x: f32) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            left: UVal::Px(offset_x),
            align_self: UAlignSelf::Center,
            ..Default::default()
        }
    }

    /// Positions the element centered vertically at the right edge with horizontal offset.
    pub fn anchor_center_right(offset_x: f32) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            right: UVal::Px(offset_x),
            align_self: UAlignSelf::Center,
            ..Default::default()
        }
    }

    /// Centers the element both horizontally and vertically inside the parent/screen.
    pub fn center() -> Self {
        Self {
            position_type: UPositionType::Absolute,
            align_self: UAlignSelf::Center,
            justify_self: Some(UAlignSelfExt::Center),
            ..Default::default()
        }
    }

    /// Alias for [`USelf::center()`].
    pub fn anchor_center() -> Self {
        Self::center()
    }

    /// Fills the parent container or screen with the given margin on all sides.
    pub fn anchor_fill(margin: USides) -> Self {
        Self {
            position_type: UPositionType::Absolute,
            left: UVal::Px(margin.left),
            right: UVal::Px(margin.right),
            top: UVal::Px(margin.top),
            bottom: UVal::Px(margin.bottom),
            ..Default::default()
        }
    }

    /// Stretches to completely fill the parent container or screen with zero margin.
    pub fn stretch_full() -> Self {
        Self::anchor_fill(USides::all(0.0))
    }

    /// Configures absolute positioning using a preset [`UAnchor`].
    pub fn anchor(anchor: UAnchor, offset: Vec2) -> Self {
        match anchor {
            UAnchor::TopLeft => Self::anchor_top_left(offset),
            UAnchor::TopCenter => Self::anchor_top_center(offset.y),
            UAnchor::TopRight => Self::anchor_top_right(offset),
            UAnchor::CenterLeft => Self::anchor_center_left(offset.x),
            UAnchor::Center => Self::center(),
            UAnchor::CenterRight => Self::anchor_center_right(offset.x),
            UAnchor::BottomLeft => Self::anchor_bottom_left(offset),
            UAnchor::BottomCenter => Self::anchor_bottom_center(offset.y),
            UAnchor::BottomRight => Self::anchor_bottom_right(offset),
            UAnchor::Fill => Self::anchor_fill(USides::axes(offset.y, offset.x)),
        }
    }
}

/// Screen or container anchor presets for game HUD and absolute positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UAnchor {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Fill,
}

/// Defines the Z-index depth behavior of the node.
///
/// This replaces `USelf::order` and handles Stacking Contexts.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum UZIndex {
    /// Normal sorting inside the parent. Does not break out.
    Auto,
    /// Explicit local sorting inside the parent. Acts like CSS relative z-index
    /// by forming a local stacking context. Does not interleave with other roots.
    Local(i32),
    /// Absolute global sorting. Breaks out of the local tree completely and
    /// sorts globally based on this integer.
    Global(i32),
}

impl Default for UZIndex {
    fn default() -> Self {
        UZIndex::Auto
    }
}

/// Self alignment options.
pub use univis_layout::types::{UAlignSelf, UPositionType};

#[derive(Debug, Clone, Copy, Reflect)]
/// Explicit top/right/bottom/left offsets in resolved scalar form.
pub struct UPosition {
    /// Resolved top offset.
    pub top: Option<f32>,
    /// Resolved right offset.
    pub right: Option<f32>,
    /// Resolved bottom offset.
    pub bottom: Option<f32>,
    /// Resolved left offset.
    pub left: Option<f32>,
}

/// Clips all descendants to the node's resolved bounds and corner radius.
#[derive(Component, Default, Clone, PartialEq, Reflect)]
#[reflect(Component)]
#[require(UNode, ComputedSize, GlobalTransform)]
pub struct UClip {
    /// Whether clipping is enabled for this node.
    pub enabled: bool,
}
impl UClip {
    /// Creates a new `UClip` component with the specified enabled state.
    pub fn enabled(enable: bool) -> Self {
        UClip { enabled: enable }
    }
}
