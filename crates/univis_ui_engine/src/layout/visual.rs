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

/// A single color stop in a [`UGradient`].
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub struct UGradientStop {
    /// Color at this stop position.
    pub color: Color,
    /// Normalized position along the gradient line in the range `[0.0, 1.0]`.
    pub position: f32,
}

impl UGradientStop {
    /// Creates a new gradient stop.
    pub fn new(position: f32, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }
}

impl From<(f32, Color)> for UGradientStop {
    fn from((position, color): (f32, Color)) -> Self {
        Self::new(position, color)
    }
}

/// Maximum number of color stops supported in a single [`UGradient`].
pub const MAX_GRADIENT_STOPS: usize = 8;

/// Interpolation mode between gradient color stops.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
#[reflect(Default)]
pub enum UGradientInterpolation {
    /// Smooth linear interpolation between adjacent stops (default).
    #[default]
    Smooth,
    /// Discrete, hard-edged color bands without interpolation (stepped stops).
    Stepped,
}

/// Linear or radial gradient fill with multi-stop color blending.
#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UGradient {
    /// Ordered list of color stops (up to 8 stops).
    pub stops: Vec<UGradientStop>,
    /// Shape and orientation of the gradient.
    pub kind: UGradientKind,
    /// Interpolation mode (smooth blending vs sharp, stepped color bands).
    pub interpolation: UGradientInterpolation,
}

impl UGradient {
    /// Creates a linear horizontal gradient (left to right).
    pub fn horizontal(start_color: Color, end_color: Color) -> Self {
        Self {
            stops: vec![
                UGradientStop::new(0.0, start_color),
                UGradientStop::new(1.0, end_color),
            ],
            kind: UGradientKind::Linear { angle: 0.0 },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a linear vertical gradient (top to bottom).
    pub fn vertical(start_color: Color, end_color: Color) -> Self {
        Self {
            stops: vec![
                UGradientStop::new(0.0, start_color),
                UGradientStop::new(1.0, end_color),
            ],
            kind: UGradientKind::Linear {
                angle: core::f32::consts::FRAC_PI_2,
            },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a linear gradient with an explicit angle in radians.
    pub fn linear(start_color: Color, end_color: Color, angle: f32) -> Self {
        Self {
            stops: vec![
                UGradientStop::new(0.0, start_color),
                UGradientStop::new(1.0, end_color),
            ],
            kind: UGradientKind::Linear { angle },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a multi-stop linear gradient along an explicit angle in radians.
    pub fn linear_stops<I, S>(angle: f32, stops: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<UGradientStop>,
    {
        let mut stops_vec: Vec<UGradientStop> = stops
            .into_iter()
            .map(Into::into)
            .take(MAX_GRADIENT_STOPS)
            .collect();
        stops_vec.sort_by(|a, b| a.position.total_cmp(&b.position));
        Self {
            stops: stops_vec,
            kind: UGradientKind::Linear { angle },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a radial gradient centered in the node.
    pub fn radial(start_color: Color, end_color: Color) -> Self {
        Self {
            stops: vec![
                UGradientStop::new(0.0, start_color),
                UGradientStop::new(1.0, end_color),
            ],
            kind: UGradientKind::Radial {
                center: Vec2::splat(0.5),
                radius: 0.5,
            },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a radial gradient with explicit center and radius.
    pub fn radial_custom(start_color: Color, end_color: Color, center: Vec2, radius: f32) -> Self {
        Self {
            stops: vec![
                UGradientStop::new(0.0, start_color),
                UGradientStop::new(1.0, end_color),
            ],
            kind: UGradientKind::Radial { center, radius },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a multi-stop radial gradient centered in the node.
    pub fn radial_stops<I, S>(stops: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<UGradientStop>,
    {
        let mut stops_vec: Vec<UGradientStop> = stops
            .into_iter()
            .map(Into::into)
            .take(MAX_GRADIENT_STOPS)
            .collect();
        stops_vec.sort_by(|a, b| a.position.total_cmp(&b.position));
        Self {
            stops: stops_vec,
            kind: UGradientKind::Radial {
                center: Vec2::splat(0.5),
                radius: 0.5,
            },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Creates a multi-stop radial gradient with custom center and radius.
    pub fn radial_stops_custom<I, S>(center: Vec2, radius: f32, stops: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<UGradientStop>,
    {
        let mut stops_vec: Vec<UGradientStop> = stops
            .into_iter()
            .map(Into::into)
            .take(MAX_GRADIENT_STOPS)
            .collect();
        stops_vec.sort_by(|a, b| a.position.total_cmp(&b.position));
        Self {
            stops: stops_vec,
            kind: UGradientKind::Radial { center, radius },
            interpolation: UGradientInterpolation::Smooth,
        }
    }

    /// Switches the gradient to stepped interpolation (hard edges, discrete color bands).
    pub fn stepped(mut self) -> Self {
        self.interpolation = UGradientInterpolation::Stepped;
        self
    }

    /// Sets the gradient interpolation mode explicitly.
    pub fn with_interpolation(mut self, interpolation: UGradientInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Appends or inserts a color stop, maintaining ascending position order.
    pub fn with_stop(mut self, position: f32, color: Color) -> Self {
        if self.stops.len() < MAX_GRADIENT_STOPS {
            self.stops.push(UGradientStop::new(position, color));
            self.stops.sort_by(|a, b| a.position.total_cmp(&b.position));
        }
        self
    }

    /// Returns the color of the first stop, or white if empty.
    pub fn start_color(&self) -> Color {
        self.stops.first().map_or(Color::WHITE, |s| s.color)
    }

    /// Returns the color of the last stop, or white if empty.
    pub fn end_color(&self) -> Color {
        self.stops.last().map_or(Color::WHITE, |s| s.color)
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
        assert_eq!(r.stops.len(), 2);
        assert_eq!(r.start_color(), Color::WHITE);
        assert_eq!(r.end_color(), Color::BLACK);
    }

    #[test]
    fn multi_stop_gradient_sorts_and_clamps() {
        let grad = UGradient::linear_stops(
            1.57,
            vec![
                (1.0, Color::srgb(1.0, 0.0, 0.0)),
                (0.0, Color::srgb(0.0, 0.0, 1.0)),
                (0.5, Color::srgb(0.0, 1.0, 0.0)),
            ],
        );
        assert_eq!(grad.stops.len(), 3);
        assert_eq!(grad.stops[0].position, 0.0);
        assert_eq!(grad.stops[1].position, 0.5);
        assert_eq!(grad.stops[2].position, 1.0);
        assert_eq!(grad.start_color(), Color::srgb(0.0, 0.0, 1.0));
        assert_eq!(grad.end_color(), Color::srgb(1.0, 0.0, 0.0));

        let builder = UGradient::linear(Color::BLACK, Color::WHITE, 0.0)
            .with_stop(0.3, Color::srgb(0.5, 0.5, 0.5));
        assert_eq!(builder.stops.len(), 3);
        assert_eq!(builder.stops[1].position, 0.3);
    }

    #[test]
    fn gradient_interpolation_modes() {
        let smooth = UGradient::linear(Color::BLACK, Color::WHITE, 0.0);
        assert_eq!(smooth.interpolation, UGradientInterpolation::Smooth);

        let stepped = smooth.stepped();
        assert_eq!(stepped.interpolation, UGradientInterpolation::Stepped);

        let custom = stepped.with_interpolation(UGradientInterpolation::Smooth);
        assert_eq!(custom.interpolation, UGradientInterpolation::Smooth);
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
