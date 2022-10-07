use bevy::prelude::{BuildChildren, Commands, Res};

use crate::{
    bundles::monster_bundle::MonsterBundle,
    components::coordinates::Coordinates,
    resources::{board::Board, game_sprites::GameSprites},
};
pub fn monster_spawn(mut commands: Commands, board: Res<Board>, spritesheet: Res<GameSprites>) {
    for x in 0..5 {
        let coord = Coordinates::new(x, 12);
        commands
            .entity(board.board.unwrap())
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert_bundle(MonsterBundle::new(coord, board.transform(&board.start, 4.)))
                    .insert_bundle(spritesheet.monster(board.tile_size));
            });
    }
}
