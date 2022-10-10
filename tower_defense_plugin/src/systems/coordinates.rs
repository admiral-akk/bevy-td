use bevy::prelude::{Added, Changed, Entity, Query, RemovedComponents, ResMut, Transform};

use crate::{
    components::{coordinates::Coordinates, start::Start},
    resources::board::Board,
};

pub fn added(
    mut added: Query<(Entity, &Coordinates, &mut Transform), Added<Coordinates>>,
    mut board: ResMut<Board>,
) {
    for (entity, coord, mut transform) in added.iter_mut() {
        *transform = board.transform(coord, transform.translation.z);
        board.entities.insert(*coord, entity);
    }
}

pub fn updated(
    mut updated: Query<(Entity, &Coordinates, &mut Transform), Changed<Coordinates>>,
    mut board: ResMut<Board>,
) {
    for (entity, coord, mut transform) in updated.iter_mut() {
        *transform = board.transform(coord, transform.translation.z);
        board.entities.update_key(&entity, *coord);
    }
}

pub fn removed(removed: RemovedComponents<Coordinates>, mut board: ResMut<Board>) {
    for removed in removed.iter() {
        bevy::log::error!("Attempt removed coordinates!");
        board.entities.remove_value(&removed);
    }
}

pub fn return_to_start(mut units: Query<(&mut Coordinates, &Start)>) {
    for (mut coord, start) in units.iter_mut() {
        *coord = start.0;
    }
}
