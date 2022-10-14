use bevy::prelude::{Component, Entity, EventReader, EventWriter, Parent, Query, Res, With};

use crate::{
    components::{
        allegiance::{Allegiance},
        attacks::attack::Attack,
        coordinates::Coordinates,
        health::Health,
        power::Power,
    },
    events::{ActiveAction, ActiveUnit, AttackEvent},
    resources::board::Board,
};

pub fn attack(
    board: Res<Board>,
    attackers: Query<(&Coordinates, &Power, &Allegiance)>,
    targets: Query<(Entity, &Allegiance), With<Health>>,
    mut active_ert: EventReader<ActiveUnit>,
    mut attack_ewr: EventWriter<AttackEvent>,
) {
    for active in active_ert.iter() {
        let (coord, power, allegiance) = attackers.get(active.0).unwrap();
        let potential_targets = board.neighbouring_entities(coord);
        for target in potential_targets {
            if let Ok((target, target_allegiance)) = targets.get(target) {
                if !allegiance.eq(target_allegiance) {
                    attack_ewr.send(AttackEvent(target, power.0 as i32));
                    break;
                }
            }
        }
    }
}

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
