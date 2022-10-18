use super::{attack::Attack, priority::ProposedAttack};
use bevy::{
    prelude::{Component, Entity},
    utils::HashMap,
};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct MeleeAttack(pub i32);

impl Attack for MeleeAttack {
    fn priority(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance, Entity),
        board: &Board,
    ) -> Vec<ProposedAttack> {
        let mut priority = Vec::new();
        for neighbour in active.0.orthogonal_neighbours(1) {
            if let Some(allegiance) = entities.get(&neighbour) {
                if !allegiance.eq(&active.1) {
                    if let Some(&entity) = board.entities.get(&neighbour) {
                        priority.push(ProposedAttack {
                            damage: self.0,
                            attacker: active.2,
                            defender: entity,
                        });
                    }
                }
            }
        }
        priority
    }
}
