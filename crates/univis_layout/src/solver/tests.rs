use super::*;
use crate::geometry::{BoxConstraints, USides, UVal, UValLength};
use crate::types::*;
use glam::Vec2;

fn base_solver_config() -> SolverConfig {
    SolverConfig::default()
}

fn item(spec: SolverSpec) -> SolverItem {
    SolverItem::new(spec, USides::default())
}

#[test]
fn resolve_flex_basis_handles_percent() {
    let value = resolve_flex_basis(UVal::Percent(0.5), 10.0, 300.0);
    assert_eq!(value, Some(150.0));
}

#[test]
fn auto_cross_size_stretches_but_content_cross_size_keeps_intrinsic_value() {
    let mut config = base_solver_config();
    config.align_items = UAlignItems::Stretch;
    let constraints = BoxConstraints::tight(Vec2::new(160.0, 100.0));

    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 40.0,
            height_mode: SolverSizeMode::Auto,
            height_val: 18.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 40.0,
            height_mode: SolverSizeMode::Content,
            height_val: 18.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(items[0].result.size.y, 100.0);
    assert_eq!(items[1].result.size.y, 18.0);
}

#[test]
fn flex_shrink_respects_min_width_and_redistributes_remaining_overflow() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(100.0, 40.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 80.0,
            min_width: 60.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 20.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 80.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 20.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
    ];

    let solved = solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(solved, Vec2::new(100.0, 40.0));
    assert_eq!(items[0].result.size.x, 60.0);
    assert_eq!(items[1].result.size.x, 40.0);
}

#[test]
fn flex_grow_respects_max_width_and_redistributes_remaining_space() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(200.0, 40.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 50.0,
            max_width: 70.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 20.0,
            flex_grow: Some(1.0),
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 50.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 20.0,
            flex_grow: Some(1.0),
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
    ];

    let solved = solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(solved, Vec2::new(200.0, 40.0));
    assert_eq!(items[0].result.size.x, 70.0);
    assert_eq!(items[1].result.size.x, 130.0);
}

