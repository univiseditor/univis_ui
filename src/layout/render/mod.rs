use bevy::{asset::embedded_asset, prelude::*, sprite_render::Material2dPlugin};
use crate::prelude::*;

pub mod material;
pub mod system;
pub mod material_3d;

pub mod prelude {
    pub use crate::layout::render::{
        material::*,
        system::*,
        material_3d::*,
        UnivisRenderPlugin,
    };
}

pub struct UnivisRenderPlugin;

impl Plugin for UnivisRenderPlugin {
    fn build(&self, app: &mut App) {

        embedded_asset!(app, "shaders/unode.wgsl");
        embedded_asset!(app, "shaders/unode_3d.wgsl");

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