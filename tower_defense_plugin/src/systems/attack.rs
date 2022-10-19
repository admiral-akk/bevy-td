use bevy::{
    prelude::{
        Children, Commands, Component, Entity, EventReader, EventWriter, Parent, Query, Res,
        Transform, With,
    },
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        allegiance::Allegiance, attacks::attack::Attack, coordinates::Coordinates, health::Health,
        targetting::target::Targets,
    },
    events::{ActiveAction, AttackEvent},
    resources::board::Board,
    structs::board_state::BoardState,
};
pub fn try_attack<T: Attack + Component>(
    mut commands: Commands,
    targetting: Query<(Entity, &Parent, &T, &Targets)>,
    entities: Query<(Entity, &Allegiance, &Health, &Coordinates)>,
    mut active_ert: EventReader<ActiveAction>,
    mut attack_ewr: EventWriter<AttackEvent>,
) {
    for ActiveAction(action) in active_ert.iter() {
        let board_state = BoardState::new(&entities);
        if let Ok((attacking_entity, parent, attack, targets)) = targetting.get(*action) {
            bevy::log::error!("Attacking!");
            if let Some(attacker) = board_state.get(parent.get()) {
                let attacks = attack.priority(attacker, &board_state, targets);
                if let Some(attack) = attacks.get(0) {
                    attack_ewr.send(AttackEvent {
                        attacker: attack.attacker.id,
                        defender: attack.defender.id,
                        damage: attack.damage,
                    });
                }
            }
            commands.entity(attacking_entity).remove::<Targets>();
        }
    }
}

pub fn animate_attack(
    mut attack_ewr: EventReader<AttackEvent>,
    entities: Query<(&Coordinates, &Children)>,
    mut sprites: Query<&mut Transform, With<TextureAtlasSprite>>,
    board: Res<Board>,
) {
    for attack in attack_ewr.iter() {
        let attacker = entities.get(attack.attacker).unwrap();
        let defender = entities.get(attack.defender).unwrap();
        let offset = board.transform(defender.0, 0.).translation
            - board.transform(attacker.0, 0.).translation;
        for child in attacker.1 {
            if let Ok(mut transform) = sprites.get_mut(*child) {
                *transform = transform.with_translation(offset);
            }
        }
    }
}