#[test]
fn wrapped_row_moves_item_to_next_line_when_min_width_blocks_further_shrink() {
    let mut config = base_solver_config();
    config.flex_wrap = UFlexWrap::Wrap;
    let constraints = BoxConstraints::tight(Vec2::new(150.0, 120.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            min_width: 90.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 24.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            min_width: 90.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 24.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(items[0].result.size.x, 90.0);
    assert_eq!(items[1].result.size.x, 90.0);
    assert!(items[1].result.pos.y >= items[0].result.size.y - 0.1);
}

#[test]
fn wrapped_column_moves_item_to_next_column_when_min_height_blocks_further_shrink() {
    let mut config = base_solver_config();
    config.flex_direction = UFlexDirection::Column;
    config.flex_wrap = UFlexWrap::Wrap;
    let constraints = BoxConstraints::tight(Vec2::new(120.0, 150.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 24.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 100.0,
            min_height: 90.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 24.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 100.0,
            min_height: 90.0,
            flex_shrink: Some(1.0),
            ..Default::default()
        }),
    ];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(items[0].result.size.y, 90.0);
    assert_eq!(items[1].result.size.y, 90.0);
    assert!(items[1].result.pos.x >= items[0].result.size.x - 0.1);
}

#[test]
fn aspect_ratio_derives_height_from_fixed_width_in_row() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 160.0,
        height_mode: SolverSizeMode::Auto,
        aspect_ratio: Some(16.0 / 9.0),
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((items[0].result.size.x - 160.0).abs() < 0.001);
    assert!((items[0].result.size.y - 90.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_derives_width_from_fixed_height_in_row() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Auto,
        height_mode: SolverSizeMode::Fixed,
        height_val: 100.0,
        aspect_ratio: Some(2.0),
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((items[0].result.size.x - 200.0).abs() < 0.001);
    assert!((items[0].result.size.y - 100.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_inhibits_implicit_cross_stretch() {
    let mut config = base_solver_config();
    config.align_items = UAlignItems::Stretch;
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 120.0,
        height_mode: SolverSizeMode::Auto,
        aspect_ratio: Some(1.0), // Square
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    // Height must be 120.0 (1:1), NOT stretched to container height 300.0!
    assert!((items[0].result.size.x - 120.0).abs() < 0.001);
    assert!((items[0].result.size.y - 120.0).abs() < 0.001);
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
        ..Default::default()
    };

    let res = solve_absolute_box(
        Vec2::new(500.0, 500.0),
        &spec,
        USides::default(),
        Vec2::ZERO,
    );

    assert!((res.size.x - 150.0).abs() < 0.001);
    assert!((res.size.y - 100.0).abs() < 0.001);
    assert_eq!(res.pos, Vec2::new(10.0, 10.0));
}

#[test]
fn absolute_box_centers_with_align_and_justify_self() {
    let spec = SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 200.0,
        height_mode: SolverSizeMode::Fixed,
        height_val: 100.0,
        position_type: UPositionType::Absolute,
        justify_self_ext: Some(crate::types::UAlignSelfExt::Center),
        align_self: Some(crate::types::UAlignSelf::Center),
        ..Default::default()
    };

    let res = solve_absolute_box(
        Vec2::new(800.0, 600.0),
        &spec,
        USides::default(),
        Vec2::ZERO,
    );

    assert_eq!(res.size, Vec2::new(200.0, 100.0));
    // Centered: (800 - 200) / 2 = 300, (600 - 100) / 2 = 250
    assert_eq!(res.pos, Vec2::new(300.0, 250.0));
}

#[test]
fn flex_layout_solves_absolute_auto_sized_item_from_intrinsic() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(1920.0, 1080.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Auto,
        width_val: 320.0,
        height_mode: SolverSizeMode::Auto,
        height_val: 80.0,
        position_type: UPositionType::Absolute,
        left: UVal::Px(32.0),
        top: UVal::Px(32.0),
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    assert_eq!(items[0].result.size, Vec2::new(320.0, 80.0));
    assert_eq!(items[0].result.pos, Vec2::new(32.0, 32.0));
}

#[test]
fn flex_layout_resolves_calc_dimension() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(500.0, 200.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Calc,
        width_uval: UVal::calc(1.0, -40.0),
        height_mode: SolverSizeMode::Fixed,
        height_val: 50.0,
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    assert!((items[0].result.size.x - 460.0).abs() < 0.001);
    assert!((items[0].result.size.y - 50.0).abs() < 0.001);
}

#[test]
fn flex_layout_resolves_clamp_dimension_in_cross_axis() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(500.0, 200.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 100.0,
        height_mode: SolverSizeMode::Calc,
        height_uval: UVal::clamp(
            UValLength::px(50.0),
            UValLength::percent(0.8),
            UValLength::px(120.0),
        ),
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    // 80% of 200 is 160, clamped to max 120
    assert!((items[0].result.size.x - 100.0).abs() < 0.001);
    assert!((items[0].result.size.y - 120.0).abs() < 0.001);
}

#[test]
fn aspect_ratio_derives_cross_from_calc_main() {
    let config = base_solver_config();
    let constraints = BoxConstraints::tight(Vec2::new(400.0, 300.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Calc,
        width_uval: UVal::calc(0.5, -10.0),
        height_mode: SolverSizeMode::Auto,
        aspect_ratio: Some(2.0),
        ..Default::default()
    })];

    solve_flex_layout(&config, constraints, &mut items);

    // width = 400 * 0.5 - 10 = 190. height = 190 / 2.0 = 95
    assert!((items[0].result.size.x - 190.0).abs() < 0.001);
    assert!((items[0].result.size.y - 95.0).abs() < 0.001);
}

#[test]
fn grid_solver_places_2x2_fixed_cells_with_gaps() {
    let mut config = base_solver_config();
    config.display = UDisplay::Grid;
    config.grid_template_columns = vec![UTrackSize::Px(100.0), UTrackSize::Px(100.0)];
    config.grid_template_rows = vec![UTrackSize::Px(50.0), UTrackSize::Px(50.0)];
    config.column_gap = Some(10.0);
    config.row_gap = Some(15.0);

    let constraints = BoxConstraints::tight(Vec2::new(300.0, 200.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 50.0,
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 50.0,
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 50.0,
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 100.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 50.0,
            ..Default::default()
        }),
    ];

    solve_grid_layout(&config, constraints, &mut items, None);

    // Item 0: Row 0, Col 0 -> (0, 0)
    assert!((items[0].result.pos.x - 0.0).abs() < 0.001);
    assert!((items[0].result.pos.y - 0.0).abs() < 0.001);
    assert!((items[0].result.size.x - 100.0).abs() < 0.001);
    assert!((items[0].result.size.y - 50.0).abs() < 0.001);

    // Item 1: Row 0, Col 1 -> (110, 0)
    assert!((items[1].result.pos.x - 110.0).abs() < 0.001);
    assert!((items[1].result.pos.y - 0.0).abs() < 0.001);

    // Item 2: Row 1, Col 0 -> (0, 65)
    assert!((items[2].result.pos.x - 0.0).abs() < 0.001);
    assert!((items[2].result.pos.y - 65.0).abs() < 0.001);

    // Item 3: Row 1, Col 1 -> (110, 65)
    assert!((items[3].result.pos.x - 110.0).abs() < 0.001);
    assert!((items[3].result.pos.y - 65.0).abs() < 0.001);
}

#[test]
fn grid_solver_distributes_fr_tracks_proportionally() {
    let mut config = base_solver_config();
    config.display = UDisplay::Grid;
    config.grid_template_columns = vec![UTrackSize::Fr(1.0), UTrackSize::Fr(2.0)];
    config.grid_template_rows = vec![UTrackSize::Px(80.0)];

    let constraints = BoxConstraints::tight(Vec2::new(300.0, 100.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Auto,
            height_mode: SolverSizeMode::Fixed,
            height_val: 80.0,
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Auto,
            height_mode: SolverSizeMode::Fixed,
            height_val: 80.0,
            ..Default::default()
        }),
    ];

    solve_grid_layout(&config, constraints, &mut items, None);

    // 1fr = 100.0, 2fr = 200.0
    assert!((items[0].result.size.x - 100.0).abs() < 0.001);
    assert!((items[0].result.pos.x - 0.0).abs() < 0.001);

    assert!((items[1].result.size.x - 200.0).abs() < 0.001);
    assert!((items[1].result.pos.x - 100.0).abs() < 0.001);
}

