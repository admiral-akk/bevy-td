use bevy::{
    prelude::{Button, Changed, Color, Query, With},
    ui::{Interaction, UiColor},
};

pub fn button_color_system(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 1.,
                    alpha: 1.,
                });
            }
            Interaction::Hovered => {
                *color = UiColor(Color::Rgba {
                    red: 0.,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                });
            }
            Interaction::None => {
                *color = UiColor(Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 1.,
                });
            }
        }
    }
}
