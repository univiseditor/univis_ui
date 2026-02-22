use bevy::math::Vec2;
use crate::prelude::*;

fn map_legacy_align_items(align: UAlignItems) -> UAlignItemsExt {
    match align {
        UAlignItems::Center => UAlignItemsExt::Center,
        UAlignItems::End | UAlignItems::FlexEnd => UAlignItemsExt::End,
        UAlignItems::Stretch => UAlignItemsExt::Stretch,
        UAlignItems::Baseline => UAlignItemsExt::Baseline,
        _ => UAlignItemsExt::Start,
    }
}

fn map_legacy_align_self(align: UAlignSelf) -> UAlignSelfExt {
    match align {
        UAlignSelf::Center => UAlignSelfExt::Center,
        UAlignSelf::End => UAlignSelfExt::End,
        UAlignSelf::Stretch => UAlignSelfExt::Stretch,
        UAlignSelf::Auto => UAlignSelfExt::Auto,
        UAlignSelf::Start => UAlignSelfExt::Start,
    }
}

fn map_items_ext_to_self_ext(align: UAlignItemsExt) -> UAlignSelfExt {
    match align {
        UAlignItemsExt::Center => UAlignSelfExt::Center,
        UAlignItemsExt::End | UAlignItemsExt::FlexEnd => UAlignSelfExt::End,
        UAlignItemsExt::Stretch => UAlignSelfExt::Stretch,
        UAlignItemsExt::Baseline => UAlignSelfExt::Baseline,
        UAlignItemsExt::FirstBaseline => UAlignSelfExt::FirstBaseline,
        UAlignItemsExt::LastBaseline => UAlignSelfExt::LastBaseline,
        UAlignItemsExt::FlexStart => UAlignSelfExt::FlexStart,
        UAlignItemsExt::SelfStart => UAlignSelfExt::SelfStart,
        UAlignItemsExt::SelfEnd => UAlignSelfExt::SelfEnd,
        UAlignItemsExt::Left => UAlignSelfExt::Left,
        UAlignItemsExt::Right => UAlignSelfExt::Right,
        UAlignItemsExt::Normal | UAlignItemsExt::Start => UAlignSelfExt::Start,
    }
}

fn canonical_align_self(value: UAlignSelfExt) -> UAlignSelfExt {
    match value {
        UAlignSelfExt::Auto | UAlignSelfExt::Normal | UAlignSelfExt::Baseline | UAlignSelfExt::FirstBaseline | UAlignSelfExt::LastBaseline => UAlignSelfExt::Start,
        UAlignSelfExt::FlexStart | UAlignSelfExt::SelfStart | UAlignSelfExt::Left => UAlignSelfExt::Start,
        UAlignSelfExt::FlexEnd | UAlignSelfExt::SelfEnd | UAlignSelfExt::Right => UAlignSelfExt::End,
        other => other,
    }
}

fn alignment_offset(align: UAlignSelfExt, free_space_raw: f32, overflow: UOverflowPosition) -> f32 {
    let free_space = if overflow == UOverflowPosition::Safe {
        free_space_raw.max(0.0)
    } else {
        free_space_raw
    };

    match canonical_align_self(align) {
        UAlignSelfExt::Start => 0.0,
        UAlignSelfExt::Center => free_space * 0.5,
        UAlignSelfExt::End => free_space,
        UAlignSelfExt::Stretch => 0.0,
        _ => 0.0,
    }
}

fn resolve_cross_align(spec: &SolverSpec, container_align: UAlignItems) -> UAlignSelfExt {
    if let Some(ext) = spec.align_self_ext {
        if !matches!(ext, UAlignSelfExt::Auto | UAlignSelfExt::Normal) {
            return canonical_align_self(ext);
        }
    }

    if let Some(legacy) = spec.align_self {
        if legacy != UAlignSelf::Auto {
            return canonical_align_self(map_legacy_align_self(legacy));
        }
    }

    canonical_align_self(map_items_ext_to_self_ext(map_legacy_align_items(container_align)))
}

