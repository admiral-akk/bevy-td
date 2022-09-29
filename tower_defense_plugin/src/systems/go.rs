use bevy::{
    prelude::{Button, Changed, Color, EventWriter, Query, With},
    ui::{Interaction, UiColor},
};

use crate::{components::go::Go, events::StartWave};

pub fn go(
    mut button: Query<(&mut UiColor, &Interaction), (With<Button>, With<Go>, Changed<Interaction>)>,
    mut start_wave_ewr: EventWriter<StartWave>,
) {
    for (mut color, interaction) in button.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
                start_wave_ewr.send(StartWave);
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

pub fn grey_out(mut button: Query<&mut UiColor, (With<Button>, With<Go>)>) {
    for mut color in button.iter_mut() {
        color.0 = Color::GRAY;
    }
}

pub fn enable(mut button: Query<&mut UiColor, (With<Button>, With<Go>)>) {
    for mut color in button.iter_mut() {
        color.0 = Color::RED;
    }
}
