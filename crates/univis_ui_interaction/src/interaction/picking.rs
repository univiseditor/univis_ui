#![allow(clippy::type_complexity)]

mod backend;
mod clip;
mod hit_math;
mod root_lookup;
#[cfg(test)]
mod tests;
mod validation;

use bevy::picking::backend::prelude::{HitData, PointerHits, PointerLocation};
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use std::collections::HashMap;

pub use self::backend::{track_pointer_generation, univis_picking_backend};
pub use self::validation::post_settle_picking_backend;

#[cfg(test)]
use self::clip::is_clipped_by_ancestors;
use self::clip::is_clipped_by_picking_context;
use self::hit_math::{CachedPointerRay, intersect_ray_with_node_plane, pointer_ray_for_camera};
use self::root_lookup::is_ancestor_of;
use crate::interaction::feedback::UInteraction;
use univis_ui_engine::layout::profiling::LayoutProfiler;
use univis_ui_engine::layout::query::{ComputedSize, UiPickingContext};
use univis_ui_engine::layout::univis_node::{UClip, UNode};

#[derive(Resource, Default)]
pub struct PickingSyncState {
    pub pointer_generation: u64,
    settled_pointer_generation: u64,
    settled_ui_generation: u64,
}

/// Validation diagnostics for interaction-owned picking checks.
#[derive(Resource, Debug, Clone, Default)]
pub struct PickingValidationState {
    ui_generation: u64,
    pointer_generation: u64,
    missing_context_count: usize,
    warning_generation: Option<(u64, u64)>,
}

impl PickingValidationState {
    pub fn record_missing_context_check(
        &mut self,
        ui_generation: u64,
        pointer_generation: u64,
        missing_context_count: usize,
    ) {
        self.ui_generation = ui_generation;
        self.pointer_generation = pointer_generation;
        self.missing_context_count = missing_context_count;
    }

    pub fn ui_generation(&self) -> u64 {
        self.ui_generation
    }

    pub fn pointer_generation(&self) -> u64 {
        self.pointer_generation
    }

    pub fn missing_context_count(&self) -> usize {
        self.missing_context_count
    }

    pub fn should_warn_missing_contexts(
        &self,
        ui_generation: u64,
        pointer_generation: u64,
        missing_context_count: usize,
    ) -> bool {
        missing_context_count > 0
            && self.warning_generation != Some((ui_generation, pointer_generation))
    }

    pub fn mark_missing_context_warning(&mut self, ui_generation: u64, pointer_generation: u64) {
        self.warning_generation = Some((ui_generation, pointer_generation));
    }
}

#[derive(Clone)]
struct RankedHit {
    entity: Entity,
    hit_data: HitData,
    root_sort_key: f32,
    local_depth_key: f32,
    hit_distance: f32,
}

#[derive(Default)]
struct CameraHitBucket {
    order: f32,
    hits: Vec<RankedHit>,
}

#[derive(Clone)]
struct PreparedPickingCandidate {
    entity: Entity,
    global_transform: GlobalTransform,
    half_size: Vec2,
    radius_vec: Vec4,
    picking_context: UiPickingContext,
    root_sort_key: f32,
    local_depth_key: f32,
}

#[derive(Default)]
struct PreparedCameraBucket {
    candidates: Vec<PreparedPickingCandidate>,
}

pub(super) fn emit_pointer_hits(
    pointers: &Query<(&PointerId, &PointerLocation)>,
    cameras: &Query<(Entity, &Camera, &GlobalTransform)>,
    nodes_query: &Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&UiPickingContext>,
        ),
        With<UInteraction>,
    >,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    profiler: Option<&mut LayoutProfiler>,
    output: &mut MessageWriter<PointerHits>,
) {
    let hits = collect_pointer_hits(
        pointers,
        cameras,
        nodes_query,
        parents_query,
        clipper_query,
        profiler,
    );
    write_pointer_hits(&hits, output);
}

fn build_picking_candidate_buckets(
    nodes_query: &Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&UiPickingContext>,
        ),
        With<UInteraction>,
    >,
) -> HashMap<Entity, PreparedCameraBucket> {
    let mut buckets: HashMap<Entity, PreparedCameraBucket> = HashMap::new();

    for (entity, node, global_transform, size, picking_context) in nodes_query.iter() {
        let Some(picking_context) = picking_context.copied() else {
            continue;
        };
        let Some(camera_entity) = picking_context.camera_entity else {
            continue;
        };

        buckets
            .entry(camera_entity)
            .or_default()
            .candidates
            .push(PreparedPickingCandidate {
                entity,
                global_transform: *global_transform,
                half_size: Vec2::new(size.width, size.height) * 0.5 * picking_context.world_scale,
                radius_vec: Vec4::new(
                    node.border_radius.top_right,
                    node.border_radius.bottom_right,
                    node.border_radius.top_left,
                    node.border_radius.bottom_left,
                ) * picking_context.world_scale,
                picking_context,
                root_sort_key: picking_context.root_sort_key,
                local_depth_key: picking_context.local_depth_key,
            });
    }

    buckets
}

