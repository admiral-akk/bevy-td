use bevy::prelude::{Commands, Component};

use crate::{
    components::{coordinates::Coordinates, health::Health, monster::Monster},
    entities::monster::add_monster,
    events::HitEvent,
    resources::board::Board,
};

use super::on_hit::OnHit;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Split;

impl OnHit for Split {
    fn apply_effect(
        &self,
        commands: &mut Commands,
        _event: HitEvent,
        board: &Board,
        defender: (Coordinates, Monster, Health),
    ) {
        let (coord, monster, health) = defender;
        if health.health > 0 {
            for neighbour in coord.orthogonal_neighbours(1) {
                if board.entities.contains_key(&neighbour) {
                    continue;
                }
                let monster = add_monster(commands, neighbour, board, monster.0);
                commands.entity(monster).insert(health.clone());
                break;
            }
        }
    }
}
