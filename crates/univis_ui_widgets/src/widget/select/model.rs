use bevy::prelude::*;
use univis_ui_engine::layout::geometry::USides;
use univis_ui_engine::layout::univis_node::{ULayout, UNode};

/// A single option inside a [`USelect`] widget.
#[derive(Clone, Debug, Reflect)]
pub struct USelectOption {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}

impl USelectOption {
    /// Creates an enabled option from a label/value pair.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Marks the option as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A single-select dropdown widget.
///
/// The widget owns its options, open state, keyboard highlight, and selected value.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct USelect {
    pub options: Vec<USelectOption>,
    pub selected_index: Option<usize>,
    pub(crate) previous_selected_index: Option<usize>,
    pub highlighted_index: Option<usize>,
    pub placeholder: String,
    pub is_open: bool,
    pub(crate) previous_open: bool,
    pub width: f32,
    pub trigger_height: f32,
    pub max_visible_options: usize,
    pub disabled: bool,
    pub font_size: f32,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub background: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub dropdown_background: Color,
    pub border_color: Color,
    pub option_hover_color: Color,
    pub option_selected_color: Color,
    pub option_disabled_text_color: Color,
    pub padding: USides,
}

impl Default for USelect {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected_index: None,
            previous_selected_index: None,
            highlighted_index: None,
            placeholder: "Select an option".to_string(),
            is_open: false,
            previous_open: false,
            width: 220.0,
            trigger_height: 38.0,
            max_visible_options: 6,
            disabled: false,
            font_size: 15.0,
            text_color: Color::WHITE,
            placeholder_color: Color::srgb(0.62, 0.67, 0.76),
            background: Color::srgb(0.14, 0.16, 0.22),
            hover_color: Color::srgb(0.18, 0.2, 0.28),
            pressed_color: Color::srgb(0.12, 0.14, 0.2),
            dropdown_background: Color::srgb(0.12, 0.14, 0.2),
            border_color: Color::srgb(0.36, 0.42, 0.52),
            option_hover_color: Color::srgb(0.2, 0.28, 0.42),
            option_selected_color: Color::srgb(0.2, 0.34, 0.54),
            option_disabled_text_color: Color::srgb(0.45, 0.48, 0.56),
            padding: USides::axes(12.0, 8.0),
        }
    }
}

impl USelect {
    /// Creates an empty select widget with default styling.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the option list and revalidates the selection.
    pub fn with_options(mut self, options: Vec<USelectOption>) -> Self {
        self.options = options;
        sanitize_select(&mut self);
        self.previous_selected_index = self.selected_index;
        self
    }

    /// Overrides the placeholder text shown when nothing is selected.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Selects an option by enabled index.
    pub fn with_selected_index(mut self, index: usize) -> Self {
        if index < self.options.len() && !self.options[index].disabled {
            self.selected_index = Some(index);
        } else {
            self.selected_index = None;
        }
        self.highlighted_index = self.selected_index;
        self.previous_selected_index = self.selected_index;
        self
    }

    /// Selects the first enabled option whose value matches.
    pub fn with_selected_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        self.selected_index = self
            .options
            .iter()
            .position(|opt| !opt.disabled && opt.value == value);
        self.highlighted_index = self.selected_index;
        self.previous_selected_index = self.selected_index;
        self
    }

    /// Overrides the trigger width and height.
    pub fn with_size(mut self, width: f32, trigger_height: f32) -> Self {
        self.width = width.max(1.0);
        self.trigger_height = trigger_height.max(1.0);
        self
    }

    /// Limits how many options remain visible before scrolling is needed.
    pub fn with_max_visible_options(mut self, max_visible_options: usize) -> Self {
        self.max_visible_options = max_visible_options.max(1);
        self
    }

    /// Disables interaction and closes the dropdown if it is open.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self.is_open = false;
        self
    }
}

pub(super) fn sanitize_select(select: &mut USelect) {
    select.width = select.width.max(1.0);
    select.trigger_height = select.trigger_height.max(1.0);
    select.max_visible_options = select.max_visible_options.max(1);

    if select.options.is_empty() {
        select.selected_index = None;
        select.highlighted_index = None;
        select.is_open = false;
        return;
    }

    if !is_enabled_index(&select.options, select.selected_index) {
        select.selected_index = None;
    }

    if !is_enabled_index(&select.options, select.highlighted_index) {
        select.highlighted_index = None;
    }

    if select.is_open && select.highlighted_index.is_none() {
        select.highlighted_index = select
            .selected_index
            .filter(|idx| is_enabled_index(&select.options, Some(*idx)))
            .or_else(|| first_enabled_index(&select.options));
    }
}

pub(super) fn selected_option(select: &USelect) -> Option<&USelectOption> {
    select
        .selected_index
        .and_then(|index| select.options.get(index))
        .filter(|option| !option.disabled)
}

pub(super) fn is_enabled_index(options: &[USelectOption], index: Option<usize>) -> bool {
    index
        .and_then(|idx| options.get(idx))
        .map(|opt| !opt.disabled)
        .unwrap_or(false)
}

pub(super) fn first_enabled_index(options: &[USelectOption]) -> Option<usize> {
    options.iter().position(|opt| !opt.disabled)
}

pub(super) fn next_enabled_index(
    options: &[USelectOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    if options.is_empty() {
        return None;
    }

    if !options.iter().any(|opt| !opt.disabled) {
        return None;
    }

    let len = options.len() as i32;
    let step = if direction >= 0 { 1 } else { -1 };
    let mut index = match current {
        Some(i) if i < options.len() => i as i32,
        _ if step > 0 => -1,
        _ => 0,
    };

    for _ in 0..options.len() {
        index += step;
        if index < 0 {
            index = len - 1;
        }
        if index >= len {
            index = 0;
        }

        let idx = index as usize;
        if !options[idx].disabled {
            return Some(idx);
        }
    }

    None
}
