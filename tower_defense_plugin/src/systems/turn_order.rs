use bevy::{
    prelude::{Added, Entity, EventReader, EventWriter, Query, Res, ResMut},
    time::Time,
};

use crate::{
    components::{turn_order::TurnOrder, unit::Unit},
    events::{ActiveUnit, Removed},
    resources::game_step_timer::GameStepTimer,
};

pub fn add_turn(mut turn_order: Query<&mut TurnOrder>, added_entities: Query<Entity, Added<Unit>>) {
    for entity in added_entities.iter() {
        turn_order.single_mut().0.push_back(entity);
    }
}

pub fn tick_active(
    time: Res<Time>,
    mut tick_timer: ResMut<GameStepTimer>,
    mut turn_order: Query<&mut TurnOrder>,
    mut active_ewr: EventWriter<ActiveUnit>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        let turn_order = &mut turn_order.single_mut().0;
        let active = turn_order.pop_front().unwrap();
        active_ewr.send(ActiveUnit(active));
        turn_order.push_back(active);
    }
}

pub fn remove_turn(mut turn_order: Query<&mut TurnOrder>, mut removed_ewr: EventReader<Removed>) {
    let turn_order = &mut turn_order.single_mut().0;
    for removed in removed_ewr.iter() {
        bevy::log::error!("Attempt removed unit!");
        for (i, e) in turn_order.iter().enumerate() {
            if removed.0.eq(e) {
                bevy::log::error!("Removed unit!");
                turn_order.remove(i);
                break;
            }
        }
    }
}
