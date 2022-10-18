use bevy::prelude::Entity;

use crate::components::{allegiance::Allegiance, coordinates::Coordinates, health::Health};

pub struct Character {
    pub id: Entity,
    pub allegiance: Allegiance,
    pub health: Health,
    pub position: Coordinates,
}

impl Character {
    pub fn new(
        id: &Entity,
        allegiance: &Allegiance,
        health: &Health,
        position: &Coordinates,
    ) -> Self {
        Self {
            id: *id,
            allegiance: *allegiance,
            health: *health,
            position: *position,
        }
    }
}

pub struct BoardState {
    pub characters: Vec<Character>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }
}
