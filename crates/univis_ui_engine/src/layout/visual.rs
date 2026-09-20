//! Advanced visual styling components for Univis UI nodes.
//!
//! Provides [`UGradient`](crate::layout::visual::UGradient) for linear and radial gradient fills and
//! [`UShadow`](crate::layout::visual::UShadow) for outer glows, drop shadows, and inner holographic glows.

use bevy::prelude::*;

/// Direction or focal mapping for a [`UGradient`].
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum UGradientKind {
    /// Linear gradient along an angle in radians (0.0 = left-to-right, PI/2 = top-to-bottom).
    Linear {
        /// Angle in radians.
        angle: f32,
    },
    /// Radial gradient centered at normalized coordinates (0.5, 0.5 = center of node).
    Radial {
        /// Center in normalized [0.0, 1.0] UV coordinates.
        center: Vec2,
        /// Relative radius (e.g. 0.5 reaches the edge).
        radius: f32,
    },
}

/// Linear or radial gradient fill for a [`crate::layout::univis_node::UNode`].
#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UGradient {
    /// Start color of the gradient.
    pub start_color: Color,
    /// End color of the gradient.
    pub end_color: Color,
    /// Shape and orientation of the gradient.
    pub kind: UGradientKind,
}

impl UGradient {
    /// Creates a linear horizontal gradient (left to right).
    pub fn horizontal(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color,
            end_color,
            kind: UGradientKind::Linear { angle: 0.0 },
        }
    }

    /// Creates a linear vertical gradient (top to bottom).
    pub fn vertical(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color,
            end_color,
            kind: UGradientKind::Linear {
                angle: core::f32::consts::FRAC_PI_2,
            },
        }
    }

    /// Creates a linear gradient with an explicit angle in radians.
    pub fn linear(start_color: Color, end_color: Color, angle: f32) -> Self {
        Self {
            start_color,
            end_color,
            kind: UGradientKind::Linear { angle },
        }
    }

    /// Creates a radial gradient centered in the node.
    pub fn radial(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color,
            end_color,
            kind: UGradientKind::Radial {
                center: Vec2::splat(0.5),
                radius: 0.5,
            },
        }
    }

    /// Creates a radial gradient with explicit center and radius.
    pub fn radial_custom(start_color: Color, end_color: Color, center: Vec2, radius: f32) -> Self {
        Self {
            start_color,
            end_color,
            kind: UGradientKind::Radial { center, radius },
        }
    }
}

/// Outer glow, drop shadow, or inner holographic glow for a [`crate::layout::univis_node::UNode`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UShadow {
    /// Color and opacity of the shadow or glow.
    pub color: Color,
    /// Offset of the shadow in logical pixels (X, Y). Set to Vec2::ZERO for a symmetrical glow.
    pub offset: Vec2,
    /// Blur softness radius in logical pixels.
    pub blur: f32,
    /// Spread distance in logical pixels (expands the base shape before blur).
    pub spread: f32,
    /// If true, the glow renders as an inner glow from the border toward the center.
    pub is_inner: bool,
}

impl Default for UShadow {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.0, 0.9, 1.0, 0.4),
            offset: Vec2::ZERO,
            blur: 12.0,
            spread: 0.0,
            is_inner: false,
        }
    }
}

impl UShadow {
    /// Creates a symmetrical outer glow (e.g. cyber neon glow).
    pub fn glow(color: Color, blur: f32) -> Self {
        Self {
            color,
            offset: Vec2::ZERO,
            blur,
            spread: 0.0,
            is_inner: false,
        }
    }

    /// Creates an inner holographic glow radiating inward from the border.
    pub fn inner(color: Color, blur: f32) -> Self {
        Self {
            color,
            offset: Vec2::ZERO,
            blur,
            spread: 0.0,
            is_inner: true,
        }
    }

    /// Creates a directional drop shadow.
    pub fn drop(color: Color, offset: Vec2, blur: f32) -> Self {
        Self {
            color,
            offset,
            blur,
            spread: 0.0,
            is_inner: false,
        }
    }
}

/// Holographic inner edge glow radiating inward from the border.
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UInnerGlow {
    /// Color and opacity of the inner glow.
    pub color: Color,
    /// Blur softness radius in logical pixels.
    pub blur: f32,
    /// Spread distance in logical pixels.
    pub spread: f32,
}

impl Default for UInnerGlow {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.0, 0.9, 1.0, 0.5),
            blur: 10.0,
            spread: 0.0,
        }
    }
}

impl UInnerGlow {
    /// Creates a holographic inner edge glow.
    pub fn new(color: Color, blur: f32) -> Self {
        Self {
            color,
            blur,
            spread: 0.0,
        }
    }

    /// Creates an inner glow with explicit spread.
    pub fn with_spread(color: Color, blur: f32, spread: f32) -> Self {
        Self {
            color,
            blur,
            spread,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_constructors_set_expected_parameters() {
        let h = UGradient::horizontal(Color::WHITE, Color::BLACK);
        assert_eq!(h.kind, UGradientKind::Linear { angle: 0.0 });

        let v = UGradient::vertical(Color::WHITE, Color::BLACK);
        assert_eq!(
            v.kind,
            UGradientKind::Linear {
                angle: core::f32::consts::FRAC_PI_2
            }
        );

        let r = UGradient::radial(Color::WHITE, Color::BLACK);
        assert_eq!(
            r.kind,
            UGradientKind::Radial {
                center: Vec2::splat(0.5),
                radius: 0.5
            }
        );
    }

    #[test]
    fn shadow_constructors_set_expected_parameters() {
        let g = UShadow::glow(Color::srgb(0.0, 1.0, 0.5), 14.0);
        assert_eq!(g.offset, Vec2::ZERO);
        assert_eq!(g.blur, 14.0);
        assert!(!g.is_inner);

        let i = UShadow::inner(Color::srgb(1.0, 0.0, 0.5), 8.0);
        assert_eq!(i.blur, 8.0);
        assert!(i.is_inner);

        let d = UShadow::drop(Color::BLACK, Vec2::new(4.0, -4.0), 10.0);
        assert_eq!(d.offset, Vec2::new(4.0, -4.0));
        assert_eq!(d.blur, 10.0);
        assert!(!d.is_inner);

        let ig = UInnerGlow::new(Color::srgb(0.0, 1.0, 1.0), 12.0);
        assert_eq!(ig.blur, 12.0);
        assert_eq!(ig.spread, 0.0);
    }
}
