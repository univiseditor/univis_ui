use crate::internal_prelude::*;
use bevy::{asset::embedded_asset, prelude::*, sprite_render::Material2dPlugin};

pub mod material;
pub mod material_3d;
pub mod system;

pub mod prelude {
    pub use crate::layout::render::{UnivisRenderPlugin, material::*, material_3d::*, system::*};
}

pub struct UnivisRenderPlugin;

impl Plugin for UnivisRenderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/unode.wgsl");
        embedded_asset!(app, "shaders/unode_3d.wgsl");

        app.add_plugins(Material2dPlugin::<UNodeMaterial>::default())
            .add_plugins(MaterialPlugin::<UNodeMaterial3d>::default())
            .register_type::<UI3d>()
            .register_type::<UPbr>()
            .init_resource::<MaterialPool>()
            .add_systems(
                PostUpdate,
                auto_propagate_ui3d
                    .in_set(UnivisPostUpdateSet::RenderSync)
                    .after(UnivisPostUpdateSet::LayoutSolve),
            )
            .add_systems(
                PostUpdate,
                update_materials_optimized
                    .in_set(UnivisPostUpdateSet::RenderSync)
                    .after(auto_propagate_ui3d),
            );
    }
}
