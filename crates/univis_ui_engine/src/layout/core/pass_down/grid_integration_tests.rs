use super::*;
use crate::layout::UnivisLayoutPlugin;
use crate::layout::grid::UTrackSize;
use crate::layout::layout_system::URootUi;
use crate::layout::query::ComputedSize;
use crate::layout::univis_node::{UDisplay, ULayout, UNode};
use crate::schedule::UiWorkState;
use bevy::prelude::*;

#[test]
fn ecs_grid_container_solves_child_geometry_and_settles() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

    let mut grid_layout = ULayout::default();
    grid_layout.display = UDisplay::Grid;
    grid_layout.container_ext.grid.template_columns =
        vec![UTrackSize::Px(120.0), UTrackSize::Px(120.0)];
    grid_layout.container_ext.grid.template_rows = vec![UTrackSize::Px(60.0)];
    grid_layout.container_ext.box_align.column_gap = Some(15.0);

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            grid_layout,
        ))
        .id();

    let child0 = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                ..default()
            },
        ))
        .id();

    let child1 = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                ..default()
            },
        ))
        .id();

    app.update();

    let size0 = app
        .world()
        .entity(child0)
        .get::<ComputedSize>()
        .copied()
        .expect("child0 should have a computed size");
    let transform0 = app
        .world()
        .entity(child0)
        .get::<Transform>()
        .copied()
        .expect("child0 should have a transform");

    let size1 = app
        .world()
        .entity(child1)
        .get::<ComputedSize>()
        .copied()
        .expect("child1 should have a computed size");
    let transform1 = app
        .world()
        .entity(child1)
        .get::<Transform>()
        .copied()
        .expect("child1 should have a transform");

    assert_eq!(size0.width, 120.0);
    assert_eq!(size0.height, 60.0);
    // In World2d, 1px = 0.001 units.
    // Child 0 center: -140.0px * 0.001 = -0.14.
    assert!((transform0.translation.x - (-0.14)).abs() < 0.001);

    assert_eq!(size1.width, 120.0);
    assert_eq!(size1.height, 60.0);
    // 120px + 15px gap = 135px -> 135.0px * 0.001 = 0.135 units.
    assert!((transform1.translation.x - transform0.translation.x - 0.135).abs() < 0.001);

    let work_state = app.world().resource::<UiWorkState>();
    assert!(work_state.is_settled());
    assert!(!work_state.budget_exhausted());
}
