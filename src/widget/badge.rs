use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisBadgePlugin;

impl Plugin for UnivisBadgePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UBadge>()
            .add_systems(Update, update_badge_visuals);
    }
}

// --- المكونات (Components) ---

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(ULayout)]
pub struct UBadge {
    pub style: BadgeStyle,
    pub size: BadgeSize,
}

#[derive(Clone, Copy, Reflect)]
pub enum BadgeStyle {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Copy, Reflect)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

impl Default for UBadge {
    fn default() -> Self {
        Self {
            style: BadgeStyle::Primary,
            size: BadgeSize::Medium,
        }
    }
}

// --- الأنظمة (Systems) ---

fn update_badge_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UBadge), Changed<UBadge>>,
) {
    for (entity, badge) in query.iter() {
        let (bg_color, padding, border_radius) = match badge.style {
            BadgeStyle::Primary => (
                Color::srgb(0.2, 0.5, 0.9),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
            BadgeStyle::Secondary => (
                Color::srgb(0.4, 0.4, 0.4),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
            BadgeStyle::Success => (
                Color::srgb(0.2, 0.8, 0.3),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
            BadgeStyle::Warning => (
                Color::srgb(0.9, 0.7, 0.2),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
            BadgeStyle::Danger => (
                Color::srgb(0.9, 0.2, 0.2),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
            BadgeStyle::Info => (
                Color::srgb(0.3, 0.7, 0.9),
                get_badge_padding(badge.size),
                get_badge_radius(badge.size),
            ),
        };

        commands.entity(entity).insert(UNode {
            padding,
            background_color: bg_color,
            border_radius,
            ..default()
        });
    }
}

// --- Helper Functions ---

fn get_badge_padding(size: BadgeSize) -> USides {
    match size {
        BadgeSize::Small => USides::axes(6.0, 2.0),
        BadgeSize::Medium => USides::axes(10.0, 4.0),
        BadgeSize::Large => USides::axes(14.0, 6.0),
    }
}

fn get_badge_radius(size: BadgeSize) -> UCornerRadius {
    match size {
        BadgeSize::Small => UCornerRadius::all(8.0),
        BadgeSize::Medium => UCornerRadius::all(12.0),
        BadgeSize::Large => UCornerRadius::all(16.0),
    }
}

impl UBadge {
    /// شارة أساسية
    pub fn primary() -> Self {
        Self {
            style: BadgeStyle::Primary,
            ..default()
        }
    }

    /// شارة نجاح
    pub fn success() -> Self {
        Self {
            style: BadgeStyle::Success,
            ..default()
        }
    }

    /// شارة تحذير
    pub fn warning() -> Self {
        Self {
            style: BadgeStyle::Warning,
            ..default()
        }
    }

    /// شارة خطر
    pub fn danger() -> Self {
        Self {
            style: BadgeStyle::Danger,
            ..default()
        }
    }

    /// شارة معلومات
    pub fn info() -> Self {
        Self {
            style: BadgeStyle::Info,
            ..default()
        }
    }

    /// شارة صغيرة
    pub fn small(mut self) -> Self {
        self.size = BadgeSize::Small;
        self
    }

    /// شارة كبيرة
    pub fn large(mut self) -> Self {
        self.size = BadgeSize::Large;
        self
    }
}

// =========================================================
// UTag - وسم قابل للإزالة
// =========================================================

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(ULayout, Pickable)]
pub struct UTag {
    pub closable: bool,
    pub style: BadgeStyle,
}

impl Default for UTag {
    fn default() -> Self {
        Self {
            closable: true,
            style: BadgeStyle::Primary,
        }
    }
}