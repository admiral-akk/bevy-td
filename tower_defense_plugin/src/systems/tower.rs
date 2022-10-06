use bevy::prelude::{EventWriter, Query, Res, With};

use crate::{
    components::{coordinates::Coordinates, power::Power, tick_timer::TickTimer, tower::Tower},
    events::Attack,
    resources::board::Board,
};

pub fn attack(
    board: Res<Board>,
    mut towers: Query<(&Coordinates, &mut TickTimer, &Power), With<Tower>>,
    mut attack_ewr: EventWriter<Attack>,
) {
    for (coord, mut timer, power) in towers.iter_mut() {
        let targets = board.neighbouring_monsters(coord);
        if targets.len() > 0 {
            if timer.active() {
                attack_ewr.send(Attack(
                    *board.monsters.get(&targets[0]).unwrap(),
                    power.0 as i32,
                ));
            }
        }
    }
}
