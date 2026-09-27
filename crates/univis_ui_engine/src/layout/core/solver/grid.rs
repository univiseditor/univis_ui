use bevy::prelude::*;
use std::collections::HashSet;

use super::absolute::solve_absolute_box;
use super::helpers::map_align_items_to_ext;
use super::types::{SolverConfig, SolverItem};
use crate::layout::geometry::BoxConstraints;
use crate::layout::grid::{UTrackBound, UTrackRepeat, UTrackSize};
use crate::layout::solver_types::SolverSizeMode;
use crate::layout::univis_node::{UAlignItemsExt, UAlignSelfExt, UGridAutoFlow, UPositionType};

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrackKind {
    Px(f32),
    Fr(f32),
    Auto,
}

fn expand_tracks(
    template: &[UTrackSize],
    available_space: f32,
    gap: f32,
    fallback_count: usize,
) -> Vec<TrackKind> {
    let mut tracks = Vec::new();

    for track in template {
        match *track {
            UTrackSize::Px(v) => tracks.push(TrackKind::Px(v.max(0.0))),
            UTrackSize::Percent(p) => tracks.push(TrackKind::Px((p * available_space).max(0.0))),
            UTrackSize::Fr(v) => tracks.push(TrackKind::Fr(v.max(0.0))),
            UTrackSize::Auto => tracks.push(TrackKind::Auto),
            UTrackSize::MinMax { min, max } => {
                // If max is Fr, min is floor. If max is Px/Percent, resolve max.
                match max {
                    UTrackBound::Fr(v) => tracks.push(TrackKind::Fr(v.max(0.0))),
                    UTrackBound::Px(v) => tracks.push(TrackKind::Px(v.max(0.0))),
                    UTrackBound::Percent(p) => {
                        tracks.push(TrackKind::Px((p * available_space).max(0.0)))
                    }
                    UTrackBound::Auto => {
                        if let Some(min_px) = min.resolve_px(available_space) {
                            tracks.push(TrackKind::Px(min_px));
                        } else {
                            tracks.push(TrackKind::Auto);
                        }
                    }
                }
            }
            UTrackSize::Repeat(count, repeat) => {
                let kind = repeat_to_track_kind(repeat, available_space);
                for _ in 0..count {
                    tracks.push(kind);
                }
            }
            UTrackSize::RepeatFill(repeat) | UTrackSize::RepeatFit(repeat) => {
                let (min_px, kind) = match repeat {
                    UTrackRepeat::Px(v) => (v, TrackKind::Px(v)),
                    UTrackRepeat::Percent(p) => {
                        (p * available_space, TrackKind::Px(p * available_space))
                    }
                    UTrackRepeat::Fr(v) => (0.0, TrackKind::Fr(v)),
                    UTrackRepeat::Auto => (0.0, TrackKind::Auto),
                    UTrackRepeat::MinMax { min, max: _ } => {
                        let px = min.resolve_px(available_space).unwrap_or(0.0);
                        (px, TrackKind::Px(px))
                    }
                };
                if min_px > 0.0 && available_space > 0.0 {
                    let count =
                        ((available_space + gap) / (min_px + gap)).floor().max(1.0) as usize;
                    for _ in 0..count {
                        tracks.push(kind);
                    }
                } else {
                    tracks.push(kind);
                }
            }
        }
    }

    if tracks.is_empty() {
        let count = fallback_count.max(1);
        for _ in 0..count {
            tracks.push(TrackKind::Fr(1.0));
        }
    }

    tracks
}

