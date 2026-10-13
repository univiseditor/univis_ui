use bevy::prelude::*;
use super::{
    URootUi, UVirtualScaleMode, UiCanvasSize, UiSpace,
    root_resolution::{RootResolutionIssue, resolve_root_canvas_size},
};

#[test]
fn virtual_scale_mode_match_height_scaling() {
    let mode = UVirtualScaleMode::MatchHeight;
    let reference = Vec2::new(1920.0, 1080.0);

    // 1080p native
    let scale_1080p = mode.compute_scale_factor(Vec2::new(1920.0, 1080.0), reference);
    assert!((scale_1080p - 1.0).abs() < 1e-4);

    // 4K UHD
    let scale_4k = mode.compute_scale_factor(Vec2::new(3840.0, 2160.0), reference);
    assert!((scale_4k - 2.0).abs() < 1e-4);

    // 1440p QHD
    let scale_1440p = mode.compute_scale_factor(Vec2::new(2560.0, 1440.0), reference);
    assert!((scale_1440p - (1440.0 / 1080.0)).abs() < 1e-4);

    // 21:9 Ultrawide at 1080p height (scale stays 1.0, width expands)
    let scale_ultrawide = mode.compute_scale_factor(Vec2::new(2560.0, 1080.0), reference);
    assert!((scale_ultrawide - 1.0).abs() < 1e-4);

    // Steam Deck 16:10 (1280x800) against 720p (1280x720)
    let deck_ref = Vec2::new(1280.0, 720.0);
    let scale_deck = mode.compute_scale_factor(Vec2::new(1280.0, 800.0), deck_ref);
    assert!((scale_deck - (800.0 / 720.0)).abs() < 1e-4);
}

#[test]
fn virtual_scale_mode_fit_and_fill() {
    let reference = Vec2::new(1920.0, 1080.0);
    let ultrawide_viewport = Vec2::new(2560.0, 1080.0);

    // Fit takes the min scale factor (1.0 vs 1.3333 -> 1.0)
    let fit_scale = UVirtualScaleMode::Fit.compute_scale_factor(ultrawide_viewport, reference);
    assert!((fit_scale - 1.0).abs() < 1e-4);

    // Fill takes the max scale factor (1.0 vs 1.3333 -> 1.3333)
    let fill_scale = UVirtualScaleMode::Fill.compute_scale_factor(ultrawide_viewport, reference);
    assert!((fill_scale - (2560.0 / 1920.0)).abs() < 1e-4);
}

#[test]
fn virtual_screen_root_constructors() {
    let root = URootUi::virtual_screen(Vec2::new(1920.0, 1080.0));
    assert_eq!(root.space, UiSpace::Screen);
    assert_eq!(
        root.canvas,
        UiCanvasSize::Virtual {
            reference: Vec2::new(1920.0, 1080.0),
            mode: UVirtualScaleMode::MatchHeight,
        }
    );

    let root_fit = URootUi::virtual_screen_mode(Vec2::new(1280.0, 720.0), UVirtualScaleMode::Fit);
    assert_eq!(root_fit.space, UiSpace::Screen);
    assert_eq!(
        root_fit.canvas,
        UiCanvasSize::Virtual {
            reference: Vec2::new(1280.0, 720.0),
            mode: UVirtualScaleMode::Fit,
        }
    );
}

#[test]
fn resolve_virtual_canvas_size_4k_produces_reference_canvas() {
    let mut app = App::new();

    // Simulate 4K window (3840 x 2160)
    app.world_mut().spawn(Window {
        resolution: (3840, 2160).into(),
        ..default()
    });

    let canvas_policy = UiCanvasSize::Virtual {
        reference: Vec2::new(1920.0, 1080.0),
        mode: UVirtualScaleMode::MatchHeight,
    };

    let resolved_size = app.world_mut().run_system_once(
        move |cameras: Query<(Entity, &Camera)>, windows: Query<&Window>| {
            let mut issue = RootResolutionIssue::None;
            resolve_root_canvas_size(canvas_policy, None, &cameras, &windows, &mut issue)
        },
    ).unwrap();

    // In 4K, 3840x2160 scaled by 2.0 produces 1920x1080 logical canvas size!
    assert!((resolved_size.x - 1920.0).abs() < 1e-3);
    assert!((resolved_size.y - 1080.0).abs() < 1e-3);
}

#[test]
fn resolve_virtual_canvas_size_ultrawide_preserves_height_and_expands_width() {
    let mut app = App::new();

    // Simulate 21:9 Ultrawide window (2560 x 1080)
    app.world_mut().spawn(Window {
        resolution: (2560, 1080).into(),
        ..default()
    });

    let canvas_policy = UiCanvasSize::Virtual {
        reference: Vec2::new(1920.0, 1080.0),
        mode: UVirtualScaleMode::MatchHeight,
    };

    let resolved_size = app.world_mut().run_system_once(
        move |cameras: Query<(Entity, &Camera)>, windows: Query<&Window>| {
            let mut issue = RootResolutionIssue::None;
            resolve_root_canvas_size(canvas_policy, None, &cameras, &windows, &mut issue)
        },
    ).unwrap();

    // Height stays 1080, width expands to 2560 for true ultrawide edge anchors
    assert!((resolved_size.x - 2560.0).abs() < 1e-3);
    assert!((resolved_size.y - 1080.0).abs() < 1e-3);
}