fn resolve_justify_self(spec: &SolverSpec, ctx: &PlacementContext) -> UAlignSelfExt {
    if let Some(ext) = spec.justify_self_ext {
        if !matches!(ext, UAlignSelfExt::Auto | UAlignSelfExt::Normal) {
            return canonical_align_self(ext);
        }
    }

    if let Some(container_justify_items) = ctx.justify_items {
        return canonical_align_self(map_items_ext_to_self_ext(container_justify_items));
    }

    UAlignSelfExt::Start
}

fn resolve_track_sizes(
    template: &[UTrackSize],
    fallback_count: usize,
    auto_track: UTrackSize,
    available_space: f32,
    gap: f32,
    required_min: usize,
) -> Vec<f32> {
    let base_count = if template.is_empty() { fallback_count.max(1) } else { template.len() };
    let mut track_defs = if template.is_empty() {
        vec![auto_track; base_count]
    } else {
        template.to_vec()
    };

    while track_defs.len() < required_min.max(1) {
        track_defs.push(auto_track);
    }

    let count = track_defs.len();
    let total_gap = if count > 1 { (count as f32 - 1.0) * gap } else { 0.0 };
    let distributable = (available_space - total_gap).max(0.0);

    let mut fixed_sum = 0.0;
    let mut fr_sum = 0.0;
    let mut auto_count = 0usize;

    for track in &track_defs {
        match *track {
            UTrackSize::Px(v) => fixed_sum += v.max(0.0),
            UTrackSize::Fr(v) => fr_sum += v.max(0.0),
            UTrackSize::Auto => auto_count += 1,
        }
    }

    let remaining = (distributable - fixed_sum).max(0.0);
    let auto_size = if fr_sum <= 0.0 && auto_count > 0 {
        remaining / auto_count as f32
    } else {
        0.0
    };

    track_defs
        .iter()
        .map(|track| match *track {
            UTrackSize::Px(v) => v.max(0.0),
            UTrackSize::Fr(v) => {
                if fr_sum > 0.0 {
                    (remaining * (v.max(0.0) / fr_sum)).max(0.0)
                } else {
                    0.0
                }
            }
            UTrackSize::Auto => auto_size.max(0.0),
        })
        .collect()
}

fn ensure_grid_rows(occupancy: &mut Vec<Vec<bool>>, rows: usize, cols: usize) {
    while occupancy.len() < rows {
        occupancy.push(vec![false; cols]);
    }
}

fn can_place_span(
    occupancy: &Vec<Vec<bool>>,
    row: usize,
    col: usize,
    row_span: usize,
    col_span: usize,
    cols: usize,
) -> bool {
    if col + col_span > cols {
        return false;
    }

    for rr in row..(row + row_span) {
        if rr >= occupancy.len() {
            continue;
        }
        for cc in col..(col + col_span) {
            if occupancy[rr][cc] {
                return false;
            }
        }
    }

    true
}

