use bevy::prelude::{Added, Entity, Parent, Query};

use crate::components::{action::Action, unit::Unit};

pub fn add_action(
    mut unit: Query<&mut Unit>,
    new_actions: Query<(Entity, &Parent), Added<Action>>,
) {
    for (action, parent) in new_actions.iter() {
        if let Ok(mut unit) = unit.get_mut(parent.get()) {
            unit.actions.push(action);
        }
    }
}
