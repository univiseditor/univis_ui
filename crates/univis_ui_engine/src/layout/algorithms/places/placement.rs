mod helpers;
#[cfg(test)]
mod tests;

use self::helpers::{
    alignment_offset, allows_explicit_stretch, allows_implicit_stretch, can_place_span,
    canonical_align_self, ensure_grid_rows, has_explicit_align_self, has_explicit_justify_self,
    mark_span, resolve_cross_align, resolve_justify_self, resolve_track_sizes,
};
use bevy::math::Vec2;

use crate::layout::algorithms::bridge::LayoutPlacer;
#[cfg(test)]
use crate::layout::core::solver::SolverResult;
use crate::layout::core::solver::{PlacementContext, SolverItem};
use crate::layout::geometry::AxisHelper;
#[cfg(test)]
use crate::layout::geometry::{USides, UVal};
use crate::layout::solver_types::{SolverSizeMode, SolverSpec};
use crate::layout::univis_node::{
    UAlignItems, UAlignItemsExt, UAlignSelf, UAlignSelfExt, UContentAlignExt, UFlexWrap,
    UGridAutoFlow, UJustifyContent, UOverflowPosition, UTrackSize,
};
#[cfg(test)]
use crate::layout::univis_node::{UFlexDirection, UPositionType};

#[derive(Clone, Copy)]
struct FlexLine {
    start: usize,
    end: usize,
    main_span: f32,
    cross_size: f32,
}

pub struct FlexPlacer;

