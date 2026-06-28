//! Core node, layout, and local positioning components.

use crate::internal_prelude::*;
use bevy::prelude::*;

/// Registers the node and layout types used by the engine layout solver.
pub struct UnivisNodePlugin;

impl Plugin for UnivisNodePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UVal>()
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
            padding: USides::default(),
            margin: USides::default(),
            background_color: Color::NONE,
            border_radius: UCornerRadius::default(),
            shape_mode: UShapeMode::Round,
        }
    }
}

impl UNode {
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

/// Layout configuration for a [`UNode`].
///
/// This controls the container algorithm and the most common alignment rules
/// used to place direct children.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
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

    /// Advanced container-only layout controls.
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
            grid_columns: 1, // Default is one column
            container_ext: ULayoutContainerExt::default(),
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

/// Defines how items are aligned on the Cross Axis.
#[derive(Clone, Copy, PartialEq, Debug, Reflect)]
pub enum UAlignItems {
    /// Automatically align based on parent or contextual layout defaults.
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
    /// Items are placed left to right.
    Row,
    /// Items are placed top to bottom.
    Column,
    /// Items are placed right to left.
    RowReverse,
    /// Items are placed bottom to top.
    ColumnReverse,
}

/// Distribution of space along the Main Axis.
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub enum UJustifyContent {
    /// Items are packed toward the start.
    #[default]
    Start,
    /// Items are packed toward the center.
    Center,
    /// Items are packed toward the end.
    End,
    /// Items are evenly distributed with equal space between them.
    SpaceBetween,
    /// Items stretch to fill available space.
    Stretch,
    /// Items are evenly distributed with half-size spaces on the ends.
    SpaceAround,
    /// Items are evenly distributed with equal space around them.
    SpaceEvenly,
}

/// Supported display/layout modes for a container node.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UDisplay {
    /// Flex-style layout using a main axis and a cross axis.
    Flex,
    /// Grid-style layout with tracks and placement rules.
    Grid,
    /// Simple stacking layout.
    Stack,
    /// Radial placement around a center point.
    Radial,
    /// Masonry-like column layout.
    Masonry,
    /// Do not lay out children.
    None,
}

/// CSS-inspired extended alignment values for self alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UAlignSelfExt {
    /// Inherit from parent.
    #[default]
    Auto,
    /// Normal default alignment.
    Normal,
    /// Align to start.
    Start,
    /// Align to end.
    End,
    /// Align to center.
    Center,
    /// Stretch to fill.
    Stretch,
    /// Baseline alignment.
    Baseline,
    /// First baseline alignment.
    FirstBaseline,
    /// Last baseline alignment.
    LastBaseline,
    /// Flex start alignment.
    FlexStart,
    /// Flex end alignment.
    FlexEnd,
    /// Self start alignment.
    SelfStart,
    /// Self end alignment.
    SelfEnd,
    /// Left alignment.
    Left,
    /// Right alignment.
    Right,
}

/// CSS-inspired extended alignment values for container item alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UAlignItemsExt {
    /// Normal default alignment.
    #[default]
    Normal,
    /// Align to start.
    Start,
    /// Align to end.
    End,
    /// Align to center.
    Center,
    /// Stretch to fill.
    Stretch,
    /// Baseline alignment.
    Baseline,
    /// First baseline alignment.
    FirstBaseline,
    /// Last baseline alignment.
    LastBaseline,
    /// Flex start alignment.
    FlexStart,
    /// Flex end alignment.
    FlexEnd,
    /// Self start alignment.
    SelfStart,
    /// Self end alignment.
    SelfEnd,
    /// Left alignment.
    Left,
    /// Right alignment.
    Right,
}

/// CSS-inspired extended alignment values for distributing lines/tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UContentAlignExt {
    /// Normal default alignment.
    #[default]
    Normal,
    /// Align to start.
    Start,
    /// Align to end.
    End,
    /// Align to center.
    Center,
    /// Stretch to fill.
    Stretch,
    /// Distribute space between items.
    SpaceBetween,
    /// Distribute space around items.
    SpaceAround,
    /// Distribute space evenly between items.
    SpaceEvenly,
    /// Flex start alignment.
    FlexStart,
    /// Flex end alignment.
    FlexEnd,
}

/// Overflow position behavior for alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UOverflowPosition {
    /// Safe overflow alignment (prevents data loss).
    Safe,
    /// Unsafe overflow alignment (default behavior).
    #[default]
    Unsafe,
}

/// Flex wrap behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UFlexWrap {
    /// Items are placed on a single line and do not wrap.
    #[default]
    NoWrap,
    /// Items wrap onto multiple lines, from top to bottom.
    Wrap,
    /// Items wrap onto multiple lines, from bottom to top.
    WrapReverse,
}

/// Grid track sizing.
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum UTrackSize {
    /// Fixed size in pixels.
    Px(f32),
    /// Fractional size taking a share of the remaining space.
    Fr(f32),
    /// Automatic sizing based on content.
    #[default]
    Auto,
}

/// Grid auto-placement flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UGridAutoFlow {
    /// Auto-placed items fill rows.
    #[default]
    Row,
    /// Auto-placed items fill columns.
    Column,
}

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
    /// Advanced item-only layout controls.
    pub item_ext: ULayoutItemExt,
}
impl Default for USelf {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            align_self: UAlignSelf::Auto,
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
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UAlignSelf {
    /// Inherit alignment from the parent.
    Auto,
    /// Align to the start of the cross axis.
    Start,
    /// Align to the center of the cross axis.
    Center,
    /// Align to the end of the cross axis.
    End,
    /// Stretch to fill the cross axis.
    Stretch,
}

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

/// Determines whether the node stays in normal flow or is taken out of flow.
#[derive(Reflect, Clone, Debug, Copy, PartialEq)]
pub enum UPositionType {
    /// Node stays in normal flow.
    Relative,
    /// Node is taken out of flow and positioned absolutely.
    Absolute,
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
