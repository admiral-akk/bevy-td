

use bevy::{
    prelude::{Bundle, Component},
};

use crate::components::{
    action::Action,
    movements::{movement::Movement},
};

#[derive(Bundle, Default)]
pub struct AttackBundle<T: Movement + Component> {
    action: Action,
    movement: T,
}

impl<T: Movement + Component> AttackBundle<T> {
    pub fn new(movement: T) -> Self {
        AttackBundle {
            action: Action,
            movement,
        }
    }
}
