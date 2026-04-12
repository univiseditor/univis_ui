use super::*;

pub fn univis_picking_backend(
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
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut output: MessageWriter<PointerHits>,
) {
    emit_pointer_hits(
        &pointers,
        &cameras,
        &nodes_query,
        &parents_query,
        &clipper_query,
        profiler.as_deref_mut(),
        &mut output,
    );
}

pub fn track_pointer_generation(
    changed_pointers: Query<(), Or<(Added<PointerLocation>, Changed<PointerLocation>)>>,
    mut removed_pointers: RemovedComponents<PointerLocation>,
    mut sync_state: ResMut<PickingSyncState>,
) {
    if !changed_pointers.is_empty() || removed_pointers.read().next().is_some() {
        sync_state.pointer_generation += 1;
    }
}
