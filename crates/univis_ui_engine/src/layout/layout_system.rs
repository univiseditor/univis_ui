#![allow(deprecated)]
#![allow(clippy::type_complexity)]
//! Root model and root-resolution systems for Univis UI.
//!
//! [`URootUi`] is the modern public entry point for authoring UI roots.
//! Every root resolves into a [`ResolvedRootUi`] plus an internal
//! [`ResolvedRootStack`] capsule, which keeps cross-root stacking sealed:
//! local ordering stays inside the root and descendants do not automatically
//! interleave above another root.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::internal_prelude::*;

mod root_resolution;
mod screen_transform;

use self::root_resolution::{
    RootResolutionIssue, clamp_canvas_size, effective_root_ui, emit_root_resolution_warning,
    normalize_meters_per_unit, resolve_root_camera, resolve_root_canvas_size,
};
use self::screen_transform::compute_screen_root_transform;

/// Describes where a root is projected after layout is solved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum UiSpace {
    /// A viewport-bound HUD root that stays visually fixed to the resolved camera.
    Screen,
    /// A flat world-space UI canvas.
    World2d,
    /// A world-space UI canvas rendered through the 3D material path.
    World3d,
}

/// Describes how a root obtains its logical canvas size.
///
/// `Viewport` is mainly for `UiSpace::Screen`, while `Fixed` and `FitContent`
/// are most useful for `World2d` and `World3d` roots.
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum UiCanvasSize {
    /// Match the resolved viewport size.
    Viewport,
    /// Use an explicit logical canvas size.
    Fixed(Vec2),
    /// Measure the root content and clamp the result if needed.
    FitContent { min: Vec2, max: Option<Vec2> },
}

/// Selects which camera a root should resolve against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum UiCameraRef {
    /// Resolve automatically; this works when exactly one compatible camera is available.
    Auto,
    /// Bind the root to a specific camera entity.
    Entity(Entity),
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
#[require(
    UNode,
    ResolvedRootUi,
    ResolvedRootStack,
    RootResolutionState,
    RootSpawnRank
)]
/// Public root component for Univis UI trees.
///
/// `URootUi` defines where a root lives (`Screen`, `World2d`, or `World3d`),
/// how its logical canvas is sized, which camera resolves it, and how world
/// scaling is derived for world-space roots.
///
/// Every `URootUi` acts as a closed stacking capsule: local ordering stays
/// inside the root and does not automatically rise above another root.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_engine::prelude::*;
///
/// fn spawn_world_panel(commands: &mut Commands) {
///     commands.spawn((
///         URootUi::world_2d(Vec2::new(960.0, 540.0)),
///         UNode {
///             width: UVal::Percent(1.0),
///             height: UVal::Percent(1.0),
///             ..default()
///         },
///     ));
/// }
/// ```
pub struct URootUi {
    /// Where the UI lives once projected into the scene.
    ///
    /// Every `URootUi` is a closed stacking capsule: local ordering stays inside the root, and
    /// descendants never escape above other roots automatically.
    pub space: UiSpace,
    /// The logical canvas size used by layout.
    pub canvas: UiCanvasSize,
    /// Which camera resolves viewport-bound roots and interaction context.
    pub camera: UiCameraRef,
    /// Physical world size per logical UI unit for world-space roots.
    pub meters_per_unit: f32,
    /// Rendering quality multiplier that stays independent from physical world size.
    pub resolution_scale: f32,
}

impl URootUi {
    /// Default world scaling used by the convenience constructors.
    pub const DEFAULT_METERS_PER_UNIT: f32 = 0.001;
    /// Default render-quality multiplier used by the convenience constructors.
    pub const DEFAULT_RESOLUTION_SCALE: f32 = 1.0;

