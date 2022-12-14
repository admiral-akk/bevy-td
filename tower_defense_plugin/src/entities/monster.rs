use assets_plugin::resources::monsters::MonsterType;
use bevy::prelude::{BuildChildren, Commands, Entity};

use crate::{
    bundles::{
        attack_bundle::AttackBundle, monster_bundle::MonsterBundle, movement_bundle::MovementBundle,
    },
    components::{
        attacks::normal::Normal,
        coordinates::Coordinates,
        movements::{cautious::Cautious, charging::Charging},
        on_hits::split::Split,
        targetting::melee::MeleeTarget,
    },
    resources::board::Board,
};

pub fn add_monster(
    commands: &mut Commands,
    coord: Coordinates,
    board: &Board,
    monster_type: MonsterType,
) -> Entity {
    let movement = commands.spawn_bundle(MovementBundle::new(Charging(3))).id();
    let attack = commands
        .spawn_bundle(AttackBundle::new(Normal(1), MeleeTarget))
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
        MonsterType::Jelly => {
            commands.entity(monster).insert(Split);
        }
        _ => {}
    }
    commands.entity(board.board.unwrap()).add_child(monster);
    monster
}
