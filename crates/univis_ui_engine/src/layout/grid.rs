//! CSS Grid track sizing and repetition types.

use bevy::prelude::*;

/// Minimum or maximum bound for a grid track.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
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
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
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
