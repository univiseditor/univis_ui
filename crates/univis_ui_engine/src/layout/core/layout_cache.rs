#![allow(clippy::type_complexity)]

mod invalidation;
#[cfg(test)]
mod tests;

use bevy::{ecs::relationship::Relationship, platform::collections::*, prelude::*};

use crate::internal::MaterialPool;
use crate::layout::components::*;
use crate::layout::geometry::ComputedSize;
use crate::layout::image::UImage;
use crate::layout::invalidation::{
    UiInvalidateRequestQueue, UiLayoutInvalidation as LayoutInvalidation,
};
use crate::layout::layout_system::*;
use crate::layout::pbr::UPbr;
use crate::layout::univis_node::*;
use crate::schedule::*;

use self::invalidation::{
    apply_layout_invalidation, classify_children_mutation, classify_layout_mutation,
    classify_node_mutation, classify_uself_mutation, mark_render_dirty_recursive,
    root_canvas_changed, root_projection_context_changed, root_render_context_changed,
};

/// Cache resource used to avoid repeated layout work across frames.
#[derive(Resource, Default)]
pub struct LayoutCache {
    /// Cached intrinsic sizes per entity.
    intrinsic_sizes: HashMap<Entity, IntrinsicSize>,

    /// Per-node settlement generations for measure/solve/render stages.
    stage_versions: HashMap<Entity, UiNodeStageVersions>,

    /// Last observed `UNode` inputs used to classify mutations precisely.
    node_snapshots: HashMap<Entity, UNode>,

    /// Last observed `ULayout` inputs for each entity.
    layout_snapshots: HashMap<Entity, ULayout>,

    /// Last observed `USelf` inputs for each entity.
    self_snapshots: HashMap<Entity, USelf>,

    /// Last observed layout-participating child list for each entity.
    layout_child_snapshots: HashMap<Entity, Vec<Entity>>,

    /// Last observed resolved root state for each root entity.
    root_snapshots: HashMap<Entity, ResolvedRootUi>,

    /// Last observed descendant-facing root stacking state for each root entity.
    root_stack_snapshots: HashMap<Entity, ResolvedRootStack>,

    /// Last observed clip state for clip entities.
    clip_snapshots: HashMap<Entity, UClip>,

    /// Nodes that need to be recomputed.
    dirty_nodes: HashSet<Entity>,

    /// Frontier queue for the upward measure stage.
    measure_frontier: HashMap<Entity, u64>,

    /// Frontier queue for the downward solve stage.
    solve_frontier: HashMap<Entity, u64>,

    /// Frontier queue for render/material sync.
    render_frontier: HashMap<Entity, u64>,

    /// Entities grouped by tree depth.
    entities_by_depth: HashMap<usize, Vec<Entity>>,

    /// Last known depth for each live entity.
    entity_depths: HashMap<Entity, usize>,

    /// Running frame counter used for diagnostics.
    frame_count: u64,

    /// Last known maximum depth.
    last_max_depth: usize,
}

impl LayoutCache {
    /// Creates an empty layout cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds the entity-to-depth index.
    pub fn rebuild_depth_map(&mut self, query: &Query<(Entity, &LayoutDepth)>, max_depth: usize) {
        // Clear the old index before rebuilding from the current depth query.
        self.entities_by_depth.clear();
        self.entity_depths.clear();
        let mut alive_entities: HashSet<Entity> = HashSet::default();

        // Rebuild the depth buckets and remember which entities are still alive.
        for (entity, depth) in query.iter() {
            alive_entities.insert(entity);
            self.entity_depths.insert(entity, depth.0);
            self.entities_by_depth
                .entry(depth.0)
                .or_default()
                .push(entity);
        }

        self.intrinsic_sizes
            .retain(|entity, _| alive_entities.contains(entity));
        self.stage_versions
            .retain(|entity, _| alive_entities.contains(entity));
        self.node_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.layout_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.self_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.layout_child_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.root_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.root_stack_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.clip_snapshots
            .retain(|entity, _| alive_entities.contains(entity));
        self.dirty_nodes
            .retain(|entity| alive_entities.contains(entity));
        self.measure_frontier
            .retain(|entity, _| alive_entities.contains(entity));
        self.solve_frontier
            .retain(|entity, _| alive_entities.contains(entity));
        self.render_frontier
            .retain(|entity, _| alive_entities.contains(entity));

        self.last_max_depth = max_depth;
    }

