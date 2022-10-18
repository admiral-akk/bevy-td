use bevy::prelude::Entity;

pub struct ProposedAttack {
    pub damage: i32,
    pub attacker: Entity,
    pub defender: Entity,
}

pub struct AttackPriority {
    pub priority: Vec<ProposedAttack>,
}
