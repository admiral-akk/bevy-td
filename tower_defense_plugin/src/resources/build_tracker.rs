use bevy::prelude::{Entity, EventWriter};

use crate::{
    components::coordinates::Coordinates,
    events::{EnterBuildTarget, ExitBuildTarget},
};

pub struct BuildTracker {
    pub target: Option<Coordinates>,
    pub blueprint: Option<Entity>,
}

impl BuildTracker {
    pub fn clear_target(&mut self, clear_target_ewr: &mut EventWriter<ExitBuildTarget>) {
        if let Some(target) = self.target {
            clear_target_ewr.send(ExitBuildTarget(target));
        }
        self.target = None;
    }
    pub fn set_target(
        &mut self,
        target: Coordinates,
        set_target_ewr: &mut EventWriter<EnterBuildTarget>,
        clear_target_ewr: &mut EventWriter<ExitBuildTarget>,
    ) {
        if let Some(target) = self.target {
            clear_target_ewr.send(ExitBuildTarget(target));
        }
        self.target = Some(target);
        set_target_ewr.send(EnterBuildTarget(target));
    }
}
