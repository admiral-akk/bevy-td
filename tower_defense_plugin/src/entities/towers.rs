use crate::{
    components::{coordinates::Coordinates, power::Power, tower::Tower},
    resources::{board::Board, game_sprites::GameSprites},
};
use bevy::{
    prelude::{Color, Commands, Entity, Name, Res, ResMut, VisibilityBundle},
    sprite::SpriteSheetBundle,
    transform::TransformBundle,
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TowerType {
    #[default]
    None,
    Peasant,
    Guard,
    Soldier,
}

pub fn get_blueprint(
    board: &Board,
    sprite_sheet: &Res<GameSprites>,
    tower: TowerType,
) -> SpriteSheetBundle {
    let mut bundle = match tower {
        TowerType::Peasant => sprite_sheet.peasant(board.tile_size),
        TowerType::Guard => sprite_sheet.guard(board.tile_size),
        TowerType::Soldier => sprite_sheet.soldier(board.tile_size),
        _ => SpriteSheetBundle::default(),
    };
    bundle.sprite.color = Color::rgba(1., 1., 1., 0.5);
    bundle
}

pub fn get_tower(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
    tower: TowerType,
) -> Option<Entity> {
    match tower {
        TowerType::None => None,
        TowerType::Peasant => Some(tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.peasant(board.tile_size),
            "Peasant",
            Power(1),
        )),
        TowerType::Guard => Some(tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.guard(board.tile_size),
            "Guard",
            Power(2),
        )),
        TowerType::Soldier => Some(tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.soldier(board.tile_size),
            "Soldier",
            Power(3),
        )),
    }
}

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
