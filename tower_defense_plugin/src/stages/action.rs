use bevy::{
    ecs::schedule::{ShouldRun, StateData},
    prelude::{
        ParallelSystemDescriptorCoercion, Res, Schedule, Stage, StageLabel, State, SystemSet,
        SystemStage,
    },
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    components::{
        attacks::{backstab::Backstab, melee::MeleeAttack},
        auras::{root::RootAura, taunt::TauntAura},
        debuffs::{root::Root, taunt::Taunt},
        movements::{cautious::Cautious, charging::Charging, cowardly::Cowardly},
        on_hits::split::Split,
        targetting::melee::MeleeTarget,
    },
    systems::{
        attack::try_attack,
        aura::apply_aura,
        health::{damage, death, update_health_bar},
        life::check_units,
        movement::{apply_move, propose_move},
        on_hit::on_hit,
        root::rooted,
        target::try_target,
        turn_order::tick_active,
    },
};
pub struct ActionStage {
    schedule: Schedule,
}

#[derive(Copy, Clone, EnumIter, StageLabel)]
pub enum GameStage {
    Tick,
    ProposeMove,
    ModifyMove,
    ApplyMove,
    PostMove,
    GenerateTargets,
    GenerateAttacks,
    FilterAttacks,
    ResolveAttack,
    OnHit,
    CheckEnd,
    CleanUp,
}

impl GameStage {
    pub fn add_stages(schedule: &mut Schedule) {
        let mut prev_stage = None;
        for stage in GameStage::iter() {
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
                GameStage::ProposeMove,
                system_set(active_state.clone())
                    .with_system(propose_move::<Charging>)
                    .with_system(propose_move::<Cautious>)
                    .with_system(propose_move::<Cowardly>),
            )
            .add_system_set_to_stage(
                GameStage::ModifyMove,
                system_set(active_state.clone()).with_system(rooted),
            )
            .add_system_set_to_stage(
                GameStage::ApplyMove,
                system_set(active_state.clone()).with_system(apply_move),
            )
            .add_system_set_to_stage(
                GameStage::PostMove,
                system_set(active_state.clone())
                    .with_system(apply_aura::<Root, RootAura>)
                    .with_system(apply_aura::<Taunt, TauntAura>),
            )
            .add_system_set_to_stage(
                GameStage::GenerateTargets,
                system_set(active_state.clone()).with_system(try_target::<MeleeTarget>),
            )
            .add_system_set_to_stage(
                GameStage::GenerateAttacks,
                system_set(active_state.clone())
                    .with_system(try_attack::<MeleeAttack>)
                    .with_system(try_attack::<Backstab>),
            )
            .add_system_set_to_stage(GameStage::FilterAttacks, system_set(active_state.clone()))
            .add_system_set_to_stage(
                GameStage::ResolveAttack,
                system_set(active_state.clone())
                    .with_system(damage)
                    .with_system(death.after(damage))
                    .with_system(update_health_bar.after(damage)),
            )
            .add_system_set_to_stage(
                GameStage::OnHit,
                system_set(active_state.clone()).with_system(on_hit::<Split>),
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