    /// Returns all entities known for a depth bucket.
    pub fn get_entities_at_depth(&self, depth: usize) -> Option<&Vec<Entity>> {
        self.entities_by_depth.get(&depth)
    }

    /// Returns the cached depth for one entity, if known.
    pub fn depth_for_entity(&self, entity: Entity) -> Option<usize> {
        self.entity_depths.get(&entity).copied()
    }

    /// Returns all cached entities ordered from root to leaves.
    pub fn all_entities_top_down(&self) -> Vec<Entity> {
        let mut entities = Vec::new();

        for depth in 0..=self.last_max_depth {
            if let Some(layer) = self.entities_by_depth.get(&depth) {
                entities.extend(layer.iter().copied());
            }
        }

        entities
    }

    /// Returns all cached entities ordered from leaves to roots.
    pub fn all_entities_bottom_up(&self) -> Vec<Entity> {
        let mut entities = Vec::new();

        for depth in (0..=self.last_max_depth).rev() {
            if let Some(layer) = self.entities_by_depth.get(&depth) {
                entities.extend(layer.iter().copied());
            }
        }

        entities
    }

    /// Marks one entity as dirty.
    pub fn mark_dirty(&mut self, entity: Entity) {
        // The `HashSet` deduplicates repeated dirty marks automatically.
        self.dirty_nodes.insert(entity);
    }

