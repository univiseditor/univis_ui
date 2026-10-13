//! Layout modes, directions, alignments, and track types.

/// Supported display/layout modes for a container node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UDisplay {
    /// Flex-style layout using a main axis and a cross axis.
    #[default]
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

/// Layout direction (Main Axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UFlexDirection {
    /// Items are placed left to right.
    #[default]
    Row,
    /// Items are placed top to bottom.
    Column,
    /// Items are placed right to left.
    RowReverse,
    /// Items are placed bottom to top.
    ColumnReverse,
}

/// Distribution of space along the Main Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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

/// Defines how items are aligned on the Cross Axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UAlignItems {
    /// Automatically align based on parent or contextual layout defaults.
    #[default]
    Auto,
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// Flex start alignment.
    FlexStart,
    /// Flex end alignment.
    FlexEnd,
    /// The items are packed along the center of the axis.
    Center,
    /// The items are packed such that their baselines align.
    Baseline,
    /// The items are stretched to fill the space they're given.
    Stretch,
}

/// Self alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UAlignSelf {
    /// Inherit alignment from the parent.
    #[default]
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

/// Flex wrap behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UFlexWrap {
    /// Items are placed on a single line and do not wrap.
    #[default]
    NoWrap,
    /// Items wrap onto multiple lines, from top to bottom.
    Wrap,
    /// Items wrap onto multiple lines, from bottom to top.
    WrapReverse,
}

/// Determines whether the node stays in normal flow or is taken out of flow.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UPositionType {
    /// Node stays in normal flow.
    #[default]
    Relative,
    /// Node is taken out of flow and positioned absolutely.
    Absolute,
}

/// CSS-inspired extended alignment values for self alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UOverflowPosition {
    /// Safe overflow alignment (prevents data loss).
    Safe,
    /// Unsafe overflow alignment (default behavior).
    #[default]
    Unsafe,
}

/// Grid auto-placement flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UGridAutoFlow {
    /// Auto-placed items fill rows.
    #[default]
    Row,
    /// Auto-placed items fill columns.
    Column,
}

/// Minimum or maximum bound for a grid track.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UTrackBound {
    /// Fixed size in pixels.
    Px(f32),
    /// Percentage of the available grid container track space (1.0 = 100%).
    Percent(f32),
    /// Fractional share of available space.
    Fr(f32),
    /// Content-derived size.
    Auto,
}

impl Default for UTrackBound {
    fn default() -> Self {
        Self::Auto
    }
}

impl UTrackBound {
    /// Creates a pixel track bound.
    pub const fn px(v: f32) -> Self {
        Self::Px(v)
    }

    /// Creates a percentage track bound.
    pub const fn percent(p: f32) -> Self {
        Self::Percent(p)
    }

    /// Creates a fractional track bound.
    pub const fn fr(v: f32) -> Self {
        Self::Fr(v)
    }

    /// Resolves the bound to an absolute pixel value when possible given the base space.
    pub fn resolve_px(&self, base: f32) -> Option<f32> {
        match *self {
            Self::Px(v) => Some(v.max(0.0)),
            Self::Percent(p) => Some((p * base).max(0.0)),
            _ => None,
        }
    }
}

impl From<f32> for UTrackBound {
    fn from(v: f32) -> Self {
        Self::Px(v)
    }
}

/// Sizing definition for repeated grid tracks.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UTrackRepeat {
    /// Fixed size in pixels.
    Px(f32),
    /// Percentage size of available container space.
    Percent(f32),
    /// Fractional size taking a share of remaining space.
    Fr(f32),
    /// Automatic sizing based on content.
    #[default]
    Auto,
    /// Clamped track sizing: `minmax(min, max)`.
    MinMax {
        /// The minimum sizing function.
        min: UTrackBound,
        /// The maximum sizing function.
        max: UTrackBound,
    },
}

impl UTrackRepeat {
    /// Creates a pixel track repeat.
    pub const fn px(v: f32) -> Self {
        Self::Px(v)
    }

