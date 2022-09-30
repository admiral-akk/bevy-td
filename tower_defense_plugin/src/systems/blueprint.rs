use bevy::{
    prelude::{Changed, Color, Query, Res, Transform, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{blueprint::Blueprint, coordinates::Coordinates, cursor::Cursor},
    resources::board::Board,
};

pub fn enter_target(
    board: Res<Board>,
    mut blueprint: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut Coordinates),
        With<Blueprint>,
    >,
    cursor: Query<&Cursor, Changed<Cursor>>,
) {
    for change in cursor.iter() {
        if let Some(coord) = change.0 {
            let mut blueprint = blueprint.single_mut();
            blueprint.0.color = Color::rgba(1., 1., 1., 0.5);
            *blueprint.1 = board.transform(&coord, 3.);
            *blueprint.2 = coord.clone();
        } else {
            let mut blueprint = blueprint.single_mut();
            blueprint.0.color = Color::rgba(1., 1., 1., 0.0);
        }
    }
}
