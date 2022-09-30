use bevy::{
    prelude::{
        BuildChildren, Commands, DespawnRecursiveExt, Entity, EventReader, Name, Query, Res,
        ResMut, Transform, With,
    },
    transform::TransformBundle,
};

use crate::{
    components::{coordinates::Coordinates, health::Health, monster::Monster},
    events::{Move, Spawn},
    resources::{
        board::Board, game_sprites::GameSprites, life_tracker::LifeTracker,
        spawn_tracker::SpawnTracker,
    },
};

pub fn monster_move(
    mut board: ResMut<Board>,
    mut move_evr: EventReader<Move>,
    mut monsters: Query<(Entity, &mut Transform, &mut Coordinates), With<Monster>>,
) {
    for _ in move_evr.iter() {
        for (e, _t, mut c) in monsters.iter_mut() {
            board.monsters.remove(&c);
            *c = board.next(&c);
            board.monsters.insert(*c, e);
        }
    }
}

pub fn monster_despawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut life: ResMut<LifeTracker>,
    monsters: Query<(Entity, &Coordinates), With<Monster>>,
) {
    for (monster, coordinates) in monsters.iter() {
        if board.is_end(coordinates) {
            board.monsters.remove(&coordinates);
            commands.entity(monster).despawn_recursive();
            life.0 -= 1;
        }
    }
}

pub fn monster_spawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    spritesheet: Res<GameSprites>,
    mut spawn_evr: EventReader<Spawn>,
    mut spawn_tracker: ResMut<SpawnTracker>,
) {
    for _ in spawn_evr.iter() {
        let coord = board.start.clone();
        let monster = commands
            .entity(board.board.unwrap())
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Name::new("Monster"))
                    .insert(Monster)
                    .insert(Health(3))
                    .insert(coord)
                    .insert_bundle(spritesheet.monster(board.tile_size))
                    .insert_bundle(TransformBundle {
                        local: board.transform(&board.start, 4.),
                        ..Default::default()
                    });
            })
            .id();
        board.monsters.insert(coord, monster);
        spawn_tracker.0 -= 1;
    }
}
