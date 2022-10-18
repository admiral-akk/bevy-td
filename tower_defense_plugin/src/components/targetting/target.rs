use bevy::prelude::Component;

use crate::structs::board_state::{BoardState, Character};

pub trait Target {
    fn get_targets(&self, attacker: Character, state: &BoardState) -> Vec<Character>;
}

#[derive(Component)]
pub struct Targets(pub Vec<Character>);
