

use crate::{
    components::{targetting::target::Targets},
    structs::board_state::{BoardState, Character},
};

use super::priority::ProposedAttack;

pub trait Attack {
    fn priority(
        &self,
        attacker: Character,
        board_state: &BoardState,
        targets: &Targets,
    ) -> Vec<ProposedAttack>;
}
