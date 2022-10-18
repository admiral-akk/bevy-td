use bevy::prelude::{Entity, Query};

use crate::components::{allegiance::Allegiance, coordinates::Coordinates, health::Health};

pub struct Character {
    pub id: Entity,
    pub allegiance: Allegiance,
    pub health: Health,
    pub position: Coordinates,
}

impl Character {
    pub fn new(
        id: Entity,
        allegiance: &Allegiance,
        health: &Health,
        position: &Coordinates,
    ) -> Self {
        Self {
            id,
            allegiance: *allegiance,
            health: *health,
            position: *position,
        }
    }
}

pub struct BoardState {
    characters: Vec<Character>,
}

impl BoardState {
    pub fn new(entities: Query<(Entity, &Allegiance, &Health, &Coordinates)>) -> Self {
        Self {
            characters: entities
                .iter()
                .map(|character| Character::new(character.0, character.1, character.2, character.3))
                .collect(),
        }
    }
}
