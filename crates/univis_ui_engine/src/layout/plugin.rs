use bevy::prelude::*;

use super::registration::{install_layout_pipeline, register_layout_types};

/// Installs the layout pipeline for Univis UI roots and nodes.
///
/// This plugin resolves roots, builds the layout hierarchy, measures content,
/// solves final geometry, and prepares render sync data in `PostUpdate`.
pub struct UnivisLayoutPlugin;

impl Plugin for UnivisLayoutPlugin {
    fn build(&self, app: &mut App) {
        register_layout_types(app);
        install_layout_pipeline(app);
    }
}
