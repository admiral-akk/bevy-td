use bevy::prelude::Component;

use crate::structs::board_state::{BoardState, Character};

use super::target::Target;

#[derive(Component)]
pub struct MeleeTarget;

impl Target for MeleeTarget {
    fn get_targets(&self, attacker: Character, state: &BoardState) -> Vec<Character> {
        state
            .get_neighbours(attacker.position)
            .iter()
            .filter(|c| !c.allegiance.eq(&attacker.allegiance))
            .map(|c| *c)
            .collect()
    }
}