pub(super) fn count_missing_picking_contexts(
    nodes_query: &Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&UiPickingContext>,
        ),
        With<UInteraction>,
    >,
) -> usize {
    nodes_query
        .iter()
        .filter(|(_, _, _, _, picking_context)| picking_context.is_none())
        .count()
}

pub(super) fn collect_pointer_hits(
    pointers: &Query<(&PointerId, &PointerLocation)>,
    cameras: &Query<(Entity, &Camera, &GlobalTransform)>,
    nodes_query: &Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&UiPickingContext>,
        ),
        With<UInteraction>,
    >,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    profiler: Option<&mut LayoutProfiler>,
) -> Vec<(PointerId, Vec<(Entity, HitData)>, f32)> {
    let candidate_buckets = build_picking_candidate_buckets(nodes_query);
    if let Some(profiler) = profiler {
        profiler.picking_bucket_count = candidate_buckets.len();
        profiler.picking_candidate_count = candidate_buckets
            .values()
            .map(|bucket| bucket.candidates.len())
            .sum();
    }

    let mut output_hits = Vec::new();

    for (pointer_id, pointer_loc) in pointers.iter() {
        let Some(location) = pointer_loc.location() else {
            continue;
        };

        let mut ray_cache: HashMap<Entity, Option<CachedPointerRay>> = HashMap::new();
        let mut hits_by_camera: HashMap<Entity, CameraHitBucket> = HashMap::new();

        for (camera_entity, bucket) in candidate_buckets.iter() {
            let ray = ray_cache
                .entry(*camera_entity)
                .or_insert_with(|| pointer_ray_for_camera(location, *camera_entity, cameras))
                .as_ref();
            let Some(ray) = ray else {
                continue;
            };

            for candidate in bucket.candidates.iter() {
                let Some((hit_world, cursor_pos_local, hit_normal, hit_distance)) =
                    intersect_ray_with_node_plane(ray, &candidate.global_transform)
                else {
                    continue;
                };

                let dist = crate::interaction::math::sd_rounded_box(
                    cursor_pos_local,
                    candidate.half_size,
                    candidate.radius_vec,
                );

                if dist <= 0.0 {
                    if is_clipped_by_picking_context(
                        Some(&candidate.picking_context),
                        candidate.entity,
                        hit_world,
                        parents_query,
                        clipper_query,
                    ) {
                        continue;
                    }

                    hits_by_camera
                        .entry(*camera_entity)
                        .or_insert_with(|| CameraHitBucket {
                            order: ray.order,
                            ..default()
                        })
                        .hits
                        .push(RankedHit {
                            entity: candidate.entity,
                            hit_data: HitData::new(
                                *camera_entity,
                                hit_distance.max(0.0),
                                Some(hit_world),
                                Some(hit_normal),
                            ),
                            root_sort_key: candidate.root_sort_key,
                            local_depth_key: candidate.local_depth_key,
                            hit_distance,
                        });
                }
            }
        }

        for bucket in hits_by_camera.into_values() {
            let mut ranked_hits = bucket.hits;
            ranked_hits.sort_by(|left, right| {
                right
                    .root_sort_key
                    .total_cmp(&left.root_sort_key)
                    .then_with(|| right.local_depth_key.total_cmp(&left.local_depth_key))
                    .then_with(|| left.hit_distance.total_cmp(&right.hit_distance))
            });

            let mut filtered_hits: Vec<(Entity, HitData)> = Vec::new();

            for ranked_hit in ranked_hits {
                let mut should_include = true;

                for (other_entity, _) in filtered_hits.iter() {
                    if is_ancestor_of(ranked_hit.entity, *other_entity, parents_query)
                        || is_ancestor_of(*other_entity, ranked_hit.entity, parents_query)
                    {
                        should_include = false;
                        break;
                    }
                }

                if should_include {
                    let mut hit_data = ranked_hit.hit_data.clone();
                    hit_data.depth = filtered_hits.len() as f32;
                    filtered_hits.push((ranked_hit.entity, hit_data));
                }
            }

            if !filtered_hits.is_empty() {
                output_hits.push((*pointer_id, filtered_hits, bucket.order));
            }
        }
    }

    output_hits
}

pub(super) fn write_pointer_hits(
    hits: &[(PointerId, Vec<(Entity, HitData)>, f32)],
    output: &mut MessageWriter<PointerHits>,
) {
    for (pointer_id, picks, order) in hits.iter() {
        output.write(PointerHits::new(*pointer_id, picks.clone(), *order));
    }
}
