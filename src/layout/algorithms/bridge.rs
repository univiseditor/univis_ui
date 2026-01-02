use bevy::math::Vec2;
use crate::prelude::*;

// =========================================================
// Traits
// =========================================================

/// A trait that all layout placers (Flex, Grid, Masonry, etc.) must implement.
///
/// Implementations receive a list of items and context, and must:
/// 1. Set the position (`result.pos`) for each item.
/// 2. Return the total used size (`Vec2`) of the container.
pub trait LayoutPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2;
}

// =========================================================
// The Bridge Function
// =========================================================

/// The "Bridge" connecting the generic Solver to specific Layout Placers.
///
/// This function acts as a factory/dispatcher:
/// 1. It selects the correct `LayoutPlacer` based on `layout.display` (Flex, Grid, Stack...).
/// 2. It filters/prepares the data.
/// 3. It executes the placement logic and returns the final container size.
pub fn final_size_with_indices(
    layout: ULayout,
    items: &mut [SolverItem],
    normal_indices: &[usize],
    axis: &AxisHelper, 
    placement_ctx: &PlacementContext,
    _container_main: f32, 
    _final_cross: f32,    
) -> Vec2 { 
    
    // 1. Prepare temporary data slice for the placer
    let temp_data: Vec<_> = normal_indices.iter()
        .map(|&i| {
            let item = &items[i];
            (i, item.spec, *item.result, item.margin) 
        })
        .collect();

    let mut temp_results: Vec<SolverResult> = temp_data.iter()
        .map(|(_, _, res, _)| *res)
        .collect();

    let mut temp_items: Vec<SolverItem> = temp_data.iter()
        .zip(temp_results.iter_mut())
        .map(|((_, spec, _, margin), result)| SolverItem { 
            spec: *spec, 
            result,
            margin: *margin 
        })
        .collect();

    // 2. Select the Placer Strategy
    let placer: Box<dyn LayoutPlacer> = match layout.display {
        UDisplay::Flex => Box::new(FlexPlacer),
        UDisplay::Stack => Box::new(StackPlacer),
        UDisplay::Grid => Box::new(GridPlacer { columns: layout.grid_columns as usize }),
        UDisplay::Masonry => Box::new(MasonryPlacer { columns: layout.grid_columns as usize }),
        UDisplay::Radial => Box::new(RadialPlacer),
        _ => Box::new(FlexPlacer),
    };
    
    // 3. Execute Placement
    let used_size = placer.place(&mut temp_items, axis, placement_ctx);
    
    // 4. Apply results back to original items
    for (temp_idx, &original_idx) in normal_indices.iter().enumerate() {
        *items[original_idx].result = temp_results[temp_idx];
    }

    // Return final utilized size
    used_size
}

// =========================================================
// Helpers
// =========================================================

/// Helper to calculate Cross Axis Alignment offset.
/// Used by various placers to respect `align_items`.
pub fn calculate_cross_align(child_cross_size: f32, ctx: &PlacementContext) -> f32 {
    let free_cross = ctx.container_cross_size - (ctx.padding_cross_start * 2.0) - child_cross_size;
    
    match ctx.align_items {
        UAlignItems::Start | UAlignItems::Stretch | UAlignItems::FlexStart => ctx.padding_cross_start,
        UAlignItems::Center => ctx.padding_cross_start + (free_cross / 2.0),
        UAlignItems::End | UAlignItems::FlexEnd => ctx.padding_cross_start + free_cross,
        _ => ctx.padding_cross_start,
    }
}