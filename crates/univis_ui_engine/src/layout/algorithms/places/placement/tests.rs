use super::*;

fn default_spec() -> SolverSpec {
    SolverSpec {
        width_mode: SolverSizeMode::Fixed,
        width_val: 0.0,
        width_flex: 0.0,
        min_width: 0.0,
        max_width: f32::INFINITY,
        height_mode: SolverSizeMode::Fixed,
        height_val: 0.0,
        height_flex: 0.0,
        min_height: 0.0,
        max_height: f32::INFINITY,
        position_type: UPositionType::Relative,
        left: UVal::Auto,
        right: UVal::Auto,
        top: UVal::Auto,
        bottom: UVal::Auto,
        align_self: None,
        align_self_ext: None,
        justify_self_ext: None,
        justify_overflow: UOverflowPosition::Unsafe,
        align_overflow: UOverflowPosition::Unsafe,
        flex_grow: None,
        flex_shrink: None,
        flex_basis: None,
        grid_column_start: None,
        grid_column_span: 1,
        grid_row_start: None,
        grid_row_span: 1,
        order: 0,
    }
}

fn base_ctx() -> PlacementContext {
    PlacementContext {
        container_main_size: 100.0,
        container_cross_size: 100.0,
        padding_main_start: 0.0,
        padding_main_end: 0.0,
        padding_cross_start: 0.0,
        gap: 0.0,
        main_gap: 0.0,
        cross_gap: 0.0,
        justify_content: UJustifyContent::Start,
        align_items: UAlignItems::Start,
        justify_items: None,
        align_content: None,
        flex_wrap: UFlexWrap::NoWrap,
        grid_columns: 1,
        grid_template_columns: Vec::new(),
        grid_template_rows: Vec::new(),
        grid_auto_flow: UGridAutoFlow::Row,
        grid_auto_rows: UTrackSize::Auto,
        grid_auto_columns: UTrackSize::Auto,
    }
}

#[test]
fn track_resolution_uses_fr_distribution() {
    let tracks = resolve_track_sizes(
        &[
            UTrackSize::Px(100.0),
            UTrackSize::Fr(1.0),
            UTrackSize::Fr(2.0),
        ],
        3,
        UTrackSize::Auto,
        400.0,
        10.0,
        3,
        &[],
    );

    assert_eq!(tracks.len(), 3);
    assert!((tracks[0] - 100.0).abs() < 0.001);
    assert!(tracks[2] > tracks[1]);
}

#[test]
fn safe_overflow_keeps_non_negative_offset() {
    let offset = alignment_offset(UAlignSelfExt::Center, -20.0, UOverflowPosition::Safe);
    assert!(offset >= 0.0);
}

#[test]
fn unsafe_overflow_allows_negative_offset() {
    let offset = alignment_offset(UAlignSelfExt::Center, -20.0, UOverflowPosition::Unsafe);
    assert!(offset < 0.0);
}

#[test]
fn ext_align_self_overrides_legacy_align_self() {
    let mut spec = default_spec();
    spec.align_self = Some(UAlignSelf::Start);
    spec.align_self_ext = Some(UAlignSelfExt::End);
    let resolved = resolve_cross_align(&spec, UAlignItems::Start);
    assert_eq!(resolved, UAlignSelfExt::End);
}

#[test]
fn justify_self_centers_grid_item_inside_cell() {
    let mut result = SolverResult {
        size: Vec2::new(50.0, 30.0),
        pos: Vec2::ZERO,
    };
    let mut spec = default_spec();
    spec.justify_self_ext = Some(UAlignSelfExt::Center);
    let mut items = vec![SolverItem::new(spec, &mut result, USides::default())];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 60.0;
    ctx.grid_columns = 1;
    ctx.grid_template_columns = vec![UTrackSize::Px(100.0)];
    ctx.grid_template_rows = vec![UTrackSize::Px(60.0)];

    let placer = GridPlacer { columns: 1 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);
    assert!((result.pos.x - 25.0).abs() < 0.1);
}

