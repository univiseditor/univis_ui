use bevy::prelude::*;

pub mod layout;
pub mod schedule;

#[doc(hidden)]
pub mod internal {
    pub use crate::layout::components::{IntrinsicSize, LayoutDepth, LayoutTreeDepth, UI3d};
    pub use crate::layout::geometry::ComputedSize;
    pub use crate::layout::render::system::{MaterialHandles, MaterialPool};
}

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::internal::*;
    pub use crate::layout::algorithms::prelude::*;
    pub use crate::layout::components::*;
    pub use crate::layout::core::prelude::*;
    pub use crate::layout::geometry::*;
    pub use crate::layout::image::*;
    pub use crate::layout::layout_system::*;
    pub use crate::layout::pbr::*;
    pub use crate::layout::pipeline::prelude::*;
    pub use crate::layout::profiling::*;
    pub use crate::layout::render::prelude::*;
    pub use crate::layout::solver_types::*;
    pub use crate::layout::univis_node::*;
    pub use crate::schedule::*;
    pub use univis_ui_style::prelude::*;
}

pub mod prelude {
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    pub use crate::layout::layout_system::{UScreenRoot, UWorldRoot};
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
    pub use crate::{layout::prelude::*, UnivisEnginePlugin};
}

pub struct UnivisEnginePlugin;

impl Plugin for UnivisEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            layout::univis_node::UnivisNodePlugin,
            layout::UnivisLayoutPlugin,
            layout::render::UnivisRenderPlugin,
        ));
    }
}
