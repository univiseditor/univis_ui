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
    !matches!(mode, SolverSizeMode::Fixed | SolverSizeMode::Percent)
}

pub(super) fn resolve_track_sizes(
    template: &[UTrackSize],
    fallback_count: usize,
    auto_track: UTrackSize,
    available_space: f32,
    gap: f32,
    required_min: usize,
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
