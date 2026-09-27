use super::*;

use crate::layout::geometry::{BoxConstraints, USides, UVal, UValLength};
use crate::layout::solver_types::SolverSpec;
use crate::layout::univis_node::{
    UAlignItems, UAlignSelf, UAlignSelfExt, UFlexDirection, UFlexWrap, UGridAutoFlow, ULayout,
    ULayoutBoxAlignSelf, ULayoutFlexItem, ULayoutGridItem, ULayoutItemExt, UNode,
    UOverflowPosition, USelf, UTrackSize,
};

fn base_solver_config() -> SolverConfig {
    SolverConfig {
        layout: ULayout::default(),
        gap: 0.0,
        row_gap: None,
        column_gap: None,
        padding: USides::default(),
        grid_columns: 1,
        justify_items: None,
        align_content: None,
        flex_wrap: UFlexWrap::NoWrap,
        flex_align_content: None,
        grid_template_columns: Vec::new(),
        grid_template_rows: Vec::new(),
        grid_auto_flow: UGridAutoFlow::Row,
        grid_auto_rows: UTrackSize::Auto,
        grid_auto_columns: UTrackSize::Auto,
        width_mode: SolverSizeMode::Fixed,
        height_mode: SolverSizeMode::Fixed,
    }
}

#[test]
fn resolve_flex_basis_handles_percent() {
    let value = resolve_flex_basis(UVal::Percent(0.5), 10.0, 300.0);
    assert_eq!(value, Some(150.0));
}

#[test]
fn translate_spec_copies_ext_fields() {
    let node = UNode {
        width: UVal::Px(40.0),
        height: UVal::Px(20.0),
        min_width: 12.0,
        max_width: 96.0,
        min_height: 8.0,
        max_height: 72.0,
        ..default()
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
        ..default()
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
        ..default()
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
        ..default()
    };

    let spec = translate_spec(&node, None);

    assert_eq!(spec.width_mode, SolverSizeMode::Content);
    assert_eq!(spec.height_mode, SolverSizeMode::Content);
}

#[test]
fn auto_cross_size_stretches_but_content_cross_size_keeps_intrinsic_value() {
    let mut config = base_solver_config();
    config.layout.align_items = UAlignItems::Stretch;
    let constraints = BoxConstraints::tight(Vec2::new(160.0, 100.0));

    let mut auto_result = SolverResult::default();
    let mut content_result = SolverResult::default();
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 40.0,
                height_mode: SolverSizeMode::Auto,
                height_val: 18.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut auto_result,
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 40.0,
                height_mode: SolverSizeMode::Content,
                height_val: 18.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut content_result,
            USides::default(),
        ),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(auto_result.size.y, 100.0);
    assert_eq!(content_result.size.y, 18.0);
}

#[test]
fn flex_shrink_respects_min_width_and_redistributes_remaining_overflow() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(100.0, 40.0));
    let mut first = SolverResult::default();
    let mut second = SolverResult::default();
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 80.0,
                min_width: 60.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 20.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut first,
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 80.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 20.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut second,
            USides::default(),
        ),
    ];

    let solved = solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(solved, Vec2::new(100.0, 40.0));
    assert_eq!(first.size.x, 60.0);
    assert_eq!(second.size.x, 40.0);
}

#[test]
fn flex_grow_respects_max_width_and_redistributes_remaining_space() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(200.0, 40.0));
    let mut first = SolverResult::default();
    let mut second = SolverResult::default();
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 50.0,
                max_width: 70.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 20.0,
                flex_grow: Some(1.0),
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut first,
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 50.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 20.0,
                flex_grow: Some(1.0),
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut second,
            USides::default(),
        ),
    ];

    let solved = solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(solved, Vec2::new(200.0, 40.0));
    assert_eq!(first.size.x, 70.0);
    assert_eq!(second.size.x, 130.0);
}

#[test]
fn wrapped_row_moves_item_to_next_line_when_min_width_blocks_further_shrink() {
    let mut config = base_solver_config();
    config.flex_wrap = UFlexWrap::Wrap;
    let constraints = BoxConstraints::tight(Vec2::new(150.0, 120.0));
    let mut first = SolverResult::default();
    let mut second = SolverResult::default();
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                min_width: 90.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 24.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut first,
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                min_width: 90.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 24.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut second,
            USides::default(),
        ),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(first.size.x, 90.0);
    assert_eq!(second.size.x, 90.0);
    assert!(second.pos.y >= first.size.y - 0.1);
}

