use std::collections::HashSet;

use bevy::prelude::{Component, Entity};

use crate::components::{allegiance::Allegiance, coordinates::Coordinates, debuffs::taunt::Taunt};

use super::aura::Aura;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TauntAura(pub u32);

impl Aura<Taunt> for TauntAura {
    fn targets(
        &self,
        entities: &Vec<(Coordinates, Allegiance)>,
        active: (Entity, Coordinates, Allegiance),
    ) -> (Taunt, Vec<Coordinates>) {
        let enemies: HashSet<Coordinates> = entities
            .iter()
            .filter(|(_, allegiance)| !allegiance.eq(&active.2))
            .map(|(coord, _)| *coord)
            .collect();
        let mut ret = Vec::new();
        for neighbour in active.1.orthogonal_neighbours(self.0 as i16) {
            if enemies.contains(&neighbour) {
                ret.push(neighbour);
            }
        }
        (Taunt(active.0), ret)
    }
}
