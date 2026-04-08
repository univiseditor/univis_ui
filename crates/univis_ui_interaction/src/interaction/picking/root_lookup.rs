use bevy::ecs::relationship::Relationship;
use bevy::prelude::{ChildOf, Entity, Query};

use univis_ui_engine::layout::components::CachedUiContext;
use univis_ui_engine::layout::layout_system::{ResolvedRootStack, ResolvedRootUi, UiSpace};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct ResolvedEntityRootContext {
    pub(super) root_entity: Entity,
    pub(super) space: UiSpace,
    pub(super) camera_entity: Option<Entity>,
    pub(super) stack: ResolvedRootStack,
}

pub(super) fn is_ancestor_of(
    potential_ancestor: Entity,
    potential_descendant: Entity,
    parents_query: &Query<&ChildOf>,
) -> bool {
    let mut current = potential_descendant;

    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if current == potential_ancestor {
            return true;
        }
    }

    false
}

pub(super) fn resolve_root_for_entity(
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
