use bevy::math::Vec2;
use crate::prelude::*;

// Helper to calculate cross axis position based on alignment
fn calculate_cross_pos_with_margin(
    child_cross_size: f32, 
    ctx: &PlacementContext,
    align: Option<UAlignSelf>,
    m_cross_start: f32, 
    m_cross_end: f32
) -> f32 {
    let total_padding_cross = ctx.padding_cross_start * 2.0; // Simplified approximation
    let occupied_cross = child_cross_size + m_cross_start + m_cross_end;
    let free_cross = (ctx.container_cross_size - total_padding_cross - occupied_cross).max(0.0);
    
    let effective_align = align.unwrap_or(match ctx.align_items {
        UAlignItems::Start | UAlignItems::FlexStart | UAlignItems::Stretch => UAlignSelf::Start,
        UAlignItems::Center => UAlignSelf::Center,
        UAlignItems::End | UAlignItems::FlexEnd => UAlignSelf::End,
        _ => UAlignSelf::Start,
    });

    let offset = match effective_align {
        UAlignSelf::Start | UAlignSelf::Stretch | UAlignSelf::Auto => 0.0,
        UAlignSelf::Center => free_cross / 2.0,
        UAlignSelf::End => free_cross,
    };

    ctx.padding_cross_start + offset + m_cross_start
}

// ------------------------------------------------------------------
// 1. FlexPlacer
// ------------------------------------------------------------------

/// Standard Flexbox Placer.
/// Handles linear layout (Row/Column) with `justify_content` distribution.
pub struct FlexPlacer;
impl LayoutPlacer for FlexPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        let count = items.len();
        if count == 0 { 
            return Vec2::new(
                ctx.padding_main_start + ctx.padding_main_end, 
                ctx.padding_cross_start * 2.0
            ); 
        }

        // Calculate total content span
        let total_items_span: f32 = items.iter()
            .map(|i| {
                let (s, _) = axis.from_world(i.result.size);
                let (ms, me, _, _) = axis.extract_margin_sides(i.margin);
                s + ms + me
            })
            .sum();

        let total_gap_size = if count > 1 { (count as f32 - 1.0) * ctx.gap } else { 0.0 };
        
        // Available space for distribution
        let content_space = ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end;
        let free_space = (content_space - total_items_span - total_gap_size).max(0.0);

        // Cursor setup
        let (mut cursor, step_extra) = match ctx.justify_content {
            UJustifyContent::Start => (ctx.padding_main_start, 0.0),
            UJustifyContent::Center => (ctx.padding_main_start + (free_space / 2.0), 0.0),
            UJustifyContent::End => (ctx.padding_main_start + free_space, 0.0),
            UJustifyContent::SpaceBetween => {
                if count > 1 { (ctx.padding_main_start, free_space / (count as f32 - 1.0)) } 
                else { (ctx.padding_main_start, 0.0) }
            },
            UJustifyContent::SpaceEvenly => {
                let gap = free_space / (count as f32 + 1.0);
                (ctx.padding_main_start + gap, gap)
            },
            UJustifyContent::SpaceAround => {
                if count > 0 {
                    let gap = free_space / count as f32;
                    (ctx.padding_main_start + (gap / 2.0), gap)
                } else { (ctx.padding_main_start, 0.0) }
            },
            _ => (ctx.padding_main_start, 0.0),
        };

        let mut max_cross_used: f32 = 0.0;

        for item in items.iter_mut() {
            let (child_main, child_cross) = axis.from_world(item.result.size);
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            // Cross Position
            let pos_cross = calculate_cross_pos_with_margin(
                child_cross, ctx, item.spec.align_self, m_cross_start, m_cross_end
            );

            // Main Position
            let pos_main = cursor + m_main_start;
            item.result.pos = axis.to_world(pos_main, pos_cross);

            // Track Max Height
            max_cross_used = max_cross_used.max(child_cross + m_cross_start + m_cross_end);

            // Move cursor
            let item_span = m_main_start + child_main + m_main_end;
            if ctx.justify_content == UJustifyContent::SpaceEvenly || ctx.justify_content == UJustifyContent::SpaceAround {
                 cursor += item_span + step_extra;
            } else {
                 cursor += item_span + ctx.gap + step_extra;
            }
        }

        // Calculate used size
        let calculated_main = total_items_span + total_gap_size + ctx.padding_main_start + ctx.padding_main_end;
        let calculated_cross = max_cross_used + ctx.padding_cross_start * 2.0;

        Vec2::new(calculated_main, calculated_cross)
    }
}

// ------------------------------------------------------------------
// 2. GridPlacer
// ------------------------------------------------------------------

/// Grid Layout Placer.
/// Divides space into `columns` and places items.
pub struct GridPlacer { pub columns: usize }
impl LayoutPlacer for GridPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        if self.columns == 0 || items.is_empty() { return Vec2::ZERO; }

        let content_w = ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end;
        let cols = self.columns as f32;
        let total_gap_w = (cols - 1.0) * ctx.gap;
        let cell_w = (content_w - total_gap_w) / cols;
        // Assume square cells for now or dynamic
        let cell_h = cell_w; 

        for (i, item) in items.iter_mut().enumerate() {
            let col_index = (i % self.columns) as f32;
            let row_index = (i / self.columns) as f32;
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let target_w = (cell_w - m_main_start - m_main_end).max(0.0);
            let target_h = (cell_h - m_cross_start - m_cross_end).max(0.0);
            
            let final_h = if item.spec.height_mode == SolverSizeMode::Fixed {
                item.result.size.y
            } else { target_h };

            item.result.size = Vec2::new(target_w, final_h);

            let pos_x = ctx.padding_main_start + col_index * (cell_w + ctx.gap) + m_main_start;
            let pos_y = ctx.padding_cross_start + row_index * (cell_h + ctx.gap) + m_cross_start;
            item.result.pos = Vec2::new(pos_x, pos_y);
        }

        // Final Size
        let count = items.len();
        let rows = (count + self.columns - 1) / self.columns;
        let rows_f = rows as f32;

        let total_height = (rows_f * cell_h) + ((rows_f - 1.0).max(0.0) * ctx.gap);
        let total_cross = total_height + ctx.padding_cross_start * 2.0;

        Vec2::new(ctx.container_main_size, total_cross)
    }
}

