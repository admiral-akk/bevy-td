use bevy::prelude::{EventWriter, Res};

use crate::{events::GameOver, resources::life_tracker::LifeTracker};

pub fn check_lives(life: Res<LifeTracker>, mut game_over_ewr: EventWriter<GameOver>) {
    if life.0 <= 0 {
        game_over_ewr.send(GameOver);
    }
}
