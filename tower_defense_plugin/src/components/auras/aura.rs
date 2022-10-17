use bevy::prelude::Component;

use crate::components::{allegiance::Allegiance, coordinates::Coordinates};

pub trait Aura<T: Component> {
    fn targets(
        &self,
        entities: &Vec<(Coordinates, Allegiance)>,
        active: (Coordinates, Allegiance),
    ) -> (T, Vec<Coordinates>);
}
