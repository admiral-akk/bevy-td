use super::attack::Attack;
use bevy::{
    prelude::{Component, Entity},
    utils::HashMap,
};

use crate::{
    components::{
        allegiance::{Allegiance},
        coordinates::Coordinates,
    },
    events::AttackEvent,
    resources::board::Board,
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

fn get_neighbouring_allies(
    board: &HashMap<Coordinates, Allegiance>,
    entity: (Coordinates, Allegiance),
) -> Vec<Coordinates> {
    entity
        .0
        .orthogonal_neighbours(1)
        .iter()
        .filter(|coord| board.contains_key(coord) && board[coord].eq(&entity.1))
        .map(|coord| *coord)
        .collect()
}

fn get_neighbouring_enemies(
    board: &HashMap<Coordinates, Allegiance>,
    entity: (Coordinates, Allegiance),
) -> Vec<Coordinates> {
    entity
        .0
        .orthogonal_neighbours(1)
        .iter()
        .filter(|coord| board.contains_key(coord) && !board[coord].eq(&entity.1))
        .map(|coord| *coord)
        .collect()
}

impl Attack for Backstab {
    fn target(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance, Entity),
        board: &Board,
    ) -> Option<AttackEvent> {
        let mut target = None;
        let mut is_backstab = false;
        for enemy in get_neighbouring_enemies(&entities, (active.0, active.1)) {
            for ally in get_neighbouring_allies(&entities, (enemy, active.1)) {
                if ally.eq(&active.0) {
                    continue;
                }
                target = Some(enemy);
                is_backstab = true;
            }
            if target.is_none() {
                target = Some(enemy);
            }
        }
        if target.is_some() {
            let damage = match is_backstab {
                true => self.base_damage * self.multiplier,
                false => self.base_damage,
            };
            return Some(AttackEvent {
                attacker: *board.entities.get(&active.0).unwrap(),
                defender: *board.entities.get(&target.unwrap()).unwrap(),
                damage,
            });
        }
        return None;
    }
}
