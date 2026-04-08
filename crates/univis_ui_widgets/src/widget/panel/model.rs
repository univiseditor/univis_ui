use super::*;

/// A styled container panel built on top of [`UNode`] and [`ULayout`].
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct UPanel {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: UCornerRadius,
    pub padding: USides,
    pub gap: f32,
    pub direction: UFlexDirection,
}

impl Default for UPanel {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.13, 0.15, 0.2),
            border_color: Color::srgba(0.75, 0.8, 0.9, 0.2),
            border_width: 1.0,
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(12.0),
            gap: 10.0,
            direction: UFlexDirection::Column,
        }
    }
}

impl UPanel {
    /// Returns the default card-like panel style.
    pub fn card() -> Self {
        Self::default()
    }

    /// Returns a translucent glass-like panel style.
    pub fn glass() -> Self {
        Self {
            background: Color::srgba(0.08, 0.1, 0.14, 0.72),
            border_color: Color::srgba(0.9, 0.95, 1.0, 0.25),
            ..default()
        }
    }

    /// Overrides the panel padding.
    pub fn with_padding(mut self, padding: USides) -> Self {
        self.padding = padding;
        self
    }

    /// Overrides the gap between direct children.
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Overrides the main layout direction.
    pub fn with_direction(mut self, direction: UFlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Overrides the panel background color.
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
}

/// Enables resize handles around a panel-like entity.
///
/// This is typically paired with [`UPanel`] to create floating tool windows.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UPanelWindow {
    pub border_hit_thickness: f32,
    pub min_width: f32,
    pub min_height: f32,
}

impl Default for UPanelWindow {
    fn default() -> Self {
        Self {
            border_hit_thickness: 8.0,
            min_width: 180.0,
            min_height: 120.0,
        }
    }
}

impl UPanelWindow {
    /// Sets the minimum allowed width and height while resizing.
    pub fn with_min_size(mut self, width: f32, height: f32) -> Self {
        self.min_width = width.max(1.0);
        self.min_height = height.max(1.0);
        self
    }

    /// Sets the thickness of the invisible resize hit region.
    pub fn with_border_hit_thickness(mut self, thickness: f32) -> Self {
        self.border_hit_thickness = thickness.max(1.0);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PanelResizeEdge {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl PanelResizeEdge {
    pub(super) const ALL: [Self; 8] = [
        Self::N,
        Self::S,
        Self::E,
        Self::W,
        Self::NE,
        Self::NW,
        Self::SE,
        Self::SW,
    ];

    pub(super) fn has_north(self) -> bool {
        matches!(self, Self::N | Self::NE | Self::NW)
    }

    pub(super) fn has_south(self) -> bool {
        matches!(self, Self::S | Self::SE | Self::SW)
    }

    pub(super) fn has_east(self) -> bool {
        matches!(self, Self::E | Self::NE | Self::SE)
    }

    pub(super) fn has_west(self) -> bool {
        matches!(self, Self::W | Self::NW | Self::SW)
    }

    pub(super) fn is_corner(self) -> bool {
        matches!(self, Self::NE | Self::NW | Self::SE | Self::SW)
    }
}

#[derive(Component, Clone, Copy)]
pub(super) struct PanelResizeHandle {
    pub(super) owner: Entity,
    pub(super) edge: PanelResizeEdge,
}

#[derive(Component)]
pub(super) struct PanelResizeChrome;

#[derive(Component, Default)]
pub(super) struct PanelResizeRuntime {
    pub(super) active_edge: Option<PanelResizeEdge>,
    pub(super) last_parent_cursor: Option<Vec2>,
}
