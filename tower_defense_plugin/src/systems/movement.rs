use bevy::prelude::{Changed, Query, Res};

use crate::{
    components::{coordinates::Coordinates, movement::Movement, tick_timer::TickTimer},
    resources::board::Board,
};

pub fn movement(
    _board: Res<Board>,
    mut monsters: Query<(&mut Coordinates, &mut TickTimer, &Movement), Changed<TickTimer>>,
) {
    for (mut c, mut timer, movement) in monsters.iter_mut() {
        if timer.active() {
            *c = Coordinates::new(c.x, c.y + movement.0);
        }
    }
}
