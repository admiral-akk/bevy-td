use bevy::{
    prelude::{Color, EventReader, Query, Res, Transform, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{blueprint::Blueprint, coordinates::Coordinates},
    events::{EnterBuildTarget, HideBuildTarget},
    resources::board::Board,
};

pub fn enter_target(
    board: Res<Board>,
    mut blueprint: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut Coordinates),
        With<Blueprint>,
    >,
    mut enter_target_evr: EventReader<EnterBuildTarget>,
) {
    for event in enter_target_evr.iter() {
        let mut blueprint = blueprint.single_mut();
        let coordinates = event.0.clone();
        blueprint.0.color = Color::rgba(1., 1., 1., 0.5);
        *blueprint.1 = board.transform(&coordinates, 3.);
        *blueprint.2 = coordinates;
    }
}

pub fn hide_blueprint(
    mut blueprint: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut Coordinates),
        With<Blueprint>,
    >,
    mut exit_target_evr: EventReader<HideBuildTarget>,
) {
    for _ in exit_target_evr.iter() {
        let mut blueprint = blueprint.single_mut();
        blueprint.0.color = Color::rgba(1., 1., 1., 0.0);
    }
}
