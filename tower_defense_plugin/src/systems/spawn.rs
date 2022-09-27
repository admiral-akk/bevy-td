use bevy::{
    prelude::{
        BuildChildren, Commands, Entity, EventReader, EventWriter, Name, Query, Res, ResMut,
    },
    time::Time,
    transform::TransformBundle,
};

use crate::{
    components::monster::Monster,
    events::{Move, Spawn},
    resources::{
        board::Board,
        game_sprites::GameSprites,
        spawn_timer::{MoveTimer, SpawnTimer},
    },
};
pub fn monster_tick(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut move_timer: ResMut<MoveTimer>,
    mut spawn_ewr: EventWriter<Spawn>,
    mut move_ewr: EventWriter<Move>,
) {
    spawn_timer.0.tick(time.delta());
    move_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        spawn_ewr.send(Spawn);
    }
    if move_timer.0.just_finished() {
        move_ewr.send(Move);
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
            .entity(board.board.unwrap())
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Name::new("Monster"))
                    .insert(Monster)
                    .insert(board.start.clone())
                    .insert_bundle(spritesheet.monster(board.tile_size))
                    .insert_bundle(TransformBundle {
                        local: board.transform(&board.start, 4.),
                        ..Default::default()
                    });
            });
    }
}