    /// Creates a percentage track repeat.
    pub const fn percent(p: f32) -> Self {
        Self::Percent(p)
    }

    /// Creates a fractional track repeat.
    pub const fn fr(v: f32) -> Self {
        Self::Fr(v)
    }

    /// Creates a `minmax(min, max)` repeated track sizing definition.
    pub const fn minmax(min: UTrackBound, max: UTrackBound) -> Self {
        Self::MinMax { min, max }
    }
}

impl From<UTrackBound> for UTrackRepeat {
    fn from(bound: UTrackBound) -> Self {
        match bound {
            UTrackBound::Px(v) => Self::Px(v),
            UTrackBound::Percent(p) => Self::Percent(p),
            UTrackBound::Fr(v) => Self::Fr(v),
            UTrackBound::Auto => Self::Auto,
        }
    }
}

impl From<f32> for UTrackRepeat {
    fn from(v: f32) -> Self {
        Self::Px(v)
    }
}

/// Grid track sizing.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum UTrackSize {
    /// Fixed size in pixels.
    Px(f32),
    /// Percentage size of the available container space.
    Percent(f32),
    /// Fractional size taking a share of the remaining space.
    Fr(f32),
    /// Automatic sizing based on content.
    #[default]
    Auto,
    /// Clamped track sizing: `minmax(min, max)`.
    MinMax {
        /// The minimum sizing function.
        min: UTrackBound,
        /// The maximum sizing function.
        max: UTrackBound,
    },
    /// Fixed track repeat: `repeat(count, track)`.
    Repeat(u16, UTrackRepeat),
    /// Responsive auto-fill repeat: `repeat(auto-fill, track)`.
    RepeatFill(UTrackRepeat),
    /// Responsive auto-fit repeat: `repeat(auto-fit, track)`.
    RepeatFit(UTrackRepeat),
}

impl UTrackSize {
    /// Creates a pixel track size.
    pub const fn px(v: f32) -> Self {
        Self::Px(v)
    }

    /// Creates a percentage track size.
    pub const fn percent(p: f32) -> Self {
        Self::Percent(p)
    }

    /// Creates a fractional track size.
    pub const fn fr(v: f32) -> Self {
        Self::Fr(v)
    }

    /// Creates a `minmax(min, max)` track sizing definition.
    pub const fn minmax(min: UTrackBound, max: UTrackBound) -> Self {
        Self::MinMax { min, max }
    }

    /// Creates a fixed repeat definition: `repeat(count, track)`.
    pub const fn repeat(count: u16, track: UTrackRepeat) -> Self {
        Self::Repeat(count, track)
    }

    /// Creates an auto-fill repeat definition: `repeat(auto-fill, track)`.
    pub const fn repeat_fill(track: UTrackRepeat) -> Self {
        Self::RepeatFill(track)
    }

    /// Creates an auto-fit repeat definition: `repeat(auto-fit, track)`.
    pub const fn repeat_fit(track: UTrackRepeat) -> Self {
        Self::RepeatFit(track)
    }
}

impl From<f32> for UTrackSize {
    fn from(v: f32) -> Self {
        Self::Px(v)
    }
}

impl From<UTrackBound> for UTrackSize {
    fn from(bound: UTrackBound) -> Self {
        match bound {
            UTrackBound::Px(v) => Self::Px(v),
            UTrackBound::Percent(p) => Self::Percent(p),
            UTrackBound::Fr(v) => Self::Fr(v),
            UTrackBound::Auto => Self::Auto,
        }
    }
}

impl From<UTrackRepeat> for UTrackSize {
    fn from(repeat: UTrackRepeat) -> Self {
        match repeat {
            UTrackRepeat::Px(v) => Self::Px(v),
            UTrackRepeat::Percent(p) => Self::Percent(p),
            UTrackRepeat::Fr(v) => Self::Fr(v),
            UTrackRepeat::Auto => Self::Auto,
            UTrackRepeat::MinMax { min, max } => Self::MinMax { min, max },
        }
    }
}
