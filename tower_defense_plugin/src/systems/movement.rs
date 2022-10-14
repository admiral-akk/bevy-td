use bevy::prelude::{Component, EventReader, Parent, Query, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates, movements::movement::Movement},
    events::ActiveAction,
    resources::board::Board,
};

pub fn apply_move<T: Movement + Component>(
    mut entities: Query<(&mut Coordinates, &Allegiance)>,
    mut action_ewr: EventReader<ActiveAction>,
    actions: Query<(&Parent, &T)>,
    board: Res<Board>,
) {
    for ActiveAction(action) in action_ewr.iter() {
        if let Ok((parent, movement)) = actions.get(*action) {
            let targets: Vec<(Coordinates, Allegiance)> = entities
                .iter()
                .map(|(coord, allegiance)| (*coord, *allegiance))
                .collect();
            if let Ok((mut coord, allegiance)) = entities.get_mut(parent.get()) {
                if let Some(new_coord) = movement.next(targets, (*coord, *allegiance), &board) {
                    if !board.entities.contains_key(&new_coord) {
                        *coord = new_coord;
                    }
                }
            }
        }
    }
}