impl LayoutPlacer for FlexPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        if items.is_empty() {
            return Vec2::ZERO;
        }

        let content_main =
            (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let wrap_enabled = ctx.flex_wrap != UFlexWrap::NoWrap;

        let mut lines: Vec<FlexLine> = Vec::new();
        let mut line_start = 0usize;
        let mut line_main = 0.0;
        let mut line_cross = 0.0;

        for (i, item) in items.iter().enumerate() {
            let (child_main, child_cross) = axis.from_world(item.result.size);
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
                axis.extract_margin_sides(item.margin);
            let item_main_span = child_main + m_main_start + m_main_end;
            let item_cross_span = child_cross + m_cross_start + m_cross_end;
            let tentative = if i == line_start {
                item_main_span
            } else {
                line_main + ctx.main_gap + item_main_span
            };

            if wrap_enabled && i > line_start && tentative > content_main {
                lines.push(FlexLine {
                    start: line_start,
                    end: i,
                    main_span: line_main,
                    cross_size: line_cross,
                });
                line_start = i;
                line_main = item_main_span;
                line_cross = item_cross_span;
            } else {
                line_main = tentative;
                line_cross = line_cross.max(item_cross_span);
            }
        }

        lines.push(FlexLine {
            start: line_start,
            end: items.len(),
            main_span: line_main,
            cross_size: line_cross,
        });

        let line_count = lines.len();
        let mut line_sizes: Vec<f32> = lines.iter().map(|l| l.cross_size).collect();
        let content_cross = (ctx.container_cross_size - (ctx.padding_cross_start * 2.0)).max(0.0);

        // In a single-line, non-wrapping flex container, the line effectively spans
        // the full cross space. This keeps `align_items: Center/End` anchored to the
        // container cross axis instead of pinning items at the top.
        if !wrap_enabled && line_count == 1 {
            line_sizes[0] = content_cross.max(line_sizes[0]);
        }

        let total_line_cross: f32 = line_sizes.iter().sum();
        let total_line_gaps = if line_count > 1 {
            (line_count as f32 - 1.0) * ctx.cross_gap
        } else {
            0.0
        };
        let mut free_cross = (content_cross - total_line_cross - total_line_gaps).max(0.0);

        let align_content = ctx.align_content.unwrap_or(UContentAlignExt::Start);
        if align_content == UContentAlignExt::Stretch && line_count > 0 && free_cross > 0.0 {
            let extra = free_cross / line_count as f32;
            for size in &mut line_sizes {
                *size += extra;
            }
            free_cross = 0.0;
        }

        let (start_offset, mut between_gap) = match align_content {
            UContentAlignExt::Center => (free_cross * 0.5, ctx.cross_gap),
            UContentAlignExt::End | UContentAlignExt::FlexEnd => (free_cross, ctx.cross_gap),
            UContentAlignExt::SpaceBetween => {
                if line_count > 1 {
                    (
                        0.0,
                        ctx.cross_gap + (free_cross / (line_count as f32 - 1.0)),
                    )
                } else {
                    (0.0, ctx.cross_gap)
                }
            }
            UContentAlignExt::SpaceAround => {
                let extra = free_cross / line_count.max(1) as f32;
                (extra * 0.5, ctx.cross_gap + extra)
            }
            UContentAlignExt::SpaceEvenly => {
                let extra = free_cross / (line_count as f32 + 1.0);
                (extra, ctx.cross_gap + extra)
            }
            _ => (0.0, ctx.cross_gap),
        };

        if line_count <= 1 {
            between_gap = 0.0;
        }

        let mut line_starts = vec![0.0; line_count];
        let mut cursor = ctx.padding_cross_start + start_offset;
        for i in 0..line_count {
            line_starts[i] = cursor;
            cursor += line_sizes[i] + between_gap;
        }

        if ctx.flex_wrap == UFlexWrap::WrapReverse {
            for i in 0..line_count {
                line_starts[i] = ctx.container_cross_size
                    - ctx.padding_cross_start
                    - ((line_starts[i] - ctx.padding_cross_start) + line_sizes[i]);
            }
        }

        let mut max_main_used: f32 = 0.0;
        for (line_idx, line) in lines.iter().enumerate() {
            let line_items = line.end - line.start;
            if line_items == 0 {
                continue;
            }

            let line_main_span = line.main_span;
            let free_main = (content_main - line_main_span).max(0.0);
            let (mut main_cursor, step_extra) = match ctx.justify_content {
                UJustifyContent::Start => (ctx.padding_main_start, 0.0),
                UJustifyContent::Center => (ctx.padding_main_start + (free_main * 0.5), 0.0),
                UJustifyContent::End => (ctx.padding_main_start + free_main, 0.0),
                UJustifyContent::SpaceBetween => {
                    if line_items > 1 {
                        (
                            ctx.padding_main_start,
                            free_main / (line_items as f32 - 1.0),
                        )
                    } else {
                        (ctx.padding_main_start, 0.0)
                    }
                }
                UJustifyContent::SpaceEvenly => {
                    let gap = free_main / (line_items as f32 + 1.0);
                    (ctx.padding_main_start + gap, gap)
                }
                UJustifyContent::SpaceAround => {
                    let gap = free_main / line_items as f32;
                    (ctx.padding_main_start + (gap * 0.5), gap)
                }
                _ => (ctx.padding_main_start, 0.0),
            };

            for item in items.iter_mut().take(line.end).skip(line.start) {
                let (child_main, mut child_cross) = axis.from_world(item.result.size);
                let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
                    axis.extract_margin_sides(item.margin);

                let cross_align = resolve_cross_align(&item.spec, ctx.align_items);
                let (_, cross_mode, _) = {
                    let (main_mode, _, _) = axis.get_main_spec(&item.spec);
                    let (cross_mode, _, _) = axis.get_cross_spec(&item.spec);
                    (main_mode, cross_mode, ())
                };
                let stretch_allowed = if has_explicit_align_self(&item.spec) {
                    allows_explicit_stretch(cross_mode)
                } else {
                    allows_implicit_stretch(cross_mode)
                };

                if canonical_align_self(cross_align) == UAlignSelfExt::Stretch && stretch_allowed {
                    child_cross = (line_sizes[line_idx] - m_cross_start - m_cross_end).max(0.0);
                    item.result.size = axis.to_world(child_main, child_cross);
                }

                let occupied_cross = child_cross + m_cross_start + m_cross_end;
                let free_cross_in_line = line_sizes[line_idx] - occupied_cross;
                let cross_offset =
                    alignment_offset(cross_align, free_cross_in_line, item.spec.align_overflow);
                let pos_cross = line_starts[line_idx] + m_cross_start + cross_offset;

                let mut pos_main = main_cursor + m_main_start;
                if axis.is_reverse() {
                    pos_main = ctx.container_main_size - (pos_main + child_main);
                }

                item.result.pos = axis.to_world(pos_main, pos_cross);
                let item_span = m_main_start + child_main + m_main_end;

                if matches!(
                    ctx.justify_content,
                    UJustifyContent::SpaceEvenly | UJustifyContent::SpaceAround
                ) {
                    main_cursor += item_span + step_extra;
                } else {
                    main_cursor += item_span + ctx.main_gap + step_extra;
                }

                max_main_used = max_main_used.max(line_main_span);
            }
        }

        let total_cross_used: f32 = line_sizes.iter().sum::<f32>()
            + if line_count > 1 {
                (line_count as f32 - 1.0) * ctx.cross_gap
            } else {
                0.0
            };
        let calculated_cross = total_cross_used + (ctx.padding_cross_start * 2.0);
        let calculated_main = max_main_used + ctx.padding_main_start + ctx.padding_main_end;

        Vec2::new(calculated_main, calculated_cross)
    }
}

