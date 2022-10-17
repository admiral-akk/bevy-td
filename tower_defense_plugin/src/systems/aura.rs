use std::collections::HashMap;

use bevy::prelude::{Commands, Component, Entity, Query};

use crate::components::{allegiance::Allegiance, auras::aura::Aura, coordinates::Coordinates};

pub fn apply_aura<Buff: Component + Clone, T: Component + Aura<Buff>>(
    mut commands: Commands,
    auras: Query<(&T, &Coordinates, &Allegiance)>,
    entities: Query<(Entity, &Coordinates, &Allegiance)>,
) {
    let all = entities
        .iter()
        .map(|(_, coord, allegiance)| (*coord, *allegiance))
        .collect();
    let coord_to_entity = entities
        .iter()
        .map(|(entity, coord, _)| (*coord, entity))
        .collect::<HashMap<Coordinates, Entity>>();
    for (aura, coord, allegiance) in auras.iter() {
        bevy::log::error!("Attemptin aura!");
        let (buff, targets) = aura.targets(&all, (*coord, *allegiance));
        for target in targets {
            commands
                .entity(coord_to_entity[&target])
                .insert(buff.clone());
        }
    }
}
