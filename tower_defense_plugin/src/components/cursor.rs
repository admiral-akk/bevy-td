use bevy::prelude::Component;

use crate::components::coordinates::Coordinates;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Cursor(pub Option<Coordinates>);
