use bevy::{
    prelude::{Changed, Entity, Query, RemovedComponents, Res, ResMut, Transform, With},
};

use crate::{
    components::{coordinates::Coordinates, monster::Monster, tower::Tower},
    resources::board::Board,
};

pub fn update_transform(
    mut updated_coordinates: Query<(&Coordinates, &mut Transform), Changed<Coordinates>>,
    board: Res<Board>,
) {
    for (coord, mut transform) in updated_coordinates.iter_mut() {
        *transform = board.transform(coord, transform.translation.z);
    }
}

pub fn update_towers(
    mut updated_coordinates: Query<(Entity, &Coordinates), (Changed<Coordinates>, With<Tower>)>,
    mut board: ResMut<Board>,
) {
    for (tower, new_coord) in updated_coordinates.iter_mut() {
        board.towers.update_key(&tower, *new_coord);
    }
}
pub fn update_monsters(
    mut updated_coordinates: Query<(Entity, &Coordinates), (Changed<Coordinates>, With<Monster>)>,
    mut board: ResMut<Board>,
) {
    for (tower, new_coord) in updated_coordinates.iter_mut() {
        board.monsters.update_key(&tower, *new_coord);
    }
}

pub fn remove_towers(
    removed: RemovedComponents<Coordinates>,
    updated_coordinates: Query<(Entity, &Coordinates), (With<Coordinates>, With<Tower>)>,
    mut board: ResMut<Board>,
) {
    for removed in removed.iter() {
        if let Ok((tower, _coord)) = updated_coordinates.get(removed) {
            board.towers.remove_value(&tower);
        }
    }
}
pub fn remove_monsters(
    removed: RemovedComponents<Coordinates>,
    updated_coordinates: Query<(Entity, &Coordinates), (With<Coordinates>, With<Monster>)>,
    mut board: ResMut<Board>,
) {
    for removed in removed.iter() {
        if let Ok((tower, _coord)) = updated_coordinates.get(removed) {
            board.monsters.remove_value(&tower);
        }
    }
}
