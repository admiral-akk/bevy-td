use bevy::{
    ecs::schedule::StateData,
    prelude::{Plugin, SystemSet},
};

use super::{
    events::Reward,
    reward::{
        button::handle_reward,
        ui::{add_reward_ui, remove_reward_ui},
    },
};

pub struct RewardPlugin<T: StateData> {
    active_state: T,
}

impl<T: StateData> RewardPlugin<T> {
    pub fn new(active_state: T) -> Self {
        Self { active_state }
    }
}
impl<T: StateData> Plugin for RewardPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(self.active_state.clone()).with_system(add_reward_ui),
        )
        .add_system_set(SystemSet::on_update(self.active_state.clone()).with_system(handle_reward))
        .add_system_set(SystemSet::on_exit(self.active_state.clone()).with_system(remove_reward_ui))
        .add_event::<Reward>();
    }
}
