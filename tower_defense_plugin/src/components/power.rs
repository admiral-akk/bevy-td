use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Power(pub u32);
