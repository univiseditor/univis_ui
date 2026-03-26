//! Layout and root systems for Univis UI.
//!
//! This module contains:
//!
//! - [`crate::layout::layout_system`] for [`crate::layout::layout_system::URootUi`]
//!   and root resolution.
//! - [`crate::layout::univis_node`] for node, layout, and local positioning components.
//! - [`crate::layout::geometry`] for logical UI units and spacing helpers.
//! - [`crate::layout::render`] for mesh/material synchronization.
//!
//! Most users reach this module through [`crate::prelude`] or
//! [`crate::layout::prelude`].

pub mod algorithms;
pub mod components;
pub mod core;
pub mod geometry;
pub mod image;
pub mod layout_system;
pub mod pbr;
pub mod pipeline;
pub mod profiling;
pub mod render;
pub mod solver_types;
pub mod univis_node;

/// Common imports for authoring layout trees and roots directly from the engine crate.
pub mod prelude {
    pub use crate::layout::UnivisLayoutPlugin;
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    #[allow(deprecated)]
    pub use crate::layout::layout_system::{
        URootUi, UScreenRoot, UWorldRoot, UiCameraRef, UiCanvasSize, UiSpace,
    };
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
}

use crate::internal_prelude::*;
use bevy::asset::AssetEventSystems;
use bevy::prelude::*;
use bevy::sprite::update_text2d_layout;

/// Installs the layout pipeline for Univis UI roots and nodes.
///
/// This plugin resolves roots, builds the layout hierarchy, measures content,
/// solves final geometry, and prepares render sync data in `PostUpdate`.
pub struct UnivisLayoutPlugin;

impl Plugin for UnivisLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<USelf>()
            .register_type::<URootUi>()
            .register_type::<UiSpace>()
            .register_type::<UiCanvasSize>()
            .register_type::<UiCameraRef>()
            .register_type::<UAlignSelf>()
            .register_type::<UPosition>()
            .register_type::<ULayoutContainerExt>()
            .register_type::<ULayoutBoxAlignContainer>()
            .register_type::<ULayoutFlexContainer>()
            .register_type::<ULayoutGridContainer>()
            .register_type::<ULayoutItemExt>()
            .register_type::<ULayoutBoxAlignSelf>()
            .register_type::<ULayoutFlexItem>()
            .register_type::<ULayoutGridItem>()
            .register_type::<UAlignSelfExt>()
            .register_type::<UAlignItemsExt>()
            .register_type::<UContentAlignExt>()
            .register_type::<UOverflowPosition>()
            .register_type::<UFlexWrap>()
            .register_type::<UTrackSize>()
            .register_type::<UGridAutoFlow>()
            .init_resource::<LayoutTreeDepth>()
            .init_resource::<RootSpawnRankCounter>()
            .add_plugins(LayoutCachePlugin)
            .configure_sets(
                PostUpdate,
                (
                    UnivisPostUpdateSet::WidgetSync,
                    UnivisPostUpdateSet::RootResolve,
                    UnivisPostUpdateSet::LayoutHierarchy,
                    UnivisPostUpdateSet::LayoutMeasure,
                    UnivisPostUpdateSet::LayoutSolve,
                    UnivisPostUpdateSet::RenderSync,
                )
                    .chain(),
            )
            .configure_sets(
                PostUpdate,
                (
                    UnivisPostUpdateSet::WidgetSync,
                    UnivisPostUpdateSet::RootResolve,
                    UnivisPostUpdateSet::LayoutHierarchy,
                    UnivisPostUpdateSet::LayoutMeasure,
                    UnivisPostUpdateSet::LayoutSolve,
                    UnivisPostUpdateSet::RenderSync,
                )
                    .after(update_text2d_layout)
                    .before(AssetEventSystems),
            )
            .add_systems(
                PostUpdate,
                (
                    resolve_root_ui,
                    assign_root_spawn_ranks,
                    resolve_root_stacking,
                    sync_root_capsule_transforms,
                )
                    .chain()
                    .in_set(UnivisPostUpdateSet::RootResolve),
            )
            .add_systems(
                PostUpdate,
                update_layout_hierarchy.in_set(UnivisPostUpdateSet::LayoutHierarchy),
            )
            .add_systems(
                PostUpdate,
                (
                    upward_measure_pass_cached,
                    sync_fit_content_root_canvas_sizes,
                )
                    .chain()
                    .in_set(UnivisPostUpdateSet::LayoutMeasure),
            )
            .add_systems(
                PostUpdate,
                downward_solve_pass_safe.in_set(UnivisPostUpdateSet::LayoutSolve),
            );
    }
}
