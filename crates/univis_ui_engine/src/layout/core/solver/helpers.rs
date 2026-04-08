use crate::internal_prelude::*;

pub(super) fn map_align_items_to_ext(align: UAlignItems) -> UAlignItemsExt {
    match align {
        UAlignItems::Center => UAlignItemsExt::Center,
        UAlignItems::End | UAlignItems::FlexEnd => UAlignItemsExt::End,
        UAlignItems::Stretch => UAlignItemsExt::Stretch,
        UAlignItems::Baseline => UAlignItemsExt::Baseline,
        _ => UAlignItemsExt::Start,
    }
}

pub(super) fn map_align_self_to_ext(align: UAlignSelf) -> UAlignSelfExt {
    match align {
        UAlignSelf::Center => UAlignSelfExt::Center,
        UAlignSelf::End => UAlignSelfExt::End,
        UAlignSelf::Stretch => UAlignSelfExt::Stretch,
        UAlignSelf::Auto => UAlignSelfExt::Auto,
        UAlignSelf::Start => UAlignSelfExt::Start,
    }
}

pub(super) fn resolve_flex_basis(
    basis: UVal,
    default_content: f32,
    available_main: f32,
) -> Option<f32> {
    match basis {
        UVal::Px(v) => Some(v.max(0.0)),
        UVal::Percent(p) => Some((p * available_main).max(0.0)),
        UVal::Content | UVal::MinContent | UVal::MaxContent => Some(default_content.max(0.0)),
        UVal::Auto | UVal::Flex(_) => None,
    }
}

pub(super) fn main_bounds_for_spec(spec: &SolverSpec, axis: &AxisHelper) -> (f32, f32) {
    if axis.is_row() {
        (spec.min_width, spec.max_width)
    } else {
        (spec.min_height, spec.max_height)
    }
}

pub(super) fn cross_bounds_for_spec(spec: &SolverSpec, axis: &AxisHelper) -> (f32, f32) {
    if axis.is_row() {
        (spec.min_height, spec.max_height)
    } else {
        (spec.min_width, spec.max_width)
    }
}

pub(super) fn clamp_main_size(spec: &SolverSpec, axis: &AxisHelper, size: f32) -> f32 {
    let (min, max) = main_bounds_for_spec(spec, axis);
    size.clamp(min, max)
}

pub(super) fn clamp_cross_size(spec: &SolverSpec, axis: &AxisHelper, size: f32) -> f32 {
    let (min, max) = cross_bounds_for_spec(spec, axis);
    size.clamp(min, max)
}

pub(super) fn flex_grow_factor(spec: &SolverSpec, legacy_main_flex_factor: f32) -> f32 {
    let ext_grow = spec.flex_grow.unwrap_or(0.0).max(0.0);
    if ext_grow > 0.0 {
        ext_grow
    } else {
        legacy_main_flex_factor.max(0.0)
    }
}

pub(super) fn resolved_flex_grow_factor(spec: &SolverSpec, axis: &AxisHelper) -> f32 {
    let (_, _, legacy_main_flex_factor) = axis.get_main_spec(spec);
    flex_grow_factor(spec, legacy_main_flex_factor)
}

pub(super) fn flex_shrink_factor(spec: &SolverSpec) -> f32 {
    spec.flex_shrink.unwrap_or(1.0).max(0.0)
}

pub(super) fn allows_implicit_stretch(mode: SolverSizeMode) -> bool {
    matches!(mode, SolverSizeMode::Auto)
}

pub(super) fn allows_explicit_stretch(mode: SolverSizeMode) -> bool {
    !matches!(mode, SolverSizeMode::Fixed | SolverSizeMode::Percent)
}

pub(super) fn has_explicit_align_self_override(spec: &SolverSpec) -> bool {
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
    ) || spec.align_self.is_some()
}

pub(super) fn is_ext_stretch(value: UAlignSelfExt) -> bool {
    matches!(value, UAlignSelfExt::Stretch)
}
