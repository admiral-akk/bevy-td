use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

use crate::components::{
    coordinates::Coordinates, health::Health, monster::Monster, tick_timer::TickTimer,
};

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    name: Name,
    monster: Monster,
    health: Health,
    coordinates: Coordinates,
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
            health: Health(3),
            coordinates,
            tick_timer: TickTimer::new(1),
            transform: TransformBundle {
                local: transform,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
