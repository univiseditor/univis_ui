//! Expressive spring physics, staggered entrances, and micro-interaction motion for Univis UI.
//!
//! Provides:
//! - [`UMotionEnter`] for fluid staggered cascading entrance animations (slide, scale, and fade).
//! - [`UMotionPunch`] for haptic button feedback, physical pulses, and error shakes.
//! - [`UMotionColor`] for smooth, continuous color blending during hover and focus events.

use bevy::prelude::*;

use crate::layout::univis_node::{UBorder, UNode};

/// Preset configurations for spring dynamics.
#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct UMotionSpringConfig {
    /// Spring stiffness / angular frequency squared (higher = faster response).
    pub stiffness: f32,
    /// Damping ratio (1.0 = critically damped, < 1.0 = bouncy overshoot, > 1.0 = overdamped).
    pub damping: f32,
}

impl Default for UMotionSpringConfig {
    fn default() -> Self {
        Self::snappy()
    }
}

impl UMotionSpringConfig {
    /// Responsive and snappy with no overshoot (stiffness 280, damping 1.0).
    pub const fn snappy() -> Self {
        Self {
            stiffness: 280.0,
            damping: 1.0,
        }
    }

    /// Playful spring with visible, pleasant bounce (stiffness 220, damping 0.68).
    pub const fn bouncy() -> Self {
        Self {
            stiffness: 220.0,
            damping: 0.68,
        }
    }

    /// Soft, organic glide (stiffness 120, damping 1.0).
    pub const fn gentle() -> Self {
        Self {
            stiffness: 120.0,
            damping: 1.0,
        }
    }

    /// Highly resonant wobble for dramatic game feedback (stiffness 320, damping 0.45).
    pub const fn wobbly() -> Self {
        Self {
            stiffness: 320.0,
            damping: 0.45,
        }
    }
}

/// Configures a physics-driven entrance animation for a UI entity when it appears.
///
/// Supports translation slide, scale pop, alpha fade, and optional stagger delay.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UMotionEnter {
    /// Initial displacement offset in physical pixels that glides to `Vec2::ZERO`.
    pub slide_offset: Vec2,
    /// Initial scale multiplier (e.g. 0.8) that springs up to `1.0`.
    pub scale_start: f32,
    /// Delay in seconds before the entrance motion begins (ideal for staggered lists).
    pub delay: f32,
    /// Whether to smoothly fade in the alpha channel from 0.0 to 1.0.
    pub fade_alpha: bool,
    /// Spring configuration driving the motion.
    pub spring: UMotionSpringConfig,

    // Runtime state
    pub(crate) current_offset: Vec2,
    pub(crate) offset_velocity: Vec2,
    pub(crate) current_scale: f32,
    pub(crate) scale_velocity: f32,
    pub(crate) elapsed: f32,
    pub(crate) completed: bool,
    pub(crate) base_bg_alpha: Option<f32>,
}

impl Default for UMotionEnter {
    fn default() -> Self {
        Self {
            slide_offset: Vec2::new(0.0, -32.0),
            scale_start: 0.9,
            delay: 0.0,
            fade_alpha: true,
            spring: UMotionSpringConfig::bouncy(),
            current_offset: Vec2::new(0.0, -32.0),
            offset_velocity: Vec2::ZERO,
            current_scale: 0.9,
            scale_velocity: 0.0,
            elapsed: 0.0,
            completed: false,
            base_bg_alpha: None,
        }
    }
}

impl UMotionEnter {
    /// Slide up from bottom with subtle scale pop and bounce.
    pub fn slide_up(distance_px: f32) -> Self {
        Self {
            slide_offset: Vec2::new(0.0, -distance_px),
            current_offset: Vec2::new(0.0, -distance_px),
            ..Default::default()
        }
    }

    /// Slide in from left with subtle scale pop.
    pub fn slide_left(distance_px: f32) -> Self {
        Self {
            slide_offset: Vec2::new(-distance_px, 0.0),
            current_offset: Vec2::new(-distance_px, 0.0),
            ..Default::default()
        }
    }

    /// Pure scale pop from center without translation slide.
    pub fn pop(scale_start: f32) -> Self {
        Self {
            slide_offset: Vec2::ZERO,
            current_offset: Vec2::ZERO,
            scale_start,
            current_scale: scale_start,
            ..Default::default()
        }
    }

    /// Sets staggered delay in seconds.
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// Sets custom spring characteristics.
    pub fn with_spring(mut self, spring: UMotionSpringConfig) -> Self {
        self.spring = spring;
        self
    }

    /// Whether this entrance animation has settled into resting state.
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

/// Applies reactive physical pulses, impact recoil, and error shakes to an entity.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UMotionPunch {
    /// Added offset displacement glided back to zero.
    pub offset: Vec2,
    pub offset_velocity: Vec2,
    /// Added scale factor glided back to zero.
    pub scale_offset: Vec2,
    pub scale_velocity: Vec2,
    /// Spring configuration restoring the component to rest.
    pub spring: UMotionSpringConfig,
}

