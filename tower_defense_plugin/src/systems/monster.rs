use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, ResMut, With};

use crate::{
    components::{coordinates::Coordinates, monster::Monster},
    resources::{board::Board, life_tracker::LifeTracker},
};

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
