use super::*;

use crate::layout::geometry::UVal;
use crate::layout::univis_node::{
    UAlignSelf, UAlignSelfExt, ULayoutBoxAlignSelf, ULayoutFlexItem, ULayoutGridItem,
    ULayoutItemExt, UNode, UOverflowPosition, USelf,
};
use univis_layout::solver::SolverSizeMode;

#[test]
fn translate_spec_copies_ext_fields() {
    let node = UNode {
        width: UVal::Px(40.0),
        height: UVal::Px(20.0),
        min_width: 12.0,
        max_width: 96.0,
        min_height: 8.0,
        max_height: 72.0,
        ..Default::default()
    };
    let uself = USelf {
        align_self: UAlignSelf::Center,
        item_ext: ULayoutItemExt {
            box_align: ULayoutBoxAlignSelf {
                justify_self: Some(UAlignSelfExt::End),
                align_self: Some(UAlignSelfExt::Start),
                justify_overflow: UOverflowPosition::Safe,
                align_overflow: UOverflowPosition::Unsafe,
            },
            flex: ULayoutFlexItem {
                flex_grow: Some(2.0),
                flex_shrink: Some(0.5),
                flex_basis: Some(UVal::Px(30.0)),
            },
            grid: ULayoutGridItem {
                column_start: Some(2),
                column_span: 3,
                row_start: Some(1),
                row_span: 2,
            },
        },
        ..Default::default()
    };

    let spec = translate_spec(&node, Some(&uself));

    assert_eq!(spec.align_self, Some(UAlignSelf::Center));
    assert_eq!(spec.align_self_ext, Some(UAlignSelfExt::Start));
    assert_eq!(spec.justify_self_ext, Some(UAlignSelfExt::End));
    assert_eq!(spec.justify_overflow, UOverflowPosition::Safe);
    assert_eq!(spec.flex_grow, Some(2.0));
    assert_eq!(spec.flex_basis, Some(UVal::Px(30.0)));
    assert_eq!(spec.min_width, 12.0);
    assert_eq!(spec.max_width, 96.0);
    assert_eq!(spec.min_height, 8.0);
    assert_eq!(spec.max_height, 72.0);
    assert_eq!(spec.grid_column_start, Some(2));
    assert_eq!(spec.grid_column_span, 3);
    assert_eq!(spec.grid_row_span, 2);
}

#[test]
fn translate_spec_maps_intrinsic_modes() {
    let node = UNode {
        width: UVal::MinContent,
        height: UVal::Auto,
        ..Default::default()
    };

    let spec = translate_spec(&node, None);

    assert_eq!(spec.width_mode, SolverSizeMode::MinContent);
    assert_eq!(spec.height_mode, SolverSizeMode::Auto);
}

#[test]
fn explicit_max_content_stays_on_content_mode() {
    let node = UNode {
        width: UVal::MaxContent,
        height: UVal::Content,
        ..Default::default()
    };

    let spec = translate_spec(&node, None);

    assert_eq!(spec.width_mode, SolverSizeMode::Content);
    assert_eq!(spec.height_mode, SolverSizeMode::Content);
}
