use bevy::prelude::*;
use crate::layout::univis_node::{UPositionType, USelf, UVal};
use crate::layout::types::{UAlignSelf, UAlignSelfExt};
use crate::layout::core::solver::translate::translate_spec;
use crate::layout::univis_node::UNode;

#[test]
fn anchor_corners_configuration() {
    let tl = USelf::anchor_top_left(Vec2::new(12.0, 16.0));
    assert_eq!(tl.position_type, UPositionType::Absolute);
    assert_eq!(tl.left, UVal::Px(12.0));
    assert_eq!(tl.top, UVal::Px(16.0));
    assert_eq!(tl.right, UVal::Auto);
    assert_eq!(tl.bottom, UVal::Auto);

    let tr = USelf::anchor_top_right(Vec2::new(20.0, 24.0));
    assert_eq!(tr.position_type, UPositionType::Absolute);
    assert_eq!(tr.right, UVal::Px(20.0));
    assert_eq!(tr.top, UVal::Px(24.0));
    assert_eq!(tr.left, UVal::Auto);
    assert_eq!(tr.bottom, UVal::Auto);

    let bl = USelf::anchor_bottom_left(Vec2::new(10.0, 30.0));
    assert_eq!(bl.position_type, UPositionType::Absolute);
    assert_eq!(bl.left, UVal::Px(10.0));
    assert_eq!(bl.bottom, UVal::Px(30.0));

    let br = USelf::anchor_bottom_right(Vec2::new(14.0, 22.0));
    assert_eq!(br.position_type, UPositionType::Absolute);
    assert_eq!(br.right, UVal::Px(14.0));
    assert_eq!(br.bottom, UVal::Px(22.0));
}

#[test]
fn anchor_centers_configuration() {
    let tc = USelf::anchor_top_center(15.0);
    assert_eq!(tc.position_type, UPositionType::Absolute);
    assert_eq!(tc.top, UVal::Px(15.0));
    assert_eq!(tc.justify_self, Some(UAlignSelfExt::Center));

    let bc = USelf::anchor_bottom_center(25.0);
    assert_eq!(bc.position_type, UPositionType::Absolute);
    assert_eq!(bc.bottom, UVal::Px(25.0));
    assert_eq!(bc.justify_self, Some(UAlignSelfExt::Center));

    let cl = USelf::anchor_center_left(18.0);
    assert_eq!(cl.position_type, UPositionType::Absolute);
    assert_eq!(cl.left, UVal::Px(18.0));
    assert_eq!(cl.align_self, UAlignSelf::Center);

    let cr = USelf::anchor_center_right(18.0);
    assert_eq!(cr.position_type, UPositionType::Absolute);
    assert_eq!(cr.right, UVal::Px(18.0));
    assert_eq!(cr.align_self, UAlignSelf::Center);

    let c = USelf::center();
    assert_eq!(c.position_type, UPositionType::Absolute);
    assert_eq!(c.align_self, UAlignSelf::Center);
    assert_eq!(c.justify_self, Some(UAlignSelfExt::Center));

    assert_eq!(USelf::anchor_center(), USelf::center());
}

#[test]
fn anchor_full_stretch() {
    let full = USelf::stretch_full();
    assert_eq!(full.position_type, UPositionType::Absolute);
    assert_eq!(full.left, UVal::Px(0.0));
    assert_eq!(full.right, UVal::Px(0.0));
    assert_eq!(full.top, UVal::Px(0.0));
    assert_eq!(full.bottom, UVal::Px(0.0));
}

#[test]
fn anchor_enum_dispatch() {
    let offset = Vec2::new(10.0, 20.0);

    assert_eq!(USelf::anchor(UAnchor::TopLeft, offset), USelf::anchor_top_left(offset));
    assert_eq!(USelf::anchor(UAnchor::TopCenter, offset), USelf::anchor_top_center(offset.y));
    assert_eq!(USelf::anchor(UAnchor::TopRight, offset), USelf::anchor_top_right(offset));
    assert_eq!(USelf::anchor(UAnchor::CenterLeft, offset), USelf::anchor_center_left(offset.x));
    assert_eq!(USelf::anchor(UAnchor::Center, offset), USelf::center());
    assert_eq!(USelf::anchor(UAnchor::CenterRight, offset), USelf::anchor_center_right(offset.x));
    assert_eq!(USelf::anchor(UAnchor::BottomLeft, offset), USelf::anchor_bottom_left(offset));
    assert_eq!(USelf::anchor(UAnchor::BottomCenter, offset), USelf::anchor_bottom_center(offset.y));
    assert_eq!(USelf::anchor(UAnchor::BottomRight, offset), USelf::anchor_bottom_right(offset));
}

#[test]
fn anchor_translates_to_solver_spec() {
    let node = UNode::fixed(200.0, 100.0);
    let uself = USelf::anchor_bottom_center(32.0);

    let spec = translate_spec(&node, Some(&uself));
    assert_eq!(spec.position_type, UPositionType::Absolute);
    assert_eq!(spec.bottom, UVal::Px(32.0));
    assert_eq!(spec.justify_self_ext, Some(UAlignSelfExt::Center));
}
