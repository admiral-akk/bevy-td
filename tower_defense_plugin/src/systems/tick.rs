use bevy::{
    prelude::{Query, Res, ResMut, With},
    time::Time,
};

use crate::{components::tick_timer::TickTimer, resources::spawn_timer::GameTickTimer};

pub fn tick(
    time: Res<Time>,
    mut tick_timer: ResMut<GameTickTimer>,
    mut tick_timers: Query<&mut TickTimer, With<TickTimer>>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        for mut tick_timer in tick_timers.iter_mut() {
            tick_timer.tick();
        }
    }
}
