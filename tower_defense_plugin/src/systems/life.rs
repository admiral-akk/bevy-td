use bevy::prelude::{EventWriter, Query};

use crate::{components::allegiance::Allegiance, events::GameOver};

pub fn check_units(units: Query<&Allegiance>, mut game_over_ewr: EventWriter<GameOver>) {
    let mut has_enemy = false;
    let mut has_friendly = false;
    for unit in units.iter() {
        has_enemy |= unit.eq(&Allegiance(1));
        has_friendly |= unit.eq(&Allegiance(0));
    }
    if !has_friendly {
        game_over_ewr.send(GameOver(true));
    } else if !has_enemy {
        game_over_ewr.send(GameOver(false));
    }
}