impl Default for UMotionPunch {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            offset_velocity: Vec2::ZERO,
            scale_offset: Vec2::ZERO,
            scale_velocity: Vec2::ZERO,
            spring: UMotionSpringConfig::wobbly(),
        }
    }
}

impl UMotionPunch {
    /// Creates a punch component with custom spring tuning.
    pub fn new(spring: UMotionSpringConfig) -> Self {
        Self {
            spring,
            ..Default::default()
        }
    }

    /// Triggers an immediate scale pulse (e.g. `Vec2::splat(0.25)` on click).
    pub fn punch_scale(&mut self, velocity: Vec2) {
        self.scale_velocity += velocity;
    }

    /// Triggers an immediate positional shake (e.g. `Vec2::new(18.0, 0.0)` for error buzz).
    pub fn punch_offset(&mut self, velocity: Vec2) {
        self.offset_velocity += velocity;
    }
}

/// Smooth continuous color blending for interactive background or border transitions.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UMotionColor {
    /// Desired target background color.
    pub target_background: Color,
    /// Desired target border color (if border present).
    pub target_border: Option<Color>,
    /// Interpolation rate (higher = snappier color glide, default 16.0).
    pub speed: f32,
}

impl Default for UMotionColor {
    fn default() -> Self {
        Self {
            target_background: Color::WHITE,
            target_border: None,
            speed: 16.0,
        }
    }
}

impl UMotionColor {
    /// Creates a smooth color glide towards `target_background`.
    pub fn to_background(target_background: Color) -> Self {
        Self {
            target_background,
            target_border: None,
            speed: 16.0,
        }
    }

    /// Sets transition speed.
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

/// Advances entrance animations (slide, scale, and alpha fade).
pub fn advance_motion_enter_system(
    time: Option<Res<Time>>,
    mut query: Query<(&mut UMotionEnter, &mut Transform, &mut UNode)>,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }

    for (mut enter, mut transform, mut node) in query.iter_mut() {
        if enter.completed {
            continue;
        }

        enter.elapsed += dt;
        if enter.elapsed < enter.delay {
            // Still in delay phase: hold initial offset and scale
            transform.translation.x += enter.slide_offset.x;
            transform.translation.y += enter.slide_offset.y;
            transform.scale *= enter.scale_start;
            if enter.fade_alpha {
                if enter.base_bg_alpha.is_none() {
                    enter.base_bg_alpha = Some(node.background_color.alpha());
                }
                node.background_color = node.background_color.with_alpha(0.0);
            }
            continue;
        }

        if enter.base_bg_alpha.is_none() {
            enter.base_bg_alpha = Some(node.background_color.alpha());
        }

        // 1. Translation spring towards Vec2::ZERO
        let omega = enter.spring.stiffness.sqrt();
        let diff_offset = Vec2::ZERO - enter.current_offset;
        let spring_force_o = diff_offset * enter.spring.stiffness;
        let damping_force_o = enter.offset_velocity * (2.0 * enter.spring.damping * omega);
        let accel_o = spring_force_o - damping_force_o;
        enter.offset_velocity += accel_o * dt;
        let offset_delta = enter.offset_velocity * dt;
        enter.current_offset += offset_delta;

        // 2. Scale spring towards 1.0
        let diff_scale = 1.0 - enter.current_scale;
        let spring_force_s = diff_scale * enter.spring.stiffness;
        let damping_force_s = enter.scale_velocity * (2.0 * enter.spring.damping * omega);
        let accel_s = spring_force_s - damping_force_s;
        enter.scale_velocity += accel_s * dt;
        let scale_delta = enter.scale_velocity * dt;
        enter.current_scale += scale_delta;

        // Apply to transform
        transform.translation.x += enter.current_offset.x;
        transform.translation.y += enter.current_offset.y;
        transform.scale *= enter.current_scale;

        // 3. Alpha fade in
        if enter.fade_alpha {
            if let Some(base_alpha) = enter.base_bg_alpha {
                let progress = (1.0 - (enter.current_offset.length() / (enter.slide_offset.length().max(1.0)))).clamp(0.0, 1.0);
                node.background_color = node.background_color.with_alpha(base_alpha * progress);
            }
        }

        // Check completion
        if diff_offset.length_squared() < 0.0001
            && enter.offset_velocity.length_squared() < 0.0001
            && diff_scale.abs() < 0.001
            && enter.scale_velocity.abs() < 0.001
        {
            enter.completed = true;
            enter.current_offset = Vec2::ZERO;
            enter.current_scale = 1.0;
            if enter.fade_alpha {
                if let Some(base_alpha) = enter.base_bg_alpha {
                    node.background_color = node.background_color.with_alpha(base_alpha);
                }
            }
        }
    }
}

