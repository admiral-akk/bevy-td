use bevy::{
    prelude::{EventWriter, Query, Res, ResMut, With},
    time::Time,
};

use crate::{
    components::{coordinates::Coordinates, power::Power, tower::Tower},
    events::Attack,
    resources::{board::Board, spawn_timer::AttackTimer},
};

pub fn attack(
    board: Res<Board>,
    towers: Query<(&Coordinates, &Power), (With<Tower>, With<Power>)>,
    mut attack_ewr: EventWriter<Attack>,
    mut attack_timer: ResMut<AttackTimer>,
    time: Res<Time>,
) {
    attack_timer.0.tick(time.delta());
    if attack_timer.0.just_finished() {
        for (coord, power) in towers.iter() {
            let targets = board.neighbouring_monsters(coord);
            if targets.len() > 0 {
                attack_ewr.send(Attack(
                    *board.monsters.get(&targets[0]).unwrap(),
                    power.0 as i32,
                ));
            }
        }
    }
}
