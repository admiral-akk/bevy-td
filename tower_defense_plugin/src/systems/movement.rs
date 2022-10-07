use std::{collections::VecDeque};

use bevy::{
    prelude::{Changed, Entity, Query, Res},
    utils::HashSet,
};

use crate::{
    components::{
        allegiance::Allegiance, coordinates::Coordinates, movement::Movement, tick_timer::TickTimer,
    },
    resources::board::Board,
};

pub fn next(start: &Coordinates, targets: Vec<Coordinates>) -> Option<Coordinates> {
    let mut visited: HashSet<Coordinates> = HashSet::from_iter(targets.iter().map(|c| c.clone()));
    let mut queue = VecDeque::from_iter(targets.iter().map(|c| c.clone()));
    while let Some(next) = queue.pop_front() {
        for neighbour in next.orthogonal_neighbours() {
            if neighbour.eq(start) {
                return Some(next);
            } else if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                queue.push_back(neighbour);
            }
        }
    }
    return None;
}

pub fn movement(
    mut moving: Query<(Entity, &mut TickTimer, &Movement), Changed<TickTimer>>,
    mut entities: Query<(&mut Coordinates, &Allegiance)>,
    board: Res<Board>,
) {
    for (entity, mut timer, _) in moving.iter_mut() {
        if timer.active() {
            if let Ok((_, a)) = entities.get(entity) {
                let targets: Vec<Coordinates> = entities
                    .iter()
                    .filter(|(_, allegiance)| !a.eq(allegiance))
                    .map(|(coord, _)| *coord)
                    .collect();
                if let Ok((mut c, _)) = entities.get_mut(entity) {
                    if let Some(next) = next(&c, targets) {
                        if !board.entities.contains_key(&next) {
                            *c = next;
                        }
                    }
                }
            }
        }
    }
}
