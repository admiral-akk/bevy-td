use bevy::prelude::{Component, Entity};

use crate::components::{allegiance::Allegiance, coordinates::Coordinates};

pub trait Aura<T: Component> {
    fn targets(
        &self,
        entities: &Vec<(Coordinates, Allegiance)>,
        active: (Entity, Coordinates, Allegiance),
    ) -> (T, Vec<Coordinates>);
}