fn repeat_to_track_kind(repeat: UTrackRepeat, available_space: f32) -> TrackKind {
    match repeat {
        UTrackRepeat::Px(v) => TrackKind::Px(v.max(0.0)),
        UTrackRepeat::Percent(p) => TrackKind::Px((p * available_space).max(0.0)),
        UTrackRepeat::Fr(v) => TrackKind::Fr(v.max(0.0)),
        UTrackRepeat::Auto => TrackKind::Auto,
        UTrackRepeat::MinMax { min, max } => match max {
            UTrackBound::Fr(v) => TrackKind::Fr(v.max(0.0)),
            UTrackBound::Px(v) => TrackKind::Px(v.max(0.0)),
            UTrackBound::Percent(p) => TrackKind::Px((p * available_space).max(0.0)),
            UTrackBound::Auto => {
                if let Some(min_px) = min.resolve_px(available_space) {
                    TrackKind::Px(min_px)
                } else {
                    TrackKind::Auto
                }
            }
        },
    }
}

/// Solves CSS Grid layout for child items.
pub fn solve_grid_layout(
    config: &SolverConfig,
    constraints: BoxConstraints,
    items: &mut [SolverItem],
) -> Vec2 {
    let padding = config.padding;
    let col_gap = config.column_gap.unwrap_or(config.gap);
    let row_gap = config.row_gap.unwrap_or(config.gap);

    let mut normal_indices = Vec::new();
    let mut absolute_indices = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if item.spec.position_type == UPositionType::Absolute {
            absolute_indices.push(i);
        } else {
            normal_indices.push(i);
        }
    }
    normal_indices.sort_by_key(|&i| items[i].spec.order);

    let available_content_w = (constraints.max_width - padding.left - padding.right).max(0.0);
    let available_content_h = (constraints.max_height - padding.top - padding.bottom).max(0.0);

    let fallback_cols = config.grid_columns.max(1) as usize;
    let mut col_tracks = expand_tracks(
        &config.grid_template_columns,
        available_content_w,
        col_gap,
        fallback_cols,
    );
    let mut row_tracks = expand_tracks(&config.grid_template_rows, available_content_h, row_gap, 1);

    // Grid placement: map each normal item index to (row, col, row_span, col_span)
    let mut placements = Vec::with_capacity(normal_indices.len());
    let mut occupied = HashSet::<(usize, usize)>::new();

    // 1. Place explicitly positioned items
    let mut unplaced = Vec::new();
    for &idx in &normal_indices {
        let item = &items[idx];
        let col_start = item
            .spec
            .grid_column_start
            .map(|s| (s.saturating_sub(1)) as usize);
        let row_start = item
            .spec
            .grid_row_start
            .map(|s| (s.saturating_sub(1)) as usize);
        let col_span = (item.spec.grid_column_span as usize).max(1);
        let row_span = (item.spec.grid_row_span as usize).max(1);

        if let (Some(r), Some(c)) = (row_start, col_start) {
            mark_occupied(&mut occupied, r, c, row_span, col_span);
            placements.push((idx, r, c, row_span, col_span));
        } else {
            unplaced.push((idx, row_start, col_start, row_span, col_span));
        }
    }

    // 2. Auto-place remaining items
    let is_row_flow = config.grid_auto_flow != UGridAutoFlow::Column;
    let num_explicit_cols = col_tracks.len();

    let mut cursor_row = 0;
    let mut cursor_col = 0;

    for (idx, r_opt, c_opt, row_span, col_span) in unplaced {
        if is_row_flow {
            if let Some(r) = r_opt {
                cursor_row = r;
                cursor_col = 0;
            }
            if let Some(c) = c_opt {
                cursor_col = c;
            }

            loop {
                if num_explicit_cols > 0 && cursor_col + col_span > num_explicit_cols {
                    cursor_col = 0;
                    cursor_row += 1;
                }

                if fits(&occupied, cursor_row, cursor_col, row_span, col_span) {
                    mark_occupied(&mut occupied, cursor_row, cursor_col, row_span, col_span);
                    placements.push((idx, cursor_row, cursor_col, row_span, col_span));
                    cursor_col += col_span;
                    if num_explicit_cols > 0 && cursor_col >= num_explicit_cols {
                        cursor_col = 0;
                        cursor_row += 1;
                    }
                    break;
                }
                cursor_col += 1;
            }
        } else {
            // Column flow
            let num_explicit_rows = row_tracks.len();
            if let Some(c) = c_opt {
                cursor_col = c;
                cursor_row = 0;
            }
            if let Some(r) = r_opt {
                cursor_row = r;
            }

            loop {
                if num_explicit_rows > 0 && cursor_row + row_span > num_explicit_rows {
                    cursor_row = 0;
                    cursor_col += 1;
                }

                if fits(&occupied, cursor_row, cursor_col, row_span, col_span) {
                    mark_occupied(&mut occupied, cursor_row, cursor_col, row_span, col_span);
                    placements.push((idx, cursor_row, cursor_col, row_span, col_span));
                    cursor_row += row_span;
                    if num_explicit_rows > 0 && cursor_row >= num_explicit_rows {
                        cursor_row = 0;
                        cursor_col += 1;
                    }
                    break;
                }
                cursor_row += 1;
            }
        }
    }

    // Determine max rows and cols needed
    let mut max_col = col_tracks.len();
    let mut max_row = row_tracks.len();
    for &(_, r, c, r_span, c_span) in &placements {
        max_col = max_col.max(c + c_span);
        max_row = max_row.max(r + r_span);
    }

    // Expand implicit columns/rows if needed
    while col_tracks.len() < max_col {
        col_tracks.push(TrackKind::Fr(1.0));
    }
    while row_tracks.len() < max_row {
        row_tracks.push(TrackKind::Auto);
    }

    // 3. Resolve Column Widths
    let col_widths = resolve_track_sizes(
        &col_tracks,
        available_content_w,
        col_gap,
        &items,
        &placements,
        true,
    );

    // 4. Resolve Row Heights
    let row_heights = resolve_track_sizes(
        &row_tracks,
        available_content_h,
        row_gap,
        &items,
        &placements,
        false,
    );

    // 5. Compute track offsets
    let mut col_offsets = Vec::with_capacity(col_widths.len());
    let mut current_x = 0.0;
    for (i, &w) in col_widths.iter().enumerate() {
        col_offsets.push(current_x);
        current_x += w;
        if i + 1 < col_widths.len() {
            current_x += col_gap;
        }
    }

    let mut row_offsets = Vec::with_capacity(row_heights.len());
    let mut current_y = 0.0;
    for (i, &h) in row_heights.iter().enumerate() {
        row_offsets.push(current_y);
        current_y += h;
        if i + 1 < row_heights.len() {
            current_y += row_gap;
        }
    }

    // 6. Size and Position Items
    for &(idx, r, c, r_span, c_span) in &placements {
        let cell_x = col_offsets.get(c).copied().unwrap_or(0.0);
        let cell_y = row_offsets.get(r).copied().unwrap_or(0.0);

        let cell_w = (0..c_span)
            .map(|offset| col_widths.get(c + offset).copied().unwrap_or(0.0))
            .sum::<f32>()
            + (c_span.saturating_sub(1) as f32) * col_gap;

        let cell_h = (0..r_span)
            .map(|offset| row_heights.get(r + offset).copied().unwrap_or(0.0))
            .sum::<f32>()
            + (r_span.saturating_sub(1) as f32) * row_gap;

        let item = &mut items[idx];
        let avail_item_w = (cell_w - item.margin.width_sum()).max(0.0);
        let avail_item_h = (cell_h - item.margin.height_sum()).max(0.0);

        let item_w = match item.spec.width_mode {
            SolverSizeMode::Fixed => item
                .spec
                .width_val
                .clamp(item.spec.min_width, item.spec.max_width),
            SolverSizeMode::Percent => {
                (item.spec.width_val * avail_item_w).clamp(item.spec.min_width, item.spec.max_width)
            }
            SolverSizeMode::Calc => item
                .spec
                .width_uval
                .resolve_or_zero(avail_item_w)
                .clamp(item.spec.min_width, item.spec.max_width),
            SolverSizeMode::MinContent | SolverSizeMode::Content => item
                .spec
                .width_val
                .clamp(item.spec.min_width, item.spec.max_width),
            SolverSizeMode::Auto | SolverSizeMode::Flex => {
                // If align_self or justify_self specifies stretch, fill cell, else intrinsic
                if item.spec.width_val > 0.0
                    && !allows_stretch(item.spec.justify_self_ext, config.justify_items)
                {
                    item.spec
                        .width_val
                        .clamp(item.spec.min_width, item.spec.max_width)
                } else {
                    avail_item_w.clamp(item.spec.min_width, item.spec.max_width)
                }
            }
        };

        let item_h = match item.spec.height_mode {
            SolverSizeMode::Fixed => item
                .spec
                .height_val
                .clamp(item.spec.min_height, item.spec.max_height),
            SolverSizeMode::Percent => (item.spec.height_val * avail_item_h)
                .clamp(item.spec.min_height, item.spec.max_height),
            SolverSizeMode::Calc => item
                .spec
                .height_uval
                .resolve_or_zero(avail_item_h)
                .clamp(item.spec.min_height, item.spec.max_height),
            SolverSizeMode::MinContent | SolverSizeMode::Content => item
                .spec
                .height_val
                .clamp(item.spec.min_height, item.spec.max_height),
            SolverSizeMode::Auto | SolverSizeMode::Flex => {
                if item.spec.height_val > 0.0
                    && !allows_stretch(
                        item.spec.align_self_ext,
                        Some(map_align_items_to_ext(config.layout.align_items)),
                    )
                {
                    item.spec
                        .height_val
                        .clamp(item.spec.min_height, item.spec.max_height)
                } else {
                    avail_item_h.clamp(item.spec.min_height, item.spec.max_height)
                }
            }
        };

        // Align within cell
        let pos_x = padding.left
            + cell_x
            + item.margin.left
            + align_offset(
                item_w,
                avail_item_w,
                item.spec.justify_self_ext,
                config.justify_items,
            );

        let pos_y = padding.top
            + cell_y
            + item.margin.top
            + align_offset(
                item_h,
                avail_item_h,
                item.spec.align_self_ext,
                Some(map_align_items_to_ext(config.layout.align_items)),
            );

        item.result.size = Vec2::new(item_w, item_h);
        item.result.pos = Vec2::new(pos_x, pos_y);
    }

    // Relative item offsets
    let grid_content_w = current_x;
    let grid_content_h = current_y;

    let mut final_container_size = Vec2::new(
        (grid_content_w + padding.width_sum()).clamp(constraints.min_width, constraints.max_width),
        (grid_content_h + padding.height_sum())
            .clamp(constraints.min_height, constraints.max_height),
    );

    if matches!(config.width_mode, SolverSizeMode::Fixed) {
        final_container_size.x = constraints.max_width;
    }
    if matches!(config.height_mode, SolverSizeMode::Fixed) {
        final_container_size.y = constraints.max_height;
    }

    for &idx in &normal_indices {
        let item = &mut items[idx];
        if item.spec.position_type == UPositionType::Relative {
            let offset_x = item.spec.left.resolve_or_zero(final_container_size.x)
                - item.spec.right.resolve_or_zero(final_container_size.x);
            let offset_y = item.spec.top.resolve_or_zero(final_container_size.y)
                - item.spec.bottom.resolve_or_zero(final_container_size.y);
            item.result.pos.x += offset_x;
            item.result.pos.y += offset_y;
        }
    }

    // Absolute items
    for &idx in &absolute_indices {
        let item = &mut items[idx];
        let intrinsic = Vec2::new(
            if matches!(
                item.spec.width_mode,
                SolverSizeMode::Content | SolverSizeMode::MinContent
            ) {
                item.spec.width_val
            } else {
                0.0
            },
            if matches!(
                item.spec.height_mode,
                SolverSizeMode::Content | SolverSizeMode::MinContent
            ) {
                item.spec.height_val
            } else {
                0.0
            },
        );
        let (new_size, new_pos) =
            solve_absolute_box(final_container_size, &item.spec, item.margin, intrinsic);
        item.result.size = new_size;
        item.result.pos = new_pos;
    }

    final_container_size
}