    /// Creates a screen-space root that behaves like a HUD tied to the resolved viewport.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bevy::prelude::*;
    /// use univis_ui_engine::prelude::*;
    ///
    /// fn spawn_hud(commands: &mut Commands) {
    ///     commands.spawn((
    ///         URootUi::screen(),
    ///         UNode {
    ///             width: UVal::Percent(1.0),
    ///             height: UVal::Percent(1.0),
    ///             ..default()
    ///         },
    ///     ));
    /// }
    /// ```
    pub fn screen() -> Self {
        Self {
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 2D root with a fixed logical canvas size.
    ///
    /// The default `meters_per_unit` is `0.001`, so the physical world size is:
    /// `world_size = canvas_size * meters_per_unit`.
    pub fn world_2d(size: Vec2) -> Self {
        Self {
            space: UiSpace::World2d,
            canvas: UiCanvasSize::Fixed(size),
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 3D root with a fixed logical canvas size.
    ///
    /// The default `meters_per_unit` is `0.001`, so the physical world size is:
    /// `world_size = canvas_size * meters_per_unit`.
    pub fn world_3d(size: Vec2) -> Self {
        Self {
            space: UiSpace::World3d,
            canvas: UiCanvasSize::Fixed(size),
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 2D root whose logical canvas grows from its measured content.
    ///
    /// This is useful for floating cards, inspectors, or node-like panels that
    /// should wrap their measured content.
    pub fn world_2d_fit_content() -> Self {
        Self {
            space: UiSpace::World2d,
            canvas: UiCanvasSize::FitContent {
                min: Vec2::ZERO,
                max: None,
            },
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 3D root whose logical canvas grows from its measured content.
    pub fn world_3d_fit_content() -> Self {
        Self {
            space: UiSpace::World3d,
            canvas: UiCanvasSize::FitContent {
                min: Vec2::ZERO,
                max: None,
            },
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }
}

const LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT: f32 = 1.0;
const SCREEN_ROOT_CAPSULE_BAND_WIDTH: f32 = 0.004;
const WORLD_ROOT_CAPSULE_BAND_UI_UNITS: f32 = 0.04;
const ROOT_CAPSULE_MIN_BAND_WIDTH: f32 = 1.0e-6;
const ROOT_CAPSULE_GAP_FACTOR: f32 = 0.05;
const ROOT_CAPSULE_MIN_GAP: f32 = 1.0e-6;
const ROOT_CAPSULE_LOCAL_LAYER_DIVISOR: f32 = 2048.0;
const ROOT_LOCAL_DEPTH_MAX: usize = 64;
const ROOT_LOCAL_ORDER_CLAMP: i32 = 32;
const ROOT_STACK_EDIT_EPSILON: f32 = 1.0e-5;

/// Derived runtime state for a resolved root.
///
/// This component is updated by the engine and should usually be treated as
/// read-only application state.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ResolvedRootUi {
    pub root_entity: Entity,
    /// Resolved UI space for this root.
    pub space: UiSpace,
    /// The source policy used to resolve the final logical canvas.
    pub canvas: UiCanvasSize,
    /// Final logical canvas size after resolving viewport or fixed sizing.
    pub canvas_size: Vec2,
    pub camera_entity: Option<Entity>,
    /// Physical world size per logical UI unit for world-space roots.
    pub meters_per_unit: f32,
    /// Rendering quality multiplier that stays independent from physical world size.
    pub resolution_scale: f32,
}

impl Default for ResolvedRootUi {
    fn default() -> Self {
        Self {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            canvas_size: Vec2::ZERO,
            camera_entity: None,
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        }
    }
}

impl ResolvedRootUi {
    /// Returns how many world units correspond to one logical UI unit for this root.
    pub fn ui_units_to_world_scale(&self) -> f32 {
        match self.space {
            UiSpace::Screen => 1.0,
            UiSpace::World2d | UiSpace::World3d => normalize_meters_per_unit(self.meters_per_unit),
        }
    }

    /// Converts a logical UI size to its physical size in world units for this root.
    pub fn world_size_for(&self, logical_size: Vec2) -> Vec2 {
        logical_size * self.ui_units_to_world_scale()
    }

    /// Converts a logical scalar to world units for this root.
    pub fn world_scalar_for(&self, logical_value: f32) -> f32 {
        logical_value * self.ui_units_to_world_scale()
    }
}

/// Derived stacking data for a root capsule.
///
/// This keeps root-vs-root ordering separate from local child ordering so one
/// root cannot visually leak above another root unless the root itself is above it.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ResolvedRootStack {
    pub authored_root_z: f32,
    pub spawn_rank: u64,
    pub capsule_sort_key: f32,
    pub capsule_band_base: f32,
    pub capsule_band_width: f32,
    pub capsule_band_step: f32,
    pub(crate) applied_root_z: f32,
    pub(crate) initialized: bool,
}

impl Default for ResolvedRootStack {
    fn default() -> Self {
        let capsule_band_width = SCREEN_ROOT_CAPSULE_BAND_WIDTH;
        Self {
            authored_root_z: 0.0,
            spawn_rank: 0,
            capsule_sort_key: 0.0,
            capsule_band_base: 0.0,
            capsule_band_width,
            capsule_band_step: capsule_band_width / ROOT_CAPSULE_LOCAL_LAYER_DIVISOR,
            applied_root_z: 0.0,
            initialized: false,
        }
    }
}

impl ResolvedRootStack {
    pub fn with_capsule(capsule_band_base: f32, capsule_band_width: f32) -> Self {
        let capsule_band_width = capsule_band_width.max(ROOT_CAPSULE_MIN_BAND_WIDTH);
        Self {
            capsule_sort_key: capsule_band_base,
            capsule_band_base,
            capsule_band_width,
            capsule_band_step: capsule_band_width / ROOT_CAPSULE_LOCAL_LAYER_DIVISOR,
            applied_root_z: capsule_band_base,
            initialized: true,
            ..default()
        }
    }

    pub fn capsule_ceiling(&self) -> f32 {
        self.capsule_band_base + self.capsule_band_width
    }

    pub fn local_depth_offset(&self, layout_depth: usize, order: i32) -> f32 {
        let reserved_steps = self.capsule_band_step * 2.0;
        let usable_band = (self.capsule_band_width - reserved_steps).max(0.0);

        self.capsule_band_step + usable_band * local_depth_fraction(layout_depth, order)
    }

    pub fn local_depth_key(&self, layout_depth: usize, order: i32) -> f32 {
        self.local_depth_offset(layout_depth, order)
    }

    pub fn text_child_offset(&self) -> f32 {
        self.capsule_band_step
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RootSpawnRank(pub(crate) u64);

#[derive(Resource, Default)]
pub(crate) struct RootSpawnRankCounter {
    next: u64,
}

/// Legacy compatibility wrapper for a screen-space HUD root.
///
/// Deprecated during `alpha2`. Prefer [`URootUi::screen`] for new code.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::screen()` instead of `UScreenRoot`."
)]
#[derive(Component, Default)]
#[require(
    UNode,
    ResolvedRootUi,
    ResolvedRootStack,
    RootResolutionState,
    RootSpawnRank
)]
pub struct UScreenRoot;

/// Legacy compatibility wrapper for a world-space root.
///
/// Deprecated during the `alpha2` migration. Prefer
/// [`URootUi::world_2d`] or [`URootUi::world_3d`] for new code.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::world_2d(size)` or `URootUi::world_3d(size)` instead. If you need the exact legacy `UWorldRoot` physical sizing during alpha2, set `meters_per_unit = 1.0` explicitly on `URootUi`."
)]
#[derive(Component)]
#[require(
    UNode,
    ResolvedRootUi,
    ResolvedRootStack,
    RootResolutionState,
    RootSpawnRank
)]
pub struct UWorldRoot {
    pub size: Vec2,
    /// Legacy compatibility flag. New code should express this through `UiSpace`.
    pub is_3d: bool,
    /// Scaling factor for text resolution/quality relative to the size.
    pub resolution_scale: f32,
}

impl Default for UWorldRoot {
    fn default() -> Self {
        Self {
            size: Vec2::new(800.0, 600.0),
            is_3d: false,
            resolution_scale: 1.0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RootResolutionState {
    issue: RootResolutionIssue,
}

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

fn root_capsule_band_width(resolved: &ResolvedRootUi) -> f32 {
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

fn local_depth_fraction(layout_depth: usize, order: i32) -> f32 {
    let depth_bucket = layout_depth.min(ROOT_LOCAL_DEPTH_MAX) as f32;
    let depth_slots = ROOT_LOCAL_DEPTH_MAX as f32 + 2.0;
    let depth_slice = 1.0 / depth_slots;
    let order_span = (ROOT_LOCAL_ORDER_CLAMP * 2 + 1) as f32;
    let order_bucket = (order.clamp(-ROOT_LOCAL_ORDER_CLAMP, ROOT_LOCAL_ORDER_CLAMP)
        + ROOT_LOCAL_ORDER_CLAMP) as f32
        / order_span;

    (((depth_bucket + 1.0) * depth_slice) + order_bucket * depth_slice * 0.9).min(1.0)
}

pub(crate) fn resolve_root_ui(
    mut roots: Query<
        (
            Entity,
            Option<&URootUi>,
            Option<&UScreenRoot>,
            Option<&UWorldRoot>,
            &mut ResolvedRootUi,
            &mut RootResolutionState,
        ),
        Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>,
    >,
    cameras: Query<(Entity, &Camera)>,
    windows: Query<&Window>,
) {
    for (entity, root_ui, screen_root, world_root, mut resolved, mut state) in roots.iter_mut() {
        let Some(effective) = effective_root_ui(root_ui, screen_root, world_root) else {
            continue;
        };

        let (camera_entity, mut issue) = resolve_root_camera(effective.camera, &cameras);
        let canvas_size = resolve_root_canvas_size(
            effective.canvas,
            camera_entity,
            &cameras,
            &windows,
            &mut issue,
        );

        let next = ResolvedRootUi {
            root_entity: entity,
            space: effective.space,
            canvas: effective.canvas,
            canvas_size,
            camera_entity,
            meters_per_unit: effective.meters_per_unit,
            resolution_scale: effective.resolution_scale,
        };

        if *resolved != next {
            *resolved = next;
        }

        if state.issue != issue {
            emit_root_resolution_warning(entity, effective.space, issue);
            state.issue = issue;
        }
    }
}

pub(crate) fn assign_root_spawn_ranks(
    mut counter: ResMut<RootSpawnRankCounter>,
    mut roots: Query<&mut RootSpawnRank, Added<RootSpawnRank>>,
) {
    for mut rank in roots.iter_mut() {
        rank.0 = counter.next;
        counter.next += 1;
    }
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

        let applied_root_z = stack.applied_root_z;
        let initialized = stack.initialized;

        let next = ResolvedRootStack {
            authored_root_z,
            spawn_rank,
            capsule_sort_key: floor,
            capsule_band_base: floor,
            capsule_band_width: band_width,
            capsule_band_step: band_step,
            applied_root_z,
            initialized,
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

        stack.applied_root_z = transform.translation.z;
        stack.initialized = true;
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

pub fn sync_cached_ui3d(
    mut commands: Commands,
    roots_changed: Query<
        (),
        (
            Changed<ResolvedRootUi>,
            Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>,
        ),
    >,
    all_nodes: Query<(Entity, Has<UI3d>), Or<(With<UNode>, With<UI3d>)>>,
    incremental_nodes: Query<
        (Entity, Has<UI3d>),
        Or<(Added<UNode>, Added<UI3d>, Changed<ChildOf>)>,
    >,
    mut removed_children: RemovedComponents<ChildOf>,
    parents_query: Query<&ChildOf>,
    root_query: Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
) {
    if roots_changed.is_empty() {
        for (entity, has_ui3d) in incremental_nodes.iter() {
            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }

        for entity in removed_children.read() {
            let Ok((_, has_ui3d)) = all_nodes.get(entity) else {
                continue;
            };

            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }
    } else {
        for (entity, has_ui3d) in all_nodes.iter() {
            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }
    }
}

fn sync_cached_ui3d_for_entity(
    entity: Entity,
    has_ui3d: bool,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
    commands: &mut Commands,
) {
    let should_have_ui3d = resolve_root_for_ui3d(entity, parents_query, root_query)
        .map(|root| matches!(root.space, UiSpace::World3d))
        .unwrap_or(false);

    match (should_have_ui3d, has_ui3d) {
        (true, false) => {
            commands.entity(entity).insert(UI3d);
        }
        (false, true) => {
            commands.entity(entity).remove::<UI3d>();
        }
        _ => {}
    }
}

fn resolve_root_for_ui3d(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
) -> Option<ResolvedRootUi> {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return Some(*root);
        }

        let parent = parents_query.get(current).ok()?;
        current = parent.parent();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::ecs::system::SystemState;

    fn screen_root(canvas_size: Vec2) -> ResolvedRootUi {
        ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            canvas_size,
            camera_entity: Some(Entity::PLACEHOLDER),
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        }
    }

    fn screen_stack(base: f32) -> ResolvedRootStack {
        ResolvedRootStack {
            capsule_band_base: base,
            capsule_sort_key: base,
            ..default()
        }
    }

    #[test]
    fn orthographic_screen_transform_tracks_camera_and_compensates_zoom() {
        let resolved = screen_root(Vec2::new(800.0, 600.0));
        let camera_transform = GlobalTransform::from(Transform {
            translation: Vec3::new(12.0, -4.0, 2.0),
            rotation: Quat::from_rotation_z(0.35),
            ..default()
        });
        let projection = OrthographicProjection {
            area: Rect::new(-800.0, -600.0, 800.0, 600.0),
            far: 1000.0,
            ..OrthographicProjection::default_2d()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Orthographic(projection)),
        )
        .expect("screen roots should resolve an orthographic transform");

        assert!(
            result
                .translation
                .abs_diff_eq(Vec3::new(12.0, -4.0, -498.0), 1e-4)
        );
        assert!(
            result
                .rotation
                .abs_diff_eq(Quat::from_rotation_z(0.35), 1e-5)
        );
        assert!(result.scale.abs_diff_eq(Vec3::new(2.0, 2.0, 1.0), 1e-5));
    }

    #[test]
    fn orthographic_screen_transform_stays_in_front_of_3d_camera() {
        let resolved = screen_root(Vec2::new(800.0, 600.0));
        let camera_transform = GlobalTransform::from(Transform::default());
        let projection = OrthographicProjection {
            near: 0.1,
            far: 100.0,
            ..OrthographicProjection::default_3d()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Orthographic(projection)),
        )
        .expect("screen roots should resolve an orthographic transform");

        assert!(result.translation.z < 0.0);
    }

    #[test]
    fn perspective_screen_transform_uses_camera_forward_and_frustum_size() {
        let resolved = screen_root(Vec2::new(200.0, 100.0));
        let camera_transform = GlobalTransform::from(Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            ..default()
        });
        let projection = PerspectiveProjection {
            fov: core::f32::consts::FRAC_PI_2,
            aspect_ratio: 2.0,
            near: 0.1,
            ..default()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Perspective(projection)),
        )
        .expect("screen roots should resolve a perspective transform");

        assert!(
            result
                .translation
                .abs_diff_eq(Vec3::new(1.0, 2.0, 2.0), 1e-4)
        );
        assert!(result.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(result.scale.abs_diff_eq(Vec3::new(0.02, 0.02, 1.0), 1e-5));
    }

    #[test]
    fn sync_root_capsule_transforms_preserves_world_root_xy_rotation_and_scale() {
        let mut app = App::new();
        app.add_systems(Update, sync_root_capsule_transforms);

        let original = Transform::from_xyz(3.0, -2.0, 7.0);
        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(640.0, 360.0)),
                original,
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                    canvas_size: Vec2::new(640.0, 360.0),
                    camera_entity: None,
                    meters_per_unit: 0.25,
                    resolution_scale: 1.0,
                },
                ResolvedRootStack {
                    authored_root_z: original.translation.z,
                    capsule_sort_key: original.translation.z,
                    capsule_band_base: original.translation.z,
                    capsule_band_width: root_capsule_band_width(&ResolvedRootUi {
                        root_entity: Entity::PLACEHOLDER,
                        space: UiSpace::World2d,
                        canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                        canvas_size: Vec2::new(640.0, 360.0),
                        camera_entity: None,
                        meters_per_unit: 0.25,
                        resolution_scale: 1.0,
                    }),
                    capsule_band_step: root_capsule_band_width(&ResolvedRootUi {
                        root_entity: Entity::PLACEHOLDER,
                        space: UiSpace::World2d,
                        canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                        canvas_size: Vec2::new(640.0, 360.0),
                        camera_entity: None,
                        meters_per_unit: 0.25,
                        resolution_scale: 1.0,
                    }) / ROOT_CAPSULE_LOCAL_LAYER_DIVISOR,
                    applied_root_z: original.translation.z,
                    initialized: true,
                    ..default()
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        app.update();

        let transform = app
            .world()
            .entity(root)
            .get::<Transform>()
            .copied()
            .expect("root should keep a transform");

        assert!(
            transform
                .translation
                .abs_diff_eq(original.translation, 1e-5)
        );
        assert!(transform.rotation.abs_diff_eq(original.rotation, 1e-5));
        assert!(transform.scale.abs_diff_eq(original.scale, 1e-5));
    }

