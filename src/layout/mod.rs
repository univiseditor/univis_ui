pub mod univis_node;
pub mod geomerty;
pub mod layout_system;
pub mod core;
pub mod render;
pub mod algorithms;
pub mod pipeline;
pub mod profiling;
/// A convenient module that re-exports all essential layout components and types.
///
/// Import this module to get access to nodes, geometry types, and the layout plugin.
pub mod prelude {
    pub use crate::layout::univis_node::*;
    pub use crate::layout::geomerty::*;
    pub use crate::layout::layout_system::*;
    pub use crate::layout::core::prelude::*;
    pub use crate::layout::render::prelude::*;
    pub use crate::layout::algorithms::prelude::*;
    pub use crate::layout::UnivisLayoutPlugin;
    pub use crate::layout::pipeline::prelude::*;
    pub use crate::layout::profiling::*;
}

use bevy::{prelude::*, sprite_render::Material2dPlugin};
pub use crate::layout::prelude::*;

/// The core Bevy Plugin that initializes the Univis Layout Engine.
///
/// This plugin is responsible for:
/// 1. Registering all custom UI types for Reflection (Inspector support).
/// 2. Initializing layout resources.
/// 3. Scheduling the core layout systems (Hierarchy -> Measure -> Solve).
/// 4. Setting up the rendering pipeline for SDF-based UI nodes.
pub struct UnivisLayoutPlugin;

impl Plugin for UnivisLayoutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // --- UI Core Setup ---
        
        app
        // Register types for Bevy Reflection (essential for Scene serialization and Inspector)
        .register_type::<USelf>()
        .register_type::<UAlignSelf>()
        .register_type::<UPosition>()

         // Initialize the resource tracking the maximum depth of the UI tree
         .init_resource::<LayoutTreeDepth>()
         
        //  .register_type::<UFlexGrow>()
        //  .register_type::<UFlexShrink>() // Registered if used

         // --- Layout Systems Pipeline ---
         // The order of these systems is critical for the layout algorithm to function correctly.
         // They run in PostUpdate to ensure they process the latest frame data.
         .add_plugins(LayoutCachePlugin)
         .add_systems(PostUpdate, (
             // 1. Calculate the depth of every node in the tree.
             update_layout_hierarchy,
             // 2. Pass Up: Calculate intrinsic sizes (Children -> Parent).
             upward_measure_pass,
             // 3. Pass Down: Enforce constraints and determine final positions (Parent -> Children).
             downward_solve_pass_safe,
         ).chain());

        // --- UI Render Setup ---
        app
         // Initialize the Material2d plugin for our custom SDF shader material
         .add_plugins(Material2dPlugin::<UNodeMaterial>::default())
         .add_plugins(MaterialPlugin::<UNodeMaterial3d>::default())
         .register_type::<UI3d>()
         .register_type::<UPbr>()
         
         // Sync the computed layout data (Size, Borders) to the GPU/Shader
         // This runs after the layout has been solved.
         .init_resource::<MaterialPool>()

         .add_systems(PostUpdate, (
            auto_propagate_ui3d.before(update_materials_optimized),
            update_materials_optimized
        )); 
        
    }
}