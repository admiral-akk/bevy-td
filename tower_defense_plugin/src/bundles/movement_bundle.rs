

use bevy::{
    prelude::{Bundle, Component},
};

use crate::components::{
    action::Action,
    movements::{movement::Movement},
};

#[derive(Bundle, Default)]
pub struct MovementBundle<T: Movement + Component> {
    action: Action,
    movement: T,
}

impl<T: Movement + Component> MovementBundle<T> {
    pub fn new(movement: T) -> Self {
        MovementBundle {
            action: Action,
            movement,
        }
    }
}
