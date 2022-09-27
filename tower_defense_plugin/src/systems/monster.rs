use bevy::prelude::{Entity, EventReader, Query, Res, Transform, Vec3, With};

use crate::{
    components::{coordinates::Coordinates, monster::Monster},
    events::Move,
    resources::board::Board,
};

pub fn monster_move(
    board: Res<Board>,
    mut move_evr: EventReader<Move>,
    mut monsters: Query<(&mut Transform, &mut Coordinates), With<Monster>>,
) {
    for _ in move_evr.iter() {
        for (mut t, mut c) in monsters.iter_mut() {
            *c = board.next(&c);
            *t = board.transform(&c, 4.);
        }
    }
}
