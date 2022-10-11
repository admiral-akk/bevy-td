use assets_plugin::resources::monsters::{MonsterSprites, MonsterType};
use bevy::prelude::{BuildChildren, Commands, Res};

use crate::{
    bundles::monster_bundle::MonsterBundle, components::coordinates::Coordinates,
    resources::board::Board,
};
pub fn monster_spawn(mut commands: Commands, board: Res<Board>, monsters: Res<MonsterSprites>) {
    for x in 0..5 {
        let coord = Coordinates::new(x, 12);
        commands
            .entity(board.board.unwrap())
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert_bundle(monsters.fetch_sprite_sheet(MonsterType::Jelly))
                    .insert_bundle(MonsterBundle::new(coord, board.transform(&board.start, 4.)));
            });
    }
}
