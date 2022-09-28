use bevy::{
    prelude::{Button, Changed, Color, EventWriter, Query, With},
    ui::{Interaction, UiColor},
};

use crate::{components::new_game::NewGame, events::StartGame};

pub fn new_game(
    mut start_button: Query<
        (&Interaction, &mut UiColor),
        (With<Button>, With<NewGame>, Changed<Interaction>),
    >,
    mut start_game_ewr: EventWriter<StartGame>,
) {
    for (interaction, mut color) in start_button.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
                start_game_ewr.send(StartGame);
            }
            Interaction::Hovered => {
                color.0 = Color::GREEN;
            }
            _ => {
                color.0 = Color::RED;
            }
        }
    }
}