fn fits(
    occupied: &HashSet<(usize, usize)>,
    row: usize,
    col: usize,
    r_span: usize,
    c_span: usize,
) -> bool {
    for r in row..row + r_span {
        for c in col..col + c_span {
            if occupied.contains(&(r, c)) {
                return false;
            }
        }
    }
    true
}

fn mark_occupied(
    occupied: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
    r_span: usize,
    c_span: usize,
) {
    for r in row..row + r_span {
        for c in col..col + c_span {
            occupied.insert((r, c));
        }
    }
}

fn allows_stretch(
    item_align: Option<UAlignSelfExt>,
    container_align: Option<UAlignItemsExt>,
) -> bool {
    match item_align {
        Some(UAlignSelfExt::Stretch) => true,
        Some(_) => false,
        None => matches!(container_align, Some(UAlignItemsExt::Stretch) | None),
    }
}

fn align_offset(
    item_size: f32,
    avail_size: f32,
    item_align: Option<UAlignSelfExt>,
    container_align: Option<UAlignItemsExt>,
) -> f32 {
    let align = item_align.unwrap_or_else(|| match container_align {
        Some(UAlignItemsExt::Center) => UAlignSelfExt::Center,
        Some(UAlignItemsExt::End) => UAlignSelfExt::End,
        _ => UAlignSelfExt::Start,
    });

    match align {
        UAlignSelfExt::Center => (avail_size - item_size).max(0.0) * 0.5,
        UAlignSelfExt::End => (avail_size - item_size).max(0.0),
        _ => 0.0,
    }
}

