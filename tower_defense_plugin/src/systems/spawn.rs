use bevy::{
    prelude::{EventWriter, Res, ResMut},
    time::Time,
};

use crate::{
    events::{Move, Spawn},
    resources::{
        spawn_timer::{MoveTimer, SpawnTimer},
        spawn_tracker::SpawnTracker,
    },
};
pub fn monster_tick(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut move_timer: ResMut<MoveTimer>,
    mut spawn_ewr: EventWriter<Spawn>,
    mut move_ewr: EventWriter<Move>,
    spawn_tracker: Res<SpawnTracker>,
) {
    spawn_timer.0.tick(time.delta());
    move_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() && spawn_tracker.0 > 0 {
        spawn_ewr.send(Spawn);
    }
    if move_timer.0.just_finished() {
        move_ewr.send(Move);
    }
}
