use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TickTimer {
    duration: u32,
    tick: u32,
    active: bool,
}

impl TickTimer {
    pub fn new(duration: u32) -> Self {
        TickTimer {
            duration,
            ..Default::default()
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn active(&mut self) -> bool {
        if self.tick >= self.duration && self.active {
            self.tick = 0;
            self.active = false;
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn reset(&mut self) {
        self.tick = self.duration;
    }
}
