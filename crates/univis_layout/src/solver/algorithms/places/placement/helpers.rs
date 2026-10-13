use super::*;

pub(super) fn map_legacy_align_items(align: UAlignItems) -> UAlignItemsExt {
    match align {
        UAlignItems::Center => UAlignItemsExt::Center,
        UAlignItems::End | UAlignItems::FlexEnd => UAlignItemsExt::End,
        UAlignItems::Stretch => UAlignItemsExt::Stretch,
        UAlignItems::Baseline => UAlignItemsExt::Baseline,
        _ => UAlignItemsExt::Start,
    }
}

pub(super) fn map_legacy_align_self(align: UAlignSelf) -> UAlignSelfExt {
    match align {
        UAlignSelf::Center => UAlignSelfExt::Center,
        UAlignSelf::End => UAlignSelfExt::End,
        UAlignSelf::Stretch => UAlignSelfExt::Stretch,
        UAlignSelf::Auto => UAlignSelfExt::Auto,
        UAlignSelf::Start => UAlignSelfExt::Start,
    }
}

pub(super) fn map_items_ext_to_self_ext(align: UAlignItemsExt) -> UAlignSelfExt {
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

pub(super) fn canonical_align_self(value: UAlignSelfExt) -> UAlignSelfExt {
    match value {
        UAlignSelfExt::Auto
        | UAlignSelfExt::Normal
        | UAlignSelfExt::Baseline
        | UAlignSelfExt::FirstBaseline
        | UAlignSelfExt::LastBaseline => UAlignSelfExt::Start,
        UAlignSelfExt::FlexStart | UAlignSelfExt::SelfStart | UAlignSelfExt::Left => {
            UAlignSelfExt::Start
        }
        UAlignSelfExt::FlexEnd | UAlignSelfExt::SelfEnd | UAlignSelfExt::Right => {
            UAlignSelfExt::End
        }
        other => other,
    }
}

pub(super) fn alignment_offset(
    align: UAlignSelfExt,
    free_space_raw: f32,
    overflow: UOverflowPosition,
) -> f32 {
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

pub(super) fn resolve_cross_align(
    spec: &SolverSpec,
    container_align: UAlignItems,
) -> UAlignSelfExt {
    if let Some(ext) = spec.align_self_ext
        && !matches!(ext, UAlignSelfExt::Auto | UAlignSelfExt::Normal)
    {
        return canonical_align_self(ext);
    }

    if let Some(legacy) = spec.align_self
        && legacy != UAlignSelf::Auto
    {
        return canonical_align_self(map_legacy_align_self(legacy));
    }

    canonical_align_self(map_items_ext_to_self_ext(map_legacy_align_items(
        container_align,
    )))
}

pub(super) fn resolve_justify_self(spec: &SolverSpec, ctx: &PlacementContext) -> UAlignSelfExt {
    if let Some(ext) = spec.justify_self_ext
        && !matches!(ext, UAlignSelfExt::Auto | UAlignSelfExt::Normal)
    {
        return canonical_align_self(ext);
    }

    if let Some(container_justify_items) = ctx.justify_items {
        return canonical_align_self(map_items_ext_to_self_ext(container_justify_items));
    }

    UAlignSelfExt::Start
}

pub(super) fn has_explicit_align_self(spec: &SolverSpec) -> bool {
    matches!(
        spec.align_self_ext,
        Some(
            UAlignSelfExt::Start
                | UAlignSelfExt::End
                | UAlignSelfExt::Center
                | UAlignSelfExt::Stretch
                | UAlignSelfExt::FlexStart
                | UAlignSelfExt::FlexEnd
                | UAlignSelfExt::SelfStart
                | UAlignSelfExt::SelfEnd
                | UAlignSelfExt::Left
                | UAlignSelfExt::Right
        )
    ) || matches!(
        spec.align_self,
        Some(UAlignSelf::Start | UAlignSelf::End | UAlignSelf::Center | UAlignSelf::Stretch)
    )
}

pub(super) fn has_explicit_justify_self(spec: &SolverSpec) -> bool {
    matches!(
        spec.justify_self_ext,
        Some(
            UAlignSelfExt::Start
                | UAlignSelfExt::End
                | UAlignSelfExt::Center
                | UAlignSelfExt::Stretch
                | UAlignSelfExt::FlexStart
                | UAlignSelfExt::FlexEnd
                | UAlignSelfExt::SelfStart
                | UAlignSelfExt::SelfEnd
                | UAlignSelfExt::Left
                | UAlignSelfExt::Right
        )
    )
}

pub(super) fn allows_implicit_stretch(mode: SolverSizeMode) -> bool {
    matches!(mode, SolverSizeMode::Auto)
}

pub(super) fn allows_explicit_stretch(mode: SolverSizeMode) -> bool {
    !matches!(
        mode,
        SolverSizeMode::Fixed | SolverSizeMode::Percent | SolverSizeMode::Calc
    )
}

pub(super) fn compute_content_floors(
    items: &[SolverItem],
    placements: &[(usize, usize, usize, usize)],
    axis: &AxisHelper,
    main_gap: f32,
    cross_gap: f32,
    total_cols: usize,
    total_rows: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut col_content_floors = vec![0.0f32; total_cols];
    let mut row_content_floors = vec![0.0f32; total_rows];

    // Pass 1: Single-span items establish track baseline floors
    for (idx, item) in items.iter().enumerate() {
        let (row, col, row_span, col_span) = placements[idx];
        let (child_main, child_cross) = axis.from_world(item.result.size);
        let (m_main_start, m_main_end, m_cross_start, m_cross_end) =
            axis.extract_margin_sides(item.margin);

        if col_span == 1 && col < col_content_floors.len() {
            let item_min_main = if axis.is_row() {
                item.spec.min_width
            } else {
                item.spec.min_height
            };
            let req_main = child_main.max(item_min_main) + m_main_start + m_main_end;
            col_content_floors[col] = col_content_floors[col].max(req_main);
        }

        if row_span == 1 && row < row_content_floors.len() {
            let item_min_cross = if axis.is_row() {
                item.spec.min_height
            } else {
                item.spec.min_width
            };
            let req_cross = child_cross.max(item_min_cross) + m_cross_start + m_cross_end;
            row_content_floors[row] = row_content_floors[row].max(req_cross);
        }
    }

    // Pass 2: Multi-span items distribute deficit across spanned columns
    let mut multi_cols: Vec<usize> = (0..items.len()).filter(|&i| placements[i].3 > 1).collect();
    multi_cols.sort_by_key(|&i| placements[i].3);

    for idx in multi_cols {
        let (_, col, _, col_span) = placements[idx];
        let item = &items[idx];
        let (child_main, _) = axis.from_world(item.result.size);
        let (m_main_start, m_main_end, _, _) = axis.extract_margin_sides(item.margin);
        let item_min_main = if axis.is_row() {
            item.spec.min_width
        } else {
            item.spec.min_height
        };
        let req_main = child_main.max(item_min_main) + m_main_start + m_main_end;

        let end_col = (col + col_span).min(col_content_floors.len());
        let span_len = end_col.saturating_sub(col);
        if span_len > 0 {
            let current_span_sum: f32 = col_content_floors[col..end_col].iter().sum::<f32>()
                + (span_len as f32 - 1.0) * main_gap;
            if req_main > current_span_sum {
                let deficit = req_main - current_span_sum;
                let share = deficit / span_len as f32;
                for c in col..end_col {
                    col_content_floors[c] += share;
                }
            }
        }
    }

    // Pass 3: Multi-span items distribute deficit across spanned rows
    let mut multi_rows: Vec<usize> = (0..items.len()).filter(|&i| placements[i].2 > 1).collect();
    multi_rows.sort_by_key(|&i| placements[i].2);

    for idx in multi_rows {
        let (row, _, row_span, _) = placements[idx];
        let item = &items[idx];
        let (_, child_cross) = axis.from_world(item.result.size);
        let (_, _, m_cross_start, m_cross_end) = axis.extract_margin_sides(item.margin);
        let item_min_cross = if axis.is_row() {
            item.spec.min_height
        } else {
            item.spec.min_width
        };
        let req_cross = child_cross.max(item_min_cross) + m_cross_start + m_cross_end;

        let end_row = (row + row_span).min(row_content_floors.len());
        let span_len = end_row.saturating_sub(row);
        if span_len > 0 {
            let current_span_sum: f32 = row_content_floors[row..end_row].iter().sum::<f32>()
                + (span_len as f32 - 1.0) * cross_gap;
            if req_cross > current_span_sum {
                let deficit = req_cross - current_span_sum;
                let share = deficit / span_len as f32;
                for r in row..end_row {
                    row_content_floors[r] += share;
                }
            }
        }
    }

    (col_content_floors, row_content_floors)
}

/// Expands template track specifications into concrete track definitions,
/// expanding `Repeat`, `RepeatFill`, and `RepeatFit` based on available space and gaps.
/// Returns the expanded tracks and a boolean vector indicating which tracks originated from `RepeatFit`.
pub(super) fn expand_grid_template(
    template: &[UTrackSize],
    available_space: f32,
    gap: f32,
) -> (Vec<UTrackSize>, Vec<bool>) {
    let mut expanded = Vec::new();
    let mut is_fit = Vec::new();

    for track in template {
        match *track {
            UTrackSize::Repeat(count, repeat_track) => {
                let concrete: UTrackSize = repeat_track.into();
                for _ in 0..count {
                    expanded.push(concrete);
                    is_fit.push(false);
                }
            }
            UTrackSize::RepeatFill(repeat_track) => {
                let concrete: UTrackSize = repeat_track.into();
                let min_size = match repeat_track {
                    UTrackRepeat::Px(v) => v.max(1.0),
                    UTrackRepeat::Percent(p) => (p * available_space).max(1.0),
                    UTrackRepeat::MinMax { min, .. } => match min {
                        UTrackBound::Px(v) => v.max(1.0),
                        UTrackBound::Percent(p) => (p * available_space).max(1.0),
                        _ => 1.0,
                    },
                    _ => 1.0,
                };
                let count = if available_space > 0.0 {
                    let fit = ((available_space + gap) / (min_size + gap)).floor() as usize;
                    fit.max(1)
                } else {
                    1
                };
                for _ in 0..count {
                    expanded.push(concrete);
                    is_fit.push(false);
                }
            }
            UTrackSize::RepeatFit(repeat_track) => {
                let concrete: UTrackSize = repeat_track.into();
                let min_size = match repeat_track {
                    UTrackRepeat::Px(v) => v.max(1.0),
                    UTrackRepeat::Percent(p) => (p * available_space).max(1.0),
                    UTrackRepeat::MinMax { min, .. } => match min {
                        UTrackBound::Px(v) => v.max(1.0),
                        UTrackBound::Percent(p) => (p * available_space).max(1.0),
                        _ => 1.0,
                    },
                    _ => 1.0,
                };
                let count = if available_space > 0.0 {
                    let fit = ((available_space + gap) / (min_size + gap)).floor() as usize;
                    fit.max(1)
                } else {
                    1
                };
                for _ in 0..count {
                    expanded.push(concrete);
                    is_fit.push(true);
                }
            }
            other => {
                expanded.push(other);
                is_fit.push(false);
            }
        }
    }

    (expanded, is_fit)
}

pub(super) fn determine_grid_max_rows(
    template_rows: &[UTrackSize],
    auto_flow: UGridAutoFlow,
    available_cross: f32,
    cross_gap: f32,
    auto_rows: UTrackSize,
    items: &[SolverItem],
    axis: &AxisHelper,
) -> usize {
    let mut max_rows = template_rows.len();
    if template_rows.is_empty() && auto_flow == UGridAutoFlow::Column && available_cross > 0.0 {
        let auto_row_height = match auto_rows {
            UTrackSize::Px(v) => v.max(1.0),
            UTrackSize::MinMax {
                min: UTrackBound::Px(v),
                ..
            } => v.max(1.0),
            _ => {
                let mut max_item_cross = 0.0f32;
                for item in items.iter() {
                    let (_, child_cross) = axis.from_world(item.result.size);
                    max_item_cross = max_item_cross.max(child_cross);
                }
                max_item_cross.max(1.0)
            }
        };
        let fit_rows =
            ((available_cross + cross_gap) / (auto_row_height + cross_gap)).floor() as usize;
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
    max_rows
}

struct InternalTrackSpec {
    floor: f32,
    max: Option<f32>,
    fr: f32,
    is_auto: bool,
}

pub(super) fn resolve_track_sizes(
    template: &[UTrackSize],
    fallback_count: usize,
    auto_track: UTrackSize,
    available_space: f32,
    gap: f32,
    required_min: usize,
    content_floors: &[f32],
) -> Vec<f32> {
    let base_count = if template.is_empty() {
        fallback_count.max(1)
    } else {
        template.len()
    };
    let mut track_defs = if template.is_empty() {
        vec![auto_track; base_count]
    } else {
        template.to_vec()
    };

    while track_defs.len() < required_min.max(1) {
        track_defs.push(auto_track);
    }

    let count = track_defs.len();
    let total_gap = if count > 1 {
        (count as f32 - 1.0) * gap
    } else {
        0.0
    };
    let distributable = (available_space - total_gap).max(0.0);

    let mut specs = Vec::with_capacity(count);

    for (i, track) in track_defs.iter().enumerate() {
        let content_floor = content_floors.get(i).copied().unwrap_or(0.0).max(0.0);
        let spec = match *track {
            UTrackSize::Px(v) => InternalTrackSpec {
                floor: v.max(0.0),
                max: Some(v.max(0.0)),
                fr: 0.0,
                is_auto: false,
            },
            UTrackSize::Percent(p) => {
                let px = (p * available_space).max(0.0);
                InternalTrackSpec {
                    floor: px,
                    max: Some(px),
                    fr: 0.0,
                    is_auto: false,
                }
            }
            UTrackSize::Fr(v) => InternalTrackSpec {
                floor: content_floor,
                max: None,
                fr: v.max(0.0),
                is_auto: false,
            },
            UTrackSize::Auto => InternalTrackSpec {
                floor: content_floor,
                max: None,
                fr: 0.0,
                is_auto: false,
            },
            UTrackSize::MinMax { min, max } => {
                let min_px = match min {
                    UTrackBound::Px(v) => v.max(0.0),
                    UTrackBound::Percent(p) => (p * available_space).max(0.0),
                    UTrackBound::Auto => content_floor,
                    UTrackBound::Fr(_) => 0.0,
                };
                match max {
                    UTrackBound::Fr(v) => InternalTrackSpec {
                        floor: min_px,
                        max: None,
                        fr: v.max(0.0),
                        is_auto: false,
                    },
                    UTrackBound::Px(v) => {
                        let mx = v.max(min_px);
                        InternalTrackSpec {
                            floor: min_px,
                            max: Some(mx),
                            fr: 0.0,
                            is_auto: mx > min_px,
                        }
                    }
                    UTrackBound::Percent(p) => {
                        let mx = (p * available_space).max(min_px);
                        InternalTrackSpec {
                            floor: min_px,
                            max: Some(mx),
                            fr: 0.0,
                            is_auto: mx > min_px,
                        }
                    }
                    UTrackBound::Auto => InternalTrackSpec {
                        floor: min_px,
                        max: None,
                        fr: 0.0,
                        is_auto: false,
                    },
                }
            }
            UTrackSize::Repeat(..) | UTrackSize::RepeatFill(..) | UTrackSize::RepeatFit(..) => {
                InternalTrackSpec {
                    floor: content_floor,
                    max: None,
                    fr: 0.0,
                    is_auto: true,
                }
            }
        };
        specs.push(spec);
    }

    let total_floor: f32 = specs.iter().map(|s| s.floor).sum();
    let fr_sum: f32 = specs.iter().map(|s| s.fr).sum();
    let auto_count = specs.iter().filter(|s| s.is_auto).count();

    if fr_sum > 0.0 {
        let non_fr_floor: f32 = specs.iter().filter(|s| s.fr == 0.0).map(|s| s.floor).sum();
        let remaining = (distributable - non_fr_floor).max(0.0);

        specs
            .iter()
            .map(|s| {
                if s.fr > 0.0 {
                    let share = remaining * (s.fr / fr_sum);
                    share.max(s.floor)
                } else {
                    s.floor
                }
            })
            .collect()
    } else if auto_count > 0 {
        let remaining = (distributable - total_floor).max(0.0);
        let extra_per_auto = remaining / auto_count as f32;

        specs
            .iter()
            .map(|s| {
                if s.is_auto {
                    let grown = s.floor + extra_per_auto;
                    match s.max {
                        Some(mx) => grown.min(mx),
                        None => grown,
                    }
                } else {
                    s.floor
                }
            })
            .collect()
    } else {
        specs.iter().map(|s| s.floor).collect()
    }
}

pub(super) fn ensure_grid_rows(occupancy: &mut Vec<Vec<bool>>, rows: usize, cols: usize) {
    while occupancy.len() < rows {
        occupancy.push(vec![false; cols]);
    }
}

pub(super) fn can_place_span(
    occupancy: &[Vec<bool>],
    row: usize,
    col: usize,
    row_span: usize,
    col_span: usize,
    cols: usize,
) -> bool {
    if col + col_span > cols {
        return false;
    }

    for row_cells in occupancy.iter().skip(row).take(row_span) {
        if row_cells
            .iter()
            .skip(col)
            .take(col_span)
            .any(|occupied| *occupied)
        {
            return false;
        }
    }

    true
}

pub(super) fn mark_span(
    occupancy: &mut [Vec<bool>],
    row: usize,
    col: usize,
    row_span: usize,
    col_span: usize,
) {
    for row_cells in occupancy.iter_mut().skip(row).take(row_span) {
        for cell in row_cells.iter_mut().skip(col).take(col_span) {
            *cell = true;
        }
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
