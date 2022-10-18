use bevy::prelude::{Commands, Component, Entity, EventReader, Parent, Query};

use crate::{
    components::{
        allegiance::Allegiance,
        coordinates::Coordinates,
        health::Health,
        targetting::target::{Target, Targets},
    },
    events::ActiveAction,
    structs::board_state::BoardState,
};
pub fn try_target<T: Target + Component>(
    mut commands: Commands,
    targetting: Query<(Entity, &Parent, &T)>,
    entities: Query<(Entity, &Allegiance, &Health, &Coordinates)>,
    mut active_ert: EventReader<ActiveAction>,
) {
    for ActiveAction(action) in active_ert.iter() {
        if let Ok((target_entity, parent, target)) = targetting.get(*action) {
            bevy::log::error!("Targetting!");
            let board_state = BoardState::new(&entities);
            if let Some(attacker) = board_state.get(parent.get()) {
                let targets = target.get_targets(attacker, &board_state);
                bevy::log::error!("attacker: {:?}, targets: {:?}!", attacker, targets);
                commands
                    .entity(target_entity)
                    .insert(Targets(target.get_targets(attacker, &board_state)));
            }
        }
    }
}