    /// Marks an entity and all descendants as dirty.
    pub fn mark_dirty_recursive(&mut self, entity: Entity, children_query: &Query<&Children>) {
        // Stop early when the node was already marked to avoid redundant recursion.
        if !self.dirty_nodes.insert(entity) {
            return;
        }

        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                self.mark_dirty_recursive(child, children_query);
            }
        }
    }

    /// Marks all ancestors from the entity up to the root as dirty.
    pub fn mark_dirty_ancestors(&mut self, entity: Entity, parents_query: &Query<&ChildOf>) {
        let mut current = entity;

        while let Ok(parent) = parents_query.get(current) {
            current = parent.get();
            self.dirty_nodes.insert(current);
        }
    }

    /// Returns `true` when the entity is marked dirty.
    pub fn is_dirty(&self, entity: Entity) -> bool {
        self.dirty_nodes.contains(&entity)
    }

    /// Clears the dirty flag for one entity.
    pub fn clear_dirty(&mut self, entity: Entity) {
        self.dirty_nodes.remove(&entity);
    }

    /// Clears all dirty flags.
    pub fn clear_all_dirty(&mut self) {
        self.dirty_nodes.clear();
    }

    /// Seeds the downward solve frontier from the current dirty set.
    pub fn begin_solve_from_dirty(&mut self) {
        let entities: Vec<_> = self.dirty_nodes.iter().copied().collect();
        for entity in entities {
            let generation = self.stage_versions(entity).solve_input_generation;
            queue_frontier_entry(&mut self.solve_frontier, entity, generation);
        }
    }

    /// Returns `true` when the entity still needs downward solve work.
    pub fn is_solve_dirty(&self, entity: Entity) -> bool {
        self.solve_frontier.contains_key(&entity)
    }

    /// Marks one entity for downward solve work.
    pub fn mark_solve_dirty(&mut self, entity: Entity) {
        let generation = self.stage_versions(entity).solve_input_generation;
        queue_frontier_entry(&mut self.solve_frontier, entity, generation);
    }

    /// Clears the downward solve flag for one entity.
    pub fn clear_solve_dirty(&mut self, entity: Entity) {
        self.solve_frontier.remove(&entity);
    }

    /// Returns how many nodes are still queued for downward solve work.
    pub fn solve_dirty_count(&self) -> usize {
        self.solve_frontier.len()
    }

    /// Stores an intrinsic size entry.
    pub fn cache_intrinsic(&mut self, entity: Entity, size: IntrinsicSize) {
        self.intrinsic_sizes.insert(entity, size);
    }

    /// Returns the cached intrinsic size for an entity, if present.
    pub fn get_cached_intrinsic(&self, entity: Entity) -> Option<IntrinsicSize> {
        self.intrinsic_sizes.get(&entity).copied()
    }

    /// Returns the tracked stage versions for an entity, if any.
    pub fn stage_versions(&self, entity: Entity) -> UiNodeStageVersions {
        self.stage_versions
            .get(&entity)
            .copied()
            .unwrap_or_default()
    }

    fn stage_versions_mut(&mut self, entity: Entity) -> &mut UiNodeStageVersions {
        self.stage_versions.entry(entity).or_default()
    }

    pub fn update_node_snapshot(&mut self, entity: Entity, node: UNode) -> Option<UNode> {
        self.node_snapshots.insert(entity, node)
    }

    pub fn update_layout_snapshot(
        &mut self,
        entity: Entity,
        layout: Option<ULayout>,
    ) -> Option<ULayout> {
        match layout {
            Some(layout) => self.layout_snapshots.insert(entity, layout),
            None => self.layout_snapshots.remove(&entity),
        }
    }

    pub fn update_self_snapshot(&mut self, entity: Entity, uself: Option<USelf>) -> Option<USelf> {
        match uself {
            Some(uself) => self.self_snapshots.insert(entity, uself),
            None => self.self_snapshots.remove(&entity),
        }
    }

    pub fn layout_children_snapshot(&self, entity: Entity) -> Option<&[Entity]> {
        self.layout_child_snapshots.get(&entity).map(Vec::as_slice)
    }

    pub fn update_layout_children_snapshot(
        &mut self,
        entity: Entity,
        layout_children: Vec<Entity>,
    ) -> Option<Vec<Entity>> {
        if layout_children.is_empty() {
            self.layout_child_snapshots.remove(&entity)
        } else {
            self.layout_child_snapshots.insert(entity, layout_children)
        }
    }

    pub fn has_node_snapshot(&self, entity: Entity) -> bool {
        self.node_snapshots.contains_key(&entity)
    }

    pub fn update_root_snapshot(
        &mut self,
        entity: Entity,
        root: ResolvedRootUi,
    ) -> Option<ResolvedRootUi> {
        self.root_snapshots.insert(entity, root)
    }

    pub fn update_root_stack_snapshot(
        &mut self,
        entity: Entity,
        stack: Option<ResolvedRootStack>,
    ) -> Option<ResolvedRootStack> {
        match stack {
            Some(stack) => self.root_stack_snapshots.insert(entity, stack),
            None => self.root_stack_snapshots.remove(&entity),
        }
    }

    pub fn update_clip_snapshot(&mut self, entity: Entity, clip: Option<UClip>) -> Option<UClip> {
        match clip {
            Some(clip) => self.clip_snapshots.insert(entity, clip),
            None => self.clip_snapshots.remove(&entity),
        }
    }

    pub fn mark_measure_dirty(&mut self, entity: Entity, generation: u64) {
        self.stage_versions_mut(entity)
            .mark_measure_dirty(generation);
        queue_frontier_entry(&mut self.measure_frontier, entity, generation);
    }

    pub fn mark_measure_dirty_recursive(
        &mut self,
        entity: Entity,
        generation: u64,
        children_query: &Query<&Children>,
    ) {
        self.mark_measure_dirty(entity, generation);

        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                self.mark_measure_dirty_recursive(child, generation, children_query);
            }
        }
    }

    pub fn mark_measure_dirty_ancestors(
        &mut self,
        entity: Entity,
        generation: u64,
        parents_query: &Query<&ChildOf>,
    ) {
        let mut current = entity;

        while let Ok(parent) = parents_query.get(current) {
            current = parent.get();
            self.mark_measure_dirty(current, generation);
        }
    }

    pub fn complete_measure(&mut self, entity: Entity) {
        self.stage_versions_mut(entity).complete_measure();
        let done_generation = self.stage_versions(entity).measure_done_generation;
        drop_completed_frontier_entry(&mut self.measure_frontier, entity, done_generation);
    }

    /// Returns how many nodes are still queued for upward measure work.
    pub fn measure_dirty_count(&self) -> usize {
        self.measure_frontier.len()
    }

    pub fn mark_solve_dirty_generation(&mut self, entity: Entity, generation: u64) {
        self.stage_versions_mut(entity).mark_solve_dirty(generation);
        queue_frontier_entry(&mut self.solve_frontier, entity, generation);
    }

    pub fn mark_solve_dirty_recursive_generation(
        &mut self,
        entity: Entity,
        generation: u64,
        children_query: &Query<&Children>,
    ) {
        self.mark_solve_dirty_generation(entity, generation);

        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                self.mark_solve_dirty_recursive_generation(child, generation, children_query);
            }
        }
    }

    pub fn mark_solve_dirty_ancestors_generation(
        &mut self,
        entity: Entity,
        generation: u64,
        parents_query: &Query<&ChildOf>,
    ) {
        let mut current = entity;

        while let Ok(parent) = parents_query.get(current) {
            current = parent.get();
            self.mark_solve_dirty_generation(current, generation);
        }
    }

    /// Increments the frame counter.
    pub fn increment_frame(&mut self) {
        self.frame_count += 1;
    }

    /// Returns the current frame counter.
    pub fn current_frame(&self) -> u64 {
        self.frame_count
    }

    /// Returns how many entities are currently dirty.
    pub fn dirty_count(&self) -> usize {
        self.dirty_nodes.len()
    }

    /// Returns the percentage of dirty entities relative to `total_nodes`.
    pub fn dirty_ratio(&self, total_nodes: usize) -> f32 {
        if total_nodes == 0 {
            return 0.0;
        }
        (self.dirty_nodes.len() as f32 / total_nodes as f32) * 100.0
    }

    pub fn complete_solve(&mut self, entity: Entity) {
        self.stage_versions_mut(entity).complete_solve();
        let done_generation = self.stage_versions(entity).solve_done_generation;
        drop_completed_frontier_entry(&mut self.solve_frontier, entity, done_generation);
    }

    pub fn mark_render_dirty(&mut self, entity: Entity, generation: u64) {
        self.stage_versions_mut(entity)
            .mark_render_dirty(generation);
        queue_frontier_entry(&mut self.render_frontier, entity, generation);
    }

    pub fn complete_render(&mut self, entity: Entity) {
        self.stage_versions_mut(entity).complete_render();
        let done_generation = self.stage_versions(entity).render_done_generation;
        drop_completed_frontier_entry(&mut self.render_frontier, entity, done_generation);
    }

    /// Returns how many nodes are still queued for render synchronization.
    pub fn render_dirty_count(&self) -> usize {
        self.render_frontier.len()
    }

    pub fn take_measure_frontier(&mut self) -> Vec<Entity> {
        take_frontier_entities(
            &mut self.measure_frontier,
            &self.entity_depths,
            &self.stage_versions,
            FrontierOrder::BottomUp,
            |versions, generation| {
                generation == 0
                    || (versions.measure_input_generation == generation
                        && versions.measure_done_generation < versions.measure_input_generation)
            },
        )
    }

    pub fn take_solve_frontier(&mut self) -> Vec<Entity> {
        take_frontier_entities(
            &mut self.solve_frontier,
            &self.entity_depths,
            &self.stage_versions,
            FrontierOrder::TopDown,
            |versions, generation| {
                generation == 0
                    || (versions.solve_input_generation == generation
                        && versions.solve_done_generation < versions.solve_input_generation)
            },
        )
    }

    pub fn take_render_frontier(&mut self) -> Vec<Entity> {
        take_frontier_entities(
            &mut self.render_frontier,
            &self.entity_depths,
            &self.stage_versions,
            FrontierOrder::TopDown,
            |versions, generation| {
                generation == 0
                    || (versions.render_input_generation == generation
                        && versions.render_done_generation < versions.render_input_generation)
            },
        )
    }
}

