use assets_plugin::resources::monsters::MonsterType;
use bevy::prelude::{Commands, Res};

use crate::{
    components::coordinates::Coordinates, entities::monster::add_monster, resources::board::Board,
};
pub fn monster_spawn(mut commands: Commands, board: Res<Board>) {
    add_monster(
        &mut commands,
        Coordinates::new(0, 12),
        &board,
        MonsterType::Jelly,
    );
    add_monster(
        &mut commands,
        Coordinates::new(1, 12),
        &board,
        MonsterType::Treant,
    );
    add_monster(
        &mut commands,
        Coordinates::new(2, 12),
        &board,
        MonsterType::Zombie,
    );
    add_monster(
        &mut commands,
        Coordinates::new(3, 12),
        &board,
        MonsterType::Bat,
    );
}
