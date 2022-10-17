use bevy::{
    prelude::{Added, Entity, EventWriter, Query, RemovedComponents, Res, ResMut},
    time::Time,
};

use crate::{
    components::{turn_order::TurnOrder, unit::Unit},
    events::ActiveAction,
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
    units: Query<&Unit>,
    mut action_ewr: EventWriter<ActiveAction>,
) {
    tick_timer.0.tick(time.delta());
    if tick_timer.0.just_finished() {
        let mut turn_order = turn_order.single_mut();
        let active = *turn_order.0.front().unwrap();
        if let Ok(unit) = units.get(active) {
            let actions = &unit.actions;
            if actions.len() > turn_order.1 {
                action_ewr.send(ActiveAction(actions[turn_order.1]));
                turn_order.1 = turn_order.1 + 1;
            } else {
                turn_order.0.pop_front();
                turn_order.0.push_back(active);
                turn_order.1 = 0;
            }
        }
    }
}

pub fn reset_turn(mut turn_order: Query<&mut TurnOrder>) {
    turn_order.single_mut().1 = 0;
}

pub fn remove_turn(mut turn_order: Query<&mut TurnOrder>, removed: RemovedComponents<Unit>) {
    let queue = &mut turn_order.single_mut();
    for removed in removed.iter() {
        bevy::log::error!("Attempt removed unit!");
        for (i, e) in queue.0.iter().enumerate() {
            if removed.eq(e) {
                bevy::log::error!("Removed unit!");
                if i == 0 {
                    queue.1 = 0;
                }
                queue.0.remove(i);
                break;
            }
        }
    }
}
