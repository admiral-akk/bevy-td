use assets_plugin::resources::heroes::HeroType;
use bevy::prelude::Component;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Hero(pub HeroType);
