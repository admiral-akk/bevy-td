use crate::{components::main_menu::MainMenu, events::ToMainMenu};
use bevy::{
    prelude::{Button, Changed, Color, EventWriter, Query, With},
    ui::{Interaction, UiColor},
};

pub fn main_menu(
    mut start_button: Query<
        (&Interaction, &mut UiColor),
        (With<Button>, With<MainMenu>, Changed<Interaction>),
    >,
    mut start_game_ewr: EventWriter<ToMainMenu>,
) {
    for (interaction, mut color) in start_button.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
                start_game_ewr.send(ToMainMenu);
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
