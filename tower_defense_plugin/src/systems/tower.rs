use bevy::{
    prelude::{BuildChildren, Commands, EventReader, EventWriter, Query, Res, ResMut, With},
    time::Time,
};

use crate::{
    components::{coordinates::Coordinates, power::Power, tower::Tower},
    entities::towers::{soldier_entity},
    events::{Attack, TryBuild},
    resources::{
        board::Board, build_tracker::BuildTracker, game_sprites::GameSprites,
        spawn_timer::AttackTimer,
    },
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
                attack_ewr.send(Attack(board.monsters[&targets[0]], power.0 as i32));
            }
        }
    }
}

pub fn try_build(
    mut commands: Commands,
    mut board: ResMut<Board>,
    spritesheet: Res<GameSprites>,
    build_tracker: Res<BuildTracker>,
    mut build_evr: EventReader<TryBuild>,
) {
    for _ in build_evr.iter() {
        if let Some(coord) = build_tracker.target {
            let peasant = soldier_entity(&mut commands, &mut board, &coord, &spritesheet);
            commands
                .entity(board.board.unwrap())
                .push_children(&[peasant]);
        }
    }
}
