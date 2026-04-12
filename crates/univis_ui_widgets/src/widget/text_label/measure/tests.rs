use super::*;
use bevy::prelude::{App, MinimalPlugins, Update};
use univis_ui_engine::layout::UnivisLayoutPlugin;
use univis_ui_engine::layout::invalidation::{UiInvalidateRequestQueue, UiLayoutInvalidation};
use univis_ui_engine::schedule::UiWorkState;

fn fixed_text_cache(size: Vec2, text: &str) -> UTextLabelLayoutCache {
    UTextLabelLayoutCache {
        measured_size: size,
        min_content_size: size,
        max_content_size: size,
        displayed_text: text.to_string(),
        line_count: 1,
        dirty: false,
        ..default()
    }
}

#[test]
fn parent_bounds_change_is_detected_from_cached_width() {
    let cache = UTextLabelLayoutCache {
        parent_bound_width: Some(120.0),
        parent_bound_height: None,
        ..default()
    };

    assert!(parent_bounds_changed(
        &cache,
        TextBounds {
            width: Some(180.0),
            height: None,
        }
    ));
    assert!(!parent_bounds_changed(
        &cache,
        TextBounds {
            width: Some(120.0),
            height: None,
        }
    ));
}

#[test]
fn parent_bounds_change_is_detected_when_bounds_appear_or_disappear() {
    let cache = UTextLabelLayoutCache {
        parent_bound_width: None,
        parent_bound_height: Some(40.0),
        ..default()
    };

    assert!(parent_bounds_changed(
        &cache,
        TextBounds {
            width: Some(200.0),
            height: Some(40.0),
        }
    ));
    assert!(parent_bounds_changed(
        &cache,
        TextBounds {
            width: None,
            height: None,
        }
    ));
}

#[test]
fn text_change_without_intrinsic_delta_does_not_dirty_parent_solve() {
    let mut app = App::new();
    app.init_resource::<UiInvalidateRequestQueue>();
    app.add_systems(
        Update,
        (sync_text_label_intrinsic_size, mark_text_label_layout_dirty).chain(),
    );

    let parent = app
        .world_mut()
        .spawn((
            UNode::default(),
            ComputedSize {
                width: 400.0,
                height: 200.0,
                ..default()
            },
        ))
        .id();
    let label = app
        .world_mut()
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                ..default()
            },
            UTextLabel {
                text: "alpha".into(),
                autosize: false,
                overflow: UTextOverflow::Visible,
                ..default()
            },
            IntrinsicSize::from_max_size(Vec2::new(80.0, 20.0)),
            fixed_text_cache(Vec2::new(80.0, 20.0), "alpha"),
        ))
        .id();

    app.update();
    app.insert_resource(UiInvalidateRequestQueue::default());

    app.world_mut()
        .entity_mut(label)
        .get_mut::<UTextLabel>()
        .expect("label should keep its text component")
        .text = "beta".into();

    app.update();

    let requests = app.world().resource::<UiInvalidateRequestQueue>();
    assert!(requests.is_empty());
    assert!(app.world().get_entity(parent).is_ok());
    assert!(app.world().get_entity(label).is_ok());
}

#[test]
fn text_change_with_intrinsic_delta_dirties_parent_once() {
    let mut app = App::new();
    app.init_resource::<UiInvalidateRequestQueue>();
    app.add_systems(
        Update,
        (sync_text_label_intrinsic_size, mark_text_label_layout_dirty).chain(),
    );

    let parent = app
        .world_mut()
        .spawn((
            UNode::default(),
            ComputedSize {
                width: 400.0,
                height: 200.0,
                ..default()
            },
        ))
        .id();
    let label = app
        .world_mut()
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                ..default()
            },
            UTextLabel {
                text: "alpha".into(),
                autosize: false,
                overflow: UTextOverflow::Visible,
                ..default()
            },
            IntrinsicSize::from_max_size(Vec2::new(80.0, 20.0)),
            fixed_text_cache(Vec2::new(80.0, 20.0), "alpha"),
        ))
        .id();

    app.update();
    app.insert_resource(UiInvalidateRequestQueue::default());

    app.world_mut().entity_mut(label).insert((
        UTextLabel {
            text: "alphabet soup".into(),
            autosize: false,
            overflow: UTextOverflow::Visible,
            ..default()
        },
        fixed_text_cache(Vec2::new(128.0, 20.0), "alphabet soup"),
    ));

    app.update();

    let intrinsic = app
        .world()
        .entity(label)
        .get::<IntrinsicSize>()
        .copied()
        .expect("label should keep intrinsic size");
    let requests = app.world().resource::<UiInvalidateRequestQueue>();

    assert_eq!(intrinsic.width, 128.0);
    assert_eq!(intrinsic.max_width, 128.0);
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests.request_for(label),
        Some(UiLayoutInvalidation::intrinsic_change())
    );
    assert_eq!(
        requests.pending_stages(),
        UiLayoutInvalidation::intrinsic_change().pending_stages()
    );
}

#[test]
fn autosize_text_stabilizes_without_requeue_loops() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
    app.add_systems(
        UiSettlementSchedule,
        (
            sync_text_label_intrinsic_size
                .in_set(UnivisPostUpdateSet::ExternalPrepare)
                .before(fit_node_to_text_size),
            fit_node_to_text_size
                .in_set(UnivisPostUpdateSet::ExternalPrepare)
                .before(mark_text_label_layout_dirty),
            mark_text_label_layout_dirty.in_set(UnivisPostUpdateSet::ExternalPrepare),
        )
            .chain(),
    );

    let root = app
        .world_mut()
        .spawn((
            URootUi::world_2d(Vec2::new(400.0, 200.0)),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout::default(),
        ))
        .id();

    let label = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(1.0),
                height: UVal::Px(1.0),
                ..default()
            },
            UTextLabel {
                text: "autosize".into(),
                autosize: true,
                overflow: UTextOverflow::Visible,
                ..default()
            },
            IntrinsicSize::default(),
            fixed_text_cache(Vec2::new(96.0, 24.0), "autosize"),
        ))
        .id();

    app.update();

    let work_state = app.world().resource::<UiWorkState>();
    let node = app
        .world()
        .entity(label)
        .get::<UNode>()
        .cloned()
        .expect("label should keep its node");

    assert!(work_state.is_settled());
    assert!(!work_state.budget_exhausted());
    assert_eq!(node.width, UVal::Px(96.0));
    assert_eq!(node.height, UVal::Px(24.0));

    app.update();

    let work_state = app.world().resource::<UiWorkState>();
    assert!(work_state.is_settled());
    assert!(!work_state.budget_exhausted());
    assert_eq!(work_state.current_generation(), 1);
}