#[test]
fn grid_solver_handles_column_spanning() {
    let mut config = base_solver_config();
    config.display = UDisplay::Grid;
    config.grid_template_columns = vec![
        UTrackSize::Px(100.0),
        UTrackSize::Px(150.0),
        UTrackSize::Px(100.0),
    ];
    config.grid_template_rows = vec![UTrackSize::Px(60.0)];
    config.column_gap = Some(10.0);

    let constraints = BoxConstraints::tight(Vec2::new(400.0, 100.0));
    let mut items = vec![item(SolverSpec {
        width_mode: SolverSizeMode::Auto,
        height_mode: SolverSizeMode::Fixed,
        height_val: 60.0,
        grid_column_span: 2,
        ..Default::default()
    })];

    solve_grid_layout(&config, constraints, &mut items, None);

    // Col 0 (100) + gap (10) + Col 1 (150) = 260.0
    assert!((items[0].result.size.x - 260.0).abs() < 0.001);
    assert!((items[0].result.pos.x - 0.0).abs() < 0.001);
}

#[test]
fn grid_solver_column_auto_flow() {
    let mut config = base_solver_config();
    config.display = UDisplay::Grid;
    config.grid_auto_flow = UGridAutoFlow::Column;
    config.grid_template_columns = vec![UTrackSize::Px(80.0), UTrackSize::Px(80.0)];
    config.grid_template_rows = vec![UTrackSize::Px(40.0), UTrackSize::Px(40.0)];
    config.row_gap = Some(10.0);
    config.column_gap = Some(10.0);

    let constraints = BoxConstraints::tight(Vec2::new(200.0, 200.0));
    let mut items = vec![
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 80.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 40.0,
            ..Default::default()
        }),
        item(SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 80.0,
            height_mode: SolverSizeMode::Fixed,
            height_val: 40.0,
            ..Default::default()
        }),
    ];

    solve_grid_layout(&config, constraints, &mut items, None);

    // In Column flow, Item 0 is at (Row 0, Col 0), Item 1 is at (Row 1, Col 0)
    assert!((items[0].result.pos.x - 0.0).abs() < 0.001);
    assert!((items[0].result.pos.y - 0.0).abs() < 0.001);

    assert!((items[1].result.pos.x - 0.0).abs() < 0.001);
    assert!((items[1].result.pos.y - 50.0).abs() < 0.001);
}
