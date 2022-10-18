use super::{attack::Attack, priority::ProposedAttack};
use bevy::{
    prelude::{Component},
};

use crate::{
    components::{targetting::target::Targets},
    structs::board_state::{BoardState, Character},
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Backstab {
    pub base_damage: i32,
    pub multiplier: i32,
}

impl Backstab {
    pub fn new(base_damage: i32, multiplier: i32) -> Self {
        Self {
            base_damage,
            multiplier,
        }
    }
}

impl Attack for Backstab {
    fn priority(
        &self,
        attacker: Character,
        board_state: &BoardState,
        targets: &Targets,
    ) -> Vec<ProposedAttack> {
        let mut priority = Vec::new();

        for target in &targets.0 {
            let damage = match board_state
                .get_neighbours(target.position)
                .iter()
                .filter(|c| {
                    c.allegiance.eq(&attacker.allegiance) && !c.position.eq(&attacker.position)
                })
                .count()
                > 0
            {
                true => self.base_damage * self.multiplier,
                false => self.base_damage,
            };
            priority.push(ProposedAttack {
                damage: damage,
                attacker,
                defender: *target,
            });
        }
        priority.sort_by(|a, b| a.damage.cmp(&b.damage));
        priority
    }
}
