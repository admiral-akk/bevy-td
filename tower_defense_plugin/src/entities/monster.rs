use assets_plugin::resources::monsters::{MonsterSprites, MonsterType};
use bevy::prelude::{BuildChildren, Commands, Res};

use crate::{
    bundles::{monster_bundle::MonsterBundle, movement_bundle::MovementBundle},
    components::{coordinates::Coordinates, movements::charging::Charging},
    resources::board::Board,
};

pub fn add_monster(
    commands: &mut Commands,
    coord: Coordinates,
    board: &Board,
    monsters: &Res<MonsterSprites>,
    monster_type: MonsterType,
) {
    let movement = commands.spawn_bundle(MovementBundle::new(Charging(1))).id();
    let monster = commands
        .spawn_bundle(MonsterBundle::new(
            coord,
            board.transform(&board.start, 4.),
            &[movement],
        ))
        .id();
    let sprite = commands
        .spawn_bundle(monsters.fetch_sprite_sheet(monster_type))
        .id();
    commands.entity(monster).push_children(&[sprite, movement]);
    commands.entity(board.board.unwrap()).add_child(monster);
}
