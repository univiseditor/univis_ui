use crate::widget::text_label::UTextLabel;
use bevy::picking::Pickable;
use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UBorder, UJustifyContent, ULayout, UNode,
};

/// Registers the checkbox widget.
pub struct UnivisCheckboxPlugin;

impl Plugin for UnivisCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UCheckbox>()
            .add_systems(
                Update,
                (
                    init_checkbox, // Build visuals on first insertion.
                                   // update_checkbox_visuals // Refresh colors and visibility.
                ),
            )
            .add_observer(toggle_checkbox_handler);
    }
}

// =========================================================
// 1. Components
// =========================================================

/// A simple checkbox with an optional text label.
///
/// The checkbox entity becomes the interactive parent, while its box and label
/// visuals are spawned as children on initialization.
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UCheckbox {
    pub checked: bool,
    pub label: Option<String>,

    // --- Style Config ---
    pub size: f32,
    pub checked_color: Color,
    pub unchecked_color: Color,
    pub border_color: Color,
}

impl Default for UCheckbox {
    fn default() -> Self {
        Self {
            checked: false,
            label: None,
            size: 24.0,
            checked_color: Color::srgb(0.2, 0.5, 0.9), // Blue
            unchecked_color: Color::srgb(0.2, 0.2, 0.2), // Dark gray
            border_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

// =========================================================
// 2. Systems
// =========================================================

/// Builds the checkbox visuals the first time the component is added.
fn init_checkbox(
    mut commands: Commands,
    query: Query<(Entity, &UCheckbox), Added<UCheckbox>>,
    _asset_server: Res<AssetServer>, // Reserved for default-font access if needed later.
) {
    for (entity, checkbox) in query.iter() {
        // Configure the main row container.
        commands
            .entity(entity)
            .insert((
                UNode {
                    // Let size follow the box plus optional label.
                    width: UVal::Content,
                    height: UVal::Content,
                    padding: USides::all(4.0), // Expand the hit area slightly.
                    background_color: Color::NONE, // Keep the wrapper visually transparent.
                    ..default()
                },
                ULayout {
                    align_items: UAlignItems::Center, // Align the text baseline with the box.
                    justify_content: UJustifyContent::Center,
                    gap: 8.0, // Space between the box and label.
                    ..default()
                },
            ))
            .with_children(|parent| {
                let color;
                let border_color;
                if checkbox.checked {
                    color = checkbox.checked_color;
                    border_color = checkbox.border_color;
                } else {
                    border_color = checkbox.border_color;
                    color = checkbox.unchecked_color;
                }

                // Spawn the square box visual.
                parent.spawn((
                    UNode {
                        width: UVal::Px(checkbox.size),
                        height: UVal::Px(checkbox.size),
                        border_radius: UCornerRadius::all(checkbox.size * 0.25), // Slightly rounded corners.
                        background_color: color,
                        ..default()
                    },
                    UBorder {
                        width: 2.0,
                        color: border_color,
                        offset: 4.0,
                        radius: UCornerRadius::all(checkbox.size * 0.25),
                    },
                    // This child is discovered later to refresh its visual state.
                ));

                // Spawn the optional label.
                if let Some(text) = &checkbox.label {
                    parent.spawn((
                        UTextLabel {
                            text: text.clone(),
                            font_size: checkbox.size * 0.75,
                            color: Color::WHITE,
                            ..default()
                        },
                        Pickable::IGNORE,
                    ));
                }
            });
    }
}

/// Toggles the checkbox state and refreshes the box visuals.
fn toggle_checkbox_handler(
    trigger: On<Pointer<Click>>,
    mut box_query: Query<(&mut UNode, &mut UBorder)>,
    mut parent_query: Query<(&mut UCheckbox, &Children)>,
) {
    let entity = trigger.entity.entity();
    if let Ok((mut checkbox, child)) = parent_query.get_mut(entity) {
        checkbox.checked = !checkbox.checked;

        for &child in child {
            if let Ok((mut node, mut border)) = box_query.get_mut(child) {
                if checkbox.checked {
                    node.background_color = checkbox.checked_color;
                    border.color = checkbox.checked_color; // Hide the border when checked for a cleaner look.
                } else {
                    node.background_color = checkbox.unchecked_color;
                    border.color = checkbox.border_color;
                }
            }
        }
    }
}

impl UCheckbox {
    /// Creates a checkbox with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            ..default()
        }
    }

    /// Sets the initial checked state.
    pub fn checked(mut self, state: bool) -> Self {
        self.checked = state;
        self
    }

    /// Overrides the accent color used when the checkbox is checked.
    pub fn with_color(mut self, color: Color) -> Self {
        self.checked_color = color;
        self
    }
}