pub struct GridPlacer {
    pub columns: usize,
}

impl LayoutPlacer for GridPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        if items.is_empty() {
            return Vec2::ZERO;
        }

        let available_main =
            (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let available_cross = (ctx.container_cross_size - ctx.padding_cross_start * 2.0).max(0.0);

        let fallback_cols = self.columns.max(ctx.grid_columns as usize).max(1);
        let mut start_cols = fallback_cols.max(ctx.grid_template_columns.len());
        for item in items.iter() {
            let col_start = item.spec.grid_column_start.unwrap_or(1) as usize;
            let col_span = item.spec.grid_column_span.max(1) as usize;
            start_cols = start_cols.max(col_start.saturating_sub(1) + col_span);
        }

        let mut max_rows = ctx.grid_template_rows.len();
        if ctx.grid_template_rows.is_empty() && ctx.grid_auto_flow == UGridAutoFlow::Column && available_cross > 0.0 {
            let auto_row_height = match ctx.grid_auto_rows {
                UTrackSize::Px(v) => v.max(1.0),
                _ => {
                    let mut max_item_cross = 0.0f32;
                    for item in items.iter() {
                        let (_, child_cross) = axis.from_world(item.result.size);
                        max_item_cross = max_item_cross.max(child_cross);
                    }
                    max_item_cross.max(1.0)
                }
            };
            let gap = ctx.cross_gap;
            let fit_rows = ((available_cross + gap) / (auto_row_height + gap)).floor() as usize;
            max_rows = fit_rows.max(1);
        } else if max_rows == 0 {
            max_rows = 1;
        }

        for item in items.iter() {
            if let Some(r_start) = item.spec.grid_row_start {
                let r_span = item.spec.grid_row_span.max(1) as usize;
                max_rows = max_rows.max(r_start.saturating_sub(1) as usize + r_span);
            }
        }

        let mut occupancy: Vec<Vec<bool>> = vec![vec![false; start_cols]; max_rows];
        let mut placements: Vec<(usize, usize, usize, usize)> = Vec::with_capacity(items.len());
        let mut auto_cursor_row = 0usize;
        let mut auto_cursor_col = 0usize;

        for item in items.iter() {
            let col_span = item.spec.grid_column_span.max(1) as usize;
            let row_span = item.spec.grid_row_span.max(1) as usize;
            let fixed_col = item
                .spec
                .grid_column_start
                .map(|v| v.saturating_sub(1) as usize);
            let fixed_row = item
                .spec
                .grid_row_start
                .map(|v| v.saturating_sub(1) as usize);
            let is_fully_auto = fixed_col.is_none() && fixed_row.is_none();

            let mut found = None;

            if let (Some(row), Some(col)) = (fixed_row, fixed_col) {
                let current_cols = occupancy[0].len();
                if col + col_span > current_cols {
                    let new_cols = col + col_span;
                    for row_vec in occupancy.iter_mut() {
                        row_vec.resize(new_cols, false);
                    }
                }
                let current_cols = occupancy[0].len();
                ensure_grid_rows(&mut occupancy, row + row_span, current_cols);
                if can_place_span(&occupancy, row, col, row_span, col_span, current_cols) {
                    found = Some((row, col));
                }
            }

            if found.is_none() {
                match ctx.grid_auto_flow {
                    UGridAutoFlow::Row => {
                        let mut row = if fixed_row.is_none() && fixed_col.is_none() {
                            auto_cursor_row
                        } else {
                            fixed_row.unwrap_or(0)
                        };
                        let mut first_col = if fixed_row.is_none() && fixed_col.is_none() {
                            auto_cursor_col.min(occupancy[0].len().saturating_sub(1))
                        } else {
                            0
                        };
                        loop {
                            let current_cols = occupancy[0].len();
                            ensure_grid_rows(&mut occupancy, row + row_span, current_cols);
                            if let Some(fc) = fixed_col {
                                if fc + col_span > current_cols {
                                    let new_cols = fc + col_span;
                                    for row_vec in occupancy.iter_mut() {
                                        row_vec.resize(new_cols, false);
                                    }
                                }
                                let current_cols = occupancy[0].len();
                                if can_place_span(&occupancy, row, fc, row_span, col_span, current_cols) {
                                    found = Some((row, fc));
                                    break;
                                }
                            } else {
                                for col in first_col..current_cols {
                                    if can_place_span(
                                        &occupancy, row, col, row_span, col_span, current_cols,
                                    ) {
                                        found = Some((row, col));
                                        break;
                                    }
                                }
                                if found.is_some() {
                                    break;
                                }
                            }
                            first_col = 0;
                            row += 1;
                        }
                    }
                    UGridAutoFlow::Column => {
                        if let Some(fc) = fixed_col {
                            let mut row = fixed_row.unwrap_or(0);
                            loop {
                                let current_cols = occupancy[0].len();
                                if fc + col_span > current_cols {
                                    let new_cols = fc + col_span;
                                    for row_vec in occupancy.iter_mut() {
                                        row_vec.resize(new_cols, false);
                                    }
                                }
                                let current_cols = occupancy[0].len();
                                ensure_grid_rows(&mut occupancy, row + row_span, current_cols);
                                if can_place_span(&occupancy, row, fc, row_span, col_span, current_cols) {
                                    found = Some((row, fc));
                                    break;
                                }
                                row += 1;
                            }
                        } else if let Some(fr) = fixed_row {
                            let mut col = 0usize;
                            loop {
                                let current_cols = occupancy[0].len();
                                if col + col_span > current_cols {
                                    let new_cols = col + col_span;
                                    for row_vec in occupancy.iter_mut() {
                                        row_vec.resize(new_cols, false);
                                    }
                                }
                                let current_cols = occupancy[0].len();
                                ensure_grid_rows(&mut occupancy, fr + row_span, current_cols);
                                if can_place_span(&occupancy, fr, col, row_span, col_span, current_cols) {
                                    found = Some((fr, col));
                                    break;
                                }
                                col += 1;
                            }
                        } else {
                            let mut col = auto_cursor_col;
                            let mut first_row = auto_cursor_row;
                            loop {
                                let current_cols = occupancy[0].len();
                                if col + col_span > current_cols {
                                    let new_cols = col + col_span;
                                    for row_vec in occupancy.iter_mut() {
                                        row_vec.resize(new_cols, false);
                                    }
                                }
                                let current_cols = occupancy[0].len();
                                ensure_grid_rows(&mut occupancy, max_rows, current_cols);
                                let mut placed = false;
                                for row in first_row..=(max_rows.saturating_sub(row_span)) {
                                    if can_place_span(&occupancy, row, col, row_span, col_span, current_cols) {
                                        found = Some((row, col));
                                        placed = true;
                                        break;
                                    }
                                }
                                if placed {
                                    break;
                                }
                                first_row = 0;
                                col += 1;
                            }
                        }
                    }
                }
            }

            let (row, col) = found.unwrap_or((0, 0));
            let current_cols = occupancy[0].len();
            ensure_grid_rows(&mut occupancy, row + row_span, current_cols);
            mark_span(&mut occupancy, row, col, row_span, col_span);
            placements.push((row, col, row_span, col_span));

            if is_fully_auto {
                match ctx.grid_auto_flow {
                    UGridAutoFlow::Row => {
                        auto_cursor_row = row;
                        auto_cursor_col = col + col_span;
                        if auto_cursor_col >= occupancy[0].len() {
                            auto_cursor_row += auto_cursor_col / occupancy[0].len();
                            auto_cursor_col %= occupancy[0].len();
                        }
                    }
                    UGridAutoFlow::Column => {
                        auto_cursor_col = col;
                        auto_cursor_row = row + row_span;
                        if auto_cursor_row >= max_rows {
                            auto_cursor_col += auto_cursor_row / max_rows;
                            auto_cursor_row %= max_rows;
                        }
                    }
                }
            }
        }

        let final_cols = placements
            .iter()
            .map(|(_, col, _, col_span)| col + col_span)
            .max()
            .unwrap_or(1);

        let col_sizes = resolve_track_sizes(
            &ctx.grid_template_columns,
            fallback_cols,
            ctx.grid_auto_columns,
            available_main,
            ctx.main_gap,
            final_cols,
        );
        let cols = col_sizes.len();

        let required_rows = placements
            .iter()
            .map(|(row, _, row_span, _)| row + row_span)
            .max()
            .unwrap_or(1);

        let row_sizes = resolve_track_sizes(
            &ctx.grid_template_rows,
            required_rows,
            ctx.grid_auto_rows,
            available_cross,
            ctx.cross_gap,
            required_rows,
        );

        let mut col_starts = vec![0.0; cols];
        for i in 1..cols {
            col_starts[i] = col_starts[i - 1] + col_sizes[i - 1] + ctx.main_gap;
        }

        let rows = row_sizes.len();
        let mut row_starts = vec![0.0; rows];
        for i in 1..rows {
            row_starts[i] = row_starts[i - 1] + row_sizes[i - 1] + ctx.cross_gap;
        }

        for (idx, item) in items.iter_mut().enumerate() {
            let (row, col, row_span, col_span) = placements[idx];
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
                axis.extract_margin_sides(item.margin);

            let cell_main_start = ctx.padding_main_start + col_starts[col];
            let cell_cross_start = ctx.padding_cross_start + row_starts[row];

            let mut cell_main_size = 0.0;
            for size in col_sizes.iter().take((col + col_span).min(cols)).skip(col) {
                cell_main_size += *size;
            }
            if col_span > 1 {
                cell_main_size += (col_span as f32 - 1.0) * ctx.main_gap;
            }

            let mut cell_cross_size = 0.0;
            for size in row_sizes.iter().take((row + row_span).min(rows)).skip(row) {
                cell_cross_size += *size;
            }
            if row_span > 1 {
                cell_cross_size += (row_span as f32 - 1.0) * ctx.cross_gap;
            }

            let (main_mode, _, _) = axis.get_main_spec(&item.spec);
            let (cross_mode, _, _) = axis.get_cross_spec(&item.spec);

            let (mut child_main, mut child_cross) = axis.from_world(item.result.size);
            let justify_stretch_allowed = if has_explicit_justify_self(&item.spec) {
                allows_explicit_stretch(main_mode)
            } else {
                allows_implicit_stretch(main_mode)
            };

            let mut justify_self = resolve_justify_self(&item.spec, ctx);
            if !has_explicit_justify_self(&item.spec)
                && ctx.justify_items.is_none()
                && allows_implicit_stretch(main_mode)
            {
                justify_self = UAlignSelfExt::Stretch;
            }
            if canonical_align_self(justify_self) == UAlignSelfExt::Stretch
                && justify_stretch_allowed
            {
                child_main = (cell_main_size - m_main_start - m_main_end).max(0.0);
            }

            let align_stretch_allowed = if has_explicit_align_self(&item.spec) {
                allows_explicit_stretch(cross_mode)
            } else {
                allows_implicit_stretch(cross_mode)
            };
            let mut align_self = resolve_cross_align(&item.spec, ctx.align_items);
            if !has_explicit_align_self(&item.spec)
                && matches!(
                    ctx.align_items,
                    UAlignItems::Auto
                        | UAlignItems::Default
                        | UAlignItems::Start
                        | UAlignItems::FlexStart
                )
                && allows_implicit_stretch(cross_mode)
            {
                align_self = UAlignSelfExt::Stretch;
            }
            if canonical_align_self(align_self) == UAlignSelfExt::Stretch && align_stretch_allowed {
                child_cross = (cell_cross_size - m_cross_start - m_cross_end).max(0.0);
            }

            item.result.size = axis.to_world(child_main, child_cross);

            let free_main = cell_main_size - (child_main + m_main_start + m_main_end);
            let free_cross = cell_cross_size - (child_cross + m_cross_start + m_cross_end);

            let main_offset = alignment_offset(justify_self, free_main, item.spec.justify_overflow);
            let cross_offset = alignment_offset(align_self, free_cross, item.spec.align_overflow);

            let pos_main = cell_main_start + m_main_start + main_offset;
            let pos_cross = cell_cross_start + m_cross_start + cross_offset;

            item.result.pos = axis.to_world(pos_main, pos_cross);
        }

        let total_main = col_sizes.iter().sum::<f32>()
            + if cols > 1 {
                (cols as f32 - 1.0) * ctx.main_gap
            } else {
                0.0
            }
            + ctx.padding_main_start
            + ctx.padding_main_end;
        let total_cross = row_sizes.iter().sum::<f32>()
            + if rows > 1 {
                (rows as f32 - 1.0) * ctx.cross_gap
            } else {
                0.0
            }
            + (ctx.padding_cross_start * 2.0);

        Vec2::new(total_main, total_cross)
    }
}

