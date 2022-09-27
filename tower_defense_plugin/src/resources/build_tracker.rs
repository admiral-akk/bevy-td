use bevy::prelude::{Entity, EventWriter};

use crate::{
    components::coordinates::Coordinates,
    events::{EnterBuildTarget, HideBuildTarget},
};

pub struct BuildTracker {
    pub target: Option<Coordinates>,
    pub blueprint: Option<Entity>,
}

impl BuildTracker {
    pub fn clear_target(&mut self, clear_target_ewr: &mut EventWriter<HideBuildTarget>) {
        if let Some(target) = self.target {
            clear_target_ewr.send(HideBuildTarget);
        }
        self.target = None;
    }
    pub fn set_target(
        &mut self,
        target: Coordinates,
        set_target_ewr: &mut EventWriter<EnterBuildTarget>,
    ) {
        self.target = Some(target);
        set_target_ewr.send(EnterBuildTarget(target));
    }
}
