use bevy::prelude::{Commands, Component, EventReader, Query, Res};

use crate::{
    components::{
        coordinates::Coordinates, health::Health, monster::Monster, on_hits::on_hit::OnHit,
    },
    events::HitEvent,
    resources::board::Board,
};

pub fn on_hit<T: OnHit + Component>(
    mut commands: Commands,
    on_hit: Query<&T>,
    mut hit_ewr: EventReader<HitEvent>,
    units: Query<(&Coordinates, &Monster, &Health)>,
    board: Res<Board>,
) {
    for hit in hit_ewr.iter() {
        if let Ok(proc) = on_hit.get(hit.defender) {
            if let Ok((coord, monster, health)) = units.get(hit.defender) {
                proc.apply_effect(&mut commands, *hit, &board, (*coord, *monster, *health));
            }
        }
    }
}
