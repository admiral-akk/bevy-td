use assets_plugin::resources::monsters::{MonsterSprites, MonsterType};
use bevy::prelude::{Commands, Res};

use crate::{
    components::coordinates::Coordinates, entities::monster::add_monster, resources::board::Board,
};
pub fn monster_spawn(mut commands: Commands, board: Res<Board>, monsters: Res<MonsterSprites>) {
    for x in 0..5 {
        let coord = Coordinates::new(x, 12);
        add_monster(&mut commands, coord, &board, &monsters, MonsterType::Jelly);
    }
}
