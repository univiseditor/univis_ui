#![allow(clippy::type_complexity)]

use crate::internal_prelude::*;
use bevy::prelude::*;

#[derive(Clone, Copy)]
struct MeasuredContainerInput {
    width: UVal,
    height: UVal,
    padding: USides,
    width_bounds: (f32, f32),
    height_bounds: (f32, f32),
    aspect_ratio: Option<f32>,
    calculated_min_width: f32,
    calculated_max_width: f32,
    calculated_min_height: f32,
    calculated_max_height: f32,
}

#[derive(Clone, Copy, Default)]
struct ChildMeasureItem {
    min_w: f32,
    max_w: f32,
    min_h: f32,
    max_h: f32,
    margin: USides,
    col_start: Option<u32>,
    col_span: u32,
    row_start: Option<u32>,
    row_span: u32,
}

#[derive(Default)]
#[doc(hidden)]
pub struct MeasurePassScratch {
    child_entities: Vec<Entity>,
    child_items: Vec<ChildMeasureItem>,
    placements: Vec<(usize, usize, usize, usize)>,
    col_min: Vec<f32>,
    col_max: Vec<f32>,
    row_min: Vec<f32>,
    row_max: Vec<f32>,
}

/// The Upward Pass (Bottom-Up) of the layout algorithm.
///
/// Iterates from the deepest tree depth up to the root.
/// Calculates the **Intrinsic Size** of containers based on their children.
/// It ignores `Absolute` items as they are out-of-flow.
///
/// Upward Pass with Caching and Reverse Direction Support
pub fn upward_measure_pass_cached(
    _tree_depth: Res<LayoutTreeDepth>,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut scratch: Local<MeasurePassScratch>,
    mut params: ParamSet<(
        Query<(&IntrinsicSize, &UNode, Option<&USelf>, Option<&ULayout>)>,
        Query<(Entity, &UNode, Option<&Children>, Option<&ULayout>)>,
        Query<&mut IntrinsicSize>,
    )>,
) {
    let start = std::time::Instant::now();
    let use_incremental_measure = rollout
        .as_ref()
        .map_or(true, |config| config.use_incremental_measure);
    let frontier = if use_incremental_measure {
        cache.take_measure_frontier()
    } else {
        cache.all_entities_bottom_up()
    };
    let mut calculated_count = 0;

    for entity in frontier {
        scratch.child_entities.clear();
        scratch.child_items.clear();
        let Some(measured_input) = (|| {
            let (
                width,
                height,
                padding,
                width_bounds,
                height_bounds,
                aspect_ratio,
                layout_clone,
                measure_scratch_grew,
            ) = {
                let nodes = params.p1();
                let Ok((_, node_spec, children_opt, layout_opt)) = nodes.get(entity) else {
                    return None;
                };

                let measure_scratch_capacity_before = scratch.child_entities.capacity();
                if let Some(children) = children_opt {
                    scratch.child_entities.reserve(children.len());
                    scratch.child_entities.extend(children.iter());
                }

                (
                    node_spec.width,
                    node_spec.height,
                    node_spec.padding,
                    node_spec.width_bounds(),
                    node_spec.height_bounds(),
                    node_spec.sanitized_aspect_ratio(),
                    layout_opt.cloned(),
                    scratch.child_entities.capacity() > measure_scratch_capacity_before,
                )
            };
            if let Some(prof) = profiler.as_mut() {
                if measure_scratch_grew {
                    prof.measure_scratch_alloc_grows += 1;
                }
                prof.measure_scratch_peak =
                    prof.measure_scratch_peak.max(scratch.child_entities.len());
            }

            if !scratch.child_entities.is_empty() {
                let q_children = params.p0();
                let MeasurePassScratch {
                    child_entities,
                    child_items,
                    ..
                } = &mut *scratch;

                for child_entity in child_entities.iter().copied() {
                    if let Ok((child_intrinsic, child_node, child_uself_opt, child_layout_opt)) =
                        q_children.get(child_entity)
                    {
                        if let Some(layout) = child_layout_opt
                            && layout.display == UDisplay::None
                        {
                            continue;
                        }
                        if let Some(uself) = child_uself_opt
                            && uself.position_type == UPositionType::Absolute
                        {
                            continue;
                        }

                        let (col_start, col_span, row_start, row_span) = child_uself_opt
                            .map(|u| {
                                (
                                    u.item_ext.grid.column_start,
                                    u.item_ext.grid.column_span,
                                    u.item_ext.grid.row_start,
                                    u.item_ext.grid.row_span,
                                )
                            })
                            .unwrap_or((None, 1, None, 1));

                        child_items.push(ChildMeasureItem {
                            min_w: child_intrinsic.min_width,
                            max_w: child_intrinsic.max_width,
                            min_h: child_intrinsic.min_height,
                            max_h: child_intrinsic.max_height,
                            margin: child_node.margin,
                            col_start,
                            col_span,
                            row_start,
                            row_span,
                        });
                    }
                }
            }

            let (
                calculated_min_width,
                calculated_max_width,
                calculated_min_height,
                calculated_max_height,
            ) = if scratch.child_items.is_empty() {
                (0.0, 0.0, 0.0, 0.0)
            } else {
                let layout = layout_clone.unwrap_or_default();
                match layout.display {
                    UDisplay::Grid => {
                        measure_grid_intrinsic(&mut *scratch, &layout, width, padding)
                    }
                    UDisplay::Stack => measure_stack_intrinsic(&scratch.child_items),
                    _ => measure_flex_intrinsic(&scratch.child_items, &layout),
                }
            };

            Some(MeasuredContainerInput {
                width,
                height,
                padding,
                width_bounds,
                height_bounds,
                aspect_ratio,
                calculated_min_width,
                calculated_max_width,
                calculated_min_height,
                calculated_max_height,
            })
        })() else {
            cache.clear_dirty(entity);
            continue;
        };

        calculated_count += 1;
        let h_pad = measured_input.padding.width_sum();
        let v_pad = measured_input.padding.height_sum();

        if let Ok(mut intrinsic) = params.p2().get_mut(entity) {
            let mut raw_min_width = match measured_input.width {
                UVal::Px(v) => v,
                _ => measured_input.calculated_min_width + h_pad,
            };
            let mut raw_max_width = match measured_input.width {
                UVal::Px(v) => v,
                _ => measured_input.calculated_max_width + h_pad,
            };
            let mut raw_min_height = match measured_input.height {
                UVal::Px(v) => v,
                _ => measured_input.calculated_min_height + v_pad,
            };
            let mut raw_max_height = match measured_input.height {
                UVal::Px(v) => v,
                _ => measured_input.calculated_max_height + v_pad,
            };

            if let Some(ratio) = measured_input.aspect_ratio {
                let w_is_px = matches!(measured_input.width, UVal::Px(_));
                let h_is_px = matches!(measured_input.height, UVal::Px(_));

                if w_is_px && !h_is_px {
                    raw_min_height = raw_min_width / ratio;
                    raw_max_height = raw_max_width / ratio;
                } else if h_is_px && !w_is_px {
                    raw_min_width = raw_min_height * ratio;
                    raw_max_width = raw_max_height * ratio;
                } else if !w_is_px && !h_is_px {
                    if raw_max_width > h_pad && raw_max_height <= v_pad {
                        raw_min_height = v_pad + (raw_min_width - h_pad).max(0.0) / ratio;
                        raw_max_height = v_pad + (raw_max_width - h_pad).max(0.0) / ratio;
                    } else if raw_max_height > v_pad && raw_max_width <= h_pad {
                        raw_min_width = h_pad + (raw_min_height - v_pad).max(0.0) * ratio;
                        raw_max_width = h_pad + (raw_max_height - v_pad).max(0.0) * ratio;
                    }
                }
            }

            let min_width =
                raw_min_width.clamp(measured_input.width_bounds.0, measured_input.width_bounds.1);
            let max_width = raw_max_width
                .clamp(measured_input.width_bounds.0, measured_input.width_bounds.1)
                .max(min_width);
            let min_height = raw_min_height.clamp(
                measured_input.height_bounds.0,
                measured_input.height_bounds.1,
            );
            let max_height = raw_max_height
                .clamp(
                    measured_input.height_bounds.0,
                    measured_input.height_bounds.1,
                )
                .max(min_height);

            let mut new_width = match measured_input.width {
                UVal::MinContent => min_width,
                _ => max_width,
            };
            let mut new_height = match measured_input.height {
                UVal::MinContent => min_height,
                _ => max_height,
            };

            if let Some(ratio) = measured_input.aspect_ratio {
                let w_is_px = matches!(measured_input.width, UVal::Px(_));
                let h_is_px = matches!(measured_input.height, UVal::Px(_));
                if w_is_px && !h_is_px {
                    new_height = (new_width / ratio).clamp(min_height, max_height);
                } else if h_is_px && !w_is_px {
                    new_width = (new_height * ratio).clamp(min_width, max_width);
                }
            }

            if (intrinsic.width - new_width).abs() > 0.001
                || (intrinsic.height - new_height).abs() > 0.001
                || (intrinsic.min_width - min_width).abs() > 0.001
                || (intrinsic.max_width - max_width).abs() > 0.001
                || (intrinsic.min_height - min_height).abs() > 0.001
                || (intrinsic.max_height - max_height).abs() > 0.001
            {
                intrinsic.width = new_width;
                intrinsic.height = new_height;
                intrinsic.min_width = min_width;
                intrinsic.max_width = max_width;
                intrinsic.min_height = min_height;
                intrinsic.max_height = max_height;
            }

            cache.cache_intrinsic(
                entity,
                IntrinsicSize {
                    width: new_width,
                    height: new_height,
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                },
            );
            cache.complete_measure(entity);
            cache.clear_dirty(entity);
        }
    }

    cache.increment_frame();

    if let Some(ref mut prof) = profiler {
        prof.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.dirty_nodes = calculated_count;
    }
}