#[derive(Clone, Copy)]
enum FrontierOrder {
    TopDown,
    BottomUp,
}

fn queue_frontier_entry(frontier: &mut HashMap<Entity, u64>, entity: Entity, generation: u64) {
    if frontier
        .get(&entity)
        .is_some_and(|queued| *queued >= generation)
    {
        return;
    }

    frontier.insert(entity, generation);
}

fn drop_completed_frontier_entry(
    frontier: &mut HashMap<Entity, u64>,
    entity: Entity,
    completed_generation: u64,
) {
    let Some(queued_generation) = frontier.get(&entity).copied() else {
        return;
    };

    if queued_generation == 0 || queued_generation <= completed_generation {
        frontier.remove(&entity);
    }
}

fn take_frontier_entities(
    frontier: &mut HashMap<Entity, u64>,
    entity_depths: &HashMap<Entity, usize>,
    stage_versions: &HashMap<Entity, UiNodeStageVersions>,
    order: FrontierOrder,
    is_pending: impl Fn(UiNodeStageVersions, u64) -> bool,
) -> Vec<Entity> {
    let mut entities: Vec<_> = core::mem::take(frontier)
        .into_iter()
        .filter_map(|(entity, generation)| {
            let depth = entity_depths.get(&entity).copied()?;
            let versions = stage_versions.get(&entity).copied().unwrap_or_default();
            is_pending(versions, generation).then_some((entity, depth))
        })
        .collect();

    entities.sort_unstable_by(|(left_entity, left_depth), (right_entity, right_depth)| {
        let depth_cmp = match order {
            FrontierOrder::TopDown => left_depth.cmp(right_depth),
            FrontierOrder::BottomUp => right_depth.cmp(left_depth),
        };

        depth_cmp.then_with(|| left_entity.to_bits().cmp(&right_entity.to_bits()))
    });

    entities.into_iter().map(|(entity, _)| entity).collect()
}

