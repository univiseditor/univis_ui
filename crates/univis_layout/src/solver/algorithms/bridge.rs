use crate::geometry::AxisHelper;
use crate::solver::algorithms::places::placement::{
    FlexPlacer, GridPlacer, MasonryPlacer, RadialPlacer, StackPlacer,
};
use crate::solver::types::{PlacementContext, SolverItem};
use crate::types::UDisplay;
use glam::Vec2;

/// A trait that all layout placers (Flex, Grid, Masonry, etc.) must implement.
pub trait LayoutPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2;
}

/// The "Bridge" connecting the generic Solver to specific Layout Placers.
pub fn final_size_with_indices(
    display: UDisplay,
    grid_columns: u32,
    items: &mut [SolverItem],
    normal_indices: &[usize],
    axis: &AxisHelper,
    placement_ctx: &PlacementContext,
    _container_main: f32,
    _final_cross: f32,
) -> Vec2 {
    let mut temp_items: Vec<SolverItem> = normal_indices.iter().map(|&i| items[i]).collect();

    let placer: Box<dyn LayoutPlacer> = match display {
        UDisplay::Flex => Box::new(FlexPlacer),
        UDisplay::Stack => Box::new(StackPlacer),
        UDisplay::Grid => Box::new(GridPlacer {
            columns: grid_columns as usize,
        }),
        UDisplay::Masonry => Box::new(MasonryPlacer {
            columns: grid_columns as usize,
        }),
        UDisplay::Radial => Box::new(RadialPlacer),
        UDisplay::None => return Vec2::ZERO,
    };

    let size = placer.place(&mut temp_items, axis, placement_ctx);

    for (&orig_idx, temp_item) in normal_indices.iter().zip(temp_items.iter()) {
        items[orig_idx].result = temp_item.result;
    }

    size
}
