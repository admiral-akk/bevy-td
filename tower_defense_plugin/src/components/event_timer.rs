use bevy::{prelude::Component, time::Timer};

#[derive(Debug, Default, Clone, Component)]
pub struct EventTimer<Event: Copy + Clone + Send + Sync>(pub Timer, pub Event);
