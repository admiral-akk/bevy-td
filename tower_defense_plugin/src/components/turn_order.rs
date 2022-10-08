use std::collections::VecDeque;

use bevy::prelude::{Component, Entity};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TurnOrder(pub VecDeque<Entity>);
