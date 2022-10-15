use bevy::prelude::Res;

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates},
    resources::board::Board,
};

pub trait Movement {
    fn next(
        &self,
        entities: Vec<(Coordinates, Allegiance)>,
        active: (Coordinates, Allegiance),
        board: &Res<Board>,
    ) -> Option<Coordinates>;
}
