use bevy::{
    prelude::{Color, EventReader, GlobalTransform, Query, Res, Transform, Vec3, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{blueprint::Blueprint, coordinates::Coordinates, tile::Tile},
    events::{EnterBuildTarget, HideBuildTarget},
    resources::board::Board,
};

pub fn enter_target(
    board: Res<Board>,
    mut blueprint: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut Coordinates),
        (With<Blueprint>),
    >,
    tiles: Query<(&GlobalTransform), (With<GlobalTransform>, With<Tile>)>,
    mut enter_target_evr: EventReader<EnterBuildTarget>,
) {
    for event in enter_target_evr.iter() {
        if let Ok(global_transform) = tiles.get_component::<GlobalTransform>(board.tiles[&event.0])
        {
            let mut blueprint = blueprint.single_mut();
            blueprint.0.color = Color::rgba(1., 1., 1., 0.5);
            blueprint.1.translation = global_transform.translation() + 3. * Vec3::Z;
            *blueprint.2 = event.0.clone();
        }
    }
}

pub fn hide_blueprint(
    mut blueprint: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut Coordinates),
        (With<Blueprint>),
    >,
    mut exit_target_evr: EventReader<HideBuildTarget>,
) {
    for _ in exit_target_evr.iter() {
        let mut blueprint = blueprint.single_mut();
        blueprint.0.color = Color::rgba(1., 1., 1., 0.0);
    }
}
