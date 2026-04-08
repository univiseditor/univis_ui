use super::*;

pub fn post_settle_picking_backend(
    rollout: Option<Res<UiRolloutConfig>>,
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform)>,
    root_query: Query<(&ResolvedRootUi, &ResolvedRootStack)>,
    nodes_query: Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&LayoutDepth>,
            Option<&USelf>,
            Option<&CachedUiContext>,
        ),
        With<UInteraction>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    work_state: Option<Res<UiWorkState>>,
    mut validation_state: ResMut<UiValidationState>,
    mut sync_state: ResMut<PickingSyncState>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut output: MessageWriter<PointerHits>,
) {
    let rollout = rollout
        .as_deref()
        .cloned()
        .unwrap_or_else(UiRolloutConfig::default);
    let current_ui_generation = work_state
        .as_ref()
        .map_or(0, |value| value.current_generation());
    let geometry_pending = work_state.as_ref().is_some_and(|value| {
        let pending = value.pending();
        pending.root_resolve || pending.hierarchy || pending.measure || pending.solve
    });

    if geometry_pending {
        return;
    }

    let validation_enabled = rollout.validation != UiValidationMode::Disabled;
    if !rollout.use_post_settle_picking && !validation_enabled {
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
        &root_query,
        &nodes_query,
        &parents_query,
        &clipper_query,
        rollout.use_cached_ui_context,
        profiler.as_deref_mut(),
    );

    if validation_enabled {
        let legacy_hits = collect_pointer_hits(
            &pointers,
            &cameras,
            &root_query,
            &nodes_query,
            &parents_query,
            &clipper_query,
            false,
            None,
        );
        let shadow_hits = collect_pointer_hits(
            &pointers,
            &cameras,
            &root_query,
            &nodes_query,
            &parents_query,
            &clipper_query,
            true,
            None,
        );
        let mismatch_count = count_pointer_hit_mismatches(&legacy_hits, &shadow_hits);
        validation_state.record_picking_shadow_check(
            current_ui_generation,
            sync_state.pointer_generation,
            mismatch_count,
        );

        if mismatch_count > 0
            && validation_state.picking_shadow_warning
                != Some((current_ui_generation, sync_state.pointer_generation))
        {
            bevy::log::warn!(
                "Univis picking shadow validation mismatch detected (ui_generation={}, pointer_generation={}, mismatches={})",
                current_ui_generation,
                sync_state.pointer_generation,
                mismatch_count
            );
            validation_state.picking_shadow_warning =
                Some((current_ui_generation, sync_state.pointer_generation));
        }
    }

    if rollout.use_post_settle_picking {
        write_pointer_hits(&current_hits, &mut output);
    }

    sync_state.settled_ui_generation = current_ui_generation;
    sync_state.settled_pointer_generation = sync_state.pointer_generation;
}
