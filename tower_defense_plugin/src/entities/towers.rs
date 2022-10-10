use crate::{
    components::{
        allegiance::Allegiance, coordinates::Coordinates, health::Health, movement::Movement,
        power::Power, start::Start, tower::Tower, unit::Unit,
    },
    resources::{board::Board, game_sprites::GameSprites},
};
use bevy::{
    prelude::{BuildChildren, Color, Commands, Name, Res, VisibilityBundle},
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
    board: &mut Board,
    coord: &Coordinates,
    sprite_sheet: &Res<GameSprites>,
    tower: TowerType,
) {
    match tower {
        TowerType::None => {}
        TowerType::Peasant => tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.peasant(board.tile_size),
            "Peasant",
            Power(1),
        ),
        TowerType::Guard => tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.guard(board.tile_size),
            "Guard",
            Power(2),
        ),
        TowerType::Soldier => tower_entity(
            commands,
            board,
            coord,
            sprite_sheet.soldier(board.tile_size),
            "Soldier",
            Power(3),
        ),
    };
}

fn tower_entity(
    commands: &mut Commands,
    board: &mut Board,
    coord: &Coordinates,
    sprite_sheet: SpriteSheetBundle,
    name: &str,
    power: Power,
) {
    commands
        .entity(board.board.unwrap())
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Tower)
                .insert(Movement(1))
                .insert(Health(40))
                .insert(Allegiance(0))
                .insert(Start(coord.clone()))
                .insert(coord.clone())
                .insert_bundle(sprite_sheet)
                .insert(Unit)
                .insert_bundle(TransformBundle {
                    local: board.transform(&coord, 4.),
                    ..Default::default()
                })
                .insert_bundle(VisibilityBundle {
                    ..Default::default()
                })
                .insert(Name::new(name.to_string()))
                .insert(power);
        });
}
