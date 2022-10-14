use assets_plugin::resources::monsters::MonsterType;
use bevy::prelude::Component;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Monster(pub MonsterType);
