use bevy::prelude::{Component, EventReader, Parent, Query, Res};

use crate::{
    components::{allegiance::Allegiance, coordinates::Coordinates, on_hits::on_hit::OnHit},
    events::ActiveAction,
    resources::board::Board,
};

pub fn on_hit<T: OnHit + Component>(
    _entities: Query<(&mut Coordinates, &Allegiance)>,
    _action_ewr: EventReader<ActiveAction>,
    _actions: Query<(&Parent, &T)>,
    _board: Res<Board>,
) {
}
