use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

use crate::components::{
    allegiance::Allegiance, coordinates::Coordinates, health::Health, monster::Monster,
    movement::Movement, power::Power, tick_timer::TickTimer,
};

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    name: Name,
    monster: Monster,
    health: Health,
    movement: Movement,
    coordinates: Coordinates,
    power: Power,
    allegiance: Allegiance,
    tick_timer: TickTimer,
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
            health: Health(3),
            movement: Movement(1),
            power: Power(1),
            coordinates,
            tick_timer: TickTimer::new(1),
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            visibility: VisibilityBundle::default(),
        }
    }
}
