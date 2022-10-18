use super::{
    attack::Attack,
    priority::{ProposedAttack},
};
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
    fn priority(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance, Entity),
        board: &Board,
    ) -> Vec<ProposedAttack> {
        let mut priority = Vec::new();
        for enemy in get_neighbouring_enemies(&entities, (active.0, active.1)) {
            let has_adjacent_ally = get_neighbouring_allies(&entities, (enemy, active.1))
                .iter()
                .filter(|coord| !(*coord).eq(&active.0))
                .count()
                > 0;
            let damage = match has_adjacent_ally {
                true => self.base_damage * self.multiplier,
                false => self.base_damage,
            };
            priority.push(ProposedAttack {
                damage: damage,
                attacker: active.2,
                defender: *board.entities.get(&enemy).unwrap(),
            });
        }
        priority.sort_by(|a, b| a.damage.cmp(&b.damage));
        priority
    }
}
