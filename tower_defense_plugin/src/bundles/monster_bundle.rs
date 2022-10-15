use assets_plugin::resources::monsters::MonsterType;
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
    pub fn new(coordinates: Coordinates, transform: Transform, monster_type: MonsterType) -> Self {
        MonsterBundle {
            name: Name::new("Monster"),
            monster: Monster(monster_type),
            allegiance: Allegiance(1),
            health: Health::new(5),
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
