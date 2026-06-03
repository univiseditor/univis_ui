use crate::internal_prelude::*;

pub fn translate_spec(node: &UNode, uself: Option<&USelf>) -> SolverSpec {
    let map_dim = |dim: UVal| -> (SolverSizeMode, f32, f32) {
        match dim {
            UVal::Px(v) => (SolverSizeMode::Fixed, v, 0.0),
            UVal::Percent(p) => (SolverSizeMode::Percent, p, 0.0),
            UVal::Flex(f) => (SolverSizeMode::Flex, 0.0, f),
            UVal::MinContent => (SolverSizeMode::MinContent, 0.0, 0.0),
            UVal::Content | UVal::MaxContent => (SolverSizeMode::Content, 0.0, 0.0),
            UVal::Auto => (SolverSizeMode::Auto, 0.0, 0.0),
        }
    };

    let (w_mode, w_val, w_flex) = map_dim(node.width);
    let (h_mode, h_val, h_flex) = map_dim(node.height);
    let (min_width, max_width) = node.width_bounds();
    let (min_height, max_height) = node.height_bounds();

    let (pos_type, l, r, t, b, align, order) = if let Some(u) = uself {
        (
            u.position_type,
            u.left,
            u.right,
            u.top,
            u.bottom,
            if u.align_self == UAlignSelf::Auto {
                None
            } else {
                Some(u.align_self)
            },
            #[allow(deprecated)]
            u.order,
        )
    } else {
        (
            UPositionType::Relative,
            UVal::Auto,
            UVal::Auto,
            UVal::Auto,
            UVal::Auto,
            None,
            0,
        )
    };
    let (align_self_ext, justify_self_ext, justify_overflow, align_overflow) =
        if let Some(u) = uself {
            (
                u.item_ext.box_align.align_self,
                u.item_ext.box_align.justify_self,
                u.item_ext.box_align.justify_overflow,
                u.item_ext.box_align.align_overflow,
            )
        } else {
            (
                None,
                None,
                UOverflowPosition::Unsafe,
                UOverflowPosition::Unsafe,
            )
        };
    let (flex_grow, flex_shrink, flex_basis) = if let Some(u) = uself {
        (
            u.item_ext.flex.flex_grow.map(|v| v.max(0.0)),
            u.item_ext.flex.flex_shrink.map(|v| v.max(0.0)),
            u.item_ext.flex.flex_basis,
        )
    } else {
        (None, None, None)
    };
    let (grid_column_start, grid_column_span, grid_row_start, grid_row_span) =
        if let Some(u) = uself {
            (
                u.item_ext.grid.column_start,
                u.item_ext.grid.column_span.max(1),
                u.item_ext.grid.row_start,
                u.item_ext.grid.row_span.max(1),
            )
        } else {
            (None, 1, None, 1)
        };

    SolverSpec {
        width_mode: w_mode,
        width_val: w_val,
        width_flex: w_flex,
        min_width,
        max_width,
        height_mode: h_mode,
        height_val: h_val,
        height_flex: h_flex,
        min_height,
        max_height,

        position_type: pos_type,
        left: l,
        right: r,
        top: t,
        bottom: b,
        align_self: align,
        align_self_ext,
        justify_self_ext,
        justify_overflow,
        align_overflow,
        flex_grow,
        flex_shrink,
        flex_basis,
        grid_column_start,
        grid_column_span,
        grid_row_start,
        grid_row_span,
        order,
    }
}
