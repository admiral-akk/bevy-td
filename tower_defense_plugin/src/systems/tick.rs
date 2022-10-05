use bevy::{
    prelude::{EventWriter, Res, ResMut},
    time::Time,
};

use crate::{
    events::{Tick},
    resources::{
        spawn_timer::{GameTickTimer},
    },
};

pub fn tick(
    time: Res<Time>,
    mut tick_timer: ResMut<GameTickTimer>,
    mut tick_ewr: EventWriter<Tick>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        tick_ewr.send(Tick);
    }
}
