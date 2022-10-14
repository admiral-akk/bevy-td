use std::collections::{HashSet, VecDeque};

use bevy::prelude::{Component, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

use super::movement::Movement;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Charging(pub u16);

impl Movement for Charging {
    fn next(
        &self,
        entities: Vec<(Coordinates, Allegiance)>,
        active: (Coordinates, Allegiance),
        board: &Res<Board>,
    ) -> Option<Coordinates> {
        let targets: Vec<Coordinates> = entities
            .iter()
            .filter(|(_, allegiance)| !active.1.eq(allegiance))
            .map(|(coord, _)| *coord)
            .collect();
        let mut visited: HashSet<Coordinates> =
            HashSet::from_iter(targets.iter().map(|c| c.clone()));
        let mut queue = VecDeque::from_iter(targets.iter().map(|c| c.clone()));
        while let Some(next) = queue.pop_front() {
            for neighbour in next.orthogonal_neighbours(1) {
                if neighbour.eq(&active.0) {
                    return Some(next);
                } else if board.entities.contains_key(&neighbour) {
                    continue;
                } else if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }
        return None;
    }
}
