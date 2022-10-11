use assets_plugin::resources::fonts::Fonts;
use bevy::prelude::{Changed, Component, EventWriter};
use bevy::{
    prelude::{
        BuildChildren, Button, ButtonBundle, Color, Commands, Entity, Query, Res, TextBundle, With,
    },
    text::{Text, TextAlignment, TextSection, TextStyle},
    ui::{
        AlignContent, AlignItems, AlignSelf, FlexDirection, Interaction, Size, Style, UiColor, Val,
    },
};

use crate::plugins::events::Reward;

#[derive(Component)]
pub struct RewardButton(pub u32);

pub fn handle_reward(
    mut button: Query<
        (&mut UiColor, &Interaction),
        (With<Button>, With<RewardButton>, Changed<Interaction>),
    >,
    mut reward_ewr: EventWriter<Reward>,
) {
    for (mut color, interaction) in button.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
                reward_ewr.send(Reward);
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

pub fn reward_button(commands: &mut Commands, fonts: &Res<Fonts>) -> Entity {
    let button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Px(130.),
                    height: Val::Px(130.),
                },
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: UiColor(Color::GRAY),
            ..Default::default()
        })
        .insert(RewardButton(1))
        .id();
    let text = commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "1!".to_string(),
                    style: TextStyle {
                        font: fonts.get_handle(),
                        font_size: 128.,
                        ..Default::default()
                    },
                }],
                alignment: TextAlignment::CENTER,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.entity(button).add_child(text);
    button
}
