use crate::{
    components::{coordinates::Coordinates, power::Power, tower::Tower},
    resources::{board::Board, game_sprites::GameSprites},
};
use bevy::{
    prelude::{Commands, Entity, Name, Res, ResMut, VisibilityBundle},
    sprite::SpriteSheetBundle,
    transform::TransformBundle,
};

fn tower_entity(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: SpriteSheetBundle,
    name: &str,
    power: Power,
) -> Entity {
    let tower = commands
        .spawn()
        .insert(Tower)
        .insert(coord.clone())
        .insert_bundle(sprite_sheet)
        .insert_bundle(TransformBundle {
            local: board.transform(&coord, 4.),
            ..Default::default()
        })
        .insert_bundle(VisibilityBundle {
            ..Default::default()
        })
        .insert(Name::new(name.to_string()))
        .insert(power)
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
    tower_entity(
        commands,
        board,
        coord,
        sprite_sheet.peasant(board.tile_size),
        "Peasant",
        Power(1),
    )
}

pub fn guard_entity(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
) -> Entity {
    tower_entity(
        commands,
        board,
        coord,
        sprite_sheet.guard(board.tile_size),
        "Guard",
        Power(2),
    )
}

pub fn soldier_entity(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
) -> Entity {
    tower_entity(
        commands,
        board,
        coord,
        sprite_sheet.soldier(board.tile_size),
        "Soldier",
        Power(3),
    )
}