fn resolve_track_sizes(
    tracks: &[TrackKind],
    available_space: f32,
    gap: f32,
    items: &[SolverItem],
    placements: &[(usize, usize, usize, usize, usize)],
    is_col: bool,
) -> Vec<f32> {
    let num_tracks = tracks.len();
    let total_gaps = (num_tracks.saturating_sub(1) as f32) * gap;
    let space_for_tracks = (available_space - total_gaps).max(0.0);

    let mut sizes = vec![0.0; num_tracks];
    let mut total_fixed = 0.0;
    let mut total_fr = 0.0;

    for (i, &track) in tracks.iter().enumerate() {
        match track {
            TrackKind::Px(v) => {
                sizes[i] = v;
                total_fixed += v;
            }
            TrackKind::Fr(v) => {
                total_fr += v;
            }
            TrackKind::Auto => {
                // Find max intrinsic size of items in this track
                let mut max_item: f32 = 0.0;
                for &(idx, r, c, r_span, c_span) in placements {
                    let (pos, span) = if is_col { (c, c_span) } else { (r, r_span) };
                    if pos == i && span == 1 {
                        let item_size = if is_col {
                            items[idx].spec.width_val + items[idx].margin.width_sum()
                        } else {
                            items[idx].spec.height_val + items[idx].margin.height_sum()
                        };
                        max_item = max_item.max(item_size);
                    }
                }
                sizes[i] = max_item;
                total_fixed += max_item;
            }
        }
    }

    // Distribute remaining space among Fr tracks
    let remaining_space = (space_for_tracks - total_fixed).max(0.0);
    if total_fr > 0.0 {
        let fr_unit = remaining_space / total_fr;
        for (i, &track) in tracks.iter().enumerate() {
            if let TrackKind::Fr(v) = track {
                sizes[i] = v * fr_unit;
            }
        }
    }

    sizes
}
