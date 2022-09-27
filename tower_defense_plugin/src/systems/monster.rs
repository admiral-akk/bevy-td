use bevy::{
    prelude::{BuildChildren, Commands, Entity, EventReader, Name, Query, Res, Transform, With},
    transform::TransformBundle,
};

use crate::{
    components::{coordinates::Coordinates, monster::Monster},
    events::{Move, Spawn},
    resources::{board::Board, game_sprites::GameSprites},
};

pub fn monster_move(
    board: Res<Board>,
    mut move_evr: EventReader<Move>,
    mut monsters: Query<(&mut Transform, &mut Coordinates), With<Monster>>,
) {
    for _ in move_evr.iter() {
        for (mut t, mut c) in monsters.iter_mut() {
            *c = board.next(&c);
            *t = board.transform(&c, 4.);
        }
    }
}

pub fn monster_despawn(
    mut commands: Commands,
    board: Res<Board>,
    monsters: Query<(Entity, &Coordinates), With<Monster>>,
) {
    for (monster, coordinates) in monsters.iter() {
        if board.is_end(coordinates) {
            commands.entity(monster).despawn();
        }
    }
}

pub fn monster_spawn(
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