#[test]
fn wrap_and_align_content_center_pushes_lines_down() {
    let mut r1 = SolverResult {
        size: Vec2::new(70.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r2 = SolverResult {
        size: Vec2::new(70.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r3 = SolverResult {
        size: Vec2::new(70.0, 20.0),
        pos: Vec2::ZERO,
    };
    let spec = default_spec();
    let mut items = vec![
        SolverItem::new(spec, &mut r1, USides::default()),
        SolverItem::new(spec, &mut r2, USides::default()),
        SolverItem::new(spec, &mut r3, USides::default()),
    ];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 200.0;
    ctx.flex_wrap = UFlexWrap::Wrap;
    ctx.align_content = Some(UContentAlignExt::Center);
    ctx.main_gap = 0.0;
    ctx.cross_gap = 10.0;

    let placer = FlexPlacer;
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!(r1.pos.y > 0.0);
    assert!(r2.pos.y > r1.pos.y);
    assert!(r3.pos.y > r2.pos.y);
}

#[test]
fn row_gap_is_applied_between_wrapped_lines() {
    let mut r1 = SolverResult {
        size: Vec2::new(70.0, 10.0),
        pos: Vec2::ZERO,
    };
    let mut r2 = SolverResult {
        size: Vec2::new(70.0, 10.0),
        pos: Vec2::ZERO,
    };
    let spec = default_spec();
    let mut items = vec![
        SolverItem::new(spec, &mut r1, USides::default()),
        SolverItem::new(spec, &mut r2, USides::default()),
    ];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 100.0;
    ctx.flex_wrap = UFlexWrap::Wrap;
    ctx.cross_gap = 20.0;

    let placer = FlexPlacer;
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    let delta = r2.pos.y - r1.pos.y;
    assert!(delta >= 30.0 - 0.1);
}

#[test]
fn nowrap_single_line_align_items_center_centers_in_container_cross_axis() {
    let mut result = SolverResult {
        size: Vec2::new(40.0, 20.0),
        pos: Vec2::ZERO,
    };
    let spec = default_spec();
    let mut items = vec![SolverItem::new(spec, &mut result, USides::default())];

    let mut ctx = base_ctx();
    ctx.container_main_size = 200.0;
    ctx.container_cross_size = 100.0;
    ctx.justify_content = UJustifyContent::Center;
    ctx.align_items = UAlignItems::Center;
    ctx.flex_wrap = UFlexWrap::NoWrap;

    let placer = FlexPlacer;
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!((result.pos.y - 40.0).abs() < 0.1);
}

#[test]
fn grid_span_affects_auto_placement() {
    let mut r1 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r2 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r3 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };

    let mut s1 = default_spec();
    s1.grid_column_span = 2;
    let s2 = default_spec();
    let s3 = default_spec();

    let mut items = vec![
        SolverItem::new(s1, &mut r1, USides::default()),
        SolverItem::new(s2, &mut r2, USides::default()),
        SolverItem::new(s3, &mut r3, USides::default()),
    ];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 100.0;
    ctx.grid_columns = 2;
    ctx.grid_template_columns = vec![UTrackSize::Px(50.0), UTrackSize::Px(50.0)];
    ctx.grid_template_rows = vec![UTrackSize::Px(30.0), UTrackSize::Px(30.0)];

    let placer = GridPlacer { columns: 2 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!(r2.pos.y >= 30.0 - 0.1);
    assert!(r3.pos.y >= 30.0 - 0.1);
}

#[test]
fn grid_auto_flow_row_is_sparse_no_backfill() {
    let mut r1 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r2 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r3 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r4 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };
    let mut r5 = SolverResult {
        size: Vec2::new(20.0, 20.0),
        pos: Vec2::ZERO,
    };

    let mut s1 = default_spec();
    s1.grid_column_span = 2;
    s1.grid_row_span = 2;
    let s2 = default_spec();
    let mut s3 = default_spec();
    s3.grid_row_span = 2;
    let mut s4 = default_spec();
    s4.grid_column_span = 2;
    let s5 = default_spec();

    let mut items = vec![
        SolverItem::new(s1, &mut r1, USides::default()),
        SolverItem::new(s2, &mut r2, USides::default()),
        SolverItem::new(s3, &mut r3, USides::default()),
        SolverItem::new(s4, &mut r4, USides::default()),
        SolverItem::new(s5, &mut r5, USides::default()),
    ];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 100.0;
    ctx.grid_columns = 4;
    ctx.grid_template_columns = vec![
        UTrackSize::Px(25.0),
        UTrackSize::Px(25.0),
        UTrackSize::Px(25.0),
        UTrackSize::Px(25.0),
    ];
    ctx.grid_template_rows = vec![
        UTrackSize::Px(10.0),
        UTrackSize::Px(10.0),
        UTrackSize::Px(10.0),
        UTrackSize::Px(10.0),
    ];
    ctx.grid_auto_flow = UGridAutoFlow::Row;

    let placer = GridPlacer { columns: 4 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!(r5.pos.y >= 20.0 - 0.1);
}

#[test]
fn grid_auto_item_stretches_to_cell_by_default() {
    let mut result = SolverResult {
        size: Vec2::ZERO,
        pos: Vec2::ZERO,
    };

    let mut spec = default_spec();
    spec.width_mode = SolverSizeMode::Auto;
    spec.height_mode = SolverSizeMode::Auto;
    spec.width_val = 0.0;
    spec.height_val = 0.0;

    let mut items = vec![SolverItem::new(spec, &mut result, USides::default())];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 60.0;
    ctx.grid_columns = 1;
    ctx.grid_template_columns = vec![UTrackSize::Px(100.0)];
    ctx.grid_template_rows = vec![UTrackSize::Px(60.0)];
    ctx.align_items = UAlignItems::Start;
    ctx.justify_items = None;

    let placer = GridPlacer { columns: 1 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!((result.size.x - 100.0).abs() < 0.1);
    assert!((result.size.y - 60.0).abs() < 0.1);
    assert!(result.pos.x.abs() < 0.1);
    assert!(result.pos.y.abs() < 0.1);
}

#[test]
fn grid_content_item_keeps_intrinsic_size_by_default() {
    let mut result = SolverResult {
        size: Vec2::new(42.0, 18.0),
        pos: Vec2::ZERO,
    };

    let mut spec = default_spec();
    spec.width_mode = SolverSizeMode::Content;
    spec.height_mode = SolverSizeMode::Content;
    spec.width_val = 42.0;
    spec.height_val = 18.0;

    let mut items = vec![SolverItem::new(spec, &mut result, USides::default())];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 60.0;
    ctx.grid_columns = 1;
    ctx.grid_template_columns = vec![UTrackSize::Px(100.0)];
    ctx.grid_template_rows = vec![UTrackSize::Px(60.0)];
    ctx.align_items = UAlignItems::Start;
    ctx.justify_items = None;

    let placer = GridPlacer { columns: 1 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    placer.place(&mut items, &axis, &ctx);

    assert!((result.size.x - 42.0).abs() < 0.1);
    assert!((result.size.y - 18.0).abs() < 0.1);
    assert!(result.pos.x.abs() < 0.1);
    assert!(result.pos.y.abs() < 0.1);
}

#[test]
fn content_aware_auto_tracks_respect_item_floors() {
    let mut r1 = SolverResult {
        size: Vec2::new(100.0, 60.0),
        pos: Vec2::ZERO,
    };
    let mut r2 = SolverResult {
        size: Vec2::new(100.0, 40.0),
        pos: Vec2::ZERO,
    };

    let mut s1 = default_spec();
    s1.height_mode = SolverSizeMode::Content;
    s1.height_val = 60.0;
    let mut s2 = default_spec();
    s2.height_mode = SolverSizeMode::Content;
    s2.height_val = 40.0;

    let mut items = vec![
        SolverItem::new(s1, &mut r1, USides::default()),
        SolverItem::new(s2, &mut r2, USides::default()),
    ];

    let mut ctx = base_ctx();
    ctx.container_main_size = 100.0;
    ctx.container_cross_size = 0.0; // unconstrained height
    ctx.grid_columns = 1;
    ctx.grid_template_columns = vec![UTrackSize::Px(100.0)];
    ctx.grid_template_rows = vec![UTrackSize::Auto, UTrackSize::Auto];
    ctx.cross_gap = 10.0;

    let placer = GridPlacer { columns: 1 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    let size = placer.place(&mut items, &axis, &ctx);

    // Row 0 floor is 60.0, Row 1 floor is 40.0
    // Total height = 60.0 + 40.0 + 10.0 gap = 110.0
    assert!((size.y - 110.0).abs() < 0.1);
    assert!(r1.pos.y.abs() < 0.1);
    // r2 must be placed below r1 with gap, completely eliminating overlap!
    assert!((r2.pos.y - 70.0).abs() < 0.1);
}

#[test]
fn auto_track_with_fr_track_takes_content_floor() {
    let tracks = resolve_track_sizes(
        &[UTrackSize::Auto, UTrackSize::Fr(1.0)],
        2,
        UTrackSize::Auto,
        300.0,
        0.0,
        2,
        &[80.0, 0.0],
    );

    assert_eq!(tracks.len(), 2);
    // Auto track gets its exact content floor (80.0)
    assert!((tracks[0] - 80.0).abs() < 0.001);
    // Fr(1.0) track gets remaining free space (300.0 - 80.0 = 220.0)
    assert!((tracks[1] - 220.0).abs() < 0.001);
}

#[test]
fn multispan_item_distributes_deficit_across_tracks() {
    let mut r1 = SolverResult {
        size: Vec2::new(120.0, 30.0),
        pos: Vec2::ZERO,
    };
    let mut s1 = default_spec();
    s1.grid_column_span = 2;
    s1.width_mode = SolverSizeMode::Content;
    s1.width_val = 120.0;

    let mut items = vec![SolverItem::new(s1, &mut r1, USides::default())];

    let mut ctx = base_ctx();
    ctx.container_main_size = 0.0;
    ctx.container_cross_size = 50.0;
    ctx.grid_columns = 2;
    ctx.grid_template_columns = vec![UTrackSize::Auto, UTrackSize::Auto];
    ctx.grid_template_rows = vec![UTrackSize::Px(30.0)];
    ctx.main_gap = 10.0;

    let placer = GridPlacer { columns: 2 };
    let axis = AxisHelper::new(UFlexDirection::Row);
    let size = placer.place(&mut items, &axis, &ctx);

    // Two tracks plus 10 gap must be at least 120.0
    assert!(size.x >= 120.0 - 0.1);
}
