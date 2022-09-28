use bevy::{
    prelude::{BuildChildren, Commands, EventReader, EventWriter, Name, Query, Res, ResMut, With},
    time::Time,
    transform::TransformBundle,
};

use crate::{
    components::{coordinates::Coordinates, tower::Tower},
    events::{Attack, TryBuild},
    resources::{
        board::Board, build_tracker::BuildTracker, game_sprites::GameSprites,
        spawn_timer::AttackTimer,
    },
};

pub fn attack(
    board: Res<Board>,
    towers: Query<&Coordinates, With<Tower>>,
    mut attack_ewr: EventWriter<Attack>,
    mut attack_timer: ResMut<AttackTimer>,
    time: Res<Time>,
) {
    attack_timer.0.tick(time.delta());
    if attack_timer.0.just_finished() {
        for tower in towers.iter() {
            let targets = board.neighbouring_monsters(tower);
            if targets.len() > 0 {
                attack_ewr.send(Attack(board.monsters[&targets[0]], 1));
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
            commands
                .entity(board.board.unwrap())
                .with_children(|parent| {
                    let tower = parent
                        .spawn()
                        .insert(Name::new("Tower"))
                        .insert(Tower)
                        .insert(coord.clone())
                        .insert_bundle(spritesheet.peasant(board.tile_size))
                        .insert_bundle(TransformBundle {
                            local: board.transform(&coord, 4.),
                            ..Default::default()
                        })
                        .id();
                    board.towers.insert(coord, tower);
                });
        }
    }
}
