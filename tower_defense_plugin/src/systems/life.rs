use bevy::{
    prelude::{EventWriter, Query, Res, With},
    text::Text,
};

use crate::{components::lives::Lives, events::GameOver, resources::life_tracker::LifeTracker};

pub fn check_lives(life: Res<LifeTracker>, mut game_over_ewr: EventWriter<GameOver>) {
    if life.0 <= 0 {
        game_over_ewr.send(GameOver);
    }
}

pub fn update_lives(life: Res<LifeTracker>, mut life_ui: Query<&mut Text, With<Lives>>) {
    life_ui.single_mut().sections[0].value = format!("Lives: {}", life.0);
}
