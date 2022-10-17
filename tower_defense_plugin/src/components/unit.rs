use bevy::prelude::{Component, Entity};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Unit {
    pub actions: Vec<Entity>,
}
