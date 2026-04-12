use super::*;
use univis_ui_engine::schedule::UiPickingRuntimeState;

pub fn post_settle_picking_backend(
    picking_runtime: Option<Res<UiPickingRuntimeState>>,
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform)>,
    nodes_query: Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&UiPickingContext>,
        ),
        With<UInteraction>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    mut validation_state: ResMut<PickingValidationState>,
    mut sync_state: ResMut<PickingSyncState>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut output: MessageWriter<PointerHits>,
) {
    let picking_runtime = picking_runtime
        .as_ref()
        .map_or_else(UiPickingRuntimeState::default, |value| {
            value.as_ref().clone()
        });
    let current_ui_generation = picking_runtime.current_ui_generation();
    let geometry_pending = picking_runtime.geometry_pending();

    if geometry_pending {
        return;
    }

    let validation_enabled = picking_runtime.validation_enabled();
    if !picking_runtime.post_settle_picking_enabled() && !validation_enabled {
        return;
    }

    if current_ui_generation == sync_state.settled_ui_generation
        && sync_state.pointer_generation == sync_state.settled_pointer_generation
    {
        return;
    }

    let current_hits = collect_pointer_hits(
        &pointers,
        &cameras,
        &nodes_query,
        &parents_query,
        &clipper_query,
        profiler.as_deref_mut(),
    );

    if validation_enabled {
        let mismatch_count = count_missing_picking_contexts(&nodes_query);
        validation_state.record_missing_context_check(
            current_ui_generation,
            sync_state.pointer_generation,
            mismatch_count,
        );

        if validation_state.should_warn_missing_contexts(
            current_ui_generation,
            sync_state.pointer_generation,
            mismatch_count,
        ) {
            bevy::log::warn!(
                "Univis picking context validation found nodes missing `UiPickingContext` (ui_generation={}, pointer_generation={}, mismatches={})",
                current_ui_generation,
                sync_state.pointer_generation,
                mismatch_count
            );
            validation_state
                .mark_missing_context_warning(current_ui_generation, sync_state.pointer_generation);
        }
    }

    if picking_runtime.post_settle_picking_enabled() {
        write_pointer_hits(&current_hits, &mut output);
    }

    sync_state.settled_ui_generation = current_ui_generation;
    sync_state.settled_pointer_generation = sync_state.pointer_generation;
}
