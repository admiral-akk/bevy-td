use crate::{
    components::{coordinates::Coordinates, tower::Tower},
    resources::{board::Board, game_sprites::GameSprites},
};
use bevy::{
    prelude::{Bundle, Commands, Entity, Name, Res, ResMut, VisibilityBundle},
    sprite::SpriteSheetBundle,
    transform::TransformBundle,
};

#[derive(Bundle)]
pub struct TowerBundle {
    tower: Tower,
    coords: Coordinates,
    name: Name,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

fn tower_entity(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
    name: &str,
) -> Entity {
    let tower = commands
        .spawn()
        .insert(Tower)
        .insert(coord.clone())
        .insert_bundle(sprite_sheet.peasant(board.tile_size))
        .insert_bundle(TransformBundle {
            local: board.transform(&coord, 4.),
            ..Default::default()
        })
        .insert_bundle(VisibilityBundle {
            ..Default::default()
        })
        .insert(Name::new(name.to_string()))
        .id();
    board.towers.insert(*coord, tower);
    tower
}

pub fn peasant_entity(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
) -> Entity {
    tower_entity(commands, board, coord, sprite_sheet, "Peasant")
}
