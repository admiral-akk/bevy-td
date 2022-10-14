use crate::{
    components::{
        allegiance::Allegiance, coordinates::Coordinates, health::Health, hero::Hero,
        movements::charging::Charging, power::Power, unit::Unit,
    },
};

use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

#[derive(Bundle, Default)]
pub struct HeroBundle {
    name: Name,
    hero: Hero,
    movement: Charging,
    health: Health,
    coordinates: Coordinates,
    power: Power,
    allegiance: Allegiance,
    unit: Unit,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}
impl HeroBundle {
    pub fn new(coordinates: Coordinates, transform: Transform) -> Self {
        HeroBundle {
            name: Name::new("Hero"),
            hero: Hero,
            movement: Charging(1),
            allegiance: Allegiance(1),
            health: Health::new(30),
            power: Power(3),
            coordinates,
            unit: Unit,
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            visibility: VisibilityBundle::default(),
        }
    }
}