pub struct StackPlacer;

impl LayoutPlacer for StackPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        let offset_step = 5.0;
        let mut max_main_used: f32 = 0.0;
        let mut max_cross_used: f32 = 0.0;

        for (i, item) in items.iter_mut().enumerate() {
            let (child_main, child_cross) = axis.from_world(item.result.size);
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
                axis.extract_margin_sides(item.margin);

            let total_child_main = child_main + m_main_start + m_main_end;
            let free_main = (ctx.container_main_size
                - (ctx.padding_main_start + ctx.padding_main_end)
                - total_child_main)
                .max(0.0);
            let center_main = ctx.padding_main_start + (free_main * 0.5) + m_main_start;

            let align_self = resolve_cross_align(&item.spec, ctx.align_items);
            let occupied_cross = child_cross + m_cross_start + m_cross_end;
            let free_cross =
                (ctx.container_cross_size - (ctx.padding_cross_start * 2.0) - occupied_cross)
                    .max(0.0);
            let center_cross = ctx.padding_cross_start
                + m_cross_start
                + alignment_offset(align_self, free_cross, item.spec.align_overflow);

            let offset = i as f32 * offset_step;
            item.result.pos = axis.to_world(center_main + offset, center_cross + offset);

            max_main_used = max_main_used.max(center_main + offset + child_main + m_main_end);
            max_cross_used = max_cross_used.max(center_cross + offset + child_cross + m_cross_end);
        }

        Vec2::new(
            max_main_used + ctx.padding_main_end,
            max_cross_used + ctx.padding_cross_start,
        )
    }
}

