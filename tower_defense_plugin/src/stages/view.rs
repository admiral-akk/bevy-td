use bevy::{
    ecs::schedule::{ShouldRun, StateData},
    prelude::{Res, Schedule, Stage, StageLabel, State, SystemSet, SystemStage},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::systems::turn_order::tick_active;
pub struct ViewStage {
    schedule: Schedule,
}

#[derive(Copy, Clone, EnumIter, StageLabel)]
pub enum ViewStep {
    Tick,
}

impl ViewStep {
    pub fn add_stages(schedule: &mut Schedule) {
        let mut prev_stage = None;
        for stage in ViewStep::iter() {
            match prev_stage {
                None => schedule.add_stage(stage, SystemStage::single_threaded()),
                Some(prev_stage) => {
                    schedule.add_stage_after(prev_stage, stage, SystemStage::single_threaded())
                }
            };
            prev_stage = Some(stage);
        }
    }
}

fn system_set<T: StateData>(active_state: T) -> SystemSet {
    SystemSet::new().with_run_criteria(move |state: Res<State<T>>| {
        if state.current().eq(&active_state) {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    })
}

impl ViewStage {
    pub fn new<T: StateData>(active_state: T) -> Self {
        let mut schedule = Schedule::default();
        ViewStep::add_stages(&mut schedule);
        schedule.add_system_set_to_stage(
            ViewStep::Tick,
            system_set(active_state.clone()).with_system(tick_active),
        );
        ViewStage { schedule }
    }
}

impl Stage for ViewStage {
    fn run(&mut self, world: &mut bevy::prelude::World) {
        self.schedule.run_once(world);
    }
}
