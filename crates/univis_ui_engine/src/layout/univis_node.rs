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
            .register_type::<UGridAutoFlow>();
    }
}

#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UShapeMode {
    /// Rounded corners.
    #[default]
    Round,
    /// Chamfered corners.
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
#[derive(Component, Clone, Reflect)]
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

    pub shape_mode: UShapeMode,
}

impl Default for UNode {
    fn default() -> Self {
        Self {
            width: UVal::Content,
            height: UVal::Content,
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

/// Layout configuration for a [`UNode`].
///
/// This controls the container algorithm and the most common alignment rules
/// used to place direct children.
#[derive(Component, Debug, Clone, Reflect)]
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
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
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
    #[default]
    Auto,
    Normal,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
    FirstBaseline,
    LastBaseline,
    FlexStart,
    FlexEnd,
    SelfStart,
    SelfEnd,
    Left,
    Right,
}

/// CSS-inspired extended alignment values for container item alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UAlignItemsExt {
    #[default]
    Normal,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
    FirstBaseline,
    LastBaseline,
    FlexStart,
    FlexEnd,
    SelfStart,
    SelfEnd,
    Left,
    Right,
}

/// CSS-inspired extended alignment values for distributing lines/tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UContentAlignExt {
    #[default]
    Normal,
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    FlexStart,
    FlexEnd,
}

/// Overflow position behavior for alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UOverflowPosition {
    Safe,
    #[default]
    Unsafe,
}

/// Flex wrap behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UFlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Grid track sizing.
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum UTrackSize {
    Px(f32),
    Fr(f32),
    #[default]
    Auto,
}

/// Grid auto-placement flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UGridAutoFlow {
    #[default]
    Row,
    Column,
}

/// Advanced container-only controls nested under [`ULayout`].
#[derive(Debug, Clone, Reflect, Default)]
pub struct ULayoutContainerExt {
    pub box_align: ULayoutBoxAlignContainer,
    pub flex: ULayoutFlexContainer,
    pub grid: ULayoutGridContainer,
}

/// Extended container-level alignment options.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ULayoutBoxAlignContainer {
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
    pub row_gap: Option<f32>,
    pub column_gap: Option<f32>,
}

impl Default for ULayoutBoxAlignContainer {
    fn default() -> Self {
        Self {
            justify_items: None,
            align_content: None,
            row_gap: None,
            column_gap: None,
        }
    }
}

/// Extended flex container options.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ULayoutFlexContainer {
    pub wrap: UFlexWrap,
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
#[derive(Debug, Clone, Reflect)]
pub struct ULayoutGridContainer {
    pub template_columns: Vec<UTrackSize>,
    pub template_rows: Vec<UTrackSize>,
    pub auto_flow: UGridAutoFlow,
    pub auto_rows: UTrackSize,
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
#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct ULayoutItemExt {
    pub box_align: ULayoutBoxAlignSelf,
    pub flex: ULayoutFlexItem,
    pub grid: ULayoutGridItem,
}

/// Extended child-level alignment options.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ULayoutBoxAlignSelf {
    pub justify_self: Option<UAlignSelfExt>,
    pub align_self: Option<UAlignSelfExt>,
    pub justify_overflow: UOverflowPosition,
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
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ULayoutFlexItem {
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<UVal>,
}

impl Default for ULayoutFlexItem {
    fn default() -> Self {
        Self {
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
        }
    }
}

/// Extended grid item placement options.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ULayoutGridItem {
    pub column_start: Option<u32>,
    pub column_span: u32,
    pub row_start: Option<u32>,
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
    /// Layout order for siblings and local stacking inside the same root capsule.
    pub order: i32,
    pub position_type: UPositionType,
    /// Advanced item-only layout controls.
    pub item_ext: ULayoutItemExt,
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
            position_type: UPositionType::Relative,
            item_ext: ULayoutItemExt::default(),
        }
    }
}

impl USelf {
    pub fn get_val(&self) -> f32 {
        match &self.left {
            UVal::Px(p) => *p,
            _ => 0.0,
        }
    }
}
/// Self alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UAlignSelf {
    Auto, // Inherit from parent
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, Reflect)]
/// Explicit top/right/bottom/left offsets in resolved scalar form.
pub struct UPosition {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

/// Determines whether the node stays in normal flow or is taken out of flow.
#[derive(Reflect, Clone, Debug, Copy, PartialEq)]
pub enum UPositionType {
    Relative, // In-flow
    Absolute, // Out-of-flow
}

/// Clips all descendants to the node's resolved bounds and corner radius.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(UNode, ComputedSize, GlobalTransform)]
pub struct UClip {
    /// Whether clipping is enabled for this node.
    pub enabled: bool,
}
impl UClip {
    pub fn enabled(enable: bool) -> Self {
        UClip { enabled: enable }
    }
}