/// Internal system that tracks layout-affecting changes.
#[doc(hidden)]
pub fn track_layout_changes(
    material_pool: Option<Res<MaterialPool>>,
    work_state: Option<Res<UiWorkState>>,
    mut cache: ResMut<LayoutCache>,
    layout_nodes: Query<(), With<UNode>>,
    nodes: Query<
        (
            Entity,
            Option<Ref<Children>>,
            Ref<UNode>,
            Option<Ref<ULayout>>,
            Option<Ref<USelf>>,
        ),
        Or<(
            Changed<UNode>,
            Changed<ULayout>,
            Changed<USelf>,
            Changed<Children>,
        )>,
    >,

    added_nodes: Query<(Entity, &UNode, Option<&ULayout>, Option<&USelf>), Added<UNode>>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    let current_generation = work_state
        .as_ref()
        .map_or(0, |state| state.current_generation());

    for (entity, children, node, layout, uself) in nodes.iter() {
        let has_children = children.as_ref().is_some_and(|kids| !kids.is_empty());
        let has_parent = parents_query.get(entity).is_ok();
        let mut invalidation = LayoutInvalidation::default();

        if node.is_changed() {
            let current = (*node).clone();
            let previous = cache.update_node_snapshot(entity, current.clone());
            invalidation.merge(classify_node_mutation(
                previous.as_ref(),
                &current,
                has_children,
                has_parent,
            ));
        }

        if let Some(layout_ref) = layout {
            let current = (*layout_ref).clone();
            if layout_ref.is_changed() {
                let previous = cache.update_layout_snapshot(entity, Some(current.clone()));
                invalidation.merge(classify_layout_mutation(
                    previous.as_ref(),
                    &current,
                    has_children,
                    has_parent,
                ));
            }
        } else {
            cache.update_layout_snapshot(entity, None);
        }

        if let Some(uself_ref) = uself {
            let current = *uself_ref;
            if uself_ref.is_changed() {
                let previous = cache.update_self_snapshot(entity, Some(current));
                invalidation.merge(classify_uself_mutation(
                    previous.as_ref(),
                    &current,
                    has_children,
                    has_parent,
                ));
            }
        } else {
            cache.update_self_snapshot(entity, None);
        }

        if children.as_ref().is_some_and(|kids| kids.is_changed()) {
            let current_layout_children =
                collect_layout_children(children.as_deref(), &layout_nodes);
            let previous_layout_children =
                cache.update_layout_children_snapshot(entity, current_layout_children.clone());

            if layout_children_changed(
                previous_layout_children.as_deref(),
                &current_layout_children,
            ) {
                invalidation.merge(classify_children_mutation(has_parent));
            }
        }

        if material_pool.is_none() {
            invalidation.render_self = false;
        }

        apply_layout_invalidation(
            &mut cache,
            entity,
            current_generation,
            invalidation,
            &parents_query,
        );
    }

    for (entity, node, layout, uself) in added_nodes.iter() {
        cache.update_node_snapshot(entity, node.clone());
        cache.update_layout_snapshot(entity, layout.cloned());
        cache.update_self_snapshot(entity, uself.copied());
        cache.update_layout_children_snapshot(
            entity,
            collect_layout_children(children_query.get(entity).ok(), &layout_nodes),
        );
        cache.mark_dirty(entity);
        cache.mark_dirty_ancestors(entity, &parents_query);
        cache.mark_measure_dirty(entity, current_generation);
        cache.mark_measure_dirty_ancestors(entity, current_generation, &parents_query);
        cache.mark_solve_dirty_generation(entity, current_generation);
        cache.mark_solve_dirty_ancestors_generation(entity, current_generation, &parents_query);
    }
}

