use bevy::{
    prelude::{
        BuildChildren, Commands, DespawnRecursiveExt, Entity, Name, Query, Res, ResMut, Transform,
        With,
    },
    transform::TransformBundle,
};

use crate::{
    components::{
        coordinates::Coordinates, health::Health, monster::Monster, spawn::Spawn,
        tick_timer::TickTimer,
    },
    resources::{board::Board, game_sprites::GameSprites, life_tracker::LifeTracker},
};

pub fn monster_move(
    mut board: ResMut<Board>,
    mut monsters: Query<(Entity, &mut Transform, &mut Coordinates, &mut TickTimer), With<Monster>>,
) {
    for (e, _t, mut c, mut timer) in monsters.iter_mut() {
        if timer.active() {
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
    mut spawn: Query<(&mut Spawn, &mut TickTimer, &Coordinates)>,
) {
    for (mut spawn, mut timer, coord) in spawn.iter_mut() {
        if timer.active() && spawn.has_spawn() {
            spawn.spawn_creep();
            let coord = coord.clone();
            let monster = commands
                .entity(board.board.unwrap())
                .with_children(|parent| {
                    parent
                        .spawn()
                        .insert(Name::new("Monster"))
                        .insert(Monster)
                        .insert(Health(3))
                        .insert(coord)
                        .insert(TickTimer::new(1))
                        .insert_bundle(spritesheet.monster(board.tile_size))
                        .insert_bundle(TransformBundle {
                            local: board.transform(&board.start, 4.),
                            ..Default::default()
                        });
                })
                .id();
            board.monsters.insert(coord, monster);
        }
    }
}