// ------------------------------------------------------------------
// 3. StackPlacer
// ------------------------------------------------------------------

/// Stack Placer (Z-Index Stacking).
/// Places items on top of each other with slight offset.
pub struct StackPlacer;
impl LayoutPlacer for StackPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        let offset_step = 5.0;
        let mut max_main_used: f32 = 0.0;
        let mut max_cross_used: f32 = 0.0;

        for (i, item) in items.iter_mut().enumerate() {
            let (child_main, child_cross) = axis.from_world(item.result.size);
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let total_child_main = child_main + m_main_start + m_main_end;
            let free_main = ctx.container_main_size - (ctx.padding_main_start * 2.0) - total_child_main;
            let center_main = ctx.padding_main_start + (free_main / 2.0) + m_main_start;
            
            let center_cross = calculate_cross_pos_with_margin(
                child_cross, ctx, item.spec.align_self, m_cross_start, m_cross_end
            );

            let offset = i as f32 * offset_step;
            item.result.pos = axis.to_world(center_main + offset, center_cross + offset);

            // Bounding Box
            max_main_used = max_main_used.max(center_main + offset + child_main + m_main_end);
            max_cross_used = max_cross_used.max(center_cross + offset + child_cross + m_cross_end);
        }

        Vec2::new(
            max_main_used + ctx.padding_main_end, 
            max_cross_used + ctx.padding_cross_start 
        )
    }
}

// ------------------------------------------------------------------
// 4. MasonryPlacer
// ------------------------------------------------------------------

/// Masonry (Waterfall) Placer.
/// Packs items into columns, placing new items in the shortest column.
pub struct MasonryPlacer { pub columns: usize }
impl LayoutPlacer for MasonryPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        if self.columns == 0 || items.is_empty() { return Vec2::ZERO; }

        let total_gaps = (self.columns as f32 - 1.0) * ctx.gap;
        let available_width = ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end;
        let col_width = (available_width - total_gaps) / self.columns as f32;

        let mut col_heights = vec![ctx.padding_cross_start; self.columns];

        for item in items.iter_mut() {
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let (shortest_col_idx, &current_y) = col_heights.iter().enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let pos_main = ctx.padding_main_start + (shortest_col_idx as f32 * (col_width + ctx.gap)) + m_main_start;
            let pos_cross = current_y + m_cross_start;

            item.result.pos = axis.to_world(pos_main, pos_cross);

            let (_, child_h) = axis.from_world(item.result.size);
            let new_child_w = (col_width - m_main_start - m_main_end).max(0.0);
            item.result.size = axis.to_world(new_child_w, child_h);

            col_heights[shortest_col_idx] += m_cross_start + child_h + m_cross_end + ctx.gap;
        }

        let max_height = *col_heights.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        Vec2::new(ctx.container_main_size, max_height + ctx.padding_cross_start)
    }
}

// ------------------------------------------------------------------
// 5. RadialPlacer
// ------------------------------------------------------------------

/// Radial (Circular) Placer.
/// Arranges items in a circle. Automatically calculates radius if needed.
pub struct RadialPlacer;
impl LayoutPlacer for RadialPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        let count = items.len();
        if count == 0 { return Vec2::ZERO; }

        let (w, h) = axis.to_world(ctx.container_main_size, ctx.container_cross_size).into();
        
        // 1. Calculate Radius
        let min_dim = w.min(h);
        let radius = if min_dim < 50.0 {
            // Estimate circumference
            let total_item_width: f32 = items.iter().map(|i| axis.from_world(i.result.size).0).sum();
            (total_item_width * 1.5 / std::f32::consts::TAU).max(100.0) 
        } else {
            (min_dim / 2.0) - 20.0
        };

        let angle_step = std::f32::consts::TAU / count as f32;

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (i, item) in items.iter_mut().enumerate() {
            if item.result.size.x == 0.0 { item.result.size.x = 50.0; }
            if item.result.size.y == 0.0 { item.result.size.y = 50.0; }

            let angle = (i as f32 * angle_step) - std::f32::consts::FRAC_PI_2;
            
            // Polar coordinates
            let cx = radius * angle.cos();
            let cy = radius * angle.sin();

            // Center to Top-Left
            let pos_x = cx - (item.result.size.x / 2.0);
            let pos_y = cy - (item.result.size.y / 2.0);

            // Track bounds
            min_x = min_x.min(pos_x);
            max_x = max_x.max(pos_x + item.result.size.x);
            min_y = min_y.min(pos_y);
            max_y = max_y.max(pos_y + item.result.size.y);
            
            item.result.pos = Vec2::new(pos_x, pos_y);
        }

        // 2. Container Size
        let content_width = max_x - min_x;
        let content_height = max_y - min_y;
        
        let total_w = content_width + ctx.padding_main_start + ctx.padding_main_end;
        let total_h = content_height + ctx.padding_cross_start * 2.0;

        // 3. Shift positions to start from padding
        let shift_x = ctx.padding_main_start - min_x;
        let shift_y = ctx.padding_cross_start - min_y;

        for item in items.iter_mut() {
            item.result.pos.x += shift_x;
            item.result.pos.y += shift_y;
        }

        axis.to_world(total_w, total_h)
    }
}