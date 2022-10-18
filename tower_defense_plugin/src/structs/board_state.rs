use bevy::prelude::{Entity, Query};

use crate::components::{allegiance::Allegiance, coordinates::Coordinates, health::Health};

#[derive(Clone, Copy, Debug)]
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
    pub fn new(entities: &Query<(Entity, &Allegiance, &Health, &Coordinates)>) -> Self {
        Self {
            characters: entities
                .iter()
                .map(|character| Character::new(character.0, character.1, character.2, character.3))
                .collect(),
        }
    }

    pub fn get(&self, entity: Entity) -> Option<Character> {
        for char in self
            .characters
            .iter()
            .filter(|c| entity.eq(&c.id))
            .map(|c| c.clone())
        {
            return Some(char);
        }
        return None;
    }

    pub fn get_neighbours(&self, coord: Coordinates) -> Vec<Character> {
        let neighbours = coord.orthogonal_neighbours(1);
        self.characters
            .iter()
            .filter(|c| neighbours.contains(&c.position))
            .map(|c| c.clone())
            .collect()
    }
}
