use bevy::prelude::Component;

use crate::entities::towers::TowerType;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Blueprint(pub TowerType);
