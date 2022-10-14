use bevy::prelude::{Component, EventReader, EventWriter, Parent, Query, Res};

use crate::{
    components::{
        allegiance::Allegiance, attacks::attack::Attack, coordinates::Coordinates,
    },
    events::{ActiveAction, AttackEvent},
    resources::board::Board,
};
pub fn try_attack<T: Attack + Component>(
    board: Res<Board>,
    attacks: Query<(&Parent, &T)>,
    units: Query<(&Coordinates, &Allegiance)>,
    mut active_ert: EventReader<ActiveAction>,
    mut attack_ewr: EventWriter<AttackEvent>,
) {
    for ActiveAction(action) in active_ert.iter() {
        if let Ok((parent, attack)) = attacks.get(*action) {
            if let Ok((&coord, &allegiance)) = units.get(parent.get()) {
                let targets = units
                    .iter()
                    .map(|(coord, allegiance)| (*coord, *allegiance))
                    .collect();
                if let Some(attack) = attack.target(targets, (coord, allegiance), &board) {
                    attack_ewr.send(attack);
                }
            }
        }
    }
}
