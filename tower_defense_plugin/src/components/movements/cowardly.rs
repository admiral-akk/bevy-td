use bevy::prelude::{Component, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

use super::movement::Movement;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Cowardly(pub u16);

impl Movement for Cowardly {
    fn next(
        &self,
        entities: Vec<(Coordinates, Allegiance)>,
        active: (Coordinates, Allegiance),
        board: &Res<Board>,
    ) -> Option<Coordinates> {
        let all: Vec<Coordinates> = entities
            .iter()
            .map(|(coord, _)| *coord)
            .filter(|coord| !coord.eq(&active.0))
            .collect();
        let enemies: Vec<Coordinates> = entities
            .iter()
            .filter(|(_, allegiance)| !active.1.eq(allegiance))
            .map(|(coord, _)| *coord)
            .collect();
        let allies: Vec<Coordinates> = entities
            .iter()
            .filter(|(coord, allegiance)| !active.0.eq(coord) && active.1.eq(allegiance))
            .map(|(coord, _)| *coord)
            .collect();
        let enemies = Coordinates::distance_field(&enemies, &all);
        let allies = Coordinates::distance_field(&allies, &all);

        let potential_next = active.0.orthogonal_neighbours(self.0 as i16);
        let mut current_pos = active.0;
        for next in potential_next {
            if board.entities.contains_key(&next) {
                continue;
            }
            if !enemies.contains_key(&next) || !allies.contains_key(&next) {
                continue;
            }
            if enemies[&next] < enemies[&current_pos] && enemies[&next] >= allies[&next] {
                current_pos = next;
            }
        }
        if board.entities.contains_key(&current_pos) {
            return None;
        } else {
            return Some(current_pos);
        }
    }
}
