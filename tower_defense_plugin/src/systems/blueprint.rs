use bevy::{
    prelude::{Color, EventReader, GlobalTransform, Query, Res, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::coordinates::Coordinates,
    events::{EnterBuildTarget, ExitBuildTarget},
    resources::board::Board,
};

pub fn enter_target(
    board: Res<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite), (With<Coordinates>, With<TextureAtlasSprite>)>,
    mut enter_target_evr: EventReader<EnterBuildTarget>,
) {
    for event in enter_target_evr.iter() {
        if let Ok(mut texture) =
            tiles.get_component_mut::<TextureAtlasSprite>(board.tiles[&event.0])
        {
            texture.color = Color::rgb(0.5, 0.5, 0.5);
        }
    }
}

pub fn exit_target(
    board: Res<Board>,
    mut tiles: Query<
        (&Coordinates, &GlobalTransform, &mut TextureAtlasSprite),
        (
            With<GlobalTransform>,
            With<Coordinates>,
            With<TextureAtlasSprite>,
        ),
    >,
    mut exit_target_evr: EventReader<ExitBuildTarget>,
) {
    for event in exit_target_evr.iter() {
        if let Ok(mut texture) =
            tiles.get_component_mut::<TextureAtlasSprite>(board.tiles[&event.0])
        {
            texture.color = Color::rgb(1., 1., 1.);
        }
    }
}
