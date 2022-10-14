use bevy::{
    ecs::schedule::{ShouldRun, StateData},
    prelude::{
        ParallelSystemDescriptorCoercion, Res, Schedule, Stage, StageLabel, State, SystemSet,
        SystemStage,
    },
};

use crate::{
    components::{attacks::melee::MeleeAttack, movements::charging::Charging},
    systems::{
        attack::{try_attack},
        health::{damage, death, update_health_bar},
        life::check_units,
        movement::apply_move,
        turn_order::tick_active,
    },
};
pub struct ActionStage {
    schedule: Schedule,
}

#[derive(Copy, Clone, StageLabel)]
pub enum GameStage {
    Tick,
    Move,
    Attack,
    ResolveAttack,
    CheckEnd,
    CleanUp,
}

impl GameStage {
    const STAGES: [GameStage; 6] = [
        GameStage::Tick,
        GameStage::Attack,
        GameStage::ResolveAttack,
        GameStage::Move,
        GameStage::CheckEnd,
        GameStage::CleanUp,
    ];

    pub fn add_stages(schedule: &mut Schedule) {
        for i in 0..GameStage::STAGES.len() {
            if i == 0 {
                schedule.add_stage(GameStage::STAGES[i], SystemStage::single_threaded());
            } else {
                schedule.add_stage_after(
                    GameStage::STAGES[i - 1],
                    GameStage::STAGES[i],
                    SystemStage::single_threaded(),
                );
            }
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

impl ActionStage {
    pub fn new<T: StateData>(active_state: T) -> Self {
        let mut schedule = Schedule::default();
        GameStage::add_stages(&mut schedule);
        schedule
            .add_system_set_to_stage(
                GameStage::Tick,
                system_set(active_state.clone()).with_system(tick_active),
            )
            .add_system_set_to_stage(
                GameStage::Move,
                system_set(active_state.clone()).with_system(apply_move::<Charging>),
            )
            .add_system_set_to_stage(
                GameStage::Attack,
                system_set(active_state.clone()).with_system(try_attack::<MeleeAttack>),
            )
            .add_system_set_to_stage(
                GameStage::ResolveAttack,
                system_set(active_state.clone())
                    .with_system(damage)
                    .with_system(death.after(damage))
                    .with_system(update_health_bar.after(damage)),
            )
            .add_system_set_to_stage(
                GameStage::CheckEnd,
                system_set(active_state.clone()).with_system(check_units),
            );
        ActionStage { schedule }
    }
}

impl Stage for ActionStage {
    fn run(&mut self, world: &mut bevy::prelude::World) {
        self.schedule.run_once(world);
    }
}