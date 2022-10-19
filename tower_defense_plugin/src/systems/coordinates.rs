use bevy::{
    prelude::{
        Added, Changed, Children, Entity, Query, RemovedComponents, ResMut, Transform,
        With, Without,
    },
    sprite::{TextureAtlasSprite},
};

use crate::{
    components::{coordinates::Coordinates, health_bar::HealthBar, start::Start},
    resources::board::Board,
};

pub fn added(
    mut added: Query<(Entity, &Coordinates, &mut Transform), Added<Coordinates>>,
    mut board: ResMut<Board>,
) {
    for (entity, coord, mut transform) in added.iter_mut() {
        let tran = board.transform(coord, transform.translation.z);
        *transform = transform.with_translation(tran.translation);
        board.entities.insert(*coord, entity);
    }
}

pub fn updated(
    mut updated: Query<
        (Entity, &Coordinates, &mut Transform, &Children),
        (Changed<Coordinates>, Without<TextureAtlasSprite>),
    >,
    mut sprites: Query<&mut Transform, (With<TextureAtlasSprite>, Without<HealthBar>)>,
    mut board: ResMut<Board>,
) {
    for (entity, coord, mut transform, children) in updated.iter_mut() {
        let tran = board.transform(coord, transform.translation.z);
        if let Some(entity) = children.iter().find(|c| sprites.contains(**c)) {
            if let Ok(mut sprite_transform) = sprites.get_mut(*entity) {
                *sprite_transform = sprite_transform.with_translation(
                    sprite_transform.translation - (tran.translation - transform.translation),
                );
            }
        }
        *transform = transform.with_translation(tran.translation);
        board.entities.update_key(&entity, *coord);
    }
}

pub fn removed(removed: RemovedComponents<Coordinates>, mut board: ResMut<Board>) {
    for removed in removed.iter() {
        bevy::log::error!("Attempt removed coordinates!");
        board.entities.remove_value(&removed);
    }
}

pub fn return_to_start(mut units: Query<(&mut Coordinates, &Start)>) {
    for (mut coord, start) in units.iter_mut() {
        *coord = start.0;
    }
}
