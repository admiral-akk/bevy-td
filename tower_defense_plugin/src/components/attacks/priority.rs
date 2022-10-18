use bevy::prelude::{Component};

use crate::structs::board_state::Character;

pub struct ProposedAttack {
    pub damage: i32,
    pub attacker: Character,
    pub defender: Character,
}

#[derive(Component)]
pub struct Attacks(pub Vec<ProposedAttack>);
