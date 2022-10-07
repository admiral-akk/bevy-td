use bevy::{
    prelude::{
        BuildChildren, Commands, Res, ResMut,
    },
};

use crate::{
    bundles::monster_bundle::MonsterBundle,
    resources::{board::Board, game_sprites::GameSprites},
};
pub fn monster_spawn(
    mut commands: Commands,
    mut board: ResMut<Board>,
    spritesheet: Res<GameSprites>,
) {
    for _ in 0..5 {
        let coord = board.monster_spawn();
        let monster = commands
            .entity(board.board.unwrap())
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert_bundle(MonsterBundle::new(coord, board.transform(&board.start, 4.)))
                    .insert_bundle(spritesheet.monster(board.tile_size));
            })
            .id();
        board.monsters.insert(coord, monster);
    }
}
