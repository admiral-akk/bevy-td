use bevy::prelude::{Commands, Query};

use crate::{
    components::{
        coordinates::Coordinates, health::Health, monster::Monster,
    },
    events::HitEvent,
    resources::board::Board,
};

pub trait OnHit {
    fn apply_effect(
        &self,
        commands: &mut Commands,
        event: HitEvent,
        board: &Board,
        units: Query<(&Coordinates, &Monster, &Health)>,
    );
}
