use bevy::prelude::{Changed, Query, Res, Transform};

use crate::{components::coordinates::Coordinates, resources::board::Board};

pub fn update_transform(
    mut updated_coordinates: Query<(&Coordinates, &mut Transform), Changed<Coordinates>>,
    board: Res<Board>,
) {
    for (coord, mut transform) in updated_coordinates.iter_mut() {
        *transform = board.transform(coord, transform.translation.z);
    }
}
