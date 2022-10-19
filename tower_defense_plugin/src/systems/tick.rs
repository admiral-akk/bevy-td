use bevy::{
    prelude::{Commands, Entity, EventWriter, Query, Res},
    time::Time,
};

use crate::components::event_timer::EventTimer;

pub fn event_tick<EventType: Copy + Clone + Send + Sync + 'static>(
    time: Res<Time>,
    mut commands: Commands,
    mut timers: Query<(Entity, &mut EventTimer<EventType>)>,
    mut event_wr: EventWriter<EventType>,
) {
    for (entity, mut event_timer) in timers.iter_mut() {
        event_timer.0.tick(time.delta());
        if event_timer.0.just_finished() {
            event_wr.send(event_timer.1);
            commands.entity(entity).despawn();
        }
    }
}
