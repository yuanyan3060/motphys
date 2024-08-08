use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

pub fn get_pick_entity(query: &Query<(Entity, &PickSelection)>) -> Option<Entity> {
    query.iter().find(|(_, x)| x.is_selected).map(|(x, _)| x)
}
