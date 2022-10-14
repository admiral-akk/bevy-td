use bevy::prelude::{Component, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

use super::movement::Movement;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Cautious(pub u16, pub i16);

impl Movement for Cautious {
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
        let targets: Vec<Coordinates> = entities
            .iter()
            .filter(|(_, allegiance)| !active.1.eq(allegiance))
            .map(|(coord, _)| *coord)
            .collect();

        let distance_field = Coordinates::distance_field(&targets, &all);
        let potential_next = active.0.orthogonal_neighbours(self.0 as i16);
        let mut current_pos = active.0;
        for next in potential_next {
            if board.entities.contains_key(&next) {
                continue;
            }
            if !distance_field.contains_key(&next) {
                continue;
            }
            if (distance_field[&next] as i16 - self.1).abs()
                < (distance_field[&current_pos] as i16 - self.1).abs()
            {
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
