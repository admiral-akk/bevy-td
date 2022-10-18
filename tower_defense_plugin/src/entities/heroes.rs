use assets_plugin::resources::heroes::HeroType;
use bevy::prelude::{BuildChildren, Commands};

use crate::{
    bundles::{
        attack_bundle::AttackBundle, hero_bundles::HeroBundle, movement_bundle::MovementBundle,
    },
    components::{
        attacks::{melee::Normal, normal::Backstab},
        auras::root::RootAura,
        coordinates::Coordinates,
        movements::{charging::Charging, cowardly::Cowardly},
        targetting::melee::MeleeTarget,
    },
    resources::board::Board,
};

pub fn add_hero(commands: &mut Commands, coord: Coordinates, board: &Board, hero_type: HeroType) {
    let movement = match hero_type {
        HeroType::Rogue => commands.spawn_bundle(MovementBundle::new(Cowardly(1))).id(),
        _ => commands.spawn_bundle(MovementBundle::new(Charging(1))).id(),
    };
    let attack = match hero_type {
        HeroType::Rogue => commands
            .spawn_bundle(AttackBundle::new(Backstab::new(1, 5), MeleeTarget))
            .id(),
        _ => commands
            .spawn_bundle(AttackBundle::new(Normal(1), MeleeTarget))
            .id(),
    };
    let hero = commands
        .spawn_bundle(HeroBundle::new(
            coord,
            board.transform(&board.start, 4.),
            hero_type,
        ))
        .id();
    commands.entity(hero).push_children(&[movement, attack]);

    match hero_type {
        HeroType::Paladin => {
            commands.entity(hero).insert(RootAura(5));
        }
        _ => {}
    };
    commands.entity(board.board.unwrap()).add_child(hero);
}
