#![allow(clippy::type_complexity)]

use super::math::sd_rounded_box;
use crate::internal_prelude::*;
use bevy::ecs::relationship::Relationship;
use bevy::picking::backend::prelude::*;
use bevy::picking::pointer::Location;
use bevy::prelude::*;
use std::collections::HashMap;
use univis_ui_engine::layout::profiling::LayoutProfiler;
use univis_ui_engine::schedule::UiWorkState;

#[derive(Resource, Default)]
pub struct PickingSyncState {
    pub pointer_generation: u64,
    settled_pointer_generation: u64,
    settled_ui_generation: u64,
}

// Accurate clipping check using ancestor transforms and rounded-box SDF tests.
fn is_clipped_by_ancestors(
    start_entity: Entity,
    cursor_world_pos: Vec3,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> bool {
    let mut current_entity = start_entity;

    // نصعد في شجرة الآباء
    while let Ok(parent) = parents_query.get(current_entity) {
        current_entity = parent.get();

        if let Ok((transform, size, node, clip)) = clipper_query.get(current_entity)
            && clip.enabled
        {
            // 1. التحويل من العالم (World) إلى المحلي (Local) الخاص بالأب القاطع
            let transform_matrix = transform.to_matrix();
            let inverse_matrix = transform_matrix.inverse();

            // تحويل النقطة
            let cursor_in_clipper_space =
                inverse_matrix.transform_point3(cursor_world_pos).truncate();

            // 2. حساب حدود القناع
            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            // 3. اختبار SDF
            let dist = sd_rounded_box(cursor_in_clipper_space, half_size, radius);

            if dist > 0.0 {
                return true; // نعم، العنصر مقصوص في هذه النقطة
            }
        }
    }

    false // غير مقصوص
}

fn is_clipped_by_cached_context(
    cached_context: Option<&CachedUiContext>,
    start_entity: Entity,
    cursor_world_pos: Vec3,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> bool {
    if let Some(clip_entity) = cached_context.and_then(|value| value.clip_ancestor)
        && let Ok((transform, size, node, clip)) = clipper_query.get(clip_entity)
        && clip.enabled
    {
        let cursor_in_clipper_space = transform
            .to_matrix()
            .inverse()
            .transform_point3(cursor_world_pos)
            .truncate();
        let half_size = Vec2::new(size.width, size.height) * 0.5;
        let radius = Vec4::new(
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        );
        return sd_rounded_box(cursor_in_clipper_space, half_size, radius) > 0.0;
    }

    is_clipped_by_ancestors(start_entity, cursor_world_pos, parents_query, clipper_query)
}

#[derive(Clone, Copy)]
struct CachedPointerRay {
    order: f32,
    origin: Vec3,
    direction: Vec3,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct ResolvedEntityRootContext {
    root_entity: Entity,
    space: UiSpace,
    camera_entity: Option<Entity>,
    stack: ResolvedRootStack,
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

// Returns `true` when `potential_ancestor` appears in the parent chain.
fn is_ancestor_of(
    potential_ancestor: Entity,
    potential_descendant: Entity,
    parents_query: &Query<&ChildOf>,
) -> bool {
    let mut current = potential_descendant;

    // نصعد في الشجرة حتى نجد الأب أو نصل للجذر
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if current == potential_ancestor {
            return true;
        }
    }

    false
}

fn resolve_root_for_entity(
    cached_context: Option<&CachedUiContext>,
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<(&ResolvedRootUi, &ResolvedRootStack)>,
) -> Option<ResolvedEntityRootContext> {
    if let Some(context) = cached_context
        && let Some(root_entity) = context.root_entity
    {
        return Some(ResolvedEntityRootContext {
            root_entity,
            space: context.space,
            camera_entity: context.camera_entity,
            stack: context.root_stack,
        });
    }

    let mut current = entity;

    loop {
        if let Ok((root, stack)) = root_query.get(current) {
            return Some(ResolvedEntityRootContext {
                root_entity: root.root_entity,
                space: root.space,
                camera_entity: root.camera_entity,
                stack: *stack,
            });
        }

        let parent = parents_query.get(current).ok()?;
        current = parent.get();
    }
}

fn pointer_ray_for_camera(
    location: &Location,
    camera_entity: Entity,
    cameras: &Query<(Entity, &Camera, &GlobalTransform)>,
) -> Option<CachedPointerRay> {
    let Ok((_, camera, camera_transform)) = cameras.get(camera_entity) else {
        return None;
    };

    if !camera
        .logical_viewport_rect()
        .is_some_and(|rect| rect.contains(location.position))
    {
        return None;
    }

    let ray = camera
        .viewport_to_world(camera_transform, location.position)
        .ok()?;
    Some(CachedPointerRay {
        order: camera.order as f32,
        origin: ray.origin,
        direction: ray.direction.as_vec3(),
    })
}

fn intersect_ray_with_node_plane(
    ray: &CachedPointerRay,
    global_transform: &GlobalTransform,
) -> Option<(Vec3, Vec2, Vec3, f32)> {
    let plane_origin = global_transform.translation();
    let plane_normal = global_transform.back().as_vec3();
    let denominator = ray.direction.dot(plane_normal);

    if denominator.abs() <= f32::EPSILON {
        return None;
    }

    let distance = (plane_origin - ray.origin).dot(plane_normal) / denominator;
    if distance < 0.0 {
        return None;
    }

    let world_hit = ray.origin + ray.direction * distance;
    let local_hit = global_transform
        .to_matrix()
        .inverse()
        .transform_point3(world_hit)
        .truncate();

    Some((world_hit, local_hit, plane_normal, distance))
}

pub fn univis_picking_backend(
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
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut output: MessageWriter<PointerHits>,
) {
    let prefer_cached_context = rollout
        .as_ref()
        .map_or(true, |config| config.use_cached_ui_context);
    emit_pointer_hits(
        &pointers,
        &cameras,
        &root_query,
        &nodes_query,
        &parents_query,
        &clipper_query,
        prefer_cached_context,
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

fn emit_pointer_hits(
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

fn collect_pointer_hits(
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
                .or_insert_with(|| pointer_ray_for_camera(location, *camera_entity, &cameras))
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

                let dist =
                    sd_rounded_box(cursor_pos_local, candidate.half_size, candidate.radius_vec);

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
                    if is_ancestor_of(ranked_hit.entity, *other_entity, &parents_query)
                        || is_ancestor_of(*other_entity, ranked_hit.entity, &parents_query)
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

fn write_pointer_hits(
    hits: &[(PointerId, Vec<(Entity, HitData)>, f32)],
    output: &mut MessageWriter<PointerHits>,
) {
    for (pointer_id, picks, order) in hits.iter() {
        output.write(PointerHits::new(*pointer_id, picks.clone(), *order));
    }
}

fn count_pointer_hit_mismatches(
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::camera::{
        CameraProjection, ComputedCameraValues, NormalizedRenderTarget, RenderTargetInfo,
    };
    use bevy::ecs::message::Messages;
    use bevy::ecs::system::SystemState;

    fn sample_root(root_entity: Entity, space: UiSpace) -> (ResolvedRootUi, ResolvedRootStack) {
        let root = ResolvedRootUi {
            root_entity,
            space,
            canvas: match space {
                UiSpace::Screen => UiCanvasSize::Viewport,
                UiSpace::World2d | UiSpace::World3d => UiCanvasSize::Fixed(Vec2::new(800.0, 600.0)),
            },
            canvas_size: Vec2::new(800.0, 600.0),
            camera_entity: Some(Entity::PLACEHOLDER),
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        };
        let band_width = if matches!(space, UiSpace::Screen) {
            0.004
        } else {
            root.ui_units_to_world_scale() * 0.04
        };
        (root, ResolvedRootStack::with_capsule(0.0, band_width))
    }

    fn run_picking_for_space(space: UiSpace, perspective: bool) -> Vec<PointerHits> {
        let mut app = App::new();
        app.add_message::<PointerHits>();
        app.add_systems(Update, univis_picking_backend);

        let mut camera = Camera::default();
        camera.computed = ComputedCameraValues {
            target_info: Some(RenderTargetInfo {
                physical_size: UVec2::new(800, 600),
                scale_factor: 1.0,
            }),
            clip_from_view: if perspective {
                PerspectiveProjection {
                    fov: core::f32::consts::FRAC_PI_2,
                    aspect_ratio: 800.0 / 600.0,
                    near: 0.1,
                    ..default()
                }
                .get_clip_from_view()
            } else {
                OrthographicProjection {
                    area: Rect::new(-400.0, -300.0, 400.0, 300.0),
                    ..OrthographicProjection::default_2d()
                }
                .get_clip_from_view()
            },
            ..default()
        };

        let camera_entity = app
            .world_mut()
            .spawn((
                camera,
                GlobalTransform::from(Transform::from_xyz(
                    0.0,
                    0.0,
                    if perspective { 5.0 } else { 1000.0 },
                )),
            ))
            .id();

        let root = app.world_mut().spawn_empty().id();
        let (resolved_root, resolved_stack) = sample_root(root, space);
        app.world_mut()
            .entity_mut(root)
            .insert((resolved_root, resolved_stack))
            .insert(GlobalTransform::default());

        let node = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UInteraction::default(),
                LayoutDepth(1),
                UNode {
                    width: UVal::Px(200.0),
                    height: UVal::Px(120.0),
                    ..default()
                },
                ComputedSize {
                    width: 200.0,
                    height: 120.0,
                    local_pos: Vec2::ZERO,
                },
                GlobalTransform::default(),
            ))
            .id();

        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .unwrap()
            .camera_entity = Some(camera_entity);

        app.world_mut().spawn((
            PointerId::Mouse,
            PointerLocation::new(Location {
                target: NormalizedRenderTarget::None {
                    width: 800,
                    height: 600,
                },
                position: Vec2::new(400.0, 300.0),
            }),
        ));

        app.update();

        let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
        let collected = hits.drain().collect::<Vec<_>>();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].picks.len(), 1);
        assert_eq!(collected[0].picks[0].0, node);
        collected
    }

    #[test]
    fn post_settle_picking_hits_first_frame_geometry_with_cached_context() {
        let mut app = App::new();
        app.add_message::<PointerHits>();
        app.init_resource::<PickingSyncState>();
        app.init_resource::<UiValidationState>();
        app.add_systems(Update, post_settle_picking_backend);

        let mut work_state = UiWorkState::default();
        work_state.begin_generation(univis_ui_engine::schedule::UiPendingStages {
            render: true,
            ..Default::default()
        });
        app.insert_resource(work_state);

        let mut camera = Camera::default();
        camera.computed = ComputedCameraValues {
            target_info: Some(RenderTargetInfo {
                physical_size: UVec2::new(800, 600),
                scale_factor: 1.0,
            }),
            clip_from_view: OrthographicProjection {
                area: Rect::new(-400.0, -300.0, 400.0, 300.0),
                ..OrthographicProjection::default_2d()
            }
            .get_clip_from_view(),
            ..default()
        };

        let camera_entity = app
            .world_mut()
            .spawn((
                camera,
                GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
            ))
            .id();
        let root = app.world_mut().spawn_empty().id();
        let (_, root_stack) = sample_root(root, UiSpace::Screen);

        let node = app
            .world_mut()
            .spawn((
                UInteraction::default(),
                LayoutDepth(1),
                UNode {
                    width: UVal::Px(200.0),
                    height: UVal::Px(120.0),
                    ..default()
                },
                ComputedSize {
                    width: 200.0,
                    height: 120.0,
                    local_pos: Vec2::ZERO,
                },
                GlobalTransform::default(),
                CachedUiContext {
                    root_entity: Some(root),
                    camera_entity: Some(camera_entity),
                    space: UiSpace::Screen,
                    ui_to_world_scale: 1.0,
                    root_stack,
                    clip_ancestor: None,
                },
            ))
            .id();

        app.world_mut().spawn((
            PointerId::Mouse,
            PointerLocation::new(Location {
                target: NormalizedRenderTarget::None {
                    width: 800,
                    height: 600,
                },
                position: Vec2::new(400.0, 300.0),
            }),
        ));
        app.world_mut()
            .resource_mut::<PickingSyncState>()
            .pointer_generation = 1;

        app.update();

        let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
        let collected = hits.drain().collect::<Vec<_>>();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].picks.len(), 1);
        assert_eq!(collected[0].picks[0].0, node);
    }

    #[test]
    fn post_settle_picking_validation_records_shadow_mismatches_without_emitting_hits() {
        let mut app = App::new();
        app.add_message::<PointerHits>();
        app.init_resource::<PickingSyncState>();
        app.init_resource::<UiValidationState>();
        app.insert_resource(UiRolloutConfig {
            use_post_settle_picking: false,
            validation: UiValidationMode::LogWarnings,
            ..default()
        });
        app.add_systems(Update, post_settle_picking_backend);

        let mut work_state = UiWorkState::default();
        work_state.begin_generation(univis_ui_engine::schedule::UiPendingStages {
            render: true,
            ..Default::default()
        });
        app.insert_resource(work_state);

        let mut camera = Camera::default();
        camera.computed = ComputedCameraValues {
            target_info: Some(RenderTargetInfo {
                physical_size: UVec2::new(800, 600),
                scale_factor: 1.0,
            }),
            clip_from_view: OrthographicProjection {
                area: Rect::new(-400.0, -300.0, 400.0, 300.0),
                ..OrthographicProjection::default_2d()
            }
            .get_clip_from_view(),
            ..default()
        };

        let camera_entity = app
            .world_mut()
            .spawn((
                camera,
                GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
            ))
            .id();
        let root = app.world_mut().spawn_empty().id();
        let (_, root_stack) = sample_root(root, UiSpace::Screen);
        let clipper = app
            .world_mut()
            .spawn((
                GlobalTransform::from(Transform::from_xyz(500.0, 500.0, 0.0)),
                ComputedSize {
                    width: 20.0,
                    height: 20.0,
                    ..default()
                },
                UNode::default(),
                UClip { enabled: true },
            ))
            .id();

        app.world_mut().spawn((
            ChildOf(root),
            UInteraction::default(),
            LayoutDepth(1),
            UNode {
                width: UVal::Px(200.0),
                height: UVal::Px(120.0),
                ..default()
            },
            ComputedSize {
                width: 200.0,
                height: 120.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::default(),
            CachedUiContext {
                root_entity: Some(root),
                camera_entity: Some(camera_entity),
                space: UiSpace::Screen,
                ui_to_world_scale: 1.0,
                root_stack,
                clip_ancestor: Some(clipper),
            },
        ));

        let (mut resolved_root, resolved_stack) = sample_root(root, UiSpace::Screen);
        resolved_root.camera_entity = Some(camera_entity);
        app.world_mut()
            .entity_mut(root)
            .insert((resolved_root, resolved_stack));

        app.world_mut().spawn((
            PointerId::Mouse,
            PointerLocation::new(Location {
                target: NormalizedRenderTarget::None {
                    width: 800,
                    height: 600,
                },
                position: Vec2::new(400.0, 300.0),
            }),
        ));
        app.world_mut()
            .resource_mut::<PickingSyncState>()
            .pointer_generation = 1;

        app.update();

        let validation_state = app.world().resource::<UiValidationState>();
        assert_eq!(validation_state.picking_shadow_ui_generation, 1);
        assert_eq!(validation_state.picking_shadow_pointer_generation, 1);
        assert_eq!(validation_state.picking_shadow_mismatches, 1);

        let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
        let collected = hits.drain().collect::<Vec<_>>();
        assert!(collected.is_empty());
    }

    #[test]
    fn resolve_root_for_entity_walks_up_the_parent_chain() {
        let mut app = App::new();

        let root = app.world_mut().spawn_empty().id();
        app.world_mut()
            .entity_mut(root)
            .insert(sample_root(root, UiSpace::Screen));

        let child = app.world_mut().spawn(ChildOf(root)).id();
        let grandchild = app.world_mut().spawn(ChildOf(child)).id();

        let mut state = SystemState::<(
            Query<&ChildOf>,
            Query<(&ResolvedRootUi, &ResolvedRootStack)>,
        )>::new(app.world_mut());
        let (parents, roots) = state.get(app.world());

        let resolved = resolve_root_for_entity(None, grandchild, &parents, &roots)
            .expect("descendants should resolve to the ancestor root");
        assert_eq!(resolved.root_entity, root);
        assert_eq!(resolved.space, UiSpace::Screen);
    }

    #[test]
    fn intersect_ray_with_node_plane_returns_world_and_local_hit() {
        let ray = CachedPointerRay {
            order: 0.0,
            origin: Vec3::new(0.0, 0.0, 5.0),
            direction: Vec3::NEG_Z,
        };
        let transform = GlobalTransform::from(Transform::default());

        let (world_hit, local_hit, normal, distance) =
            intersect_ray_with_node_plane(&ray, &transform)
                .expect("ray should intersect the default UI plane");

        assert!(world_hit.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(local_hit.abs_diff_eq(Vec2::ZERO, 1e-5));
        assert!(normal.abs_diff_eq(Vec3::Z, 1e-5));
        assert!((distance - 5.0).abs() <= 1e-5);
    }

    #[test]
    fn is_clipped_by_ancestors_detects_points_outside_enabled_clipper() {
        let mut app = App::new();

        let clipper = app
            .world_mut()
            .spawn((
                GlobalTransform::default(),
                ComputedSize {
                    width: 100.0,
                    height: 100.0,
                    ..default()
                },
                UNode::default(),
                UClip { enabled: true },
            ))
            .id();
        let child = app.world_mut().spawn(ChildOf(clipper)).id();

        let mut state = SystemState::<(
            Query<&ChildOf>,
            Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
        )>::new(app.world_mut());
        let (parents, clippers) = state.get(app.world());

        assert!(!is_clipped_by_ancestors(
            child,
            Vec3::new(0.0, 0.0, 0.0),
            &parents,
            &clippers,
        ));
        assert!(is_clipped_by_ancestors(
            child,
            Vec3::new(80.0, 80.0, 0.0),
            &parents,
            &clippers,
        ));
    }

    #[test]
    fn picking_prefers_the_higher_root_capsule_over_a_deeper_child_in_a_lower_root() {
        let mut app = App::new();
        app.add_message::<PointerHits>();
        app.add_systems(Update, univis_picking_backend);

        let mut camera = Camera::default();
        camera.computed = ComputedCameraValues {
            target_info: Some(RenderTargetInfo {
                physical_size: UVec2::new(800, 600),
                scale_factor: 1.0,
            }),
            clip_from_view: OrthographicProjection {
                area: Rect::new(-400.0, -300.0, 400.0, 300.0),
                ..OrthographicProjection::default_2d()
            }
            .get_clip_from_view(),
            ..default()
        };

        let camera_entity = app
            .world_mut()
            .spawn((
                camera,
                GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
            ))
            .id();

        let lower_root = app.world_mut().spawn_empty().id();
        let (mut lower_resolved, mut lower_stack) = sample_root(lower_root, UiSpace::World2d);
        lower_resolved.camera_entity = Some(camera_entity);
        lower_stack.capsule_sort_key = 0.0;
        app.world_mut()
            .entity_mut(lower_root)
            .insert((lower_resolved, lower_stack));

        let upper_root = app.world_mut().spawn_empty().id();
        let (mut upper_resolved, mut upper_stack) = sample_root(upper_root, UiSpace::World2d);
        upper_resolved.camera_entity = Some(camera_entity);
        upper_stack.capsule_sort_key = 0.05;
        app.world_mut()
            .entity_mut(upper_root)
            .insert((upper_resolved, upper_stack));

        let lower_node = app
            .world_mut()
            .spawn((
                ChildOf(lower_root),
                UInteraction::default(),
                LayoutDepth(3),
                USelf {
                    order: 32,
                    ..default()
                },
                UNode {
                    width: UVal::Px(200.0),
                    height: UVal::Px(120.0),
                    ..default()
                },
                ComputedSize {
                    width: 200.0,
                    height: 120.0,
                    local_pos: Vec2::ZERO,
                },
                GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.08)),
            ))
            .id();

        let upper_node = app
            .world_mut()
            .spawn((
                ChildOf(upper_root),
                UInteraction::default(),
                LayoutDepth(1),
                UNode {
                    width: UVal::Px(200.0),
                    height: UVal::Px(120.0),
                    ..default()
                },
                ComputedSize {
                    width: 200.0,
                    height: 120.0,
                    local_pos: Vec2::ZERO,
                },
                GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.02)),
            ))
            .id();

        app.world_mut().spawn((
            PointerId::Mouse,
            PointerLocation::new(Location {
                target: NormalizedRenderTarget::None {
                    width: 800,
                    height: 600,
                },
                position: Vec2::new(400.0, 300.0),
            }),
        ));

        app.update();

        let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
        let collected = hits.drain().collect::<Vec<_>>();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].picks.len(), 2);
        assert_eq!(collected[0].picks[0].0, upper_node);
        assert_eq!(collected[0].picks[1].0, lower_node);
    }

    #[test]
    fn picking_backend_hits_screen_nodes() {
        let hits = run_picking_for_space(UiSpace::Screen, false);
        assert_eq!(hits[0].order, 0.0);
    }

    #[test]
    fn picking_backend_hits_world2d_nodes() {
        let hits = run_picking_for_space(UiSpace::World2d, false);
        assert_eq!(hits[0].order, 0.0);
    }

    #[test]
    fn picking_backend_hits_world3d_nodes_with_perspective_camera() {
        let hits = run_picking_for_space(UiSpace::World3d, true);
        assert_eq!(hits[0].order, 0.0);
    }
}
