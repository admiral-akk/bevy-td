use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Health {
    pub max: i32,
    pub health: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { max, health: max }
    }
}
