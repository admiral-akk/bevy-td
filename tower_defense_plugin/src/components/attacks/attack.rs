use bevy::{prelude::Entity, utils::HashMap};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

use super::priority::ProposedAttack;

pub trait Attack {
    fn priority(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance, Entity),
        board: &Board,
    ) -> Vec<ProposedAttack>;
}
