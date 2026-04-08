use bevy::prelude::*;
use std::collections::HashMap;

use super::root_resolution::clamp_canvas_size;
use super::roots::{
    ROOT_CAPSULE_GAP_FACTOR, ROOT_CAPSULE_LOCAL_LAYER_DIVISOR, ROOT_CAPSULE_MIN_BAND_WIDTH,
    ROOT_CAPSULE_MIN_GAP, ROOT_STACK_EDIT_EPSILON, ResolvedRootStack, ResolvedRootUi,
    RootSpawnRank, SCREEN_ROOT_CAPSULE_BAND_WIDTH, UiCanvasSize, UiSpace,
    WORLD_ROOT_CAPSULE_BAND_UI_UNITS,
};
use super::screen_transform::compute_screen_root_transform;
use super::{URootUi, UScreenRoot, UWorldRoot};
use crate::internal_prelude::*;

#[derive(Clone, Copy)]
struct RootStackCandidate {
    entity: Entity,
    authored_root_z: f32,
    spawn_rank: u64,
    band_width: f32,
}

fn capture_authored_root_z(current_root_z: f32, stack: &ResolvedRootStack) -> f32 {
    if !stack.initialized || (current_root_z - stack.applied_root_z).abs() > ROOT_STACK_EDIT_EPSILON
    {
        current_root_z
    } else {
        stack.authored_root_z
    }
}

pub(super) fn root_capsule_band_width(resolved: &ResolvedRootUi) -> f32 {
    match resolved.space {
        UiSpace::Screen => SCREEN_ROOT_CAPSULE_BAND_WIDTH,
        UiSpace::World2d | UiSpace::World3d => (resolved.ui_units_to_world_scale()
            * WORLD_ROOT_CAPSULE_BAND_UI_UNITS)
            .max(ROOT_CAPSULE_MIN_BAND_WIDTH),
    }
}

fn root_capsule_gap(previous_band_width: f32, current_band_width: f32) -> f32 {
    previous_band_width
        .max(current_band_width)
        .mul_add(ROOT_CAPSULE_GAP_FACTOR, 0.0)
        .max(ROOT_CAPSULE_MIN_GAP)
}

pub(crate) fn resolve_root_stacking(
    mut roots: ParamSet<(
        Query<
            (
                Entity,
                &ResolvedRootUi,
                &Transform,
                &RootSpawnRank,
                &ResolvedRootStack,
            ),
            Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>,
        >,
        Query<&mut ResolvedRootStack>,
    )>,
) {
    let mut groups: HashMap<(UiSpace, Option<Entity>), Vec<RootStackCandidate>> = HashMap::new();

    for (entity, resolved, transform, spawn_rank, stack) in roots.p0().iter() {
        let authored_root_z = capture_authored_root_z(transform.translation.z, stack);
        groups
            .entry((resolved.space, resolved.camera_entity))
            .or_default()
            .push(RootStackCandidate {
                entity,
                authored_root_z,
                spawn_rank: spawn_rank.0,
                band_width: root_capsule_band_width(resolved),
            });
    }

    let mut next_states = Vec::new();

    for ((space, _camera), mut candidates) in groups {
        candidates.sort_by(|left, right| {
            left.authored_root_z
                .total_cmp(&right.authored_root_z)
                .then_with(|| left.spawn_rank.cmp(&right.spawn_rank))
        });

        let mut next_floor = 0.0;
        let mut previous_band_width = 0.0;
        let mut first = true;

        for candidate in candidates {
            let floor = match space {
                UiSpace::Screen => {
                    if first {
                        0.0
                    } else {
                        next_floor + root_capsule_gap(previous_band_width, candidate.band_width)
                    }
                }
                UiSpace::World2d | UiSpace::World3d => {
                    if first {
                        candidate.authored_root_z
                    } else {
                        candidate.authored_root_z.max(
                            next_floor
                                + root_capsule_gap(previous_band_width, candidate.band_width),
                        )
                    }
                }
            };

            let band_step = candidate.band_width / ROOT_CAPSULE_LOCAL_LAYER_DIVISOR;
            next_states.push((
                candidate.entity,
                candidate.authored_root_z,
                candidate.spawn_rank,
                floor,
                candidate.band_width,
                band_step,
            ));

            next_floor = floor + candidate.band_width;
            previous_band_width = candidate.band_width;
            first = false;
        }
    }

    let mut writable_stacks = roots.p1();

    for (entity, authored_root_z, spawn_rank, floor, band_width, band_step) in next_states {
        let Ok(mut stack) = writable_stacks.get_mut(entity) else {
            continue;
        };

        let next = ResolvedRootStack {
            authored_root_z,
            spawn_rank,
            capsule_sort_key: floor,
            capsule_band_base: floor,
            capsule_band_width: band_width,
            capsule_band_step: band_step,
            applied_root_z: stack.applied_root_z,
            initialized: stack.initialized,
        };

        if *stack != next {
            *stack = next;
        }
    }
}

pub(crate) fn sync_root_capsule_transforms(
    mut roots: Query<
        (&ResolvedRootUi, &mut ResolvedRootStack, &mut Transform),
        Or<(With<UScreenRoot>, With<URootUi>, With<UWorldRoot>)>,
    >,
    cameras: Query<(&GlobalTransform, Option<&Projection>), With<Camera>>,
) {
    for (resolved, mut stack, mut transform) in roots.iter_mut() {
        match resolved.space {
            UiSpace::Screen => {
                let Some(camera_entity) = resolved.camera_entity else {
                    continue;
                };
                let Ok((camera_transform, projection)) = cameras.get(camera_entity) else {
                    continue;
                };
                let Some(next_transform) =
                    compute_screen_root_transform(resolved, &stack, camera_transform, projection)
                else {
                    continue;
                };

                if *transform != next_transform {
                    *transform = next_transform;
                }
            }
            UiSpace::World2d | UiSpace::World3d => {
                if (transform.translation.z - stack.capsule_band_base).abs() > f32::EPSILON {
                    transform.translation.z = stack.capsule_band_base;
                }
            }
        }

        if (stack.applied_root_z - transform.translation.z).abs() > f32::EPSILON {
            stack.applied_root_z = transform.translation.z;
        }
        if !stack.initialized {
            stack.initialized = true;
        }
    }
}

pub(crate) fn sync_fit_content_root_canvas_sizes(
    mut roots: Query<(&IntrinsicSize, &mut ResolvedRootUi), With<ResolvedRootUi>>,
) {
    for (intrinsic, mut resolved) in roots.iter_mut() {
        let UiCanvasSize::FitContent { min, max } = resolved.canvas else {
            continue;
        };

        if resolved.space == UiSpace::Screen {
            continue;
        }

        let measured = Vec2::new(intrinsic.width, intrinsic.height).max(Vec2::ZERO);
        let next_canvas_size = clamp_canvas_size(measured, min, max);

        if resolved.canvas_size != next_canvas_size {
            resolved.canvas_size = next_canvas_size;
        }
    }
}
