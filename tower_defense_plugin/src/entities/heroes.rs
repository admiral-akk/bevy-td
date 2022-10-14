use assets_plugin::resources::heroes::{HeroSprites, HeroType};
use bevy::prelude::{BuildChildren, Commands, Res};

use crate::{
    bundles::hero_bundles::HeroBundle, components::coordinates::Coordinates,
    resources::board::Board,
};

pub fn add_hero(
    mut commands: Commands,
    coord: Coordinates,
    board: Res<Board>,
    heroes: Res<HeroSprites>,
    hero_type: HeroType,
) {
    let hero = commands
        .spawn_bundle(HeroBundle::new(coord, board.transform(&board.start, 4.)))
        .id();
    let sprite = commands
        .spawn_bundle(heroes.fetch_sprite_sheet(hero_type))
        .id();
    commands.entity(hero).add_child(sprite);
    commands.entity(board.board.unwrap()).add_child(hero);
}
