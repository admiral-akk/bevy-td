use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, With};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates, health::Health, power::Power},
    events::{ActiveUnit, Attack},
    resources::board::Board,
};

pub fn attack(
    board: Res<Board>,
    attackers: Query<(&Coordinates, &Power, &Allegiance)>,
    targets: Query<(Entity, &Allegiance), With<Health>>,
    mut active_ert: EventReader<ActiveUnit>,
    mut attack_ewr: EventWriter<Attack>,
) {
    for active in active_ert.iter() {
        let (coord, power, allegiance) = attackers.get(active.0).unwrap();
        let potential_targets = board.neighbouring_entities(coord);
        for target in potential_targets {
            if let Ok((target, target_allegiance)) = targets.get(target) {
                if !allegiance.eq(target_allegiance) {
                    attack_ewr.send(Attack(target, power.0 as i32));
                    break;
                }
            }
        }
    }
}
