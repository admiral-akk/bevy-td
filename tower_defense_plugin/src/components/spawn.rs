use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Spawn {
    spawns_remaining: u32,
}

impl Spawn {
    pub fn new() -> Self {
        Spawn {
            spawns_remaining: 0,
        }
    }

    pub fn has_spawn(&self) -> bool {
        self.spawns_remaining > 0
    }

    pub fn spawn_creep(&mut self) {
        self.spawns_remaining -= 1;
    }

    pub fn set_creep_count(&mut self, count: u32) {
        self.spawns_remaining = count;
    }
}
