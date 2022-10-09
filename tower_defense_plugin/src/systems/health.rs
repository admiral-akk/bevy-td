use bevy::prelude::{
    Changed, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Query,
};

use crate::{
    components::health::Health,
    events::{Attack, Removed},
};

pub fn damage(mut monsters: Query<&mut Health>, mut attack_evr: EventReader<Attack>) {
    for attack in attack_evr.iter() {
        if let Ok(mut health) = monsters.get_mut(attack.0) {
            health.0 -= attack.1;
        }
    }
}

pub fn death(
    mut commands: Commands,
    entities: Query<(Entity, &Health), Changed<Health>>,
    mut removed_ewr: EventWriter<Removed>,
) {
    for (entity, health) in entities.iter() {
        if health.0 <= 0 {
            removed_ewr.send(Removed(entity));
            commands.entity(entity).despawn_recursive();
        }
    }
}
