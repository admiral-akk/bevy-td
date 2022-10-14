use assets_plugin::resources::monsters::{MonsterSprites, MonsterType};
use bevy::prelude::{BuildChildren, Commands, Res};

use crate::{
    bundles::monster_bundle::MonsterBundle, components::coordinates::Coordinates,
    resources::board::Board,
};

pub fn add_monster(
    commands: &mut Commands,
    coord: Coordinates,
    board: &Res<Board>,
    monsters: &Res<MonsterSprites>,
    monster_type: MonsterType,
) {
    let monster = commands
        .spawn_bundle(MonsterBundle::new(coord, board.transform(&board.start, 4.)))
        .id();
    let sprite = commands
        .spawn_bundle(monsters.fetch_sprite_sheet(monster_type))
        .id();
    commands.entity(monster).add_child(sprite);
    commands.entity(board.board.unwrap()).add_child(monster);
}
