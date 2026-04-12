use bevy::asset::AssetEventSystems;
use bevy::prelude::*;
use bevy::sprite::update_text2d_layout;

use crate::schedule::{
    UiPickingRuntimeState, UiRolloutConfig, UiSettlementConfig, UiSettlementRuntimeState,
    UiSettlementSchedule, UiValidationState, UiWorkState, UnivisPostUpdateSet,
    sync_settlement_runtime_state,
};

use super::{components, core, invalidation, layout_system, query, settlement_loop, univis_node};

pub(super) fn register_layout_types(app: &mut App) {
    app.register_type::<univis_node::USelf>()
        .register_type::<layout_system::URootUi>()
        .register_type::<layout_system::UiRootSettlementState>()
        .register_type::<layout_system::UiSpace>()
        .register_type::<layout_system::UiCanvasSize>()
        .register_type::<layout_system::UiCameraRef>()
        .register_type::<query::UiPickingContext>()
        .register_type::<univis_node::UAlignSelf>()
        .register_type::<univis_node::UPosition>()
        .register_type::<univis_node::ULayoutContainerExt>()
        .register_type::<univis_node::ULayoutBoxAlignContainer>()
        .register_type::<univis_node::ULayoutFlexContainer>()
        .register_type::<univis_node::ULayoutGridContainer>()
        .register_type::<univis_node::ULayoutItemExt>()
        .register_type::<univis_node::ULayoutBoxAlignSelf>()
        .register_type::<univis_node::ULayoutFlexItem>()
        .register_type::<univis_node::ULayoutGridItem>()
        .register_type::<univis_node::UAlignSelfExt>()
        .register_type::<univis_node::UAlignItemsExt>()
        .register_type::<univis_node::UContentAlignExt>()
        .register_type::<univis_node::UOverflowPosition>()
        .register_type::<univis_node::UFlexWrap>()
        .register_type::<univis_node::UTrackSize>()
        .register_type::<univis_node::UGridAutoFlow>();
}

pub(super) fn install_layout_pipeline(app: &mut App) {
    app.init_resource::<components::LayoutTreeDepth>()
        .init_resource::<UiRolloutConfig>()
        .init_resource::<UiPickingRuntimeState>()
        .init_resource::<UiSettlementRuntimeState>()
        .init_resource::<UiValidationState>()
        .init_resource::<UiSettlementConfig>()
        .init_resource::<UiWorkState>()
        .init_resource::<invalidation::UiInvalidateRequestQueue>()
        .init_resource::<layout_system::RootSpawnRankCounter>()
        .init_schedule(UiSettlementSchedule)
        .add_plugins(core::layout_cache::UnivisLayoutCachePlugin)
        .configure_sets(
            UiSettlementSchedule,
            (
                UnivisPostUpdateSet::ExternalPrepare,
                UnivisPostUpdateSet::RootResolve,
                UnivisPostUpdateSet::LayoutHierarchy,
                UnivisPostUpdateSet::LayoutMeasure,
                UnivisPostUpdateSet::LayoutSolve,
                UnivisPostUpdateSet::RenderSync,
                UnivisPostUpdateSet::ExternalPostSolve,
                UnivisPostUpdateSet::UiSettled,
            )
                .chain(),
        )
        .add_systems(
            UiSettlementSchedule,
            (
                core::layout_cache::apply_external_invalidation_requests,
                settlement_loop::begin_ui_settlement_work,
                layout_system::resolve_root_ui,
                layout_system::assign_root_spawn_ranks,
                layout_system::resolve_root_stacking,
                layout_system::sync_root_capsule_transforms,
                settlement_loop::mark_root_resolve_complete,
            )
                .chain()
                .in_set(UnivisPostUpdateSet::RootResolve),
        )
        .add_systems(
            UiSettlementSchedule,
            (
                core::hierarchy::update_layout_hierarchy,
                core::hierarchy::update_cached_ui_contexts,
                core::hierarchy::update_picking_contexts,
                core::layout_cache::update_depth_cache,
                core::layout_cache::track_root_layout_changes,
                core::layout_cache::track_layout_changes,
                settlement_loop::mark_hierarchy_complete,
            )
                .chain()
                .in_set(UnivisPostUpdateSet::LayoutHierarchy),
        )
        .add_systems(
            UiSettlementSchedule,
            (
                core::pass_up::upward_measure_pass_cached,
                layout_system::sync_fit_content_root_canvas_sizes,
            )
                .chain()
                .in_set(UnivisPostUpdateSet::LayoutMeasure),
        )
        .add_systems(
            UiSettlementSchedule,
            (core::pass_down::downward_solve_pass_safe,)
                .chain()
                .in_set(UnivisPostUpdateSet::LayoutSolve),
        )
        .add_systems(
            PostUpdate,
            settlement_loop::run_ui_settlement_loop
                .after(update_text2d_layout)
                .before(AssetEventSystems),
        )
        .add_systems(
            UiSettlementSchedule,
            core::layout_cache::track_render_stage_changes
                .in_set(UnivisPostUpdateSet::RenderSync)
                .after(UnivisPostUpdateSet::LayoutSolve)
                .before(layout_system::sync_cached_ui3d),
        )
        .add_systems(
            UiSettlementSchedule,
            (
                settlement_loop::refresh_root_settlement_state,
                settlement_loop::mark_measure_complete,
                settlement_loop::mark_solve_complete,
                settlement_loop::mark_render_complete,
                core::hierarchy::validate_cached_ui_contexts,
                sync_settlement_runtime_state,
            )
                .chain()
                .in_set(UnivisPostUpdateSet::UiSettled),
        );
}
