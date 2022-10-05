
use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TickTimer {
    duration: u32,
    tick: u32,
}

impl TickTimer {
    pub fn new(duration: u32) -> Self {
        TickTimer { duration, tick: 0 }
    }

    pub fn active(&self) -> bool {
        self.tick == 0
    }

    pub fn tick(&mut self) {
        self.tick = (self.tick + 1) % self.duration;
    }
}