pub struct MasonryPlacer {
    pub columns: usize,
}

impl LayoutPlacer for MasonryPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        if self.columns == 0 || items.is_empty() {
            return Vec2::ZERO;
        }

        let total_gaps = (self.columns as f32 - 1.0) * ctx.main_gap;
        let available_width =
            (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let col_width = (available_width - total_gaps).max(0.0) / self.columns as f32;

        let mut col_heights = vec![ctx.padding_cross_start; self.columns];

        for item in items.iter_mut() {
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
                axis.extract_margin_sides(item.margin);

            let (shortest_col_idx, &current_y) = col_heights
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let pos_main = ctx.padding_main_start
                + (shortest_col_idx as f32 * (col_width + ctx.main_gap))
                + m_main_start;
            let pos_cross = current_y + m_cross_start;
            item.result.pos = axis.to_world(pos_main, pos_cross);

            let (_, child_cross) = axis.from_world(item.result.size);
            let new_main = (col_width - m_main_start - m_main_end).max(0.0);
            item.result.size = axis.to_world(new_main, child_cross);

            col_heights[shortest_col_idx] +=
                m_cross_start + child_cross + m_cross_end + ctx.cross_gap;
        }

        let max_height = *col_heights
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        Vec2::new(
            ctx.container_main_size,
            max_height + ctx.padding_cross_start,
        )
    }
}

