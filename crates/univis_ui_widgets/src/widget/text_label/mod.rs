//! Text rendering widgets for Univis UI.
//!
//! [`UTextLabel`](crate::widget::text_label::UTextLabel) measures text through
//! Bevy's text pipeline, then renders the final result through Univis SDF
//! materials so the same widget can stay sharp in screen and world roots.

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use univis_ui_engine::schedule::{UiSettlementSchedule, UnivisPostUpdateSet};

mod measure;
mod model;
mod render;

use self::measure::invalidation::mark_text_label_layout_dirty;
pub use self::measure::{
    fit_node_to_text_size, measure_text_label_layout, sync_text_label_intrinsic_size,
};
pub use self::model::{
    TextChildMarker, UTextLabel, UTextLabelLayoutCache, UTextOverflow, UTextTruncateSide,
};
use self::render::atlas::UTextLabelAtlasCache;
use self::render::clip_sync::sync_text_clipper_materials;
use self::render::{UTextLabelSdfMaterial, UTextLabelSdfMaterial3d, sync_text_label_meshes};

#[cfg(test)]
use self::measure::bidi::{
    ISOLATE_END, LTR_ISOLATE_START, TextBaseDirection, base_direction_for_text,
    build_truncate_candidate, resolve_truncate_side, text_char_boundaries,
};
#[cfg(test)]
use self::measure::bounds::{
    desired_text_label_intrinsic_size, label_measure_bounds, measured_text_outer_size,
};
#[cfg(test)]
use self::render::clip_sync::{LocalClipRect, clip_quad_to_rect, label_content_clip_rect};
#[cfg(test)]
use self::render::mesh::{TextGlyphQuad, text_horizontal_offset};
#[cfg(test)]
use bevy::text::TextBounds;
#[cfg(test)]
use univis_ui_engine::layout::geometry::{USides, UVal};
#[cfg(test)]
use univis_ui_engine::layout::query::{ComputedSize, IntrinsicSize};
#[cfg(test)]
use univis_ui_engine::layout::univis_node::UNode;

/// Registers the `UTextLabel` measurement and SDF rendering pipeline.
pub struct UnivisTextPlugin;

