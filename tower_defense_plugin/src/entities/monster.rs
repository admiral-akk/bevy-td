use assets_plugin::resources::monsters::{MonsterType};
use bevy::prelude::{BuildChildren, Commands};

use crate::{
    bundles::{
        attack_bundle::AttackBundle, monster_bundle::MonsterBundle, movement_bundle::MovementBundle,
    },
    components::{
        attacks::melee::MeleeAttack,
        coordinates::Coordinates,
        movements::{cautious::Cautious, charging::Charging},
    },
    resources::board::Board,
};

pub fn add_monster(
    commands: &mut Commands,
    coord: Coordinates,
    board: &Board,
    monster_type: MonsterType,
) {
    let movement = commands.spawn_bundle(MovementBundle::new(Charging(3))).id();
    let attack = commands
        .spawn_bundle(AttackBundle::new(MeleeAttack(1)))
        .id();
    let monster = commands
        .spawn_bundle(MonsterBundle::new(
            coord,
            board.transform(&board.start, 4.),
            monster_type,
        ))
        .id();
    commands.entity(monster).push_children(&[movement, attack]);
    match monster_type {
        MonsterType::Bat => {
            let movement = commands
                .spawn_bundle(MovementBundle::new(Cautious(2, 3)))
                .id();
            commands.entity(monster).push_children(&[movement]);
        }
        _ => {}
    }
    commands.entity(board.board.unwrap()).add_child(monster);
}
