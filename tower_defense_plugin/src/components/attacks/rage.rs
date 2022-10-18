use super::{attack::Attack, priority::ProposedAttack};
use bevy::prelude::Component;

use crate::{
    components::targetting::target::Targets,
    structs::board_state::{BoardState, Character},
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Rage(pub i32);

impl Attack for Rage {
    fn priority(
        &self,
        attacker: Character,
        _board_state: &BoardState,
        targets: &Targets,
    ) -> Vec<ProposedAttack> {
        let mut priority = Vec::new();

        for target in &targets.0 {
            priority.push(ProposedAttack {
                damage: self.0 + (attacker.health.max - attacker.health.health),
                attacker,
                defender: *target,
            });
        }
        priority.sort_by(|a, b| a.damage.cmp(&b.damage));
        priority
    }
}