fn measure_flex_intrinsic(items: &[ChildMeasureItem], layout: &ULayout) -> (f32, f32, f32, f32) {
    let direction = layout.flex_direction;
    let legacy_gap = layout.gap;
    let row_gap = layout.container_ext.box_align.row_gap;
    let column_gap = layout.container_ext.box_align.column_gap;

    let gap = if matches!(direction, UFlexDirection::Row | UFlexDirection::RowReverse) {
        column_gap.unwrap_or(legacy_gap)
    } else {
        row_gap.unwrap_or(legacy_gap)
    };

    let mut accum_main_min: f32 = 0.0;
    let mut accum_main_max: f32 = 0.0;
    let mut max_cross_min: f32 = 0.0;
    let mut max_cross_max: f32 = 0.0;
    let mut visible_count = 0;

    for item in items {
        let min_w = item.min_w;
        let max_w = item.max_w;
        let min_h = item.min_h;
        let max_h = item.max_h;
        let margin = item.margin;

        match direction {
            UFlexDirection::Row | UFlexDirection::RowReverse => {
                accum_main_min += min_w + margin.left + margin.right;
                accum_main_max += max_w + margin.left + margin.right;
                max_cross_min = max_cross_min.max(min_h + margin.top + margin.bottom);
                max_cross_max = max_cross_max.max(max_h + margin.top + margin.bottom);
            }
            UFlexDirection::Column | UFlexDirection::ColumnReverse => {
                accum_main_min += min_h + margin.top + margin.bottom;
                accum_main_max += max_h + margin.top + margin.bottom;
                max_cross_min = max_cross_min.max(min_w + margin.left + margin.right);
                max_cross_max = max_cross_max.max(max_w + margin.left + margin.right);
            }
        }
        visible_count += 1;
    }

    if visible_count > 1 {
        let gap_total = (visible_count - 1) as f32 * gap;
        accum_main_min += gap_total;
        accum_main_max += gap_total;
    }

    match direction {
        UFlexDirection::Row | UFlexDirection::RowReverse => {
            (accum_main_min, accum_main_max, max_cross_min, max_cross_max)
        }
        UFlexDirection::Column | UFlexDirection::ColumnReverse => {
            (max_cross_min, max_cross_max, accum_main_min, accum_main_max)
        }
    }
}

