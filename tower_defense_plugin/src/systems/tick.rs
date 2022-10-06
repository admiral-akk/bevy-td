use bevy::{
    prelude::{Query, Res, ResMut},
    time::Time,
};

use crate::{components::tick_timer::TickTimer, resources::game_step_timer::GameStepTimer};

pub fn tick(
    time: Res<Time>,
    mut tick_timer: ResMut<GameStepTimer>,
    mut tick_timers: Query<&mut TickTimer>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        for mut tick_timer in tick_timers.iter_mut() {
            tick_timer.tick();
        }
    }
}

pub fn reset(mut tick_timers: Query<&mut TickTimer>) {
    for mut tick_timer in tick_timers.iter_mut() {
        tick_timer.reset();
    }
}
