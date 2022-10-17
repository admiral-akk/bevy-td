use bevy::prelude::{Added, Commands, Component, Entity, EventReader, Parent, Query, Res};

use crate::{
    components::{
        allegiance::Allegiance,
        coordinates::Coordinates,
        movements::{movement::Movement, plan::Plan},
    },
    events::ActiveAction,
    resources::board::Board,
};

pub fn propose_move<T: Movement + Component>(
    mut commands: Commands,
    mut entities: Query<(&Coordinates, &Allegiance)>,
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
            if let Ok((coord, allegiance)) = entities.get_mut(parent.get()) {
                if let Some(new_coord) = movement.next(targets, (*coord, *allegiance), &board) {
                    if !board.entities.contains_key(&new_coord) {
                        commands
                            .entity(parent.get())
                            .insert(Plan { target: new_coord });
                    }
                }
            }
        }
    }
}

pub fn apply_move(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Coordinates, &Plan), Added<Plan>>,
) {
    for (entity, mut coord, plan) in entities.iter_mut() {
        *coord = plan.target;
        commands.entity(entity).remove::<Plan>();
    }
}
