

use bevy::{
    prelude::{Query, Res, ResMut, With, Without},
    time::Time,
};

use crate::{
    components::{tick_timer::TickTimer, tower::Tower},
    resources::game_step_timer::GameStepTimer,
};

pub fn tick(
    time: Res<Time>,
    mut tick_timer: ResMut<GameStepTimer>,
    mut tower_timers: Query<&mut TickTimer, With<Tower>>,
    mut monster_timers: Query<&mut TickTimer, Without<Tower>>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        match tick_timer.1 {
            true => {
                for mut timer in tower_timers.iter_mut() {
                    timer.tick();
                    timer.set_active(true);
                }
                for mut timer in monster_timers.iter_mut() {
                    timer.set_active(false);
                }
            }
            false => {
                for mut timer in monster_timers.iter_mut() {
                    timer.tick();
                    timer.set_active(true);
                }
                for mut timer in tower_timers.iter_mut() {
                    timer.set_active(false);
                }
            }
        };
        tick_timer.1 = !tick_timer.1;
    }
}

pub fn reset(mut tick_timers: Query<&mut TickTimer>) {
    for mut tick_timer in tick_timers.iter_mut() {
        tick_timer.reset();
    }
}
