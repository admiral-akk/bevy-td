use crate::{
    components::{
        allegiance::Allegiance, coordinates::Coordinates, health::Health, movement::Movement,
        power::Power, start::Start, tower::Tower, unit::Unit,
    },
    resources::{board::Board, game_sprites::GameSprites},
};
use assets_plugin::resources::heroes::{HeroSprites, HeroTypes};
use bevy::{
    prelude::{BuildChildren, Commands, Name, Res, Vec3, VisibilityBundle},
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

pub fn get_tower(
    commands: &mut Commands,
    board: &Board,
    coord: &Coordinates,
    _sprite_sheet: &Res<GameSprites>,
    tower: TowerType,
    hero_sprites: &Res<HeroSprites>,
) {
    match tower {
        TowerType::None => {}
        TowerType::Peasant => tower_entity(
            commands,
            board,
            coord,
            hero_sprites.fetch_sprite_sheet(HeroTypes::Shepard),
            "Peasant",
            Power(1),
        ),
        TowerType::Guard => tower_entity(
            commands,
            board,
            coord,
            hero_sprites.fetch_sprite_sheet(HeroTypes::Rogue),
            "Guard",
            Power(2),
        ),
        TowerType::Soldier => tower_entity(
            commands,
            board,
            coord,
            hero_sprites.fetch_sprite_sheet(HeroTypes::Assassin),
            "Soldier",
            Power(3),
        ),
    };
}

fn tower_entity(
    commands: &mut Commands,
    board: &Board,
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
                .insert(Health::new(40))
                .insert(Allegiance(0))
                .insert(Start(coord.clone()))
                .insert(coord.clone())
                .insert_bundle(sprite_sheet)
                .insert(Unit)
                .insert_bundle(TransformBundle {
                    local: board.transform(&coord, 4.).with_scale(Vec3::splat(2.)),
                    ..Default::default()
                })
                .insert_bundle(VisibilityBundle {
                    ..Default::default()
                })
                .insert(Name::new(name.to_string()))
                .insert(power);
        });
}
