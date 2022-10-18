use bevy::prelude::{Commands, Entity, Query};

use crate::components::{debuffs::taunt::Taunt, targetting::target::Targets};

pub fn taunted(mut commands: Commands, mut taunted: Query<(Entity, &mut Targets, &Taunt)>) {
    for (entity, mut targets, taunt) in taunted.iter_mut() {
        if let Some(taunting_entity) = targets.0.iter().find(|c| c.id.eq(&taunt.0)) {
            targets.0 = vec![*taunting_entity];
        }
        commands.entity(entity).remove::<Taunt>();
    }
}
