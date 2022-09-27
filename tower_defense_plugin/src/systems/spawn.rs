use bevy::{
    prelude::{Commands, EventReader, EventWriter, Res, ResMut},
    time::Time,
};

use crate::{
    components::monster::Monster,
    events::Spawn,
    resources::{board::Board, game_sprites::GameSprites, spawn_timer::SpawnTimer},
};
pub fn spawn_tick(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_ewr: EventWriter<Spawn>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        spawn_ewr.send(Spawn);
    }
}

pub fn spawn(
    mut commands: Commands,
    board: Res<Board>,
    spritesheet: Res<GameSprites>,
    mut spawn_evr: EventReader<Spawn>,
) {
    for _ in spawn_evr.iter() {
        commands
            .spawn()
            .insert(Monster)
            .insert(board.start.clone())
            .insert_bundle(spritesheet.monster(board.tile_size));
    }
}
