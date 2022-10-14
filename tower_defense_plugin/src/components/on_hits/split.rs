use bevy::prelude::{Commands, Component, Query};

use crate::{
    components::{coordinates::Coordinates, health::Health, monster::Monster},
    events::HitEvent,
    resources::board::Board,
};

use super::on_hit::OnHit;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Split();

impl OnHit for Split {
    fn apply_effect(
        &self,
        _commands: &mut Commands,
        event: HitEvent,
        board: &Board,
        units: Query<(&Coordinates, &Monster, &Health)>,
    ) {
        if let Ok((coord, _monster, health)) = units.get(event.1) {
            if health.health > 0 {
                for neighbour in coord.orthogonal_neighbours(1) {
                    if board.entities.contains_key(&neighbour) {
                        continue;
                    }
                }
            }
        }
    }
}
