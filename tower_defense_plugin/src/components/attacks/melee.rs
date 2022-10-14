use super::attack::Attack;
use bevy::{prelude::Component, utils::HashMap};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    events::AttackEvent,
    resources::board::Board,
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct MeleeAttack(pub i32);

impl Attack for MeleeAttack {
    fn target(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance),
        board: &Board,
    ) -> Option<AttackEvent> {
        for neighbour in active.0.orthogonal_neighbours(1) {
            if let Some(allegiance) = entities.get(&neighbour) {
                if !allegiance.eq(&active.1) {
                    if let Some(&entity) = board.entities.get(&neighbour) {
                        return Some(AttackEvent(entity, self.0));
                    }
                }
            }
        }
        return None;
    }
}