pub struct RadialPlacer;

impl LayoutPlacer for RadialPlacer {
    fn place(&self, items: &mut [SolverItem], axis: &AxisHelper, ctx: &PlacementContext) -> Vec2 {
        let count = items.len();
        if count == 0 {
            return Vec2::ZERO;
        }

        let world_size = axis.to_world(ctx.container_main_size, ctx.container_cross_size);
        let w = world_size.x;
        let h = world_size.y;

        let min_dim = w.min(h);
        let radius = if min_dim < 50.0 {
            let total_item_width: f32 =
                items.iter().map(|i| axis.from_world(i.result.size).0).sum();
            (total_item_width * 1.5 / std::f32::consts::TAU).max(100.0)
        } else {
            (min_dim * 0.5) - 20.0
        };

        let angle_step = std::f32::consts::TAU / count as f32;

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (i, item) in items.iter_mut().enumerate() {
            if item.result.size.x == 0.0 {
                item.result.size.x = 50.0;
            }
            if item.result.size.y == 0.0 {
                item.result.size.y = 50.0;
            }

            let angle = (i as f32 * angle_step) - std::f32::consts::FRAC_PI_2;
            let cx = radius * angle.cos();
            let cy = radius * angle.sin();

            let pos_x = cx - (item.result.size.x * 0.5);
            let pos_y = cy - (item.result.size.y * 0.5);

            min_x = min_x.min(pos_x);
            max_x = max_x.max(pos_x + item.result.size.x);
            min_y = min_y.min(pos_y);
            max_y = max_y.max(pos_y + item.result.size.y);

            item.result.pos = Vec2::new(pos_x, pos_y);
        }

        let content_width = max_x - min_x;
        let content_height = max_y - min_y;

        let total_w = content_width + ctx.padding_main_start + ctx.padding_main_end;
        let total_h = content_height + ctx.padding_cross_start * 2.0;

        let shift_x = ctx.padding_main_start - min_x;
        let shift_y = ctx.padding_cross_start - min_y;

        for item in items.iter_mut() {
            item.result.pos.x += shift_x;
            item.result.pos.y += shift_y;
        }

        axis.to_world(total_w, total_h)
    }
}