fn measure_stack_intrinsic(items: &[ChildMeasureItem]) -> (f32, f32, f32, f32) {
    let mut min_w = 0.0f32;
    let mut max_w = 0.0f32;
    let mut min_h = 0.0f32;
    let mut max_h = 0.0f32;

    for item in items {
        let w_extra = item.margin.left + item.margin.right;
        let h_extra = item.margin.top + item.margin.bottom;
        min_w = min_w.max(item.min_w + w_extra);
        max_w = max_w.max(item.max_w + w_extra);
        min_h = min_h.max(item.min_h + h_extra);
        max_h = max_h.max(item.max_h + h_extra);
    }

    (min_w, max_w, min_h, max_h)
}

#[inline]
fn spans_overlap(
    r1: usize,
    c1: usize,
    r_span1: usize,
    c_span1: usize,
    r2: usize,
    c2: usize,
    r_span2: usize,
    c_span2: usize,
) -> bool {
    r1 < r2 + r_span2 && r1 + r_span1 > r2 && c1 < c2 + c_span2 && c1 + c_span1 > c2
}

#[inline]
fn can_place_item(
    placements: &[(usize, usize, usize, usize)],
    r: usize,
    c: usize,
    r_span: usize,
    c_span: usize,
) -> bool {
    for &(pr, pc, pr_span, pc_span) in placements {
        if spans_overlap(r, c, r_span, c_span, pr, pc, pr_span, pc_span) {
            return false;
        }
    }
    true
}

