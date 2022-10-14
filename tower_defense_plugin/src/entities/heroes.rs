use assets_plugin::resources::heroes::{HeroSprites, HeroType};
use bevy::prelude::{BuildChildren, Commands};

use crate::{
    bundles::{hero_bundles::HeroBundle, movement_bundle::MovementBundle},
    components::{coordinates::Coordinates, movements::charging::Charging},
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
    let hero = commands
        .spawn_bundle(HeroBundle::new(
            coord,
            board.transform(&board.start, 4.),
            &[movement],
        ))
        .id();
    let sprite = commands
        .spawn_bundle(heroes.fetch_sprite_sheet(hero_type))
        .id();
    commands.entity(hero).push_children(&[sprite, movement]);
    commands.entity(board.board.unwrap()).add_child(hero);
}
