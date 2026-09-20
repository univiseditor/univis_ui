use crate::internal_prelude::*;
use bevy::prelude::*;

pub(super) fn solve_absolute_box(
    container_size: Vec2,
    spec: &SolverSpec,
    margin: USides,
    intrinsic_size: Vec2,
) -> (Vec2, Vec2) {
    let is_h_stretch = !matches!(spec.left, UVal::Auto) && !matches!(spec.right, UVal::Auto);
    let mut width = if is_h_stretch {
        let l = spec.left.resolve_or_zero(container_size.x);
        let r = spec.right.resolve_or_zero(container_size.x);
        (container_size.x - l - r - margin.left - margin.right).max(0.0)
    } else {
        match spec.width_mode {
            SolverSizeMode::Fixed => spec.width_val,
            SolverSizeMode::Percent => spec.width_val * container_size.x,
            SolverSizeMode::Calc => spec
                .width_uval
                .resolve(container_size.x)
                .unwrap_or(intrinsic_size.x),
            _ => intrinsic_size.x,
        }
    }
    .clamp(spec.min_width, spec.max_width);

    let is_v_stretch = !matches!(spec.top, UVal::Auto) && !matches!(spec.bottom, UVal::Auto);
    let mut height = if is_v_stretch {
        let t = spec.top.resolve_or_zero(container_size.y);
        let b = spec.bottom.resolve_or_zero(container_size.y);
        (container_size.y - t - b - margin.top - margin.bottom).max(0.0)
    } else {
        match spec.height_mode {
            SolverSizeMode::Fixed => spec.height_val,
            SolverSizeMode::Percent => spec.height_val * container_size.y,
            SolverSizeMode::Calc => spec
                .height_uval
                .resolve(container_size.y)
                .unwrap_or(intrinsic_size.y),
            _ => intrinsic_size.y,
        }
    }
    .clamp(spec.min_height, spec.max_height);

    if let Some(ratio) = spec.aspect_ratio {
        let w_is_auto = matches!(spec.width_mode, SolverSizeMode::Auto) && !is_h_stretch;
        let h_is_auto = matches!(spec.height_mode, SolverSizeMode::Auto) && !is_v_stretch;

        if w_is_auto && !h_is_auto {
            width = (height * ratio).clamp(spec.min_width, spec.max_width);
        } else if h_is_auto && !w_is_auto {
            height = (width / ratio).clamp(spec.min_height, spec.max_height);
        }
    }

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
