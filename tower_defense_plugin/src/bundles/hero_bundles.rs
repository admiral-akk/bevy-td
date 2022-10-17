use crate::components::{
    allegiance::Allegiance, coordinates::Coordinates, health::Health, hero::Hero, start::Start,
    unit::Unit,
};

use assets_plugin::resources::heroes::HeroType;
use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

#[derive(Bundle, Default)]
pub struct HeroBundle {
    name: Name,
    hero: Hero,
    health: Health,
    coordinates: Coordinates,
    start: Start,
    allegiance: Allegiance,
    unit: Unit,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}
impl HeroBundle {
    pub fn new(coordinates: Coordinates, transform: Transform, hero_type: HeroType) -> Self {
        HeroBundle {
            name: Name::new("Hero"),
            hero: Hero(hero_type),
            allegiance: Allegiance(0),
            health: Health::new(30),
            start: Start(coordinates),
            coordinates,
            unit: Unit {
                actions: Vec::new(),
            },
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            visibility: VisibilityBundle::default(),
        }
    }
}