#[test]
fn wrapped_column_moves_item_to_next_column_when_min_height_blocks_further_shrink() {
    let mut config = base_solver_config();
    config.layout.flex_direction = UFlexDirection::Column;
    config.flex_wrap = UFlexWrap::Wrap;
    let constraints = BoxConstraints::tight(Vec2::new(120.0, 150.0));
    let mut first = SolverResult::default();
    let mut second = SolverResult::default();
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 24.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 100.0,
                min_height: 90.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut first,
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 24.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 100.0,
                min_height: 90.0,
                flex_shrink: Some(1.0),
                ..default()
            },
            &mut second,
            USides::default(),
        ),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(first.size.y, 90.0);
    assert_eq!(second.size.y, 90.0);
    assert!(second.pos.x >= first.size.x - 0.1);
}

#[test]
fn aspect_ratio_derives_height_from_fixed_width_in_row() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 160.0,
            height_mode: SolverSizeMode::Auto,
            aspect_ratio: Some(16.0 / 9.0),
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((result.size.x - 160.0).abs() < 0.001);
    assert!((result.size.y - 90.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_derives_width_from_fixed_height_in_row() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Auto,
            height_mode: SolverSizeMode::Fixed,
            height_val: 100.0,
            aspect_ratio: Some(2.0),
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((result.size.x - 200.0).abs() < 0.001);
    assert!((result.size.y - 100.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_inhibits_implicit_cross_stretch() {
    let mut config = base_solver_config();
    config.layout.align_items = UAlignItems::Stretch;
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 120.0,
            height_mode: SolverSizeMode::Auto,
            aspect_ratio: Some(1.0), // Square
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    // Height must be 120.0 (1:1), NOT stretched to container height 300.0!
    assert!((result.size.x - 120.0).abs() < 0.001);
    assert!((result.size.y - 120.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_absolute_box_resolves_auto_dimension() {
    let spec = SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 150.0,
        height_mode: SolverSizeMode::Auto,
        aspect_ratio: Some(1.5),
        position_type: UPositionType::Absolute,
        left: UVal::Px(10.0),
        top: UVal::Px(10.0),
        ..default()
    };

    let (size, pos) = solve_absolute_box(
        Vec2::new(500.0, 500.0),
        &spec,
        USides::default(),
        Vec2::ZERO,
    );

    assert!((size.x - 150.0).abs() < 0.001);
    assert!((size.y - 100.0).abs() < 0.001);
    assert_eq!(pos, Vec2::new(10.0, 10.0));
}

#[test]
fn flex_layout_resolves_calc_dimension() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(500.0, 200.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Calc,
            width_uval: UVal::calc(1.0, -40.0),
            height_mode: SolverSizeMode::Fixed,
            height_val: 50.0,
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((result.size.x - 460.0).abs() < 0.001);
    assert!((result.size.y - 50.0).abs() < 0.001);
}

#[test]
fn flex_layout_resolves_clamp_dimension_in_cross_axis() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(500.0, 200.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            height_mode: SolverSizeMode::Calc,
            height_uval: UVal::clamp(
                UValLength::px(50.0),
                UValLength::percent(0.8),
                UValLength::px(120.0),
            ),
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    // 80% of 200 is 160, clamped to max 120
    assert!((result.size.x - 100.0).abs() < 0.001);
    assert!((result.size.y - 120.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_derives_cross_from_calc_main() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut result = SolverResult::default();
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Calc,
            width_uval: UVal::calc(0.5, -10.0),
            height_mode: SolverSizeMode::Auto,
            aspect_ratio: Some(2.0),
            ..default()
        },
        &mut result,
        USides::default(),
    )];

    solve_flex_layout(&config, constraints, &mut items);

    // width = 400 * 0.5 - 10 = 190. height = 190 / 2.0 = 95
    assert!((result.size.x - 190.0).abs() < 0.001);
    assert!((result.size.y - 95.0).abs() < 0.001);
}

#[test]
fn grid_solver_places_2x2_fixed_cells_with_gaps() {
    let mut config = base_solver_config();
    config.layout.display = UDisplay::Grid;
    config.grid_template_columns = vec![UTrackSize::Px(100.0), UTrackSize::Px(100.0)];
    config.grid_template_rows = vec![UTrackSize::Px(50.0), UTrackSize::Px(50.0)];
    config.column_gap = Some(10.0);
    config.row_gap = Some(15.0);

    let constraints = BoxConstraints::tight(Vec2::new(300.0, 200.0));
    let mut res = [
        SolverResult::default(),
        SolverResult::default(),
        SolverResult::default(),
        SolverResult::default(),
    ];
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 50.0,
                ..default()
            },
            &mut res[0],
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 50.0,
                ..default()
            },
            &mut res[1],
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 50.0,
                ..default()
            },
            &mut res[2],
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 100.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 50.0,
                ..default()
            },
            &mut res[3],
            USides::default(),
        ),
    ];

    solve_grid_layout(&config, constraints, &mut items);

    // Item 0: Row 0, Col 0 -> (0, 0)
    assert!((res[0].pos.x - 0.0).abs() < 0.001);
    assert!((res[0].pos.y - 0.0).abs() < 0.001);
    assert!((res[0].size.x - 100.0).abs() < 0.001);
    assert!((res[0].size.y - 50.0).abs() < 0.001);

    // Item 1: Row 0, Col 1 -> (110, 0)
    assert!((res[1].pos.x - 110.0).abs() < 0.001);
    assert!((res[1].pos.y - 0.0).abs() < 0.001);

    // Item 2: Row 1, Col 0 -> (0, 65)
    assert!((res[2].pos.x - 0.0).abs() < 0.001);
    assert!((res[2].pos.y - 65.0).abs() < 0.001);

    // Item 3: Row 1, Col 1 -> (110, 65)
    assert!((res[3].pos.x - 110.0).abs() < 0.001);
    assert!((res[3].pos.y - 65.0).abs() < 0.001);
}

