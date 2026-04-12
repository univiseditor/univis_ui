use bevy::prelude::*;
use std::collections::HashMap;

use crate::schedule::UiPendingStages;

/// Public layout invalidation flags that external crates can request from the engine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiLayoutInvalidation {
    pub dirty_self: bool,
    pub dirty_ancestors: bool,
    pub measure_self: bool,
    pub measure_ancestors: bool,
    pub solve_self: bool,
    pub solve_ancestors: bool,
    pub render_self: bool,
}

impl UiLayoutInvalidation {
    pub fn merge(&mut self, other: Self) {
        self.dirty_self |= other.dirty_self;
        self.dirty_ancestors |= other.dirty_ancestors;
        self.measure_self |= other.measure_self;
        self.measure_ancestors |= other.measure_ancestors;
        self.solve_self |= other.solve_self;
        self.solve_ancestors |= other.solve_ancestors;
        self.render_self |= other.render_self;
    }

    /// Generic intrinsic/content-size invalidation used by widgets that
    /// changed measured content and need layout to re-run.
    pub fn intrinsic_change() -> Self {
        Self {
            dirty_self: true,
            dirty_ancestors: true,
            measure_self: true,
            measure_ancestors: true,
            solve_self: true,
            solve_ancestors: true,
            render_self: false,
        }
    }

    pub fn pending_stages(self) -> UiPendingStages {
        UiPendingStages {
            root_resolve: false,
            hierarchy: self.dirty_self || self.dirty_ancestors,
            measure: self.measure_self || self.measure_ancestors,
            solve: self.solve_self || self.solve_ancestors,
            render: self.render_self,
        }
    }
}

/// Queue of external layout invalidation requests that the engine drains into
/// its internal layout cache during settlement.
#[derive(Resource, Default)]
pub struct UiInvalidateRequestQueue {
    requests: HashMap<Entity, UiLayoutInvalidation>,
}

impl UiInvalidateRequestQueue {
    pub fn request(&mut self, entity: Entity, invalidation: UiLayoutInvalidation) {
        self.requests.entry(entity).or_default().merge(invalidation);
    }

    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    pub fn len(&self) -> usize {
        self.requests.len()
    }

    pub fn request_for(&self, entity: Entity) -> Option<UiLayoutInvalidation> {
        self.requests.get(&entity).copied()
    }

    pub fn pending_stages(&self) -> UiPendingStages {
        let mut pending = UiPendingStages::default();
        for invalidation in self.requests.values().copied() {
            pending.merge(invalidation.pending_stages());
        }
        pending
    }

    pub(crate) fn drain(&mut self) -> HashMap<Entity, UiLayoutInvalidation> {
        std::mem::take(&mut self.requests)
    }
}
