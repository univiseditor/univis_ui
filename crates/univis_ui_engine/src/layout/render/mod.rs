use bevy::{asset::embedded_asset, prelude::*, sprite_render::Material2dPlugin};

/// Material definitions for the Univis UI engine.
pub mod material;
/// 3D material definitions for the Univis UI engine.
pub mod material_3d;
/// Systems for synchronizing UI node state to render materials and meshes.
pub mod system;

use crate::layout::components::UI3d;
use crate::layout::layout_system::sync_cached_ui3d;
use crate::layout::pbr::UPbr;
use crate::layout::render::material::UNodeMaterial;
use crate::layout::render::material_3d::UNodeMaterial3d;
use crate::layout::render::system::{MaterialPool, MeshPool, update_materials_optimized};
use crate::schedule::{UiSettlementSchedule, UnivisPostUpdateSet};

/// Convenience exports for Univis UI rendering.
pub mod prelude {
    pub use crate::layout::render::{UnivisRenderPlugin, material::*, material_3d::*, system::*};
}

/// Core rendering plugin for Univis UI materials and mesh synchronization.
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
            .init_resource::<MeshPool>()
            .add_systems(
                UiSettlementSchedule,
                sync_cached_ui3d
                    .in_set(UnivisPostUpdateSet::RenderSync)
                    .after(UnivisPostUpdateSet::LayoutSolve),
            )
            .add_systems(
                UiSettlementSchedule,
                update_materials_optimized
                    .in_set(UnivisPostUpdateSet::RenderSync)
                    .after(sync_cached_ui3d),
            );
    }
}
