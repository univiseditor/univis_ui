pub mod algorithms;
pub mod components;
pub mod core;
pub mod geometry;
pub mod image;
pub mod layout_system;
pub mod pipeline;
pub mod pbr;
pub mod profiling;
pub mod render;
pub mod solver_types;
pub mod univis_node;

pub mod prelude {
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    pub use crate::layout::layout_system::{UScreenRoot, UWorldRoot};
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
    pub use crate::layout::UnivisLayoutPlugin;
}

use bevy::prelude::*;
use crate::internal_prelude::*;

pub struct UnivisLayoutPlugin;

impl Plugin for UnivisLayoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<USelf>()
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
            .add_plugins(LayoutCachePlugin)
            .configure_sets(
                PostUpdate,
                (
                    UnivisPostUpdateSet::WidgetSync,
                    UnivisPostUpdateSet::LayoutHierarchy,
                    UnivisPostUpdateSet::LayoutMeasure,
                    UnivisPostUpdateSet::LayoutSolve,
                    UnivisPostUpdateSet::RenderSync,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                update_layout_hierarchy.in_set(UnivisPostUpdateSet::LayoutHierarchy),
            )
            .add_systems(
                PostUpdate,
                upward_measure_pass_cached.in_set(UnivisPostUpdateSet::LayoutMeasure),
            )
            .add_systems(
                PostUpdate,
                downward_solve_pass_safe.in_set(UnivisPostUpdateSet::LayoutSolve),
            );
    }
}
