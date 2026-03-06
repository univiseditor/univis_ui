use bevy::prelude::*;
use crate::prelude::*;

pub fn container_box(
    query: Query<(Entity, &mut UNode, &Children)>
) {
    for (_entity, _node, _childer) in query.iter() {

    }
}