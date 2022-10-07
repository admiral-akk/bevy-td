use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, With,
    },
};

use crate::{
    components::{
        coordinates::Coordinates, monster::Monster,
        tick_timer::TickTimer,
    },
    resources::{board::Board, life_tracker::LifeTracker},
};

pub fn monster_move(
    board: Res<Board>,
    mut monsters: Query<(&mut Coordinates, &mut TickTimer), With<Monster>>,
) {
    for (mut c, mut timer) in monsters.iter_mut() {
        if timer.active() && board.monsters.contains_key(&c) {
            *c = board.next(&c);
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
