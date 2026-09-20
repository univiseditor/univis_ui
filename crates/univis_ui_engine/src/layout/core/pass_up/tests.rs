use super::*;

#[test]
fn test_flex_row_intrinsic() {
    let items = vec![
        ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 50.0,
            max_h: 50.0,
            ..Default::default()
        },
        ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 60.0,
            max_h: 60.0,
            ..Default::default()
        },
    ];
    let layout = ULayout {
        flex_direction: UFlexDirection::Row,
        gap: 10.0,
        ..Default::default()
    };
    let (min_w, max_w, min_h, max_h) = measure_flex_intrinsic(&items, &layout);
    assert_eq!(min_w, 210.0);
    assert_eq!(max_w, 210.0);
    assert_eq!(min_h, 60.0);
    assert_eq!(max_h, 60.0);
}

#[test]
fn test_flex_column_intrinsic() {
    let items = vec![
        ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 50.0,
            max_h: 50.0,
            ..Default::default()
        },
        ChildMeasureItem {
            min_w: 120.0,
            max_w: 120.0,
            min_h: 60.0,
            max_h: 60.0,
            ..Default::default()
        },
    ];
    let layout = ULayout {
        flex_direction: UFlexDirection::Column,
        gap: 10.0,
        ..Default::default()
    };
    let (min_w, max_w, min_h, max_h) = measure_flex_intrinsic(&items, &layout);
    assert_eq!(min_w, 120.0);
    assert_eq!(max_w, 120.0);
    assert_eq!(min_h, 120.0); // 50 + 60 + 10
    assert_eq!(max_h, 120.0);
}

#[test]
fn test_stack_intrinsic() {
    let items = vec![
        ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 50.0,
            max_h: 50.0,
            ..Default::default()
        },
        ChildMeasureItem {
            min_w: 120.0,
            max_w: 120.0,
            min_h: 40.0,
            max_h: 40.0,
            ..Default::default()
        },
        ChildMeasureItem {
            min_w: 80.0,
            max_w: 80.0,
            min_h: 70.0,
            max_h: 70.0,
            ..Default::default()
        },
    ];
    let (min_w, max_w, min_h, max_h) = measure_stack_intrinsic(&items);
    assert_eq!(min_w, 120.0);
    assert_eq!(max_w, 120.0);
    assert_eq!(min_h, 70.0);
    assert_eq!(max_h, 70.0);
}

#[test]
fn test_grid_2x2_intrinsic() {
    let mut scratch = MeasurePassScratch::default();
    for _ in 0..4 {
        scratch.child_items.push(ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 50.0,
            max_h: 50.0,
            col_span: 1,
            row_span: 1,
            ..Default::default()
        });
    }
    let layout = ULayout {
        display: UDisplay::Grid,
        grid_columns: 2,
        gap: 10.0,
        ..Default::default()
    };
    let (min_w, max_w, min_h, max_h) =
        measure_grid_intrinsic(&mut scratch, &layout, UVal::Auto, USides::default());
    // 2 cols of 100 + 10 gap = 210
    assert_eq!(min_w, 210.0);
    assert_eq!(max_w, 210.0);
    // 2 rows of 50 + 10 gap = 110
    assert_eq!(min_h, 110.0);
    assert_eq!(max_h, 110.0);
}

#[test]
fn test_grid_with_spans() {
    let mut scratch = MeasurePassScratch::default();
    scratch.child_items.push(ChildMeasureItem {
        min_w: 150.0,
        max_w: 150.0,
        min_h: 50.0,
        max_h: 50.0,
        col_span: 2,
        row_span: 1,
        ..Default::default()
    });
    scratch.child_items.push(ChildMeasureItem {
        min_w: 60.0,
        max_w: 60.0,
        min_h: 50.0,
        max_h: 50.0,
        col_span: 1,
        row_span: 1,
        ..Default::default()
    });

    let layout = ULayout {
        display: UDisplay::Grid,
        grid_columns: 3,
        gap: 10.0,
        ..Default::default()
    };
    let (min_w, max_w, min_h, max_h) =
        measure_grid_intrinsic(&mut scratch, &layout, UVal::Auto, USides::default());
    // Col 0 + 1 deficit share from 150 - 10 = 140 -> 70 each
    // Col 2 is 60. Total width = 70 + 70 + 60 + 2 * 10 = 220
    assert_eq!(min_w, 220.0);
    assert_eq!(max_w, 220.0);
    assert_eq!(min_h, 50.0);
    assert_eq!(max_h, 50.0);
}