/// Advances haptic punch, pulse, and recoil physics.
pub fn advance_motion_punch_system(
    time: Option<Res<Time>>,
    mut query: Query<(&mut UMotionPunch, &mut Transform)>,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }

    for (mut punch, mut transform) in query.iter_mut() {
        let omega = punch.spring.stiffness.sqrt();

        // 1. Position recoil restoration
        let diff_o = Vec2::ZERO - punch.offset;
        let force_o = diff_o * punch.spring.stiffness;
        let damp_o = punch.offset_velocity * (2.0 * punch.spring.damping * omega);
        punch.offset_velocity += (force_o - damp_o) * dt;
        let o_dt = punch.offset_velocity * dt;
        punch.offset += o_dt;

        // 2. Scale pulse restoration
        let diff_s = Vec2::ZERO - punch.scale_offset;
        let force_s = diff_s * punch.spring.stiffness;
        let damp_s = punch.scale_velocity * (2.0 * punch.spring.damping * omega);
        punch.scale_velocity += (force_s - damp_s) * dt;
        let s_dt = punch.scale_velocity * dt;
        punch.scale_offset += s_dt;

        transform.translation.x += punch.offset.x;
        transform.translation.y += punch.offset.y;
        transform.scale.x = (transform.scale.x + punch.scale_offset.x).max(0.01);
        transform.scale.y = (transform.scale.y + punch.scale_offset.y).max(0.01);

        if diff_o.length_squared() < 0.00001
            && punch.offset_velocity.length_squared() < 0.00001
            && diff_s.length_squared() < 0.00001
            && punch.scale_velocity.length_squared() < 0.00001
        {
            punch.offset = Vec2::ZERO;
            punch.scale_offset = Vec2::ZERO;
        }
    }
}

/// Advances smooth continuous color transitions for background and border.
pub fn advance_motion_color_system(
    time: Option<Res<Time>>,
    mut query: Query<(&UMotionColor, &mut UNode, Option<&mut UBorder>)>,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }

    for (m_color, mut node, border_opt) in query.iter_mut() {
        let blend = (1.0 - (-m_color.speed * dt).exp()).clamp(0.0, 1.0);

        // Blend background color in LinearRgba space
        let current_bg = LinearRgba::from(node.background_color);
        let target_bg = LinearRgba::from(m_color.target_background);
        let blended_bg = LinearRgba::new(
            current_bg.red + (target_bg.red - current_bg.red) * blend,
            current_bg.green + (target_bg.green - current_bg.green) * blend,
            current_bg.blue + (target_bg.blue - current_bg.blue) * blend,
            current_bg.alpha + (target_bg.alpha - current_bg.alpha) * blend,
        );
        node.background_color = Color::LinearRgba(blended_bg);

        // Blend border color if specified
        if let (Some(mut border), Some(target_b_color)) = (border_opt, m_color.target_border) {
            let current_b = LinearRgba::from(border.color);
            let target_b = LinearRgba::from(target_b_color);
            let blended_b = LinearRgba::new(
                current_b.red + (target_b.red - current_b.red) * blend,
                current_b.green + (target_b.green - current_b.green) * blend,
                current_b.blue + (target_b.blue - current_b.blue) * blend,
                current_b.alpha + (target_b.alpha - current_b.alpha) * blend,
            );
            border.color = Color::LinearRgba(blended_b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_spring_presets() {
        let bouncy = UMotionSpringConfig::bouncy();
        assert!(bouncy.damping < 1.0);

        let snappy = UMotionSpringConfig::snappy();
        assert_eq!(snappy.damping, 1.0);
    }

    #[test]
    fn test_motion_punch_impulse() {
        let mut app = App::new();
        app.add_systems(Update, advance_motion_punch_system);

        let entity = app
            .world_mut()
            .spawn((
                UMotionPunch::default(),
                Transform::default(),
            ))
            .id();

        // Trigger punch
        app.world_mut()
            .get_mut::<UMotionPunch>(entity)
            .unwrap()
            .punch_scale(Vec2::splat(10.0));

        // Advance simulation
        let mut time: Time<()> = Time::default();
        time.advance_by(std::time::Duration::from_millis(16));
        app.insert_resource(time);
        app.update();

        let punch = app.world().get::<UMotionPunch>(entity).unwrap();
        assert!(punch.scale_offset.x > 0.0);
    }

    #[test]
    fn test_motion_color_glide() {
        let mut app = App::new();
        app.add_systems(Update, advance_motion_color_system);

        let entity = app
            .world_mut()
            .spawn((
                UNode {
                    background_color: Color::BLACK,
                    ..default()
                },
                UMotionColor::to_background(Color::WHITE).with_speed(20.0),
            ))
            .id();

        let mut time: Time<()> = Time::default();
        time.advance_by(std::time::Duration::from_millis(32));
        app.insert_resource(time);
        app.update();

        let node = app.world().get::<UNode>(entity).unwrap();
        let bg_linear = LinearRgba::from(node.background_color);
        // Color should have moved from 0.0 towards 1.0
        assert!(bg_linear.red > 0.1);
    }
}
