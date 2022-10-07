use bevy::prelude::{Commands, Res, ResMut};

use crate::{
    components::coordinates::Coordinates,
    entities::towers::{get_tower, TowerType},
    resources::{board::Board, game_sprites::GameSprites},
};

pub fn spawn_reward(
    mut commands: Commands,
    mut board: ResMut<Board>,
    spritesheets: Res<GameSprites>,
) {
    let spawn = Coordinates::new(0, 0);
    get_tower(
        &mut commands,
        &mut board,
        &spawn,
        &spritesheets,
        TowerType::Guard,
    );
}
