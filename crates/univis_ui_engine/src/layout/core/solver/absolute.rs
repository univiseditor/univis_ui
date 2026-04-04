use crate::internal_prelude::*;
use bevy::prelude::*;

pub(super) fn solve_absolute_box(
    container_size: Vec2,
    spec: &SolverSpec,
    margin: USides,
    intrinsic_size: Vec2,
) -> (Vec2, Vec2) {
    let is_h_stretch = !matches!(spec.left, UVal::Auto) && !matches!(spec.right, UVal::Auto);
    let width = if is_h_stretch {
        let l = spec.left.resolve_or_zero(container_size.x);
        let r = spec.right.resolve_or_zero(container_size.x);
        (container_size.x - l - r - margin.left - margin.right).max(0.0)
    } else {
        match spec.width_mode {
            SolverSizeMode::Fixed => spec.width_val,
            SolverSizeMode::Percent => spec.width_val * container_size.x,
            _ => intrinsic_size.x,
        }
    }
    .clamp(spec.min_width, spec.max_width);

    let is_v_stretch = !matches!(spec.top, UVal::Auto) && !matches!(spec.bottom, UVal::Auto);
    let height = if is_v_stretch {
        let t = spec.top.resolve_or_zero(container_size.y);
        let b = spec.bottom.resolve_or_zero(container_size.y);
        (container_size.y - t - b - margin.top - margin.bottom).max(0.0)
    } else {
        match spec.height_mode {
            SolverSizeMode::Fixed => spec.height_val,
            SolverSizeMode::Percent => spec.height_val * container_size.y,
            _ => intrinsic_size.y,
        }
    }
    .clamp(spec.min_height, spec.max_height);

    let x = if let Some(l) = spec.left.resolve(container_size.x) {
        l + margin.left
    } else if let Some(r) = spec.right.resolve(container_size.x) {
        container_size.x - r - width - margin.right
    } else {
        margin.left
    };

    let y = if let Some(t) = spec.top.resolve(container_size.y) {
        t + margin.top
    } else if let Some(b) = spec.bottom.resolve(container_size.y) {
        container_size.y - b - height - margin.bottom
    } else {
        margin.top
    };

    (Vec2::new(width, height), Vec2::new(x, y))
}
