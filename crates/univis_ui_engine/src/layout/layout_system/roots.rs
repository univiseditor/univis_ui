use bevy::prelude::*;

use super::root_resolution::{RootResolutionIssue, normalize_meters_per_unit};
use crate::internal_prelude::*;

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
    FitContent {
        /// The minimum size to enforce.
        min: Vec2,
        /// The maximum size to enforce, or None for unbounded.
        max: Option<Vec2>,
    },
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
    UiRootSettlementState,
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

pub(crate) const LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT: f32 = 1.0;
pub(super) const SCREEN_ROOT_CAPSULE_BAND_WIDTH: f32 = 0.004;
pub(super) const WORLD_ROOT_CAPSULE_BAND_UI_UNITS: f32 = 0.04;
pub(super) const ROOT_CAPSULE_MIN_BAND_WIDTH: f32 = 1.0e-6;
pub(super) const ROOT_CAPSULE_GAP_FACTOR: f32 = 0.05;
pub(super) const ROOT_CAPSULE_MIN_GAP: f32 = 1.0e-6;
pub(crate) const ROOT_CAPSULE_LOCAL_LAYER_DIVISOR: f32 = 2048.0;
pub(super) const ROOT_LOCAL_DEPTH_MAX: usize = 64;
pub(super) const ROOT_LOCAL_ORDER_CLAMP: i32 = 32;
pub(super) const ROOT_STACK_EDIT_EPSILON: f32 = 1.0e-5;

/// Derived runtime state for a resolved root.
///
/// This component is updated by the engine and should usually be treated as
/// read-only application state.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ResolvedRootUi {
    /// The entity ID of this root node.
    pub root_entity: Entity,
    /// Resolved UI space for this root.
    pub space: UiSpace,
    /// The source policy used to resolve the final logical canvas.
    pub canvas: UiCanvasSize,
    /// Final logical canvas size after resolving viewport or fixed sizing.
    pub canvas_size: Vec2,
    /// The camera entity this root is rendering to, if applicable.
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
    /// The Z-index provided by the user.
    pub authored_root_z: f32,
    /// The spawn order of this root used for deterministic tie-breaking.
    pub spawn_rank: u64,
    /// The final key used for sorting roots globally.
    pub capsule_sort_key: f32,
    /// The base Z-depth for this root's local stacking context.
    pub capsule_band_base: f32,
    /// The total depth span allocated to this root.
    pub capsule_band_width: f32,
    /// The depth step increment used per internal local layer.
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
    /// Creates a new root stack state from external capsule metrics.
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

    /// Returns the descendant-facing stack data used by cached solve context.
    ///
    /// Child local depth placement only depends on the reserved band metrics.
    /// Root ordering, authored z, and transform-sync bookkeeping stay on the
    /// root entity itself and should not invalidate descendant cached context.
    pub(crate) fn descendant_context_snapshot(self) -> Self {
        Self {
            capsule_band_width: self.capsule_band_width,
            capsule_band_step: self.capsule_band_step,
            ..default()
        }
    }

    /// Returns the highest Z-depth used by this root's capsule.
    pub fn capsule_ceiling(&self) -> f32 {
        self.capsule_band_base + self.capsule_band_width
    }

    /// Calculates a local Z-offset based on logical depth and layout order.
    pub fn local_depth_offset(&self, layout_depth: usize, order: i32) -> f32 {
        self.local_depth_offset_for_fraction(local_depth_fraction(layout_depth, order))
    }

    /// Calculates a local Z-offset from a [0.0, 1.0] fractional depth map.
    pub fn local_depth_offset_for_fraction(&self, fraction: f32) -> f32 {
        let reserved_steps = self.capsule_band_step * 2.0;
        let usable_band = (self.capsule_band_width - reserved_steps).max(0.0);

        self.capsule_band_step + usable_band * fraction.clamp(0.0, 1.0)
    }

    /// Resolves the absolute local depth key used for visual rendering sorting.
    pub fn local_depth_key(&self, layout_depth: usize, order: i32) -> f32 {
        self.local_depth_offset(layout_depth, order)
    }

    /// Recommends an offset for text nodes specifically to ensure they render above generic backgrounds.
    pub fn text_child_offset(&self) -> f32 {
        self.capsule_band_step
    }
}

/// Derived settlement counters for one root subtree.
///
/// `current_generation` tracks the latest generation observed anywhere under
/// the root, while the pending counters show how many nodes have not finished
/// each stage yet.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct UiRootSettlementState {
    /// The highest layout generation observed under this root.
    pub current_generation: u64,
    /// Number of nodes waiting to be measured.
    pub pending_measure: u32,
    /// Number of nodes waiting to be solved.
    pub pending_solve: u32,
    /// Number of nodes waiting to be rendered.
    pub pending_render: u32,
}

impl UiRootSettlementState {
    /// Returns true if any stage is pending settlement in this root.
    pub fn has_pending(&self) -> bool {
        self.pending_measure > 0 || self.pending_solve > 0 || self.pending_render > 0
    }

    /// Returns true if the root is fully settled.
    pub fn is_settled(&self) -> bool {
        !self.has_pending()
    }

    /// Observes the state of a descendant node and updates the counters.
    pub fn observe_node(&mut self, versions: UiNodeStageVersions) {
        self.current_generation = self.current_generation.max(
            versions
                .measure_input_generation
                .max(versions.solve_input_generation)
                .max(versions.render_input_generation),
        );

        if versions.measure_done_generation < versions.measure_input_generation {
            self.pending_measure += 1;
        }

        if versions.solve_done_generation < versions.solve_input_generation {
            self.pending_solve += 1;
        }

        if versions.render_done_generation < versions.render_input_generation {
            self.pending_render += 1;
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RootSpawnRank(pub(crate) u64);

#[derive(Resource, Default)]
pub(crate) struct RootSpawnRankCounter {
    pub(super) next: u64,
}

/// Legacy compatibility wrapper for a screen-space HUD root.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::screen()` instead of `UScreenRoot`."
)]
#[derive(Component, Default)]
#[require(
    UNode,
    ResolvedRootUi,
    ResolvedRootStack,
    UiRootSettlementState,
    RootResolutionState,
    RootSpawnRank
)]
pub struct UScreenRoot;

/// Legacy compatibility wrapper for a world-space root.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::world_2d(size)` or `URootUi::world_3d(size)` instead. If you need the exact legacy `UWorldRoot` physical sizing, set `meters_per_unit = 1.0` explicitly on `URootUi`."
)]
#[derive(Component)]
#[require(
    UNode,
    ResolvedRootUi,
    ResolvedRootStack,
    UiRootSettlementState,
    RootResolutionState,
    RootSpawnRank
)]
pub struct UWorldRoot {
    /// Legacy explicit size mapping for world roots.
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
    pub(super) issue: RootResolutionIssue,
}

pub(super) fn local_depth_fraction(layout_depth: usize, order: i32) -> f32 {
    let depth_bucket = layout_depth.min(ROOT_LOCAL_DEPTH_MAX) as f32;
    let depth_slots = ROOT_LOCAL_DEPTH_MAX as f32 + 2.0;
    let depth_slice = 1.0 / depth_slots;
    let order_span = (ROOT_LOCAL_ORDER_CLAMP * 2 + 1) as f32;
    let order_bucket = (order.clamp(-ROOT_LOCAL_ORDER_CLAMP, ROOT_LOCAL_ORDER_CLAMP)
        + ROOT_LOCAL_ORDER_CLAMP) as f32
        / order_span;

    (((depth_bucket + 1.0) * depth_slice) + order_bucket * depth_slice * 0.9).min(1.0)
}
