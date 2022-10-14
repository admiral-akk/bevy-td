use bevy::prelude::{Bundle, Component};

use crate::components::{action::Action, attacks::attack::Attack};

#[derive(Bundle, Default)]
pub struct AttackBundle<T: Attack + Component> {
    action: Action,
    attack: T,
}

impl<T: Attack + Component> AttackBundle<T> {
    pub fn new(attack: T) -> Self {
        AttackBundle {
            action: Action,
            attack,
        }
    }
}
