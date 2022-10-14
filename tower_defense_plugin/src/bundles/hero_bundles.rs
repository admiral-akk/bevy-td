

use crate::components::{
    allegiance::Allegiance, coordinates::Coordinates, health::Health, hero::Hero, power::Power, start::Start, unit::Unit,
};

use bevy::{
    prelude::{Bundle, Entity, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

#[derive(Bundle, Default)]
pub struct HeroBundle {
    name: Name,
    hero: Hero,
    health: Health,
    coordinates: Coordinates,
    start: Start,
    power: Power,
    allegiance: Allegiance,
    unit: Unit,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}
impl HeroBundle {
    pub fn new(coordinates: Coordinates, transform: Transform, actions: &[Entity]) -> Self {
        HeroBundle {
            name: Name::new("Hero"),
            hero: Hero,
            allegiance: Allegiance(0),
            health: Health::new(30),
            power: Power(3),
            start: Start(coordinates),
            coordinates,
            unit: Unit(Vec::from(actions)),
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            visibility: VisibilityBundle::default(),
        }
    }
}
