use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

use crate::components::{
    allegiance::Allegiance, coordinates::Coordinates, health::Health, monster::Monster, unit::Unit,
};

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    name: Name,
    monster: Monster,
    health: Health,
    coordinates: Coordinates,
    allegiance: Allegiance,
    unit: Unit,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}

impl MonsterBundle {
    pub fn new(coordinates: Coordinates, transform: Transform) -> Self {
        MonsterBundle {
            name: Name::new("Monster"),
            monster: Monster,
            allegiance: Allegiance(1),
            health: Health::new(3),
            coordinates,
            unit: Unit(Vec::new()),
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            visibility: VisibilityBundle::default(),
        }
    }
}
