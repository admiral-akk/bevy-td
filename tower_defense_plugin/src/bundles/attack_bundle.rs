use bevy::prelude::{Bundle, Component};

use crate::components::{action::Action, attacks::attack::Attack, targetting::target::Target};

#[derive(Bundle, Default)]
pub struct AttackBundle<AttackType: Attack + Component, TargetType: Target + Component> {
    action: Action,
    attack: AttackType,
    target: TargetType,
}

impl<AttackType: Attack + Component, TargetType: Target + Component>
    AttackBundle<AttackType, TargetType>
{
    pub fn new(attack: AttackType, target: TargetType) -> Self {
        AttackBundle {
            action: Action,
            attack,
            target,
        }
    }
}
