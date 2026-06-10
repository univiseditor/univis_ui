use bevy::prelude::*;

use super::{ResolvedRootUi, URootUi, UScreenRoot, UWorldRoot, UiSpace};
use crate::internal_prelude::*;

/// Synchronizes the `UI3d` component on nodes based on whether their root is in 3D space.
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