#[test]
fn test_grid_fixed_tracks() {
    let mut scratch = MeasurePassScratch::default();
    scratch.child_items.push(ChildMeasureItem {
        min_w: 50.0,
        max_w: 50.0,
        min_h: 30.0,
        max_h: 30.0,
        col_span: 1,
        row_span: 1,
        ..Default::default()
    });
    scratch.child_items.push(ChildMeasureItem {
        min_w: 40.0,
        max_w: 40.0,
        min_h: 30.0,
        max_h: 30.0,
        col_span: 1,
        row_span: 1,
        ..Default::default()
    });

    let mut layout = ULayout {
        display: UDisplay::Grid,
        grid_columns: 2,
        gap: 10.0,
        ..Default::default()
    };
    layout.container_ext.grid.template_columns = vec![UTrackSize::Px(200.0), UTrackSize::Px(100.0)];
    let (min_w, max_w, min_h, max_h) =
        measure_grid_intrinsic(&mut scratch, &layout, UVal::Auto, USides::default());
    // Fixed columns: 200 + 100 + 10 = 310
    assert_eq!(min_w, 310.0);
    assert_eq!(max_w, 310.0);
    // 1 row of 30
    assert_eq!(min_h, 30.0);
    assert_eq!(max_h, 30.0);
}

#[test]
fn test_grid_column_auto_flow() {
    let mut scratch = MeasurePassScratch::default();
    for _ in 0..4 {
        scratch.child_items.push(ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 50.0,
            max_h: 50.0,
            col_span: 1,
            row_span: 1,
            ..Default::default()
        });
    }

    let mut layout = ULayout {
        display: UDisplay::Grid,
        grid_columns: 1,
        gap: 10.0,
        ..Default::default()
    };
    layout.container_ext.grid.auto_flow = UGridAutoFlow::Column;
    layout.container_ext.grid.template_rows = vec![UTrackSize::Auto, UTrackSize::Auto];

    let (min_w, max_w, min_h, max_h) =
        measure_grid_intrinsic(&mut scratch, &layout, UVal::Auto, USides::default());
    // 4 items flow down 2 rows -> 2 cols x 2 rows
    assert_eq!(min_w, 210.0);
    assert_eq!(max_w, 210.0);
    assert_eq!(min_h, 110.0);
    assert_eq!(max_h, 110.0);
}

#[test]
fn test_grid_repeat_fit_intrinsic_with_explicit_width() {
    let mut scratch = MeasurePassScratch::default();
    for _ in 0..6 {
        scratch.child_items.push(ChildMeasureItem {
            min_w: 100.0,
            max_w: 100.0,
            min_h: 110.0,
            max_h: 110.0,
            col_span: 1,
            row_span: 1,
            ..Default::default()
        });
    }

    let mut layout = ULayout {
        display: UDisplay::Grid,
        grid_columns: 1,
        gap: 14.0,
        ..Default::default()
    };
    layout.container_ext.grid.template_columns = vec![UTrackSize::repeat_fit(
        UTrackRepeat::minmax(UTrackBound::px(200.0), UTrackBound::fr(1.0)),
    )];
    layout.container_ext.grid.template_rows = vec![UTrackSize::Auto];

    // Explicit 960px width with 16px padding on sides (available = 928px)
    // (928 + 14) / (200 + 14) = 4 columns
    // 6 items in 4 columns = 2 rows!
    let (_, _, min_h, max_h) =
        measure_grid_intrinsic(&mut scratch, &layout, UVal::Px(960.0), USides::all(16.0));

    // 2 rows of 110 + 1 gap of 14 = 234.0
    assert_eq!(min_h, 234.0);
    assert_eq!(max_h, 234.0);
}