impl Plugin for UnivisTextPlugin {
    fn build(&self, app: &mut App) {
        super::register_widget_embedded_assets(app);

        app.register_type::<UTextLabel>()
            .register_type::<UTextOverflow>()
            .register_type::<UTextTruncateSide>()
            .register_type::<UTextLabelLayoutCache>()
            .init_resource::<UTextLabelAtlasCache>()
            .add_plugins(Material2dPlugin::<UTextLabelSdfMaterial>::default())
            .add_plugins(MaterialPlugin::<UTextLabelSdfMaterial3d>::default())
            .add_systems(
                UiSettlementSchedule,
                measure_text_label_layout
                    .in_set(UnivisPostUpdateSet::ExternalPrepare)
                    .before(sync_text_label_intrinsic_size),
            )
            .add_systems(
                UiSettlementSchedule,
                sync_text_label_intrinsic_size
                    .in_set(UnivisPostUpdateSet::ExternalPrepare)
                    .before(fit_node_to_text_size),
            )
            .add_systems(
                UiSettlementSchedule,
                fit_node_to_text_size
                    .in_set(UnivisPostUpdateSet::ExternalPrepare)
                    .before(mark_text_label_layout_dirty),
            )
            .add_systems(
                UiSettlementSchedule,
                mark_text_label_layout_dirty
                    .in_set(UnivisPostUpdateSet::ExternalPrepare)
                    .before(sync_text_label_meshes),
            )
            .add_systems(
                UiSettlementSchedule,
                sync_text_label_meshes
                    .in_set(UnivisPostUpdateSet::RenderSync)
                    .after(univis_ui_engine::layout::layout_system::sync_cached_ui3d),
            )
            .add_systems(
                UiSettlementSchedule,
                sync_text_clipper_materials.in_set(UnivisPostUpdateSet::RenderSync),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_quad_to_rect_clips_size_and_uvs() {
        let quad = TextGlyphQuad {
            center: Vec2::new(0.0, 0.0),
            size: Vec2::new(10.0, 6.0),
            uv_min: Vec2::new(0.1, 0.2),
            uv_max: Vec2::new(0.5, 0.8),
        };
        let clip_rect = LocalClipRect {
            min: Vec2::new(-2.0, -3.0),
            max: Vec2::new(5.0, 3.0),
        };

        let clipped = clip_quad_to_rect(quad, clip_rect).expect("quad should be clipped");

        assert_eq!(clipped.size, Vec2::new(7.0, 6.0));
        assert_eq!(clipped.center, Vec2::new(1.5, 0.0));
        assert!((clipped.uv_min.x - 0.22).abs() < 0.001);
        assert!((clipped.uv_max.x - 0.5).abs() < 0.001);
    }

    #[test]
    fn clip_quad_to_rect_returns_none_when_outside() {
        let quad = TextGlyphQuad {
            center: Vec2::new(20.0, 0.0),
            size: Vec2::new(4.0, 4.0),
            uv_min: Vec2::ZERO,
            uv_max: Vec2::ONE,
        };
        let clip_rect = LocalClipRect {
            min: Vec2::new(-5.0, -5.0),
            max: Vec2::new(5.0, 5.0),
        };

        assert!(clip_quad_to_rect(quad, clip_rect).is_none());
    }

    #[test]
    fn desired_intrinsic_size_uses_content_dimensions_only() {
        let node = UNode {
            width: UVal::Content,
            height: UVal::Px(48.0),
            padding: USides::axes(6.0, 4.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            min_content_size: Vec2::new(48.0, 14.0),
            max_content_size: Vec2::new(120.0, 22.0),
            ..default()
        };
        let current = IntrinsicSize {
            width: 1.0,
            height: 48.0,
            ..default()
        };

        let desired = desired_text_label_intrinsic_size(&node, &layout_cache, current);

        assert_eq!(desired.width, 132.0);
        assert_eq!(desired.height, 48.0);
        assert_eq!(desired.min_width, 60.0);
        assert_eq!(desired.max_width, 132.0);
    }

    #[test]
    fn desired_intrinsic_size_uses_min_content_dimensions_when_requested() {
        let node = UNode {
            width: UVal::MinContent,
            height: UVal::MinContent,
            padding: USides::axes(6.0, 4.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            min_content_size: Vec2::new(48.0, 14.0),
            max_content_size: Vec2::new(120.0, 22.0),
            ..default()
        };

        let desired =
            desired_text_label_intrinsic_size(&node, &layout_cache, IntrinsicSize::default());

        assert_eq!(desired.width, 60.0);
        assert_eq!(desired.height, 22.0);
        assert_eq!(desired.min_width, 60.0);
        assert_eq!(desired.max_width, 132.0);
        assert_eq!(desired.min_height, 22.0);
        assert_eq!(desired.max_height, 30.0);
    }

    #[test]
    fn measured_text_outer_size_includes_padding_for_empty_text() {
        let node = UNode {
            padding: USides::axes(8.0, 10.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            measured_size: Vec2::ZERO,
            ..default()
        };

        let outer = measured_text_outer_size(&node, &layout_cache);

        assert_eq!(outer, Vec2::new(16.0, 20.0));
    }

    #[test]
    fn grapheme_boundaries_keep_joined_emoji_together() {
        let boundaries = text_char_boundaries("A🧑‍💻B");

        assert_eq!(boundaries.len(), 4);
        assert_eq!(&"A🧑‍💻B"[..boundaries[2]], "A🧑‍💻");
    }

    #[test]
    fn base_direction_detects_rtl_text() {
        assert_eq!(
            base_direction_for_text("مرحبا بالعالم"),
            TextBaseDirection::Rtl
        );
        assert_eq!(
            base_direction_for_text("hello world"),
            TextBaseDirection::Ltr
        );
    }

    #[test]
    fn auto_truncate_side_defaults_to_end_for_ltr_and_rtl() {
        assert_eq!(
            resolve_truncate_side(UTextTruncateSide::Auto, TextBaseDirection::Ltr),
            UTextTruncateSide::End
        );
        assert_eq!(
            resolve_truncate_side(UTextTruncateSide::Auto, TextBaseDirection::Rtl),
            UTextTruncateSide::End
        );
    }

    #[test]
    fn default_text_label_uses_ellipsis_overflow() {
        let label = UTextLabel::default();

        assert_eq!(label.overflow, UTextOverflow::Ellipsis);
    }

    #[test]
    fn default_text_label_uses_auto_truncate_side() {
        let label = UTextLabel::default();

        assert_eq!(label.truncate_side, UTextTruncateSide::Auto);
    }

    #[test]
    fn left_justify_offsets_text_toward_start_edge() {
        let label = UTextLabel {
            justify: Justify::Left,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(200.0, 20.0));

        assert_eq!(offset, 60.0);
    }

    #[test]
    fn right_justify_offsets_text_toward_end_edge() {
        let label = UTextLabel {
            justify: Justify::Right,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(40.0, 20.0));

        assert_eq!(offset, 20.0);
    }

    #[test]
    fn center_justify_keeps_zero_offset() {
        let label = UTextLabel {
            justify: Justify::Center,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(40.0, 20.0));

        assert_eq!(offset, 0.0);
    }

    #[test]
    fn local_clip_rect_exists_for_ellipsis_even_with_autosize() {
        let label = UTextLabel {
            autosize: true,
            overflow: UTextOverflow::Ellipsis,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 6.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 120.0,
            height: 40.0,
            ..default()
        };

        let clip = label_content_clip_rect(&label, &node, &computed_size).unwrap();

        assert_eq!(clip.min, Vec2::new(-50.0, -14.0));
        assert_eq!(clip.max, Vec2::new(50.0, 14.0));
    }

    #[test]
    fn visible_overflow_disables_local_clip_rect() {
        let label = UTextLabel {
            overflow: UTextOverflow::Visible,
            ..default()
        };
        let node = UNode {
            padding: USides::all(8.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 120.0,
            height: 40.0,
            ..default()
        };

        assert!(label_content_clip_rect(&label, &node, &computed_size).is_none());
    }

    #[test]
    fn autosize_measure_bounds_use_parent_constraints_when_available() {
        let label = UTextLabel {
            autosize: true,
            overflow: UTextOverflow::Ellipsis,
            ..default()
        };
        let node = UNode::default();
        let bounds = label_measure_bounds(
            &label,
            &node,
            None,
            TextBounds {
                width: Some(180.0),
                height: Some(44.0),
            },
        );

        assert_eq!(bounds.width, Some(180.0));
        assert_eq!(bounds.height, Some(44.0));
    }

    #[test]
    fn truncate_candidate_end_keeps_prefix() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::End,
            3,
            TextBaseDirection::Ltr,
        );

        assert_eq!(candidate, format!("{LTR_ISOLATE_START}abc{ISOLATE_END}..."));
    }

    #[test]
    fn truncate_candidate_start_keeps_suffix() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::Start,
            3,
            TextBaseDirection::Ltr,
        );

        assert_eq!(candidate, format!("...{LTR_ISOLATE_START}def{ISOLATE_END}"));
    }

    #[test]
    fn truncate_candidate_middle_keeps_edges() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::Middle,
            4,
            TextBaseDirection::Ltr,
        );

        assert_eq!(
            candidate,
            format!("{LTR_ISOLATE_START}ab{ISOLATE_END}...{LTR_ISOLATE_START}ef{ISOLATE_END}")
        );
    }
}