/// Applies external invalidation requests emitted by crates outside the engine.
#[doc(hidden)]
pub fn apply_external_invalidation_requests(
    work_state: Option<Res<UiWorkState>>,
    mut cache: ResMut<LayoutCache>,
    mut requests: ResMut<UiInvalidateRequestQueue>,
    parents_query: Query<&ChildOf>,
) {
    if requests.is_empty() {
        return;
    }

    let current_generation = work_state.as_ref().map_or(0, |state| {
        if state.pending().any() {
            state.current_generation()
        } else {
            state.current_generation().saturating_add(1)
        }
    });

    for (entity, invalidation) in requests.drain() {
        apply_layout_invalidation(
            &mut cache,
            entity,
            current_generation,
            invalidation,
            &parents_query,
        );
    }
}

/// Marks root-driven layout work so root/camera/capsule changes reach measure,
/// solve, and render even when no `UNode` fields changed directly.
#[doc(hidden)]
pub fn track_root_layout_changes(
    material_pool: Option<Res<MaterialPool>>,
    work_state: Option<Res<UiWorkState>>,
    mut cache: ResMut<LayoutCache>,
    changed_roots: Query<
        (Entity, Ref<ResolvedRootUi>, Option<Ref<ResolvedRootStack>>),
        (
            With<ResolvedRootUi>,
            Or<(Changed<ResolvedRootUi>, Changed<ResolvedRootStack>)>,
        ),
    >,
    children_query: Query<&Children>,
) {
    let current_generation = work_state
        .as_ref()
        .map_or(0, |state| state.current_generation());
    if current_generation == 0 {
        return;
    }

    for (entity, root, stack) in changed_roots.iter() {
        let previous_root = cache.update_root_snapshot(entity, *root);
        let previous_stack = cache.update_root_stack_snapshot(
            entity,
            stack
                .as_ref()
                .map(|stack| stack.descendant_context_snapshot()),
        );
        let mut solve_subtree = false;
        let mut render_subtree = false;

        if root.is_changed() {
            if let Some(previous_root) = previous_root {
                if root_canvas_changed(previous_root, *root) {
                    cache.mark_solve_dirty_generation(entity, current_generation);
                }

                if root_projection_context_changed(previous_root, *root) {
                    solve_subtree = true;
                    render_subtree = true;
                } else if root_render_context_changed(previous_root, *root) {
                    render_subtree = true;
                }
            } else {
                solve_subtree = true;
                render_subtree = true;
            }
        }

        if let Some(stack) = stack.as_ref()
            && stack.is_changed()
        {
            let current_stack = stack.descendant_context_snapshot();
            if previous_stack.is_none_or(|previous| previous != current_stack) {
                solve_subtree = true;
            }
        }

        if solve_subtree {
            cache.mark_solve_dirty_recursive_generation(
                entity,
                current_generation,
                &children_query,
            );
        }

        if render_subtree && material_pool.is_some() {
            mark_render_dirty_recursive(&mut cache, entity, current_generation, &children_query);
        }
    }
}

