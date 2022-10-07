use bevy::{
    prelude::{Added, Changed, Query, RemovedComponents, Res, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{coordinates::Coordinates, cursor::Cursor, owned::Owned, projection::Projection},
    resources::board::Board,
};

pub fn added_blueprint(
    mut added: Query<&mut TextureAtlasSprite, (Added<Projection>, With<Owned>)>,
) {
    for mut sprite in added.iter_mut() {
        sprite.color.set_a(0.5);
    }
}
pub fn move_blueprint(
    mut blueprint: Query<
        (&mut Coordinates, &mut TextureAtlasSprite),
        (With<Projection>, With<Owned>),
    >,
    cursor: Query<&Cursor, Changed<Cursor>>,
    board: Res<Board>,
) {
    for cursor in cursor.iter() {
        for (mut coord, mut sprite) in blueprint.iter_mut() {
            if let Some(cursor) = cursor.0 {
                if board.empty(&coord) {
                    sprite.color.set_a(0.5);
                } else {
                    sprite.color.set_a(0.0);
                }
                *coord = cursor;
            } else {
                sprite.color.set_a(0.);
            }
        }
    }
}

pub fn removed_blueprint(
    removed: RemovedComponents<Projection>,
    mut sprites: Query<&mut TextureAtlasSprite, (With<Projection>, With<Owned>)>,
) {
    for e in removed.iter() {
        sprites.get_mut(e).unwrap().color.set_a(1.);
    }
}
