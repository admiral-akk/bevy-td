use bevy::prelude::{Commands, Entity, EventReader, Query, ResMut, With};

use crate::{
    components::{coordinates::Coordinates, health::Health, monster::Monster},
    events::Attack,
    resources::board::Board,
};

pub fn damage(
    mut monsters: Query<&mut Health, With<Monster>>,
    mut attack_evr: EventReader<Attack>,
) {
    for attack in attack_evr.iter() {
        if let Ok(mut health) = monsters.get_mut(attack.0) {
            health.0 -= attack.1;
        }
    }
}

pub fn death(
    mut commands: Commands,
    mut board: ResMut<Board>,
    monsters: Query<(Entity, &Health, &Coordinates), With<Monster>>,
) {
    for (monster, health, coord) in monsters.iter() {
        if health.0 <= 0 {
            board.monsters.remove(coord);
            commands.entity(monster).despawn();
        }
    }
}