/// Tracks render-stage invalidation separately so node render sync only claims
/// completion after render-facing inputs actually changed.
#[doc(hidden)]
pub fn track_render_stage_changes(
    material_pool: Option<Res<MaterialPool>>,
    work_state: Option<Res<UiWorkState>>,
    mut cache: ResMut<LayoutCache>,
    all_nodes: Query<Entity, With<UNode>>,
    changed_nodes: Query<
        Entity,
        (
            With<UNode>,
            Or<(
                Added<UNode>,
                Changed<ComputedSize>,
                Added<UBorder>,
                Changed<UBorder>,
                Added<UImage>,
                Changed<UImage>,
                Added<UPbr>,
                Changed<UPbr>,
                Changed<ChildOf>,
            )>,
        ),
    >,
    changed_clips: Query<(Entity, Ref<UClip>), Or<(Added<UClip>, Changed<UClip>)>>,
    children_query: Query<&Children>,
    mut removed_clips: RemovedComponents<UClip>,
) {
    if material_pool.is_none() {
        return;
    }

    let current_generation = work_state
        .as_ref()
        .map_or(0, |state| state.current_generation());
    if current_generation == 0 {
        return;
    }

    if removed_clips.read().next().is_some() {
        for entity in all_nodes.iter() {
            cache.mark_render_dirty(entity, current_generation);
        }
        return;
    }

    for (entity, clip) in changed_clips.iter() {
        let current = (*clip).clone();
        let previous = cache.update_clip_snapshot(entity, Some(current.clone()));
        if previous.is_none_or(|previous| previous != current) {
            mark_render_dirty_recursive(&mut cache, entity, current_generation, &children_query);
        }
    }

    for entity in changed_nodes.iter() {
        cache.mark_render_dirty(entity, current_generation);
    }
}

pub(crate) fn collect_layout_children(
    children: Option<&Children>,
    layout_nodes: &Query<(), With<UNode>>,
) -> Vec<Entity> {
    children.map_or_else(Vec::new, |children| {
        children
            .iter()
            .filter(|child| layout_nodes.get(*child).is_ok())
            .collect()
    })
}

pub(crate) fn layout_children_changed(previous: Option<&[Entity]>, current: &[Entity]) -> bool {
    previous.unwrap_or(&[]) != current
}

/// Internal system that rebuilds the depth cache when the tree changes.
#[doc(hidden)]
pub fn update_depth_cache(
    mut cache: ResMut<LayoutCache>,
    tree_depth: Res<LayoutTreeDepth>,

    // Full depth query used when the cache must be rebuilt.
    depth_query: Query<(Entity, &LayoutDepth)>,

    // Track newly inserted depth components; they imply new nodes entered the tree.
    added_nodes: Query<Entity, Added<LayoutDepth>>,

    // Track removed depth components so deleted nodes are evicted from the cache.
    mut removed_nodes: RemovedComponents<LayoutDepth>,
) {
    // Rebuild when the structure changed, the max depth changed, or the cache is empty.
    let structure_changed = !added_nodes.is_empty() || removed_nodes.read().count() > 0;
    if structure_changed
        || tree_depth.max_depth != cache.last_max_depth
        || cache.entities_by_depth.is_empty()
    {
        cache.rebuild_depth_map(&depth_query, tree_depth.max_depth);
    }
}
/// Internal plugin that installs the layout cache systems.
#[doc(hidden)]
pub struct UnivisLayoutCachePlugin;

impl Plugin for UnivisLayoutCachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LayoutCache>();
    }
}

#[deprecated(note = "Use `UnivisLayoutCachePlugin` instead.")]
pub type LayoutCachePlugin = UnivisLayoutCachePlugin;