    #[test]
    fn resolve_root_stacking_packs_same_z_world_roots_by_spawn_order() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (
                assign_root_spawn_ranks,
                resolve_root_stacking,
                sync_root_capsule_transforms,
            )
                .chain(),
        );

        let resolved = ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::World2d,
            canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
            canvas_size: Vec2::new(320.0, 180.0),
            camera_entity: None,
            meters_per_unit: 1.0,
            resolution_scale: 1.0,
        };

        let older = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::default(),
                resolved,
            ))
            .id();
        let newer = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::default(),
                resolved,
            ))
            .id();

        app.world_mut()
            .entity_mut(older)
            .get_mut::<ResolvedRootUi>()
            .expect("older root should have resolved ui")
            .root_entity = older;
        app.world_mut()
            .entity_mut(newer)
            .get_mut::<ResolvedRootUi>()
            .expect("newer root should have resolved ui")
            .root_entity = newer;

        app.update();

        let older_stack = *app
            .world()
            .entity(older)
            .get::<ResolvedRootStack>()
            .expect("older root should have a stack");
        let newer_stack = *app
            .world()
            .entity(newer)
            .get::<ResolvedRootStack>()
            .expect("newer root should have a stack");

        assert!(newer_stack.capsule_band_base > older_stack.capsule_ceiling());
    }

    #[test]
    fn resolve_root_stacking_preserves_authored_world_root_gaps_when_they_do_not_overlap() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (assign_root_spawn_ranks, resolve_root_stacking).chain(),
        );

        let lower = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
                    canvas_size: Vec2::new(320.0, 180.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        let upper = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
                    canvas_size: Vec2::new(320.0, 180.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(lower)
            .get_mut::<ResolvedRootUi>()
            .expect("lower root should have resolved ui")
            .root_entity = lower;
        app.world_mut()
            .entity_mut(upper)
            .get_mut::<ResolvedRootUi>()
            .expect("upper root should have resolved ui")
            .root_entity = upper;

        app.update();

        let lower_stack = *app
            .world()
            .entity(lower)
            .get::<ResolvedRootStack>()
            .expect("lower root should have a stack");
        let upper_stack = *app
            .world()
            .entity(upper)
            .get::<ResolvedRootStack>()
            .expect("upper root should have a stack");

        assert!((lower_stack.capsule_band_base - 0.0).abs() <= 1e-5);
        assert!((upper_stack.capsule_band_base - 1.0).abs() <= 1e-5);
    }

    #[test]
    fn resolve_root_stacking_packs_same_z_screen_roots_by_spawn_order() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (assign_root_spawn_ranks, resolve_root_stacking).chain(),
        );

        let older = app
            .world_mut()
            .spawn((
                URootUi::screen(),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(800.0, 600.0),
                    camera_entity: Some(Entity::PLACEHOLDER),
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
                },
            ))
            .id();
        let newer = app
            .world_mut()
            .spawn((
                URootUi::screen(),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(800.0, 600.0),
                    camera_entity: Some(Entity::PLACEHOLDER),
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(older)
            .get_mut::<ResolvedRootUi>()
            .expect("older screen root should have resolved ui")
            .root_entity = older;
        app.world_mut()
            .entity_mut(newer)
            .get_mut::<ResolvedRootUi>()
            .expect("newer screen root should have resolved ui")
            .root_entity = newer;

        app.update();

        let older_stack = *app
            .world()
            .entity(older)
            .get::<ResolvedRootStack>()
            .expect("older screen root should have a stack");
        let newer_stack = *app
            .world()
            .entity(newer)
            .get::<ResolvedRootStack>()
            .expect("newer screen root should have a stack");

        assert!(newer_stack.capsule_band_base > older_stack.capsule_ceiling());
    }

    #[test]
    fn resolve_root_camera_auto_reports_missing_single_and_ambiguous_states() {
        let mut app = App::new();

        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::AutoMissingCamera);

        let expected = app.world_mut().spawn(Camera::default()).id();
        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, Some(expected));
        assert_eq!(issue, RootResolutionIssue::None);

        app.world_mut().spawn(Camera::default());
        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::AutoAmbiguousCamera);
    }

    #[test]
    fn resolve_root_camera_reports_missing_and_inactive_explicit_targets() {
        let mut app = App::new();
        let inactive = app
            .world_mut()
            .spawn(Camera {
                is_active: false,
                ..default()
            })
            .id();

        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());

        let (camera_entity, issue) =
            resolve_root_camera(UiCameraRef::Entity(Entity::PLACEHOLDER), &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::ExplicitCameraMissing);

        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Entity(inactive), &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::ExplicitCameraInactive);
    }

    #[test]
    fn resolve_root_canvas_size_returns_fixed_size_verbatim() {
        let mut app = App::new();
        app.world_mut().spawn(Window {
            resolution: (1280, 720).into(),
            ..default()
        });

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Fixed(Vec2::new(320.0, 240.0)),
            None,
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(320.0, 240.0));
        assert_eq!(issue, RootResolutionIssue::None);
    }

    #[test]
    fn resolve_root_canvas_size_reads_logical_viewport_size_from_camera() {
        let mut app = App::new();
        let camera = app
            .world_mut()
            .spawn(Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::ZERO,
                    physical_size: UVec2::new(600, 300),
                    depth: 0.0..1.0,
                }),
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo {
                        physical_size: UVec2::new(1200, 900),
                        scale_factor: 1.0,
                    }),
                    ..default()
                },
                ..default()
            })
            .id();

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(600.0, 300.0));
        assert_eq!(issue, RootResolutionIssue::None);
    }

    #[test]
    fn resolve_root_canvas_size_falls_back_to_window_when_viewport_size_is_unavailable() {
        let mut app = App::new();
        app.world_mut().spawn(Window {
            resolution: (1280, 720).into(),
            ..default()
        });

        let missing_camera = app.world_mut().spawn_empty().id();

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(missing_camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(1280.0, 720.0));
        assert_eq!(issue, RootResolutionIssue::ViewportSizeUnavailable);
    }

    #[test]
    fn resolve_root_canvas_size_uses_default_when_no_window_exists() {
        let mut app = App::new();

        let missing_camera = app.world_mut().spawn_empty().id();
        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(missing_camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(800.0, 600.0));
        assert_eq!(issue, RootResolutionIssue::ViewportSizeUnavailable);
    }

    fn world_root(space: UiSpace) -> (UWorldRoot, ResolvedRootUi) {
        (
            UWorldRoot {
                is_3d: matches!(space, UiSpace::World3d),
                ..default()
            },
            ResolvedRootUi {
                root_entity: Entity::PLACEHOLDER,
                space,
                canvas: UiCanvasSize::Fixed(Vec2::new(800.0, 600.0)),
                canvas_size: Vec2::new(800.0, 600.0),
                camera_entity: None,
                meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
            },
        )
    }

    #[test]
    fn cached_ui3d_is_added_for_world3d_roots_and_children() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        assert!(app.world().entity(root).contains::<UI3d>());
        assert!(app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_removed_when_root_leaves_world3d() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have a resolved context")
            .space = UiSpace::World2d;

        app.update();

        assert!(!app.world().entity(root).contains::<UI3d>());
        assert!(!app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_applied_to_children_added_after_root_resolution() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();

        app.update();

        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        assert!(app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_removed_when_child_detaches_from_world3d_root() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        app.world_mut().entity_mut(child).remove::<ChildOf>();

        app.update();

        assert!(!app.world().entity(child).contains::<UI3d>());
    }
}
