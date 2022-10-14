use bevy::utils::HashMap;

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    events::AttackEvent,
    resources::board::Board,
};

pub trait Attack {
    fn target(
        &self,
        entities: HashMap<Coordinates, Allegiance>,
        active: (Coordinates, Allegiance),
        board: &Board,
    ) -> Option<AttackEvent>;
}