#[test]
fn grid_solver_distributes_fr_tracks_proportionally() {
    let mut config = base_solver_config();
    config.layout.display = UDisplay::Grid;
    config.grid_template_columns = vec![UTrackSize::Fr(1.0), UTrackSize::Fr(2.0)];
    config.grid_template_rows = vec![UTrackSize::Px(80.0)];

    let constraints = BoxConstraints::tight(Vec2::new(300.0, 100.0));
    let mut res = [SolverResult::default(), SolverResult::default()];
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Auto,
                height_mode: SolverSizeMode::Fixed,
                height_val: 80.0,
                ..default()
            },
            &mut res[0],
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Auto,
                height_mode: SolverSizeMode::Fixed,
                height_val: 80.0,
                ..default()
            },
            &mut res[1],
            USides::default(),
        ),
    ];

    solve_grid_layout(&config, constraints, &mut items);

    // 1fr = 100.0, 2fr = 200.0
    assert!((res[0].size.x - 100.0).abs() < 0.001);
    assert!((res[0].pos.x - 0.0).abs() < 0.001);

    assert!((res[1].size.x - 200.0).abs() < 0.001);
    assert!((res[1].pos.x - 100.0).abs() < 0.001);
}

#[test]
fn grid_solver_handles_column_spanning() {
    let mut config = base_solver_config();
    config.layout.display = UDisplay::Grid;
    config.grid_template_columns = vec![
        UTrackSize::Px(100.0),
        UTrackSize::Px(150.0),
        UTrackSize::Px(100.0),
    ];
    config.grid_template_rows = vec![UTrackSize::Px(60.0)];
    config.column_gap = Some(10.0);

    let constraints = BoxConstraints::tight(Vec2::new(400.0, 100.0));
    let mut res = [SolverResult::default()];
    let mut items = vec![SolverItem::new(
        SolverSpec {
            width_mode: SolverSizeMode::Auto,
            height_mode: SolverSizeMode::Fixed,
            height_val: 60.0,
            grid_column_span: 2,
            ..default()
        },
        &mut res[0],
        USides::default(),
    )];

    solve_grid_layout(&config, constraints, &mut items);

    // Col 0 (100) + gap (10) + Col 1 (150) = 260.0
    assert!((res[0].size.x - 260.0).abs() < 0.001);
    assert!((res[0].pos.x - 0.0).abs() < 0.001);
}

#[test]
fn grid_solver_column_auto_flow() {
    let mut config = base_solver_config();
    config.layout.display = UDisplay::Grid;
    config.grid_auto_flow = UGridAutoFlow::Column;
    config.grid_template_columns = vec![UTrackSize::Px(80.0), UTrackSize::Px(80.0)];
    config.grid_template_rows = vec![UTrackSize::Px(40.0), UTrackSize::Px(40.0)];
    config.row_gap = Some(10.0);
    config.column_gap = Some(10.0);

    let constraints = BoxConstraints::tight(Vec2::new(200.0, 200.0));
    let mut res = [SolverResult::default(), SolverResult::default()];
    let mut items = vec![
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 80.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 40.0,
                ..default()
            },
            &mut res[0],
            USides::default(),
        ),
        SolverItem::new(
            SolverSpec {
                width_mode: SolverSizeMode::Fixed,
                width_val: 80.0,
                height_mode: SolverSizeMode::Fixed,
                height_val: 40.0,
                ..default()
            },
            &mut res[1],
            USides::default(),
        ),
    ];

    solve_grid_layout(&config, constraints, &mut items);

    // In Column flow, Item 0 is at (Row 0, Col 0), Item 1 is at (Row 1, Col 0)
    assert!((res[0].pos.x - 0.0).abs() < 0.001);
    assert!((res[0].pos.y - 0.0).abs() < 0.001);

    assert!((res[1].pos.x - 0.0).abs() < 0.001);
    assert!((res[1].pos.y - 50.0).abs() < 0.001);
}