fn mark_span(
    occupancy: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    row_span: usize,
    col_span: usize,
) {
    for rr in row..(row + row_span) {
        for cc in col..(col + col_span) {
            occupancy[rr][cc] = true;
        }
    }
}

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

        let content_main = (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let wrap_enabled = ctx.flex_wrap != UFlexWrap::NoWrap;

        let mut lines: Vec<FlexLine> = Vec::new();
        let mut line_start = 0usize;
        let mut line_main = 0.0;
        let mut line_cross = 0.0;

        for i in 0..items.len() {
            let item = &items[i];
            let (child_main, child_cross) = axis.from_world(item.result.size);
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);
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
        let total_line_cross: f32 = line_sizes.iter().sum();
        let total_line_gaps = if line_count > 1 {
            (line_count as f32 - 1.0) * ctx.cross_gap
        } else {
            0.0
        };
        let content_cross = (ctx.container_cross_size - (ctx.padding_cross_start * 2.0)).max(0.0);
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
                    (0.0, ctx.cross_gap + (free_cross / (line_count as f32 - 1.0)))
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
                line_starts[i] =
                    ctx.container_cross_size - ctx.padding_cross_start - ((line_starts[i] - ctx.padding_cross_start) + line_sizes[i]);
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
                        (ctx.padding_main_start, free_main / (line_items as f32 - 1.0))
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
                let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

                let cross_align = resolve_cross_align(&item.spec, ctx.align_items);
                let (_, cross_mode, _) = {
                    let (main_mode, _, _) = axis.get_main_spec(&item.spec);
                    let (cross_mode, _, _) = axis.get_cross_spec(&item.spec);
                    (main_mode, cross_mode, ())
                };

                if canonical_align_self(cross_align) == UAlignSelfExt::Stretch && cross_mode != SolverSizeMode::Fixed {
                    child_cross = (line_sizes[line_idx] - m_cross_start - m_cross_end).max(0.0);
                    item.result.size = axis.to_world(child_main, child_cross);
                }

                let occupied_cross = child_cross + m_cross_start + m_cross_end;
                let free_cross_in_line = line_sizes[line_idx] - occupied_cross;
                let cross_offset = alignment_offset(cross_align, free_cross_in_line, item.spec.align_overflow);
                let pos_cross = line_starts[line_idx] + m_cross_start + cross_offset;

                let mut pos_main = main_cursor + m_main_start;
                if axis.is_reverse() {
                    pos_main = ctx.container_main_size - (pos_main + child_main);
                }

                item.result.pos = axis.to_world(pos_main, pos_cross);
                let item_span = m_main_start + child_main + m_main_end;

                if matches!(ctx.justify_content, UJustifyContent::SpaceEvenly | UJustifyContent::SpaceAround) {
                    main_cursor += item_span + step_extra;
                } else {
                    main_cursor += item_span + ctx.main_gap + step_extra;
                }

                max_main_used = max_main_used.max(line_main_span);
            }
        }

        let total_cross_used: f32 = line_sizes.iter().sum::<f32>() + if line_count > 1 { (line_count as f32 - 1.0) * ctx.cross_gap } else { 0.0 };
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

        let available_main = (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let available_cross = (ctx.container_cross_size - ctx.padding_cross_start * 2.0).max(0.0);

        let fallback_cols = self.columns.max(ctx.grid_columns as usize).max(1);
        let mut required_cols = fallback_cols;
        for item in items.iter() {
            let col_start = item.spec.grid_column_start.unwrap_or(1) as usize;
            let col_span = item.spec.grid_column_span.max(1) as usize;
            required_cols = required_cols.max(col_start.saturating_sub(1) + col_span);
        }

        let col_sizes = resolve_track_sizes(
            &ctx.grid_template_columns,
            fallback_cols,
            ctx.grid_auto_columns,
            available_main,
            ctx.main_gap,
            required_cols,
        );
        let cols = col_sizes.len().max(1);

        let mut occupancy: Vec<Vec<bool>> = vec![vec![false; cols]];
        let mut placements: Vec<(usize, usize, usize, usize)> = Vec::with_capacity(items.len());

        for item in items.iter() {
            let col_span = item.spec.grid_column_span.max(1) as usize;
            let row_span = item.spec.grid_row_span.max(1) as usize;
            let fixed_col = item.spec.grid_column_start.map(|v| v.saturating_sub(1) as usize).map(|v| v.min(cols.saturating_sub(1)));
            let fixed_row = item.spec.grid_row_start.map(|v| v.saturating_sub(1) as usize);

            let mut found = None;

            if let (Some(row), Some(col)) = (fixed_row, fixed_col) {
                ensure_grid_rows(&mut occupancy, row + row_span, cols);
                if can_place_span(&occupancy, row, col, row_span, col_span, cols) {
                    found = Some((row, col));
                }
            }

            if found.is_none() {
                match ctx.grid_auto_flow {
                    UGridAutoFlow::Row => {
                        let mut row = fixed_row.unwrap_or(0);
                        loop {
                            ensure_grid_rows(&mut occupancy, row + row_span, cols);
                            if let Some(fc) = fixed_col {
                                if can_place_span(&occupancy, row, fc, row_span, col_span, cols) {
                                    found = Some((row, fc));
                                    break;
                                }
                            } else {
                                for col in 0..cols {
                                    if can_place_span(&occupancy, row, col, row_span, col_span, cols) {
                                        found = Some((row, col));
                                        break;
                                    }
                                }
                                if found.is_some() {
                                    break;
                                }
                            }
                            row += 1;
                        }
                    }
                    UGridAutoFlow::Column => {
                        let mut search_rows = occupancy.len().max(1);
                        loop {
                            ensure_grid_rows(&mut occupancy, search_rows + row_span, cols);
                            if let Some(fc) = fixed_col {
                                for row in fixed_row.unwrap_or(0)..(search_rows + row_span) {
                                    if can_place_span(&occupancy, row, fc, row_span, col_span, cols) {
                                        found = Some((row, fc));
                                        break;
                                    }
                                }
                            } else {
                                for col in 0..cols {
                                    for row in fixed_row.unwrap_or(0)..(search_rows + row_span) {
                                        if can_place_span(&occupancy, row, col, row_span, col_span, cols) {
                                            found = Some((row, col));
                                            break;
                                        }
                                    }
                                    if found.is_some() {
                                        break;
                                    }
                                }
                            }
                            if found.is_some() {
                                break;
                            }
                            search_rows += 1;
                        }
                    }
                }
            }

            let (row, col) = found.unwrap_or((0, 0));
            ensure_grid_rows(&mut occupancy, row + row_span, cols);
            mark_span(&mut occupancy, row, col, row_span, col_span);
            placements.push((row, col, row_span, col_span));
        }

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
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let cell_main_start = ctx.padding_main_start + col_starts[col];
            let cell_cross_start = ctx.padding_cross_start + row_starts[row];

            let mut cell_main_size = 0.0;
            for c in col..(col + col_span).min(cols) {
                cell_main_size += col_sizes[c];
            }
            if col_span > 1 {
                cell_main_size += (col_span as f32 - 1.0) * ctx.main_gap;
            }

            let mut cell_cross_size = 0.0;
            for r in row..(row + row_span).min(rows) {
                cell_cross_size += row_sizes[r];
            }
            if row_span > 1 {
                cell_cross_size += (row_span as f32 - 1.0) * ctx.cross_gap;
            }

            let (main_mode, _, _) = axis.get_main_spec(&item.spec);
            let (cross_mode, _, _) = axis.get_cross_spec(&item.spec);

            let (mut child_main, mut child_cross) = axis.from_world(item.result.size);

            let justify_self = resolve_justify_self(&item.spec, ctx);
            if canonical_align_self(justify_self) == UAlignSelfExt::Stretch && main_mode != SolverSizeMode::Fixed {
                child_main = (cell_main_size - m_main_start - m_main_end).max(0.0);
            }

            let align_self = resolve_cross_align(&item.spec, ctx.align_items);
            if canonical_align_self(align_self) == UAlignSelfExt::Stretch && cross_mode != SolverSizeMode::Fixed {
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

        let total_main = col_sizes.iter().sum::<f32>() + if cols > 1 { (cols as f32 - 1.0) * ctx.main_gap } else { 0.0 } + ctx.padding_main_start + ctx.padding_main_end;
        let total_cross = row_sizes.iter().sum::<f32>() + if rows > 1 { (rows as f32 - 1.0) * ctx.cross_gap } else { 0.0 } + (ctx.padding_cross_start * 2.0);

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
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let total_child_main = child_main + m_main_start + m_main_end;
            let free_main = (ctx.container_main_size - (ctx.padding_main_start + ctx.padding_main_end) - total_child_main).max(0.0);
            let center_main = ctx.padding_main_start + (free_main * 0.5) + m_main_start;

            let align_self = resolve_cross_align(&item.spec, ctx.align_items);
            let occupied_cross = child_cross + m_cross_start + m_cross_end;
            let free_cross = (ctx.container_cross_size - (ctx.padding_cross_start * 2.0) - occupied_cross).max(0.0);
            let center_cross = ctx.padding_cross_start + m_cross_start + alignment_offset(align_self, free_cross, item.spec.align_overflow);

            let offset = i as f32 * offset_step;
            item.result.pos = axis.to_world(center_main + offset, center_cross + offset);

            max_main_used = max_main_used.max(center_main + offset + child_main + m_main_end);
            max_cross_used = max_cross_used.max(center_cross + offset + child_cross + m_cross_end);
        }

        Vec2::new(max_main_used + ctx.padding_main_end, max_cross_used + ctx.padding_cross_start)
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
        let available_width = (ctx.container_main_size - ctx.padding_main_start - ctx.padding_main_end).max(0.0);
        let col_width = (available_width - total_gaps).max(0.0) / self.columns as f32;

        let mut col_heights = vec![ctx.padding_cross_start; self.columns];

        for item in items.iter_mut() {
            let (m_main_start, m_main_end, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);

            let (shortest_col_idx, &current_y) = col_heights
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let pos_main = ctx.padding_main_start + (shortest_col_idx as f32 * (col_width + ctx.main_gap)) + m_main_start;
            let pos_cross = current_y + m_cross_start;
            item.result.pos = axis.to_world(pos_main, pos_cross);

            let (_, child_cross) = axis.from_world(item.result.size);
            let new_main = (col_width - m_main_start - m_main_end).max(0.0);
            item.result.size = axis.to_world(new_main, child_cross);

            col_heights[shortest_col_idx] += m_cross_start + child_cross + m_cross_end + ctx.cross_gap;
        }

        let max_height = *col_heights.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        Vec2::new(ctx.container_main_size, max_height + ctx.padding_cross_start)
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
            let total_item_width: f32 = items.iter().map(|i| axis.from_world(i.result.size).0).sum();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn default_spec() -> SolverSpec {
        SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 0.0,
            width_flex: 0.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 0.0,
            height_flex: 0.0,
            position_type: UPositionType::Relative,
            left: UVal::Auto,
            right: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            align_self: None,
            align_self_ext: None,
            justify_self_ext: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            grid_column_start: None,
            grid_column_span: 1,
            grid_row_start: None,
            grid_row_span: 1,
            order: 0,
        }
    }

    fn base_ctx() -> PlacementContext {
        PlacementContext {
            container_main_size: 100.0,
            container_cross_size: 100.0,
            padding_main_start: 0.0,
            padding_main_end: 0.0,
            padding_cross_start: 0.0,
            gap: 0.0,
            main_gap: 0.0,
            cross_gap: 0.0,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Start,
            justify_items: None,
            align_content: None,
            flex_wrap: UFlexWrap::NoWrap,
            grid_columns: 1,
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_auto_flow: UGridAutoFlow::Row,
            grid_auto_rows: UTrackSize::Auto,
            grid_auto_columns: UTrackSize::Auto,
        }
    }

    #[test]
    fn track_resolution_uses_fr_distribution() {
        let tracks = resolve_track_sizes(
            &[UTrackSize::Px(100.0), UTrackSize::Fr(1.0), UTrackSize::Fr(2.0)],
            3,
            UTrackSize::Auto,
            400.0,
            10.0,
            3,
        );

        assert_eq!(tracks.len(), 3);
        assert!((tracks[0] - 100.0).abs() < 0.001);
        assert!(tracks[2] > tracks[1]);
    }

    #[test]
    fn safe_overflow_keeps_non_negative_offset() {
        let offset = alignment_offset(UAlignSelfExt::Center, -20.0, UOverflowPosition::Safe);
        assert!(offset >= 0.0);
    }

    #[test]
    fn unsafe_overflow_allows_negative_offset() {
        let offset = alignment_offset(UAlignSelfExt::Center, -20.0, UOverflowPosition::Unsafe);
        assert!(offset < 0.0);
    }

    #[test]
    fn ext_align_self_overrides_legacy_align_self() {
        let mut spec = default_spec();
        spec.align_self = Some(UAlignSelf::Start);
        spec.align_self_ext = Some(UAlignSelfExt::End);
        let resolved = resolve_cross_align(&spec, UAlignItems::Start);
        assert_eq!(resolved, UAlignSelfExt::End);
    }

    #[test]
    fn justify_self_centers_grid_item_inside_cell() {
        let mut result = SolverResult {
            size: Vec2::new(50.0, 30.0),
            pos: Vec2::ZERO,
        };
        let mut spec = default_spec();
        spec.justify_self_ext = Some(UAlignSelfExt::Center);
        let mut items = vec![SolverItem {
            spec,
            result: &mut result,
            margin: USides::default(),
        }];

        let mut ctx = base_ctx();
        ctx.container_main_size = 100.0;
        ctx.container_cross_size = 60.0;
        ctx.grid_columns = 1;
        ctx.grid_template_columns = vec![UTrackSize::Px(100.0)];
        ctx.grid_template_rows = vec![UTrackSize::Px(60.0)];

        let placer = GridPlacer { columns: 1 };
        let axis = AxisHelper::new(UFlexDirection::Row);
        placer.place(&mut items, &axis, &ctx);
        assert!((result.pos.x - 25.0).abs() < 0.1);
    }

    #[test]
    fn wrap_and_align_content_center_pushes_lines_down() {
        let mut r1 = SolverResult { size: Vec2::new(70.0, 20.0), pos: Vec2::ZERO };
        let mut r2 = SolverResult { size: Vec2::new(70.0, 20.0), pos: Vec2::ZERO };
        let mut r3 = SolverResult { size: Vec2::new(70.0, 20.0), pos: Vec2::ZERO };
        let spec = default_spec();
        let mut items = vec![
            SolverItem { spec, result: &mut r1, margin: USides::default() },
            SolverItem { spec, result: &mut r2, margin: USides::default() },
            SolverItem { spec, result: &mut r3, margin: USides::default() },
        ];

        let mut ctx = base_ctx();
        ctx.container_main_size = 100.0;
        ctx.container_cross_size = 200.0;
        ctx.flex_wrap = UFlexWrap::Wrap;
        ctx.align_content = Some(UContentAlignExt::Center);
        ctx.main_gap = 0.0;
        ctx.cross_gap = 10.0;

        let placer = FlexPlacer;
        let axis = AxisHelper::new(UFlexDirection::Row);
        placer.place(&mut items, &axis, &ctx);

        assert!(r1.pos.y > 0.0);
        assert!(r2.pos.y > r1.pos.y);
        assert!(r3.pos.y > r2.pos.y);
    }

    #[test]
    fn row_gap_is_applied_between_wrapped_lines() {
        let mut r1 = SolverResult { size: Vec2::new(70.0, 10.0), pos: Vec2::ZERO };
        let mut r2 = SolverResult { size: Vec2::new(70.0, 10.0), pos: Vec2::ZERO };
        let spec = default_spec();
        let mut items = vec![
            SolverItem { spec, result: &mut r1, margin: USides::default() },
            SolverItem { spec, result: &mut r2, margin: USides::default() },
        ];

        let mut ctx = base_ctx();
        ctx.container_main_size = 100.0;
        ctx.container_cross_size = 100.0;
        ctx.flex_wrap = UFlexWrap::Wrap;
        ctx.cross_gap = 20.0;

        let placer = FlexPlacer;
        let axis = AxisHelper::new(UFlexDirection::Row);
        placer.place(&mut items, &axis, &ctx);

        let delta = r2.pos.y - r1.pos.y;
        assert!(delta >= 30.0 - 0.1);
    }

    #[test]
    fn grid_span_affects_auto_placement() {
        let mut r1 = SolverResult { size: Vec2::new(20.0, 20.0), pos: Vec2::ZERO };
        let mut r2 = SolverResult { size: Vec2::new(20.0, 20.0), pos: Vec2::ZERO };
        let mut r3 = SolverResult { size: Vec2::new(20.0, 20.0), pos: Vec2::ZERO };

        let mut s1 = default_spec();
        s1.grid_column_span = 2;
        let s2 = default_spec();
        let s3 = default_spec();

        let mut items = vec![
            SolverItem { spec: s1, result: &mut r1, margin: USides::default() },
            SolverItem { spec: s2, result: &mut r2, margin: USides::default() },
            SolverItem { spec: s3, result: &mut r3, margin: USides::default() },
        ];

        let mut ctx = base_ctx();
        ctx.container_main_size = 100.0;
        ctx.container_cross_size = 100.0;
        ctx.grid_columns = 2;
        ctx.grid_template_columns = vec![UTrackSize::Px(50.0), UTrackSize::Px(50.0)];
        ctx.grid_template_rows = vec![UTrackSize::Px(30.0), UTrackSize::Px(30.0)];

        let placer = GridPlacer { columns: 2 };
        let axis = AxisHelper::new(UFlexDirection::Row);
        placer.place(&mut items, &axis, &ctx);

        assert!(r2.pos.y >= 30.0 - 0.1);
        assert!(r3.pos.y >= 30.0 - 0.1);
    }
}
