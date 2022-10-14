use std::collections::VecDeque;

use bevy::{
    prelude::{Component, EventReader, Query, Res},
    utils::HashSet,
};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates, movement::Movement},
    events::ActiveUnit,
    resources::board::Board,
};

pub fn apply_move<T: Movement + Component>(
    mut entities: Query<(&mut Coordinates, &Allegiance)>,
    movement: Query<&T>,
    mut active_ert: EventReader<ActiveUnit>,
    board: Res<Board>,
) {
    for ActiveUnit(entity) in active_ert.iter() {
        if let Ok(movement) = movement.get(*entity) {
            let targets: Vec<(Coordinates, Allegiance)> = entities
                .iter()
                .map(|(coord, allegiance)| (*coord, *allegiance))
                .collect();
            if let Ok((mut coord, allegiance)) = entities.get_mut(*entity) {
                if let Some(new_coord) = movement.next(targets, (*coord, *allegiance), &board) {
                    if !board.entities.contains_key(&new_coord) {
                        *coord = new_coord;
                    }
                }
            }
        }
    }
}

pub fn next(
    start: &Coordinates,
    targets: Vec<Coordinates>,
    board: &Res<Board>,
) -> Option<Coordinates> {
    let mut visited: HashSet<Coordinates> = HashSet::from_iter(targets.iter().map(|c| c.clone()));
    let mut queue = VecDeque::from_iter(targets.iter().map(|c| c.clone()));
    while let Some(next) = queue.pop_front() {
        for neighbour in next.orthogonal_neighbours() {
            if neighbour.eq(start) {
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

pub fn movement(
    mut entities: Query<(&mut Coordinates, &Allegiance)>,
    mut active_ert: EventReader<ActiveUnit>,
    board: Res<Board>,
) {
    for active in active_ert.iter() {
        let a = entities.get(active.0).unwrap().1;
        let targets: Vec<Coordinates> = entities
            .iter()
            .filter(|(_, allegiance)| !a.eq(allegiance))
            .map(|(coord, _)| *coord)
            .collect();

        let mut coord = entities.get_mut(active.0).unwrap().0;
        bevy::log::error!("Trying move: {:?}", coord);
        if let Some(next) = next(&coord, targets, &board) {
            bevy::log::error!("Move to: {:?}", next);
            if !board.entities.contains_key(&next) {
                *coord = next;
            }
        }
    }
}
