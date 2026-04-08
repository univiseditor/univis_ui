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
use self::clip::is_clipped_by_cached_context;
use self::hit_math::{CachedPointerRay, intersect_ray_with_node_plane, pointer_ray_for_camera};
use self::root_lookup::{is_ancestor_of, resolve_root_for_entity};
use crate::interaction::feedback::UInteraction;
use univis_ui_engine::layout::components::{CachedUiContext, LayoutDepth};
use univis_ui_engine::layout::geometry::ComputedSize;
use univis_ui_engine::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};
use univis_ui_engine::layout::profiling::LayoutProfiler;
use univis_ui_engine::layout::univis_node::{UClip, UNode, USelf};
use univis_ui_engine::schedule::{
    UiRolloutConfig, UiValidationMode, UiValidationState, UiWorkState,
};

#[derive(Resource, Default)]
pub struct PickingSyncState {
    pub pointer_generation: u64,
    settled_pointer_generation: u64,
    settled_ui_generation: u64,
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
    cached_context: Option<CachedUiContext>,
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
    root_query: &Query<(&ResolvedRootUi, &ResolvedRootStack)>,
    nodes_query: &Query<
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
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    prefer_cached_context: bool,
    profiler: Option<&mut LayoutProfiler>,
    output: &mut MessageWriter<PointerHits>,
) {
    let hits = collect_pointer_hits(
        pointers,
        cameras,
        root_query,
        nodes_query,
        parents_query,
        clipper_query,
        prefer_cached_context,
        profiler,
    );
    write_pointer_hits(&hits, output);
}

fn build_picking_candidate_buckets(
    root_query: &Query<(&ResolvedRootUi, &ResolvedRootStack)>,
    nodes_query: &Query<
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
    parents_query: &Query<&ChildOf>,
    prefer_cached_context: bool,
) -> HashMap<Entity, PreparedCameraBucket> {
    let mut buckets: HashMap<Entity, PreparedCameraBucket> = HashMap::new();

    for (entity, node, global_transform, size, depth_comp, uself, cached_context) in
        nodes_query.iter()
    {
        let resolved_cached_context = if prefer_cached_context {
            cached_context.copied()
        } else {
            None
        };
        let Some(root_context) = resolve_root_for_entity(
            resolved_cached_context.as_ref(),
            entity,
            parents_query,
            root_query,
        ) else {
            continue;
        };
        let Some(camera_entity) = root_context.camera_entity else {
            continue;
        };

        let layout_depth = depth_comp.map(|d| d.0).unwrap_or(0);
        let order = uself.map(|value| value.order).unwrap_or(0);
        let local_depth_key = root_context.stack.local_depth_key(layout_depth, order);

        buckets
            .entry(camera_entity)
            .or_default()
            .candidates
            .push(PreparedPickingCandidate {
                entity,
                global_transform: *global_transform,
                half_size: Vec2::new(size.width, size.height) * 0.5,
                radius_vec: Vec4::new(
                    node.border_radius.top_right,
                    node.border_radius.bottom_right,
                    node.border_radius.top_left,
                    node.border_radius.bottom_left,
                ),
                cached_context: resolved_cached_context,
                root_sort_key: root_context.stack.capsule_sort_key,
                local_depth_key,
            });
    }

    buckets
}

pub(super) fn collect_pointer_hits(
    pointers: &Query<(&PointerId, &PointerLocation)>,
    cameras: &Query<(Entity, &Camera, &GlobalTransform)>,
    root_query: &Query<(&ResolvedRootUi, &ResolvedRootStack)>,
    nodes_query: &Query<
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
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    prefer_cached_context: bool,
    profiler: Option<&mut LayoutProfiler>,
) -> Vec<(PointerId, Vec<(Entity, HitData)>, f32)> {
    let candidate_buckets = build_picking_candidate_buckets(
        root_query,
        nodes_query,
        parents_query,
        prefer_cached_context,
    );
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
                    if is_clipped_by_cached_context(
                        candidate.cached_context.as_ref(),
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

pub(super) fn count_pointer_hit_mismatches(
    left: &[(PointerId, Vec<(Entity, HitData)>, f32)],
    right: &[(PointerId, Vec<(Entity, HitData)>, f32)],
) -> usize {
    if left.len() != right.len() {
        return left.len().abs_diff(right.len()).max(1);
    }

    left.iter()
        .zip(right.iter())
        .map(
            |((left_pointer, left_hits, _), (right_pointer, right_hits, _))| {
                let pointer_mismatch = usize::from(left_pointer != right_pointer);
                let len_mismatch = left_hits.len().abs_diff(right_hits.len());
                let entity_mismatches = left_hits
                    .iter()
                    .zip(right_hits.iter())
                    .filter(|((left_entity, _), (right_entity, _))| left_entity != right_entity)
                    .count();

                pointer_mismatch + len_mismatch + entity_mismatches
            },
        )
        .sum()
}
