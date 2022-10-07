use bevy::prelude::{Changed, Entity, EventWriter, Query, Res, With};

use crate::{
    components::{
        allegiance::Allegiance, coordinates::Coordinates, health::Health, power::Power,
        tick_timer::TickTimer,
    },
    events::Attack,
    resources::board::Board,
};

pub fn attack(
    board: Res<Board>,
    mut attackers: Query<(&Coordinates, &mut TickTimer, &Power, &Allegiance), Changed<TickTimer>>,
    targets: Query<(Entity, &Allegiance), With<Health>>,
    mut attack_ewr: EventWriter<Attack>,
) {
    for (coord, mut timer, power, allegiance) in attackers.iter_mut() {
        if timer.active() {
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
}
