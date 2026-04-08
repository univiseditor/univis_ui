use super::*;

/// A single radio button that optionally belongs to a [`URadioGroup`].
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable, UInteraction)]
pub struct URadioButton {
    /// Unique value represented by this button inside a group.
    pub value: String,
    /// Whether this button is currently selected.
    pub checked: bool,
    /// Previous selection state used internally for change detection.
    pub(crate) previous_checked: bool,
    pub size: f32,
    pub ring_color: Color,
    pub ring_checked_color: Color,
    pub dot_color: Color,
    pub ring_width: f32,
    pub disabled: bool,
    pub animation_speed: f32,
    pub current_scale: f32,
    pub group: Option<Entity>,
}

impl Default for URadioButton {
    fn default() -> Self {
        Self {
            value: String::new(),
            checked: false,
            previous_checked: false,
            size: 20.0,
            ring_color: Color::srgb(0.4, 0.4, 0.45),
            ring_checked_color: Color::srgb(0.2, 0.6, 1.0),
            dot_color: Color::srgb(0.2, 0.6, 1.0),
            ring_width: 2.0,
            disabled: false,
            animation_speed: 12.0,
            current_scale: 0.0,
            group: None,
        }
    }
}

impl URadioButton {
    /// Creates a radio button with the provided value.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            ..default()
        }
    }

    /// Marks the button as initially selected.
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self.previous_checked = true;
        self.current_scale = 1.0;
        self
    }

    /// Overrides the button size.
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Overrides the ring and dot colors.
    pub fn with_colors(mut self, ring: Color, checked: Color, dot: Color) -> Self {
        self.ring_color = ring;
        self.ring_checked_color = checked;
        self.dot_color = dot;
        self
    }

    /// Marks the button as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn primary_style(value: impl Into<String>) -> Self {
        Self::new(value).with_colors(
            Color::srgb(0.4, 0.4, 0.45),
            Color::srgb(0.2, 0.6, 1.0),
            Color::srgb(0.2, 0.6, 1.0),
        )
    }

    pub fn success_style(value: impl Into<String>) -> Self {
        Self::new(value).with_colors(
            Color::srgb(0.3, 0.4, 0.35),
            Color::srgb(0.2, 0.8, 0.3),
            Color::srgb(0.2, 0.8, 0.3),
        )
    }

    pub fn danger_style(value: impl Into<String>) -> Self {
        Self::new(value).with_colors(
            Color::srgb(0.4, 0.3, 0.3),
            Color::srgb(0.9, 0.2, 0.2),
            Color::srgb(0.9, 0.2, 0.2),
        )
    }
}

/// A controller component that keeps a set of radio buttons mutually exclusive.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct URadioGroup {
    /// Currently selected value, if any.
    pub selected_value: Option<String>,
    /// Previous selected value used internally for change detection.
    pub(crate) previous_value: Option<String>,
    /// Buttons currently associated with this group.
    pub buttons: Vec<Entity>,
    /// Whether the group requires one option to stay selected.
    pub require_selection: bool,
    pub gap: f32,
    pub direction: UFlexDirection,
}

impl Default for URadioGroup {
    fn default() -> Self {
        Self {
            selected_value: None,
            previous_value: None,
            buttons: Vec::new(),
            require_selection: true,
            gap: 15.0,
            direction: UFlexDirection::Column,
        }
    }
}

impl URadioGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self.previous_value = self.selected_value.clone();
        self
    }

    pub fn allow_deselect(mut self) -> Self {
        self.require_selection = false;
        self
    }

    /// Switches the group layout to a horizontal row.
    pub fn horizontal(mut self) -> Self {
        self.direction = UFlexDirection::Row;
        self
    }

    /// Switches the group layout to a vertical column.
    pub fn vertical(mut self) -> Self {
        self.direction = UFlexDirection::Column;
        self
    }

    /// Overrides the gap between buttons.
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}
