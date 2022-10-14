use assets_plugin::resources::heroes::{HeroSprites, HeroType};
use bevy::prelude::{BuildChildren, Commands};

use crate::{
    bundles::{
        attack_bundle::AttackBundle, hero_bundles::HeroBundle, movement_bundle::MovementBundle,
    },
    components::{
        attacks::melee::MeleeAttack, coordinates::Coordinates, movements::charging::Charging,
    },
    resources::board::Board,
};

pub fn add_hero(
    commands: &mut Commands,
    coord: Coordinates,
    board: &Board,
    heroes: &HeroSprites,
    hero_type: HeroType,
) {
    let movement = commands.spawn_bundle(MovementBundle::new(Charging(1))).id();
    let attack = commands
        .spawn_bundle(AttackBundle::new(MeleeAttack(1)))
        .id();
    let hero = commands
        .spawn_bundle(HeroBundle::new(coord, board.transform(&board.start, 4.)))
        .id();
    let sprite = commands
        .spawn_bundle(heroes.fetch_sprite_sheet(hero_type))
        .id();
    commands
        .entity(hero)
        .push_children(&[sprite, movement, attack]);
    commands.entity(board.board.unwrap()).add_child(hero);
}
