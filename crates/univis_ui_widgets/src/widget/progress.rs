use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
use univis_ui_engine::layout::univis_node::{ULayout, UNode};

// 1. Component
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct UProgressBar {
    pub value: f32, // Normalized value in the `0.0..=1.0` range.
    pub bar_color: Color,
}

impl Default for UProgressBar {
    fn default() -> Self {
        Self {
            value: 0.5,
            bar_color: Color::srgb(0.2, 0.8, 0.2),
        }
    }
}

pub struct UnivisProgressPlugin;

impl Plugin for UnivisProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_progress_bars);
    }
}

// Marker for the internal fill bar child.
#[derive(Component)]
struct ProgressBarFill;

// 2. Visual update system
fn update_progress_bars(
    mut commands: Commands,
    query: Query<(Entity, &UProgressBar, Option<&Children>), Changed<UProgressBar>>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<ProgressBarFill>>,
) {
    for (entity, bar, children_opt) in query.iter() {
        // Ensure the outer container is initialized.
        commands.entity(entity).insert(UNode {
            height: UVal::Px(10.0),    // Default height
            width: UVal::Percent(1.0), // Full width
            background_color: Color::BLACK.with_alpha(0.3),
            border_radius: UCornerRadius::all(5.0),
            padding: USides::all(0.0), // No inner padding
            ..default()
        });

        let mut fill_found = false;

        // Find the child that owns the fill visual.
        if let Some(children) = children_opt {
            for &child in children {
                if let Ok((mut node, mut vis)) = fill_query.get_mut(child) {
                    // Update width from the current normalized value.
                    let clamped = bar.value.clamp(0.0, 1.0);
                    node.width = UVal::Percent(clamped);
                    node.background_color = bar.bar_color;

                    // Hide the fill when the value is effectively zero.
                    *vis = if clamped > 0.001 {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };

                    fill_found = true;
                    break;
                }
            }
        }

        // Lazily create the fill child if it does not exist yet.
        if !fill_found {
            commands
                .entity(entity)
                .insert(UNode {
                    height: UVal::Px(10.0),    // Default height
                    width: UVal::Percent(1.0), // Full width
                    background_color: Color::BLACK.with_alpha(0.3),
                    border_radius: UCornerRadius::all(5.0),
                    padding: USides::all(0.0), // No inner padding
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        UNode {
                            width: UVal::Percent(bar.value.clamp(0.0, 1.0)),
                            height: UVal::Percent(1.0), // Fill the parent height
                            background_color: bar.bar_color,
                            border_radius: UCornerRadius::all(5.0),
                            ..default()
                        },
                        ProgressBarFill,
                    ));
                });
        }
    }
}
