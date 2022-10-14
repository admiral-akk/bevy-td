use bevy::prelude::{Commands};

use crate::{
    components::{coordinates::Coordinates, health::Health, monster::Monster},
    events::HitEvent,
    resources::board::Board,
};

pub trait OnHit {
    fn apply_effect(
        &self,
        commands: &mut Commands,
        event: HitEvent,
        board: &Board,
        defender: (Coordinates, Monster, Health),
    );
}