fn measure_grid_intrinsic(
    scratch: &mut MeasurePassScratch,
    layout: &ULayout,
    width: UVal,
    padding: USides,
) -> (f32, f32, f32, f32) {
    let fallback_cols = layout.grid_columns.max(1) as usize;
    let col_gap = layout
        .container_ext
        .box_align
        .column_gap
        .unwrap_or(layout.gap);
    let template_cols_len: usize = layout
        .container_ext
        .grid
        .template_columns
        .iter()
        .map(|t| match *t {
            UTrackSize::Repeat(count, _) => count as usize,
            UTrackSize::RepeatFit(track_repeat) | UTrackSize::RepeatFill(track_repeat) => {
                if let UVal::Px(w) = width {
                    let avail = (w - padding.left - padding.right).max(0.0);
                    let min_bound = match track_repeat {
                        UTrackRepeat::Px(px) => px,
                        UTrackRepeat::Percent(p) => p * avail,
                        UTrackRepeat::MinMax { min, .. } => match min {
                            UTrackBound::Px(px) => px,
                            UTrackBound::Percent(p) => p * avail,
                            _ => 0.0,
                        },
                        _ => 0.0,
                    };
                    if min_bound > 0.0 && avail > 0.0 {
                        let count = ((avail + col_gap) / (min_bound + col_gap)).floor() as usize;
                        let count = count.max(1);
                        if matches!(*t, UTrackSize::RepeatFit(..)) {
                            count.min(scratch.child_items.len().max(1))
                        } else {
                            count
                        }
                    } else {
                        1
                    }
                } else {
                    1
                }
            }
            _ => 1,
        })
        .sum();
    let template_rows_len: usize = layout
        .container_ext
        .grid
        .template_rows
        .iter()
        .map(|t| match *t {
            UTrackSize::Repeat(count, _) => count as usize,
            _ => 1,
        })
        .sum();

    let mut num_cols = if template_cols_len > 0 {
        template_cols_len
    } else {
        fallback_cols
    };

    let mut max_rows = template_rows_len.max(1);

    for item in &scratch.child_items {
        let col_start = item.col_start.unwrap_or(1).saturating_sub(1) as usize;
        let col_span = (item.col_span as usize).max(1);
        num_cols = num_cols.max(col_start + col_span);

        if let Some(r_start) = item.row_start {
            let r_span = (item.row_span as usize).max(1);
            max_rows = max_rows.max(r_start.saturating_sub(1) as usize + r_span);
        }
    }

    scratch.placements.clear();

    let mut auto_cursor_row = 0usize;
    let mut auto_cursor_col = 0usize;

    for i in 0..scratch.child_items.len() {
        let item = scratch.child_items[i];
        let col_span = (item.col_span as usize).max(1);
        let row_span = (item.row_span as usize).max(1);
        let fixed_col = item.col_start.map(|v| v.saturating_sub(1) as usize);
        let fixed_row = item.row_start.map(|v| v.saturating_sub(1) as usize);

        let (found_r, found_c) = if let (Some(r), Some(c)) = (fixed_row, fixed_col) {
            (r, c)
        } else {
            match layout.container_ext.grid.auto_flow {
                UGridAutoFlow::Row => {
                    if let Some(r) = fixed_row {
                        let mut c = 0;
                        while !can_place_item(&scratch.placements, r, c, row_span, col_span) {
                            c += 1;
                        }
                        (r, c)
                    } else if let Some(c) = fixed_col {
                        let mut r = 0;
                        while !can_place_item(&scratch.placements, r, c, row_span, col_span) {
                            r += 1;
                        }
                        (r, c)
                    } else {
                        let mut r = auto_cursor_row;
                        let mut c = auto_cursor_col;
                        loop {
                            if c + col_span <= num_cols
                                && can_place_item(&scratch.placements, r, c, row_span, col_span)
                            {
                                break (r, c);
                            }
                            c += 1;
                            if c + col_span > num_cols {
                                c = 0;
                                r += 1;
                            }
                        }
                    }
                }
                UGridAutoFlow::Column => {
                    if let Some(c) = fixed_col {
                        let mut r = 0;
                        while !can_place_item(&scratch.placements, r, c, row_span, col_span) {
                            r += 1;
                        }
                        (r, c)
                    } else if let Some(r) = fixed_row {
                        let mut c = 0;
                        while !can_place_item(&scratch.placements, r, c, row_span, col_span) {
                            c += 1;
                        }
                        (r, c)
                    } else {
                        let mut r = auto_cursor_row;
                        let mut c = auto_cursor_col;
                        loop {
                            if r + row_span <= max_rows
                                && can_place_item(&scratch.placements, r, c, row_span, col_span)
                            {
                                break (r, c);
                            }
                            r += 1;
                            if r + row_span > max_rows {
                                r = 0;
                                c += 1;
                            }
                        }
                    }
                }
            }
        };

        scratch
            .placements
            .push((found_r, found_c, row_span, col_span));

        if fixed_col.is_none() && fixed_row.is_none() {
            match layout.container_ext.grid.auto_flow {
                UGridAutoFlow::Row => {
                    auto_cursor_row = found_r;
                    auto_cursor_col = found_c + col_span;
                    if auto_cursor_col >= num_cols {
                        auto_cursor_row += auto_cursor_col / num_cols;
                        auto_cursor_col %= num_cols;
                    }
                }
                UGridAutoFlow::Column => {
                    auto_cursor_col = found_c;
                    auto_cursor_row = found_r + row_span;
                    if auto_cursor_row >= max_rows {
                        auto_cursor_col += auto_cursor_row / max_rows;
                        auto_cursor_row %= max_rows;
                    }
                }
            }
        }
    }

    let final_cols = scratch
        .placements
        .iter()
        .map(|(_, c, _, c_span)| c + c_span)
        .max()
        .unwrap_or(0)
        .max(template_cols_len)
        .max(fallback_cols)
        .max(1);

    let final_rows = scratch
        .placements
        .iter()
        .map(|(r, _, r_span, _)| r + r_span)
        .max()
        .unwrap_or(0)
        .max(template_rows_len)
        .max(1);

    scratch.col_min.clear();
    scratch.col_min.resize(final_cols, 0.0);
    scratch.col_max.clear();
    scratch.col_max.resize(final_cols, 0.0);
    scratch.row_min.clear();
    scratch.row_min.resize(final_rows, 0.0);
    scratch.row_max.clear();
    scratch.row_max.resize(final_rows, 0.0);

    let is_fixed_col = |c: usize| -> Option<f32> {
        let first_is_repeat = layout
            .container_ext
            .grid
            .template_columns
            .first()
            .is_some_and(|t| matches!(t, UTrackSize::RepeatFit(_) | UTrackSize::RepeatFill(_)));
        if first_is_repeat {
            match layout.container_ext.grid.template_columns[0] {
                UTrackSize::RepeatFit(rep) | UTrackSize::RepeatFill(rep) => match rep {
                    UTrackRepeat::Px(v) => return Some(v.max(0.0)),
                    UTrackRepeat::MinMax {
                        min: UTrackBound::Px(v),
                        ..
                    } => return Some(v.max(0.0)),
                    _ => return None,
                },
                _ => {}
            }
        }
        let track = layout
            .container_ext
            .grid
            .template_columns
            .get(c)
            .copied()
            .unwrap_or(layout.container_ext.grid.auto_columns);
        match track {
            UTrackSize::Px(v) => Some(v.max(0.0)),
            UTrackSize::MinMax {
                min: UTrackBound::Px(v),
                ..
            } => Some(v.max(0.0)),
            _ => None,
        }
    };

    let is_fixed_row = |r: usize| -> Option<f32> {
        let track = layout
            .container_ext
            .grid
            .template_rows
            .get(r)
            .copied()
            .unwrap_or(layout.container_ext.grid.auto_rows);
        match track {
            UTrackSize::Px(v) => Some(v.max(0.0)),
            UTrackSize::MinMax {
                min: UTrackBound::Px(v),
                ..
            } => Some(v.max(0.0)),
            _ => None,
        }
    };

    for c in 0..final_cols {
        if let Some(fixed) = is_fixed_col(c) {
            scratch.col_min[c] = fixed;
            scratch.col_max[c] = fixed;
        }
    }

    for r in 0..final_rows {
        if let Some(fixed) = is_fixed_row(r) {
            scratch.row_min[r] = fixed;
            scratch.row_max[r] = fixed;
        }
    }

    let col_gap = layout
        .container_ext
        .box_align
        .column_gap
        .unwrap_or(layout.gap);
    let row_gap = layout.container_ext.box_align.row_gap.unwrap_or(layout.gap);

    // Single-span items establish track baseline
    for i in 0..scratch.child_items.len() {
        let (r, c, r_span, c_span) = scratch.placements[i];
        let item = scratch.child_items[i];
        let w_margin = item.margin.left + item.margin.right;
        let h_margin = item.margin.top + item.margin.bottom;

        if c_span == 1 && c < final_cols && is_fixed_col(c).is_none() {
            scratch.col_min[c] = scratch.col_min[c].max(item.min_w + w_margin);
            scratch.col_max[c] = scratch.col_max[c].max(item.max_w + w_margin);
        }

        if r_span == 1 && r < final_rows && is_fixed_row(r).is_none() {
            scratch.row_min[r] = scratch.row_min[r].max(item.min_h + h_margin);
            scratch.row_max[r] = scratch.row_max[r].max(item.max_h + h_margin);
        }
    }

    // Multi-span columns: distribute deficit to spanned tracks
    for current_span in 2..=final_cols {
        for i in 0..scratch.child_items.len() {
            let (_, c, _, c_span) = scratch.placements[i];
            if c_span == current_span && c < final_cols {
                let item = scratch.child_items[i];
                let w_margin = item.margin.left + item.margin.right;
                let end_c = (c + c_span).min(final_cols);
                let span_len = end_c.saturating_sub(c);
                if span_len > 0 {
                    let span_gap = (span_len - 1) as f32 * col_gap;
                    let non_fixed_count =
                        (c..end_c).filter(|&sc| is_fixed_col(sc).is_none()).count();
                    let target_count = if non_fixed_count > 0 {
                        non_fixed_count
                    } else {
                        span_len
                    };

                    let cur_min_sum: f32 = scratch.col_min[c..end_c].iter().sum::<f32>() + span_gap;
                    let req_min = item.min_w + w_margin;
                    if req_min > cur_min_sum {
                        let deficit = req_min - cur_min_sum;
                        let share = deficit / target_count as f32;
                        for sc in c..end_c {
                            if non_fixed_count == 0 || is_fixed_col(sc).is_none() {
                                scratch.col_min[sc] += share;
                            }
                        }
                    }

                    let cur_max_sum: f32 = scratch.col_max[c..end_c].iter().sum::<f32>() + span_gap;
                    let req_max = item.max_w + w_margin;
                    if req_max > cur_max_sum {
                        let deficit = req_max - cur_max_sum;
                        let share = deficit / target_count as f32;
                        for sc in c..end_c {
                            if non_fixed_count == 0 || is_fixed_col(sc).is_none() {
                                scratch.col_max[sc] += share;
                            }
                        }
                    }
                }
            }
        }
    }

    // Multi-span rows: distribute deficit to spanned tracks
    for current_span in 2..=final_rows {
        for i in 0..scratch.child_items.len() {
            let (r, _, r_span, _) = scratch.placements[i];
            if r_span == current_span && r < final_rows {
                let item = scratch.child_items[i];
                let h_margin = item.margin.top + item.margin.bottom;
                let end_r = (r + r_span).min(final_rows);
                let span_len = end_r.saturating_sub(r);
                if span_len > 0 {
                    let span_gap = (span_len - 1) as f32 * row_gap;
                    let non_fixed_count =
                        (r..end_r).filter(|&sr| is_fixed_row(sr).is_none()).count();
                    let target_count = if non_fixed_count > 0 {
                        non_fixed_count
                    } else {
                        span_len
                    };

                    let cur_min_sum: f32 = scratch.row_min[r..end_r].iter().sum::<f32>() + span_gap;
                    let req_min = item.min_h + h_margin;
                    if req_min > cur_min_sum {
                        let deficit = req_min - cur_min_sum;
                        let share = deficit / target_count as f32;
                        for sr in r..end_r {
                            if non_fixed_count == 0 || is_fixed_row(sr).is_none() {
                                scratch.row_min[sr] += share;
                            }
                        }
                    }

                    let cur_max_sum: f32 = scratch.row_max[r..end_r].iter().sum::<f32>() + span_gap;
                    let req_max = item.max_h + h_margin;
                    if req_max > cur_max_sum {
                        let deficit = req_max - cur_max_sum;
                        let share = deficit / target_count as f32;
                        for sr in r..end_r {
                            if non_fixed_count == 0 || is_fixed_row(sr).is_none() {
                                scratch.row_max[sr] += share;
                            }
                        }
                    }
                }
            }
        }
    }

    let total_col_gap = if final_cols > 1 {
        (final_cols - 1) as f32 * col_gap
    } else {
        0.0
    };
    let total_row_gap = if final_rows > 1 {
        (final_rows - 1) as f32 * row_gap
    } else {
        0.0
    };

    let calc_min_w = scratch.col_min.iter().sum::<f32>() + total_col_gap;
    let calc_max_w = scratch.col_max.iter().sum::<f32>() + total_col_gap;
    let calc_min_h = scratch.row_min.iter().sum::<f32>() + total_row_gap;
    let calc_max_h = scratch.row_max.iter().sum::<f32>() + total_row_gap;

    (calc_min_w, calc_max_w, calc_min_h, calc_max_h)
}

#[cfg(test)]
mod tests;
