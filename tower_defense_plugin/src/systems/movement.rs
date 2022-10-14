use bevy::prelude::{Component, EventReader, Query, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates, movements::movement::Movement},
    events::ActiveUnit,
    resources::board::Board,
};

pub fn apply_move<T: Movement + Component>(
    mut entities: Query<(&mut Coordinates, &Allegiance)>,
    movement: Query<&T>,
    mut active_ert: EventReader<ActiveUnit>,
    board: Res<Board>,
) {
    for ActiveUnit(entity) in active_ert.iter() {
        if let Ok(movement) = movement.get(*entity) {
            let targets: Vec<(Coordinates, Allegiance)> = entities
                .iter()
                .map(|(coord, allegiance)| (*coord, *allegiance))
                .collect();
            if let Ok((mut coord, allegiance)) = entities.get_mut(*entity) {
                if let Some(new_coord) = movement.next(targets, (*coord, *allegiance), &board) {
                    if !board.entities.contains_key(&new_coord) {
                        *coord = new_coord;
                    }
                }
            }
        }
    }
}
